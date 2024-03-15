use std::ops::Neg;

use crate::expand::expand;
use crate::lexer::Delim;
use crate::lexer::{lex, Binary, Delim::*, Token, Token::BinOp, Token::Delimiter, Unary};
use crate::{ast, ast::AssignExpr::*, ast::Expr, ast::Statement};

// use serde::de;
use winnow::ascii::{alpha1, alphanumeric0};
use winnow::combinator::{fail, opt, separated};
use winnow::prelude::*;
use winnow::{
    ascii::{float, multispace0 as multispaces},
    combinator::{alt, delimited, repeat},
    token::{one_of, take},
};

pub fn parse(input: &mut &str) -> Statement {
    let mut i_str: &str = &expand(input);
    program(&mut i_str).unwrap()
}

fn program(i: &mut &[Token]) -> PResult<Statement> {
    separated(1.., statement, alt((",", ";")))
        .map(Statement::Sequence)
        .parse_next(i)
}

fn statement(i: &mut &[Token]) -> PResult<Statement> {
    let body = opt(alt((
        assign_array,
        assign_add,
        assign_sub,
        assign_mul,
        assign_div,
        assign_inc,
        assign_dec,
        assign_scalar,
        return_statement,
        program,
    )));
    let result =
        delimited(one_of(Delimiter(LBrace)), body, one_of(Delimiter(RBrace))).parse_next(i)?;
    match result {
        Some(s) => Ok(s),
        None => Ok(Statement::Empty),
    }
}

fn ternary(i: &mut &[Token]) -> PResult<Expr> {
    let condition = logic.parse_next(i)?;
    delimited(multispaces, "?", multispaces).parse_next(i)?;
    let if_true = expr.parse_next(i)?;
    delimited(multispaces, ":", multispaces).parse_next(i)?;
    let if_false = expr.parse_next(i)?;
    Ok(Expr::TernaryOp(
        Box::new(condition),
        Box::new(if_true),
        Box::new(if_false),
    ))
}

fn expr(i: &mut &[Token]) -> PResult<Expr> {
    alt((ternary, logic)).parse_next(i)
}

fn comp(i: &mut &[Token]) -> PResult<Expr> {
    use BinOp::*;
    let lhs = sum.parse_next(i)?;
    let r = opt((alt((gt, ge, lt, le, equal)), sum)).parse_next(i)?;
    let r = match r {
        Some(r) => r,
        None => return Ok(lhs),
    };
    match r.0 {
        "==" => Ok(Expr::BinaryOp(Eq(Box::new(lhs), Box::new(r.1)))),
        ">=" => Ok(Expr::BinaryOp(GreaterEq(Box::new(lhs), Box::new(r.1)))),
        "<=" => Ok(Expr::BinaryOp(LessEq(Box::new(lhs), Box::new(r.1)))),
        ">" => Ok(Expr::BinaryOp(Greater(Box::new(lhs), Box::new(r.1)))),
        "<" => Ok(Expr::BinaryOp(Less(Box::new(lhs), Box::new(r.1)))),
        _ => panic!("Unknown operator: {}", r.0),
    }
}

fn logic(i: &mut &[Token]) -> PResult<Expr> {
    use BinOp::*;
    let init = comp.parse_next(i)?;

    repeat(0.., (alt((and, or)), comp))
        .fold(
            move || init.clone(),
            |acc, (op, val): (&str, Expr)| {
                if op == "&&" {
                    Expr::BinaryOp(And(Box::new(acc), Box::new(val)))
                } else {
                    Expr::BinaryOp(Or(Box::new(acc), Box::new(val)))
                }
            },
        )
        .parse_next(i)
}

fn sum(i: &mut &[Token]) -> PResult<Expr> {
    use ast::BinOp::*;
    let init = product.parse_next(i)?;

    repeat(
        0..,
        (
            one_of([Token::BinOp(Binary::Add), Token::BinOp(Binary::Sub)]),
            product,
        ),
    )
    .fold(
        move || init.clone(),
        |acc, (op, val): (Token, Expr)| {
            if op == Token::BinOp(Binary::Add) {
                Expr::BinaryOp(Add(Box::new(acc), Box::new(val)))
            } else {
                Expr::BinaryOp(Sub(Box::new(acc), Box::new(val)))
            }
        },
    )
    .parse_next(i)
}

fn product(i: &mut &[Token]) -> PResult<Expr> {
    use ast::BinOp::*;
    let init = power.parse_next(i)?;

    repeat(
        0..,
        (
            one_of([Token::BinOp(Binary::Mul), Token::BinOp(Binary::Div)]),
            power,
        ),
    )
    .fold(
        move || init.clone(),
        |acc, (op, val): (Token, Expr)| {
            if op == Token::BinOp(Binary::Mul) {
                Expr::BinaryOp(Mul(Box::new(acc), Box::new(val)))
            } else {
                Expr::BinaryOp(Div(Box::new(acc), Box::new(val)))
            }
        },
    )
    .parse_next(i)
}

fn power(i: &mut &[Token]) -> PResult<Expr> {
    use ast::BinOp::*;
    let lhs = factor.parse_next(i)?;
    let r = opt((one_of(Token::BinOp(Binary::Pow)), factor)).parse_next(i)?;
    match r {
        Some(r) => Ok(Expr::BinaryOp(Pow(Box::new(lhs), Box::new(r.1)))),
        None => Ok(lhs),
    }
}

fn factor(i: &mut &[Token]) -> PResult<Expr> {
    let negation = opt(one_of(Token::UnOp(Unary::Neg))).map(|op| op.is_some());
    let expression = alt((float.map(Expr::Scalar), assign, function, variable, parens));
    (negation, expression)
        .map(|(n, e)| if n { Expr::Negate(Box::new(e)) } else { e })
        .parse_next(i)
}

fn assign(i: &mut &[Token]) -> PResult<Expr> {
    let var = identifier.parse_next(i)?;
    let op = alt((
        one_of(Token::UnOp(Unary::Inc)),
        one_of(Token::UnOp(Unary::Dec)),
    ))
    .parse_next(i)?;
    match op {
        Token::UnOp(Unary::Inc) => Ok(Expr::Assign(Inc(var))),
        Token::UnOp(Unary::Dec) => Ok(Expr::Assign(Dec(var))),
        _ => unreachable!(),
    }
}

fn parens(i: &mut &[Token]) -> PResult<Expr> {
    delimited(one_of(Delimiter(LParen)), expr, one_of(Delimiter(RParen)))
        .map(|e| Expr::Paren(Box::new(e)))
        .parse_next(i)
}

fn assign_scalar(i: &mut &[Token]) -> PResult<Statement> {
    let var = identifier.parse_next(i)?;
    one_of(Token::BinOp(Binary::Assign)).parse_next(i)?;
    let rhs = expr.parse_next(i)?;
    Ok(Statement::Assign {
        var,
        rhs: Box::new(rhs),
    })
}

fn assign_array(i: &mut &[Token]) -> PResult<Statement> {
    let vars: Vec<String> = delimited(
        one_of(Token::Delimiter(Delim::LBracket)),
        separated(1.., identifier, one_of(Token::Delimiter(Delim::Comma))),
        one_of(Token::Delimiter(Delim::RBracket)),
    )
    .parse_next(i)?;
    let expression = expr.parse_next(i)?;
    Ok(Statement::AssignArray {
        vars,
        rhs: Box::new(expression),
    })
}

fn assign_add(i: &mut &[Token]) -> PResult<Statement> {
    use crate::ast::BinOp::*; // Not sure why this is needed, otherwise it complains about Add not being in scope.
    let var = identifier.parse_next(i)?;
    one_of(Token::BinOp(Binary::AssignAdd)).parse_next(i)?;
    let rhs = expr.parse_next(i)?;
    Ok(Statement::Assign {
        var: var.clone(),
        rhs: Box::new(Expr::BinaryOp(Add(
            Box::new(Expr::Variable(var)),
            Box::new(rhs),
        ))),
    })
}

fn assign_sub(i: &mut &[Token]) -> PResult<Statement> {
    use crate::ast::BinOp::*; // Not sure why this is needed, otherwise it complains about Sub not being in scope.
    let var = identifier.parse_next(i)?;
    one_of(Token::BinOp(Binary::AssignSub)).parse_next(i)?;
    let rhs = expr.parse_next(i)?;
    Ok(Statement::Assign {
        var: var.clone(),
        rhs: Box::new(Expr::BinaryOp(Sub(
            Box::new(Expr::Variable(var)),
            Box::new(rhs),
        ))),
    })
}

fn assign_mul(i: &mut &[Token]) -> PResult<Statement> {
    use crate::ast::BinOp::*; // Not sure why this is needed, otherwise it complains about Mul not being in scope.
    let var = identifier.parse_next(i)?;
    one_of(Token::BinOp(Binary::AssignMul)).parse_next(i)?;
    let rhs = expr.parse_next(i)?;
    Ok(Statement::Assign {
        var: var.clone(),
        rhs: Box::new(Expr::BinaryOp(Mul(
            Box::new(Expr::Variable(var)),
            Box::new(rhs),
        ))),
    })
}

fn assign_div(i: &mut &[Token]) -> PResult<Statement> {
    use crate::ast::BinOp::*;
    let var = identifier.parse_next(i)?;
    one_of(Token::BinOp(Binary::AssignDiv)).parse_next(i)?;
    let rhs = expr.parse_next(i)?;
    Ok(Statement::Assign {
        var: var.clone(),
        rhs: Box::new(Expr::BinaryOp(Div(
            Box::new(Expr::Variable(var)),
            Box::new(rhs),
        ))),
    })
}

fn assign_inc(i: &mut &[Token]) -> PResult<Statement> {
    use crate::ast::BinOp::*;
    let var = one_of(Token::Variable).parse_next(i)?;
    one_of(Token::UnOp(Unary::Inc)).parse_next(i)?;
    let rhs = expr.parse_next(i)?;
    Ok(Statement::Assign {
        var: var.clone(),
        rhs: Box::new(Expr::BinaryOp(Add(
            Box::new(Expr::Variable(var)),
            Box::new(Expr::Scalar(1.0)),
        ))),
    })
}

fn assign_dec(i: &mut &[Token]) -> PResult<Statement> {
    use crate::ast::BinOp::*;
    let var = identifier.parse_next(i)?;
    one_of(Token::UnOp(Unary::Dec)).parse_next(i)?;
    let rhs = expr.parse_next(i)?;
    Ok(Statement::Assign {
        var: var.clone(),
        rhs: Box::new(Expr::BinaryOp(Sub(
            Box::new(Expr::Variable(var)),
            Box::new(Expr::Scalar(1.0)),
        ))),
    })
}

fn return_statement(i: &mut &[Token]) -> PResult<Statement> {
    expr.map(|e| Statement::Return(Box::new(e))).parse_next(i)
}

fn list(i: &mut &[Token]) -> PResult<Vec<Expr>> {
    delimited(lbracket, separated(0.., expr, comma), rbracket).parse_next(i)
}

fn arg_list(i: &mut &[Token]) -> PResult<Vec<Expr>> {
    let ls: Vec<Vec<Expr>> = separated(1.., list, comma).parse_next(i)?;
    Ok(ls.into_iter().flatten().collect())
}

fn args(i: &mut &[Token]) -> PResult<Vec<Expr>> {
    let r = alt((arg_list, separated(0.., expr, comma))).parse_next(i)?;
    let _ = opt(comma).parse_next(i);
    Ok(r)
}

fn function(i: &mut &[Token]) -> PResult<Expr> {
    let name = function_name.parse_next(i)?;
    delimited(multispaces, "(", multispaces).parse_next(i)?;
    let args = args.parse_next(i)?;
    delimited(multispaces, ")", multispaces).parse_next(i)?;
    Ok(Expr::Function { name, args })
}

mod tests {
    #[allow(unused_imports)]
    use super::*;
    // use serde_json::to_string;
    //     if let Ok((_, s)) = statements.parse_peek(input) {
    //         let serialized = to_string(&s).unwrap();
    //         println!("serialized = {}", serialized);
    //     }

    #[test]
    fn tt() {
        // let input = "s=1;@5{@xyz{$=B($*2)-8,}s*=.5,}(L(x,y,z)-8)*s";
        // let input = "s=2.5,h=s/2,d=(s+h)/2,q=20,y-=10,[x,y]=r0(x,y),@xyz{$/=q,}c=1,t=0,@7{@xyz{$=mod($-h,s)-h,}t=d/D([x,y,z],[x,y,z]),@xyzc{$*=t,}}d=L(x,y,z)/c*2.-.025";
        // let input =  "[x,z]=r0(x,z), x+=11, z+=15, y+=10, h=exp(-1.5*B(nz(x,0,z,.1,1))), g=y-10*h-nz(x,0,z,10,1)*0.05, b = y-12, a=rU( L(x-cl(x,-2,2),b*1.3,z)-3, U( L(x+5,b-1,z)-1.7, L(x+5,b-2,B(z)-1.5)-0.8, bx3(x-5,b-1,z,0.2,0.1,0.2)-0.5, bx3(x+5,b-1,z,1.9,.1,.1)-.5, L(B(x)-3.5,b-cl(b,-4,0),B(z)-1.5)-.8,),1.5 )-nz(x,0,z,12,1)*0.15, s=(L(x>7?(mod(x,4)-2)/2:x,x<1?y:b/3+2,B(z)-1.5)-1.8)-nz(x,y,z,.5,1)*2, rG(U(a,g),-s,1)";
        // let input = "a=rU( L(x-cl(x,-2,2),b*1.3,z)-3, U( L(x+5,b-1,z)-1.7, L(x+5,b-2,B(z)-1.5)-0.8, bx3(x-5,b-1,z,0.2,0.1,0.2)-0.5, bx3(x+5,b-1,z,1.9,.1,.1)-.5, L(B(x)-3.5,b-cl(b,-4,0),B(z)-1.5)-.8,),1.5 )-nz(x,0,z,12,1)*0.15", rG(U(a,g),-s,1)";
        // let input = "x>7?(mod(x,4)-2)/2:x";
        // let input = "@xyz{$m=mod($,20)-10,$i=Z($/20),}d=99,g=.05,y-=20,[z,x]=r0(z,x),n=nz(x,y,z,.1,1),n1=nz(x,y,z,.3,2,3),@4{x-=20,o=$*200+20,e=B(y+n1/2+sin(z*.05+o)*10)-1,e=rG(e,B(z+sin(x*.05+o)*25)-5+n1*2,.2),@xz{$1=mod($+n*10,3)-1.5,}e=rG(e,-(B(z1)-g),.25),e=rG(e,-(B(x1)-g),.25),d=U(d,e),[x,z]=r1(z,x),y+=20,}U(d,ri(xi,yi,zi)>.4&&L(xi,yi,zi)>3?L(xm,ym,zm)-2:10)";
        // let input = "U(d,ri(xi,yi,zi)>.4&&L(xi,yi,zi)>3?L(xm,ym,zm)-2:10)";
        // let input = "ri(xi,yi,zi)>.4&&L(xi,yi,zi)>3?L(xm,ym,zm)-2:10";
        // let input = "a = (x > 7)"; // && (y < 10)";
        // let input = "b=0; a=rU( L(x-cl(x,-2,2),b*1.3,z)-3, U( L(x+5,b-1,z)-1.7, L(x+5,b-2,B(z)-1.5)-0.8, bx3(x-5,b-1,z,0.2,0.1,0.2)-0.5, bx3(x+5,b-1,z,1.9,.1,.1)-.5, L(B(x)-3.5,b-cl(b,-4,0),B(z)-1.5)-.8),1.5 )";
        // let input = "s=.5,y+=6, a=k(y+22,B(z+10*g(x*.005+.2))-16)-4,b=TR(x/40+.2)*40,c; [b,y]=r1(b,y), [y,z]=r0(y,z+15), r=rU(U(@byz{bx2(B($$$)-20,bx2($,$$,23)+3,3)-.4,}),a,3)"; //, @4{ [x,y]=r1(x,y), [y,z]=r0(y,z), a=nz(x,y,z,.02/s,$+5), a=B(a)*50-3, r=rU(r,rG(r-7*s,a*s,s*2),s*2), s*=.5, } r";
        // let input = "a=rU(1,2,3,4,5,6)";
        // let input = "k(y+22, 2)";
        // let input = "U(@xyz{L($$$,$$,$)-8,})";
        // let input = "i=0, sin(i++)";
        // let input = "a = sin(x) > cos(x) && sin(y) < cos(y)";
        // let input = "ri(x,y,z)>.4&&L(x,y,z)>3?L(x,y,z)-2:10";
        // let input = "@xyz{$m=mod($,20)-10,$i=Z($/20),}d=99,g=.05,y-=20,[z,x]=r0(z,x),n=nz(x,y,z,.1,1),n1=nz(x,y,z,.3,2,3),@4{x-=20,o=$*200+20,e=B(y+n1/2+sin(z*.05+o)*10)-1,e=rG(e,B(z+sin(x*.05+o)*25)-5+n1*2,.2),@xz{$1=mod($+n*10,3)-1.5,}e=rG(e,-(B(z1)-g),.25),e=rG(e,-(B(x1)-g),.25),d=U(d,e),[x,z]=r1(z,x),y+=20,}U(d,ri(xi,yi,zi)>.4&&L(xi,yi,zi)>3?L(xm,ym,zm)-2:10)";
        let input = "y-=1, r=bx3(x,y,z,9)-2,s=1,ti=U(L(x,y)-.6, L(y,z)-.6,L(z,x)-.6); @4{ @xyz{$=(mod($+9,18)-9)*3,} s/=3, r=k(r+s,-U(@xyz{L($,$$)-12,})*s)-s, } U(r, ti)";
        let i = &expand(input);
        dbg!(&i);
        let _ = dbg!(program.parse_peek(i));
    }
    #[test]
    fn no_more_ray() {
        let input = "U(L(x+28,y-10,z+8)-12, don(x-cl(x,-15,15),y-18,z-20,10,3), bx3(x-20,y-20,z+20,8)-10, L(x+3,y-16)-4)";
        let expected = Ok((
            "",
            String::from("Sequence([Return(Function { name: Union, args: [BinaryOp(Sub(Function { name: Length, args: [BinaryOp(Add(Variable(\"x\"), Scalar(28.0))), BinaryOp(Sub(Variable(\"y\"), Scalar(10.0))), BinaryOp(Add(Variable(\"z\"), Scalar(8.0)))] }, Scalar(12.0))), Function { name: Torus, args: [BinaryOp(Sub(Variable(\"x\"), Function { name: Clamp, args: [Variable(\"x\"), Negate(Scalar(15.0)), Scalar(15.0)] })), BinaryOp(Sub(Variable(\"y\"), Scalar(18.0))), BinaryOp(Sub(Variable(\"z\"), Scalar(20.0))), Scalar(10.0), Scalar(3.0)] }, BinaryOp(Sub(Function { name: Box3, args: [BinaryOp(Sub(Variable(\"x\"), Scalar(20.0))), BinaryOp(Sub(Variable(\"y\"), Scalar(20.0))), BinaryOp(Add(Variable(\"z\"), Scalar(20.0))), Scalar(8.0)] }, Scalar(10.0))), BinaryOp(Sub(Function { name: Length, args: [BinaryOp(Add(Variable(\"x\"), Scalar(3.0))), BinaryOp(Sub(Variable(\"y\"), Scalar(16.0)))] }, Scalar(4.0)))] })])"),
        ));
        assert_eq!(
            program.map(|e| format!("{e:?}")).parse_peek(input),
            expected
        );
    }

    #[test]
    fn random_python() {
        let input = "U(don(mod(x+-8.22,4.46),mod(y+3.88,4.36),TR(z+5.17),4.19,9.74),bx3(x+8.88,y+3.14,z+-7.53,6.72,2.08,8.98)-3.77,bx3(x+-0.14,mod(y+-2.22,4.17),z+-2.84,1.88,3.59,6.38)-0.57,L(x+4.15,TR(y+-4.79),mod(z+9.16,-4.84))-0.16,don(B(x+-0.87)-4,B(y+-3.58)-3,TR(z+-8.70),9.79,8.58),L(x+9.67,B(y+6.01)-5)-4.49,L(B(x+-4.68)-4,y+-8.46)-1.78,don(x+-6.66,y+4.27,z+6.62,4.38,8.19))";
        let expected = Ok(("", String::from("Sequence([Return(Function { name: Union, args: [Function { name: Torus, args: [Function { name: Mod, args: [BinaryOp(Add(Variable(\"x\"), Negate(Scalar(8.22)))), Scalar(4.46)] }, Function { name: Mod, args: [BinaryOp(Add(Variable(\"y\"), Scalar(3.88))), Scalar(4.36)] }, Function { name: Triangle, args: [BinaryOp(Add(Variable(\"z\"), Scalar(5.17)))] }, Scalar(4.19), Scalar(9.74)] }, BinaryOp(Sub(Function { name: Box3, args: [BinaryOp(Add(Variable(\"x\"), Scalar(8.88))), BinaryOp(Add(Variable(\"y\"), Scalar(3.14))), BinaryOp(Add(Variable(\"z\"), Negate(Scalar(7.53)))), Scalar(6.72), Scalar(2.08), Scalar(8.98)] }, Scalar(3.77))), BinaryOp(Sub(Function { name: Box3, args: [BinaryOp(Add(Variable(\"x\"), Negate(Scalar(0.14)))), Function { name: Mod, args: [BinaryOp(Add(Variable(\"y\"), Negate(Scalar(2.22)))), Scalar(4.17)] }, BinaryOp(Add(Variable(\"z\"), Negate(Scalar(2.84)))), Scalar(1.88), Scalar(3.59), Scalar(6.38)] }, Scalar(0.57))), BinaryOp(Sub(Function { name: Length, args: [BinaryOp(Add(Variable(\"x\"), Scalar(4.15))), Function { name: Triangle, args: [BinaryOp(Add(Variable(\"y\"), Negate(Scalar(4.79))))] }, Function { name: Mod, args: [BinaryOp(Add(Variable(\"z\"), Scalar(9.16))), Negate(Scalar(4.84))] }] }, Scalar(0.16))), Function { name: Torus, args: [BinaryOp(Sub(Function { name: Abs, args: [BinaryOp(Add(Variable(\"x\"), Negate(Scalar(0.87))))] }, Scalar(4.0))), BinaryOp(Sub(Function { name: Abs, args: [BinaryOp(Add(Variable(\"y\"), Negate(Scalar(3.58))))] }, Scalar(3.0))), Function { name: Triangle, args: [BinaryOp(Add(Variable(\"z\"), Negate(Scalar(8.7))))] }, Scalar(9.79), Scalar(8.58)] }, BinaryOp(Sub(Function { name: Length, args: [BinaryOp(Add(Variable(\"x\"), Scalar(9.67))), BinaryOp(Sub(Function { name: Abs, args: [BinaryOp(Add(Variable(\"y\"), Scalar(6.01)))] }, Scalar(5.0)))] }, Scalar(4.49))), BinaryOp(Sub(Function { name: Length, args: [BinaryOp(Sub(Function { name: Abs, args: [BinaryOp(Add(Variable(\"x\"), Negate(Scalar(4.68))))] }, Scalar(4.0))), BinaryOp(Add(Variable(\"y\"), Negate(Scalar(8.46))))] }, Scalar(1.78))), Function { name: Torus, args: [BinaryOp(Add(Variable(\"x\"), Negate(Scalar(6.66)))), BinaryOp(Add(Variable(\"y\"), Scalar(4.27))), BinaryOp(Add(Variable(\"z\"), Scalar(6.62))), Scalar(4.38), Scalar(8.19)] }] })])")));
        assert_eq!(
            program.map(|e| format!("{e:?}")).parse_peek(input),
            expected
        );
    }

    #[test]
    fn g67() {
        let input = "s=10; @1{a=sin(y),b=sin(x),c=sin(z),d=x,e=s+1,} SM(a,b,c,d,e)-5";
        let i = expand(input);
        dbg!(&i);
        let expected =Ok(("", String::from("Sequence([Assign { var: \"s\", rhs: Scalar(10.0) }, Assign { var: \"a\", rhs: Function { name: Sin, args: [Variable(\"y\")] } }, Assign { var: \"b\", rhs: Function { name: Sin, args: [Variable(\"x\")] } }, Assign { var: \"c\", rhs: Function { name: Sin, args: [Variable(\"z\")] } }, Assign { var: \"d\", rhs: Variable(\"x\") }, Assign { var: \"e\", rhs: BinaryOp(Add(Variable(\"s\"), Scalar(1.0))) }, Return(BinaryOp(Sub(Function { name: Smoothstep, args: [Variable(\"a\"), Variable(\"b\"), Variable(\"c\"), Variable(\"d\"), Variable(\"e\")] }, Scalar(5.0))))])")));
        assert_eq!(program.map(|e| format!("{e:?}")).parse_peek(&i), expected);
    }

    #[test]
    fn quanta() {
        let input = "s=20,[x,z]=r0(x,z),[y,x]=r1(y,x),z+=17,y+=27,i=0,z+=ri(Z(x/s))*70,@xz{$-=nz(x,y,z,.1,i++)*5*i,$i=Z($/s),$=mod($,s)-s/2,}i=ri(xi,zi),j=ri(xi,floor(y/5)),d=i>.1?rU(L(x,z)-1*i-.5*(cos(y/4)+1),bx2(L(x,z)-(cos(floor(y/4))+1)*2,mod(y,4)-2,.1,.2)-.05,1):L(x,mod(y,5)-2.5,z)-G(j,0)*2";
        let i = expand(input);
        let expected = Ok(("", String::from("Sequence([Assign { var: \"s\", rhs: Scalar(20.0) }, AssignArray { vars: [\"x\", \"z\"], rhs: Function { name: Rot0, args: [Variable(\"x\"), Variable(\"z\")] } }, AssignArray { vars: [\"y\", \"x\"], rhs: Function { name: Rot1, args: [Variable(\"y\"), Variable(\"x\")] } }, Assign { var: \"z\", rhs: BinaryOp(Add(Variable(\"z\"), Scalar(17.0))) }, Assign { var: \"y\", rhs: BinaryOp(Add(Variable(\"y\"), Scalar(27.0))) }, Assign { var: \"i\", rhs: Scalar(0.0) }, Assign { var: \"z\", rhs: BinaryOp(Add(Variable(\"z\"), BinaryOp(Mul(Function { name: Hash, args: [Function { name: Floor, args: [BinaryOp(Div(Variable(\"x\"), Variable(\"s\")))] }] }, Scalar(70.0))))) }, Assign { var: \"x\", rhs: BinaryOp(Sub(Variable(\"x\"), BinaryOp(Mul(BinaryOp(Mul(Function { name: ValueNoise, args: [Variable(\"x\"), Variable(\"y\"), Variable(\"z\"), Scalar(0.1), Assign(Inc(\"i\"))] }, Scalar(5.0))), Variable(\"i\"))))) }, Assign { var: \"xi\", rhs: Function { name: Floor, args: [BinaryOp(Div(Variable(\"x\"), Variable(\"s\")))] } }, Assign { var: \"x\", rhs: BinaryOp(Sub(Function { name: Mod, args: [Variable(\"x\"), Variable(\"s\")] }, BinaryOp(Div(Variable(\"s\"), Scalar(2.0))))) }, Assign { var: \"z\", rhs: BinaryOp(Sub(Variable(\"z\"), BinaryOp(Mul(BinaryOp(Mul(Function { name: ValueNoise, args: [Variable(\"x\"), Variable(\"y\"), Variable(\"z\"), Scalar(0.1), Assign(Inc(\"i\"))] }, Scalar(5.0))), Variable(\"i\"))))) }, Assign { var: \"zi\", rhs: Function { name: Floor, args: [BinaryOp(Div(Variable(\"z\"), Variable(\"s\")))] } }, Assign { var: \"z\", rhs: BinaryOp(Sub(Function { name: Mod, args: [Variable(\"z\"), Variable(\"s\")] }, BinaryOp(Div(Variable(\"s\"), Scalar(2.0))))) }, Assign { var: \"i\", rhs: Function { name: Hash, args: [Variable(\"xi\"), Variable(\"zi\")] } }, Assign { var: \"j\", rhs: Function { name: Hash, args: [Variable(\"xi\"), Function { name: Floor, args: [BinaryOp(Div(Variable(\"y\"), Scalar(5.0)))] }] } }, Assign { var: \"d\", rhs: TernaryOp(BinaryOp(Greater(Variable(\"i\"), Scalar(0.1))), Function { name: Union, args: [BinaryOp(Sub(BinaryOp(Sub(Function { name: Length, args: [Variable(\"x\"), Variable(\"z\")] }, BinaryOp(Mul(Scalar(1.0), Variable(\"i\"))))), BinaryOp(Mul(Scalar(0.5), Paren(BinaryOp(Add(Function { name: Cos, args: [BinaryOp(Div(Variable(\"y\"), Scalar(4.0)))] }, Scalar(1.0)))))))), BinaryOp(Sub(Function { name: Box2, args: [BinaryOp(Sub(Function { name: Length, args: [Variable(\"x\"), Variable(\"z\")] }, BinaryOp(Mul(Paren(BinaryOp(Add(Function { name: Cos, args: [Function { name: Floor, args: [BinaryOp(Div(Variable(\"y\"), Scalar(4.0)))] }] }, Scalar(1.0)))), Scalar(2.0))))), BinaryOp(Sub(Function { name: Mod, args: [Variable(\"y\"), Scalar(4.0)] }, Scalar(2.0))), Scalar(0.1), Scalar(0.2)] }, Scalar(0.05))), Scalar(1.0)] }, BinaryOp(Sub(Function { name: Length, args: [Variable(\"x\"), BinaryOp(Sub(Function { name: Mod, args: [Variable(\"y\"), Scalar(5.0)] }, Scalar(2.5))), Variable(\"z\")] }, BinaryOp(Mul(Function { name: Intersect, args: [Variable(\"j\"), Scalar(0.0)] }, Scalar(2.0)))))) }])")));
        assert_eq!(program.map(|e| format!("{e:?}")).parse_peek(&i), expected);
    }

    #[test]
    fn statement_test() {
        let input = "a = 1.0";
        let expected = Ok(("", String::from("Assign { var: \"a\", rhs: Scalar(1.0) }")));
        assert_eq!(
            statement.map(|e| format!("{e:?}")).parse_peek(input),
            expected
        );

        let input = "[x,y,z] = sin(1.0)";
        let expected = Ok((
            "",
            String::from("AssignArray { vars: [\"x\", \"y\", \"z\"], rhs: Function { name: Sin, args: [Scalar(1.0)] } }"),
        ));
        assert_eq!(
            statement.map(|e| format!("{e:?}")).parse_peek(input),
            expected
        );

        let input = "p=B(y-18)-13,n=nz(x,y,z,.4,0,2)*2,q=mod(p,12+n*z)-1.8";
        let expected = Ok((
            "",
            String::from("Sequence([Assign { var: \"p\", rhs: BinaryOp(Sub(Function { name: Abs, args: [BinaryOp(Sub(Variable(\"y\"), Scalar(18.0)))] }, Scalar(13.0))) }, Assign { var: \"n\", rhs: BinaryOp(Mul(Function { name: ValueNoise, args: [Variable(\"x\"), Variable(\"y\"), Variable(\"z\"), Scalar(0.4), Scalar(0.0), Scalar(2.0)] }, Scalar(2.0))) }, Assign { var: \"q\", rhs: BinaryOp(Sub(Function { name: Mod, args: [Variable(\"p\"), BinaryOp(Add(Scalar(12.0), BinaryOp(Mul(Variable(\"n\"), Variable(\"z\")))))] }, Scalar(1.8))) }])"),
        ));
        assert_eq!(
            program.map(|e| format!("{e:?}")).parse_peek(input),
            expected
        );

        let input = "a = 1.0, a++";
        let expected = Ok((
            "",
            String::from("Sequence([Assign { var: \"a\", rhs: Scalar(1.0) }, Assign { var: \"a\", rhs: BinaryOp(Add(Variable(\"a\"), Scalar(1.0))) }])"),
        ));
        assert_eq!(
            program.map(|e| format!("{e:?}")).parse_peek(input),
            expected
        );

        let input = "@xyz{$=B($)-6,} L(x,y,z)-5";
        let i = expand(input);
        let expected = Ok((
            "",
            String::from("Sequence([Assign { var: \"x\", rhs: BinaryOp(Sub(Function { name: Abs, args: [Variable(\"x\")] }, Scalar(6.0))) }, Assign { var: \"y\", rhs: BinaryOp(Sub(Function { name: Abs, args: [Variable(\"y\")] }, Scalar(6.0))) }, Assign { var: \"z\", rhs: BinaryOp(Sub(Function { name: Abs, args: [Variable(\"z\")] }, Scalar(6.0))) }, Return(BinaryOp(Sub(Function { name: Length, args: [Variable(\"x\"), Variable(\"y\"), Variable(\"z\")] }, Scalar(5.0))))])")),
        );
        assert_eq!(program.map(|e| format!("{e:?}")).parse_peek(&i), expected);
    }

    #[test]
    fn expr_test() {
        let input = " 1.1 +  2.12 ";
        let expected = Ok(("", String::from("BinaryOp(Add(Scalar(1.1), Scalar(2.12)))")));
        assert_eq!(expr.map(|e| format!("{e:?}")).parse_peek(input), expected);

        let input = " 12.4321 + 6.321 - 4.21+  3.";
        let expected = Ok((
        "",
        String::from("BinaryOp(Add(BinaryOp(Sub(BinaryOp(Add(Scalar(12.4321), Scalar(6.321))), Scalar(4.21))), Scalar(3.0)))"),
    ));
        assert_eq!(expr.map(|e| format!("{e:?}")).parse_peek(input), expected);

        let input = " 1.3 + 2.3*3.3 + 4.31";
        let expected = Ok((
        "",
        String::from("BinaryOp(Add(BinaryOp(Add(Scalar(1.3), BinaryOp(Mul(Scalar(2.3), Scalar(3.3))))), Scalar(4.31)))"),
    ));
        assert_eq!(expr.map(|e| format!("{e:?}")).parse_peek(input), expected);

        let input = "1.0 * 0.004 + 0.1";
        let expected = Ok((
            "",
            String::from("BinaryOp(Add(BinaryOp(Mul(Scalar(1.0), Scalar(0.004))), Scalar(0.1)))"),
        ));
        assert_eq!(expr.map(|e| format!("{e:?}")).parse_peek(input), expected);

        let input = "sin(3.14) / 2.0";
        let expected = Ok((
            "",
            String::from(
                "BinaryOp(Div(Function { name: Sin, args: [Scalar(3.14)] }, Scalar(2.0)))",
            ),
        ));
        assert_eq!(expr.map(|e| format!("{e:?}")).parse_peek(input), expected);
    }

    #[test]
    fn assign_test() {
        let input = "aa = sin(1.0) / 2.0";
        let expected = Ok((
            "",
            String::from("Assign { var: \"aa\", rhs: BinaryOp(Div(Function { name: Sin, args: [Scalar(1.0)] }, Scalar(2.0))) }"),
        ));
        assert_eq!(
            assign_scalar.map(|e| format!("{e:?}")).parse_peek(input),
            expected
        );

        let input = "x3 = (1.0 + 2.0) * 3.0";
        let expected = Ok((
            "",
            String::from("Assign { var: \"x3\", rhs: BinaryOp(Mul(Paren(BinaryOp(Add(Scalar(1.0), Scalar(2.0)))), Scalar(3.0))) }"),
        ));
        assert_eq!(
            assign_scalar.map(|e| format!("{e:?}")).parse_peek(input),
            expected
        );
        let input = "a=z*-0.004+0.1";
        let expected = Ok((
            "",
            String::from("Assign { var: \"a\", rhs: BinaryOp(Add(BinaryOp(Mul(Variable(\"z\"), Negate(Scalar(0.004)))), Scalar(0.1))) }"),
        ));
        assert_eq!(
            assign_scalar.map(|e| format!("{e:?}")).parse_peek(input),
            expected
        );
    }

    #[test]
    fn product_test() {
        let input = " 12.7 *2.7 /  3.7";
        let expected = Ok((
            "",
            String::from("BinaryOp(Div(BinaryOp(Mul(Scalar(12.7), Scalar(2.7))), Scalar(3.7)))"),
        ));
        assert_eq!(
            product.map(|e| format!("{e:?}")).parse_peek(input),
            expected
        );

        let input = " 2.3* 3.3  *2.3 *2.3 /  3.3";
        let expected = Ok((
            "",
            String::from(
                "BinaryOp(Div(BinaryOp(Mul(BinaryOp(Mul(BinaryOp(Mul(Scalar(2.3), Scalar(3.3))), Scalar(2.3))), Scalar(2.3))), Scalar(3.3)))",
            ),
        ));
        assert_eq!(
            product.map(|e| format!("{e:?}")).parse_peek(input),
            expected
        );

        let input = " 48.4 /  3.4/2.5";
        let expected = Ok((
            "",
            String::from("BinaryOp(Div(BinaryOp(Div(Scalar(48.4), Scalar(3.4))), Scalar(2.5)))"),
        ));
        assert_eq!(
            product.map(|e| format!("{e:?}")).parse_peek(input),
            expected
        );
    }

    #[test]
    fn parens_test() {
        let input = " (  2.0 )";
        let expected = Ok(("", String::from("Paren(Scalar(2.0))")));
        assert_eq!(expr.map(|e| format!("{e:?}")).parse_peek(input), expected);

        let input = " 2.1 * (  3.23 + 4.456 ) ";
        let expected = Ok((
            "",
            String::from(
                "BinaryOp(Mul(Scalar(2.1), Paren(BinaryOp(Add(Scalar(3.23), Scalar(4.456))))))",
            ),
        ));
        assert_eq!(expr.map(|e| format!("{e:?}")).parse_peek(input), expected);

        let input = "  2.5 * 2.5  / ( 5.0 - 1.0) + 3.25";
        let expected = Ok((
            "",
            String::from(
                "BinaryOp(Add(BinaryOp(Div(BinaryOp(Mul(Scalar(2.5), Scalar(2.5))), Paren(BinaryOp(Sub(Scalar(5.0), Scalar(1.0)))))), Scalar(3.25)))",
            )),
        );
        assert_eq!(expr.map(|e| format!("{e:?}")).parse_peek(input), expected);
    }

    #[test]
    fn function_test() {
        let input = "sin(1.0)";
        let expected = Ok((
            "",
            String::from("Function { name: Sin, args: [Scalar(1.0)] }"),
        ));
        assert_eq!(
            function.map(|e| format!("{e:?}")).parse_peek(input),
            expected
        );

        let input = "cl(-2.0 * 5.1, 0.0, 10.0)";
        let expected = Ok((
            "",
            String::from("Function { name: Clamp, args: [BinaryOp(Mul(Negate(Scalar(2.0)), Scalar(5.1))), Scalar(0.0), Scalar(10.0)] }"),
        ));
        assert_eq!(
            function.map(|e| format!("{e:?}")).parse_peek(input),
            expected
        );

        let input = "cos( sin(3.14* 2.0) )";
        let expected = Ok((
            "",
            String::from("Function { name: Cos, args: [Function { name: Sin, args: [BinaryOp(Mul(Scalar(3.14), Scalar(2.0)))] }] }"),
        ));
        assert_eq!(
            function.map(|e| format!("{e:?}")).parse_peek(input),
            expected
        );
    }

    #[test]
    fn identifier_test() {
        let input = "a";
        let expected = Ok(("", String::from("\"a\"")));
        assert_eq!(
            identifier.map(|e| format!("{e:?}")).parse_peek(input),
            expected
        );

        let input = "a2";
        let expected = Ok(("", String::from("\"a2\"")));
        assert_eq!(
            identifier.map(|e| format!("{e:?}")).parse_peek(input),
            expected
        );
    }

    #[test]
    fn assign_scalar_test() {
        let input = "ab = 1.57";
        let expected = Ok((
            "",
            String::from("Assign { var: \"ab\", rhs: Scalar(1.57) }"),
        ));
        assert_eq!(
            assign_scalar.map(|e| format!("{e:?}")).parse_peek(input),
            expected
        );

        let input = "a = sin(1.0) / 2.0";
        let expected = Ok((
            "",
            String::from("Assign { var: \"a\", rhs: BinaryOp(Div(Function { name: Sin, args: [Scalar(1.0)] }, Scalar(2.0))) }"),
        ));
        assert_eq!(
            assign_scalar.map(|e| format!("{e:?}")).parse_peek(input),
            expected
        );

        let input = "a= -0.004 + 0.1";
        let expected = Ok((
            "",
            String::from(
                "Assign { var: \"a\", rhs: BinaryOp(Add(Negate(Scalar(0.004)), Scalar(0.1))) }",
            ),
        ));
        assert_eq!(
            assign_scalar.map(|e| format!("{e:?}")).parse_peek(input),
            expected
        );
    }
}
