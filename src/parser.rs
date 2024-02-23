use crate::ast::*;
use crate::expand::expand;

use winnow::ascii::{alpha1, alphanumeric0};
use winnow::combinator::{fail, opt, separated};
use winnow::prelude::*;
use winnow::token::take;
use winnow::{
    ascii::{float, multispace0 as multispaces},
    combinator::{alt, delimited, repeat},
    token::one_of,
};

pub fn parse(input: &mut &str) -> Statement {
    let mut i_str: &str = &expand(input);
    program(&mut i_str).unwrap()
}

fn lbracket<'a>(i: &mut &'a str) -> PResult<&'a str> {
    delimited(multispaces, "[", multispaces).parse_next(i)
}

fn rbracket<'a>(i: &mut &'a str) -> PResult<&'a str> {
    delimited(multispaces, "]", multispaces).parse_next(i)
}

fn rbrace<'a>(i: &mut &'a str) -> PResult<&'a str> {
    delimited(multispaces, "}", multispaces).parse_next(i)
}

fn comma<'a>(i: &mut &'a str) -> PResult<&'a str> {
    delimited(multispaces, ",", multispaces).parse_next(i)
}

fn gt<'a>(i: &mut &'a str) -> PResult<&'a str> {
    delimited(multispaces, ">", multispaces).parse_next(i)
}

fn ge<'a>(i: &mut &'a str) -> PResult<&'a str> {
    delimited(multispaces, ">=", multispaces).parse_next(i)
}

fn lt<'a>(i: &mut &'a str) -> PResult<&'a str> {
    delimited(multispaces, "<", multispaces).parse_next(i)
}

fn le<'a>(i: &mut &'a str) -> PResult<&'a str> {
    delimited(multispaces, "<=", multispaces).parse_next(i)
}

fn equal<'a>(i: &mut &'a str) -> PResult<&'a str> {
    delimited(multispaces, "==", multispaces).parse_next(i)
}

fn program(i: &mut &str) -> PResult<Statement> {
    separated(1.., statement, alt((",", ";")))
        .map(|stmts| Statement::Sequence(stmts))
        .parse_next(i)
}

fn statement(i: &mut &str) -> PResult<Statement> {
    delimited(
        multispaces,
        alt((
            rbrace.map(|_| Statement::Empty),
            assign_array,
            assign_scalar,
            assign_add,
            assign_sub,
            assign_mul,
            assign_div,
            assign_inc,
            return_statement,
            program,
        )),
        multispaces,
    )
    .parse_next(i)
}

fn ternary(i: &mut &str) -> PResult<Expr> {
    let condition = comp.parse_next(i)?;
    dbg!(&condition);
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

// fn scalar(i: &mut &str) -> PResult<Expr> {
//     let negation = opt('-').map(|op| op.is_some());
//     let expression = delimited(
//         multispaces,
//         alt((float.map(Expr::Scalar), function, parens, variable, ternary)),
//         multispaces,
//     );
//     (negation, expression)
//         .map(|(n, e)| if n { Expr::Negate(Box::new(e)) } else { e })
//         .parse_next(i)
// }

fn expr(i: &mut &str) -> PResult<Expr> {
    delimited(
        multispaces,
        alt((
            sum,
            product,
            function,
            parens,
            variable,
            ternary,
            comp,
            float.map(Expr::Scalar),
        )),
        multispaces,
    )
    .parse_next(i)
}

fn comp(i: &mut &str) -> PResult<Expr> {
    use BinOp::*;
    let lhs = sum.parse_next(i)?;
    let op = alt((gt, ge, lt, le, equal)).parse_next(i)?;
    let rhs = sum.parse_next(i)?;
    match op {
        "==" => Ok(Expr::BinaryOp(Eq(Box::new(lhs), Box::new(rhs)))),
        ">=" => Ok(Expr::BinaryOp(GreaterEq(Box::new(lhs), Box::new(rhs)))),
        "<=" => Ok(Expr::BinaryOp(LessEq(Box::new(lhs), Box::new(rhs)))),
        ">" => Ok(Expr::BinaryOp(Greater(Box::new(lhs), Box::new(rhs)))),
        _ => Ok(Expr::BinaryOp(Less(Box::new(lhs), Box::new(rhs)))),
    }
}

fn sum(i: &mut &str) -> PResult<Expr> {
    use BinOp::*;
    let init = product.parse_next(i)?;

    repeat(0.., (one_of(['+', '-']), product))
        .fold(
            move || init.clone(),
            |acc, (op, val): (char, Expr)| {
                if op == '+' {
                    Expr::BinaryOp(Add(Box::new(acc), Box::new(val)))
                } else {
                    Expr::BinaryOp(Sub(Box::new(acc), Box::new(val)))
                }
            },
        )
        .parse_next(i)
}

fn product(i: &mut &str) -> PResult<Expr> {
    use BinOp::*;
    let init = expr.parse_next(i)?;

    repeat(0.., (one_of(['*', '/']), expr))
        .fold(
            move || init.clone(),
            |acc, (op, val): (char, Expr)| {
                if op == '*' {
                    Expr::BinaryOp(Mul(Box::new(acc), Box::new(val)))
                } else {
                    Expr::BinaryOp(Div(Box::new(acc), Box::new(val)))
                }
            },
        )
        .parse_next(i)
}

fn assign_scalar(i: &mut &str) -> PResult<Statement> {
    let var = identifier.parse_next(i)?;
    delimited(multispaces, "=", multispaces).parse_next(i)?;
    let rhs = expr.parse_next(i)?;
    Ok(Statement::Assign {
        var,
        rhs: Box::new(rhs),
    })
}

fn assign_array(i: &mut &str) -> PResult<Statement> {
    let vars: Vec<String> =
        delimited(lbracket, separated(1.., identifier, comma), rbracket).parse_next(i)?;
    delimited(multispaces, "=", multispaces).parse_next(i)?;
    let expression = expr.parse_next(i)?;
    Ok(Statement::AssignArray {
        vars,
        rhs: Box::new(expression),
    })
}

fn assign_add(i: &mut &str) -> PResult<Statement> {
    use crate::ast::BinOp::*; // Not sure why this is needed, otherwise it complains about Add not being in scope.
    let var = identifier.parse_next(i)?;
    delimited(multispaces, "+=", multispaces).parse_next(i)?;
    let rhs = expr.parse_next(i)?;
    Ok(Statement::Assign {
        var: var.clone(),
        rhs: Box::new(Expr::BinaryOp(Add(
            Box::new(Expr::Variable(var)),
            Box::new(rhs),
        ))),
    })
}

fn assign_sub(i: &mut &str) -> PResult<Statement> {
    use crate::ast::BinOp::*; // Not sure why this is needed, otherwise it complains about Sub not being in scope.
    let var = identifier.parse_next(i)?;
    delimited(multispaces, "-=", multispaces).parse_next(i)?;
    let rhs = expr.parse_next(i)?;
    Ok(Statement::Assign {
        var: var.clone(),
        rhs: Box::new(Expr::BinaryOp(Sub(
            Box::new(Expr::Variable(var)),
            Box::new(rhs),
        ))),
    })
}

fn assign_mul(i: &mut &str) -> PResult<Statement> {
    use crate::ast::BinOp::*; // Not sure why this is needed, otherwise it complains about Mul not being in scope.
    let var = identifier.parse_next(i)?;
    delimited(multispaces, "*=", multispaces).parse_next(i)?;
    let rhs = expr.parse_next(i)?;
    Ok(Statement::Assign {
        var: var.clone(),
        rhs: Box::new(Expr::BinaryOp(Mul(
            Box::new(Expr::Variable(var)),
            Box::new(rhs),
        ))),
    })
}

fn assign_div(i: &mut &str) -> PResult<Statement> {
    use crate::ast::BinOp::*; // Not sure why this is needed, otherwise it complains about Div not being in scope.
    let var = identifier.parse_next(i)?;
    delimited(multispaces, "/=", multispaces).parse_next(i)?;
    let rhs = expr.parse_next(i)?;
    Ok(Statement::Assign {
        var: var.clone(),
        rhs: Box::new(Expr::BinaryOp(Div(
            Box::new(Expr::Variable(var)),
            Box::new(rhs),
        ))),
    })
}

fn assign_inc(i: &mut &str) -> PResult<Statement> {
    let var = identifier.parse_next(i)?;
    delimited(multispaces, "++", multispaces).parse_next(i)?;
    Ok(Statement::Assign {
        var: var.clone(),
        rhs: Box::new(Expr::BinaryOp(BinOp::Add(
            Box::new(Expr::Variable(var)),
            Box::new(Expr::Scalar(1.0)),
        ))),
    })
}

fn return_statement(i: &mut &str) -> PResult<Statement> {
    expr.map(|e| Statement::Return(Box::new(e))).parse_next(i)
}

fn identifier(i: &mut &str) -> PResult<String> {
    let c1 = opt("$").parse_next(i)?;
    if c1 == Some("$") {
        return Ok("$".to_string());
    };
    let c1 = take(1u8).and_then(alpha1).parse_next(i)?;
    let c2 = opt(alphanumeric0).parse_next(i)?;
    match c2 {
        Some(c) => {
            if c.len() > 1 {
                fail(i)
            } else {
                Ok(format!("{}{}", c1, c))
            }
        }
        None => Ok(c1.to_string()),
    }
}

fn variable(i: &mut &str) -> PResult<Expr> {
    identifier.map(|v| Expr::Variable(v)).parse_next(i)
}

// rot Infinity map reduce

fn function_name(i: &mut &str) -> PResult<FunctionName> {
    use FunctionName::*;
    alt((
        alt((
            "sin".map(|_| Sin),
            "cos".map(|_| Cos),
            "tan".map(|_| Tan),
            "atan2".map(|_| Atan2),
            "exp".map(|_| Exp),
            "exp2".map(|_| Exp2),
            "log".map(|_| Log),
            "log2".map(|_| Log2),
            "pow".map(|_| Pow),
            "sqrt".map(|_| Sqrt),
            "abs".map(|_| Abs),
            "sign".map(|_| Sign),
            "floor".map(|_| Floor),
            "ceil".map(|_| Ceil),
            "fract".map(|_| Fract),
            "FR".map(|_| Fract),
            "mod".map(|_| Mod),
            "min".map(|_| Min),
            "max".map(|_| Max),
            "cl".map(|_| Clamp),
            "mix".map(|_| Mix),
        )),
        alt((
            "B".map(|_| Abs),
            "SM".map(|_| Smoothstep),
            "L".map(|_| Length),
            "H".map(|_| Distance),
            "A".map(|_| AddMul),
            "D".map(|_| Dot),
            "X".map(|_| Cross),
            "N".map(|_| Normalize),
            "U".map(|_| Union),
            "G".map(|_| Intersect),
            "Z".map(|_| Floors),
            "nz".map(|_| ValueNoise),
            "don".map(|_| Torus),
            "bx2".map(|_| Box2),
            "bx3".map(|_| Box3),
            "r0".map(|_| Rot0),
            "r1".map(|_| Rot1),
            "TR".map(|_| Triangle),
            "k".map(|_| Corner),
            "sB".map(|_| SmoothAbs),
            "scl".map(|_| SmoothClamp),
        )),
        alt((
            "r0".map(|_| Rot0),
            "r1".map(|_| Rot1),
            "rG".map(|_| RoundMax),
            "rmax".map(|_| RoundMax),
            "rU".map(|_| RoundMin),
            "rmin".map(|_| RoundMin),
            "acos".map(|_| Acos),
            "asin".map(|_| Asin),
            "atan".map(|_| Atan),
            "sinh".map(|_| Sinh),
            "cosh".map(|_| Cosh),
            "tanh".map(|_| Tanh),
            "trunc".map(|_| Trunc),
            "acosh".map(|_| Acosh),
            "asinh".map(|_| Asinh),
            "atanh".map(|_| Atanh),
            "qB".map(|_| PolySmoothAbs),
            "sabs".map(|_| SmoothAbs),
            "round".map(|_| Round),
            "qcl".map(|_| PolySmoothClamp),
            "g".map(|_| FakeSine),
            // 21
        )),
    ))
    .parse_next(i)
}

fn parens(i: &mut &str) -> PResult<Expr> {
    delimited("(", expr, ")")
        .map(|e| Expr::Paren(Box::new(e)))
        .parse_next(i)
}

fn list(i: &mut &str) -> PResult<Vec<Expr>> {
    delimited(lbracket, separated(0.., expr, ","), rbracket).parse_next(i)
}

fn arg_list(i: &mut &str) -> PResult<Vec<Expr>> {
    let ls: Vec<Vec<Expr>> = separated(1.., list, ",").parse_next(i)?;
    Ok(ls.into_iter().flatten().collect())
}

fn args(i: &mut &str) -> PResult<Vec<Expr>> {
    let r = alt((arg_list, separated(0.., expr, ","))).parse_next(i)?;
    let _ = opt(comma).parse_next(i);
    Ok(r)
}

fn function(i: &mut &str) -> PResult<Expr> {
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
    fn ts() {
        // let input = "L(z,y,x)-8,L(x,z,y)-8,L(y,x,z)-8,";
        let input = "1,2,3";
        // let s: IResult<&str, Vec<&str>> = separated(0.., "a", ",").parse_peek(input);
        let s = args.parse_peek(input);
        let _ = dbg!(s);
    }

    #[test]
    fn tt() {
        // let input = "s=1;@5{@xyz{$=B($*2)-8,}s*=.5,}(L(x,y,z)-8)*s";
        // let input = "s=2.5,h=s/2,d=(s+h)/2,q=20,y-=10,[x,y]=r0(x,y),@xyz{$/=q,}c=1,t=0,@7{@xyz{$=mod($-h,s)-h,}t=d/D([x,y,z],[x,y,z]),@xyzc{$*=t,}}d=L(x,y,z)/c*2.-.025";
        // let input =  "[x,z]=r0(x,z), x+=11, z+=15, y+=10, h=exp(-1.5*B(nz(x,0,z,.1,1))), g=y-10*h-nz(x,0,z,10,1)*0.05, b = y-12, a=rU( L(x-cl(x,-2,2),b*1.3,z)-3, U( L(x+5,b-1,z)-1.7, L(x+5,b-2,B(z)-1.5)-0.8, bx3(x-5,b-1,z,0.2,0.1,0.2)-0.5, bx3(x+5,b-1,z,1.9,.1,.1)-.5, L(B(x)-3.5,b-cl(b,-4,0),B(z)-1.5)-.8,),1.5 )-nz(x,0,z,12,1)*0.15, s=(L(x>7?(mod(x,4)-2)/2:x,x<1?y:b/3+2,B(z)-1.5)-1.8)-nz(x,y,z,.5,1)*2, rG(U(a,g),-s,1)";
        // let input = "a=rU( L(x-cl(x,-2,2),b*1.3,z)-3, U( L(x+5,b-1,z)-1.7, L(x+5,b-2,B(z)-1.5)-0.8, bx3(x-5,b-1,z,0.2,0.1,0.2)-0.5, bx3(x+5,b-1,z,1.9,.1,.1)-.5, L(B(x)-3.5,b-cl(b,-4,0),B(z)-1.5)-.8,),1.5 )-nz(x,0,z,12,1)*0.15", rG(U(a,g),-s,1)";
        // let input = "x>7?(mod(x,4)-2)/2:x";
        // let input = "b=0; a=rU( L(x-cl(x,-2,2),b*1.3,z)-3, U( L(x+5,b-1,z)-1.7, L(x+5,b-2,B(z)-1.5)-0.8, bx3(x-5,b-1,z,0.2,0.1,0.2)-0.5, bx3(x+5,b-1,z,1.9,.1,.1)-.5, L(B(x)-3.5,b-cl(b,-4,0),B(z)-1.5)-.8),1.5 )";
        // let input = "s=.5,y+=6, a=k(y+22,B(z+10*g(x*.005+.2))-16)-4,b=TR(x/40+.2)*40,c; [b,y]=r1(b,y), [y,z]=r0(y,z+15), r=rU(U(@byz{bx2(B($$$)-20,bx2($,$$,23)+3,3)-.4,}),a,3)"; //, @4{ [x,y]=r1(x,y), [y,z]=r0(y,z), a=nz(x,y,z,.02/s,$+5), a=B(a)*50-3, r=rU(r,rG(r-7*s,a*s,s*2),s*2), s*=.5, } r";
        // let input = "a=rU(1,2,3,4,5,6)";
        // let input = "a=k(y+22, 2)";
        let input = "U(@xyz{L($$$,$$,$)-8,})";
        let i = &expand(input);
        dbg!(&i);
        // let input = "U(L(z,y,x)-8,L(x,z,y)-8,L(y,x,z)-8)";
        let _ = dbg!(program.parse_peek(i));
    }
    #[test]
    fn no_more_ray() {
        let input = "U(L(x+28,y-10,z+8)-12, don(x-cl(x,-15,15),y-18,z-20,10,3), bx3(x-20,y-20,z+20,8)-10, L(x+3,y-16)-4)";
        let expected = Ok((
            input,
            String::from("Sequence([Return(Function { name: Union, args: [BinaryOp(Sub(Function { name: Length, args: [BinaryOp(Add(Variable(\"x\"), Scalar(28.0))), BinaryOp(Sub(Variable(\"y\"), Scalar(10.0))), BinaryOp(Add(Variable(\"z\"), Scalar(8.0)))] }, Scalar(12.0))), Function { name: Torus, args: [BinaryOp(Sub(Variable(\"x\"), Function { name: Clamp, args: [Variable(\"x\"), Scalar(-15.0), Scalar(15.0)] })), BinaryOp(Sub(Variable(\"y\"), Scalar(18.0))), BinaryOp(Sub(Variable(\"z\"), Scalar(20.0))), Scalar(10.0), Scalar(3.0)] }, BinaryOp(Sub(Function { name: Box3, args: [BinaryOp(Sub(Variable(\"x\"), Scalar(20.0))), BinaryOp(Sub(Variable(\"y\"), Scalar(20.0))), BinaryOp(Add(Variable(\"z\"), Scalar(20.0))), Scalar(8.0)] }, Scalar(10.0))), BinaryOp(Sub(Function { name: Length, args: [BinaryOp(Add(Variable(\"x\"), Scalar(3.0))), BinaryOp(Sub(Variable(\"y\"), Scalar(16.0)))] }, Scalar(4.0)))] })])"),
        ));
        assert_eq!(
            program.map(|e| format!("{e:?}")).parse_peek(input),
            expected
        );
    }

    #[test]
    fn random_python() {
        let input = "U(don(mod(x+-8.22,4.46),mod(y+3.88,4.36),TR(z+5.17),4.19,9.74),bx3(x+8.88,y+3.14,z+-7.53,6.72,2.08,8.98)-3.77,bx3(x+-0.14,mod(y+-2.22,4.17),z+-2.84,1.88,3.59,6.38)-0.57,L(x+4.15,TR(y+-4.79),mod(z+9.16,-4.84))-0.16,don(B(x+-0.87)-4,B(y+-3.58)-3,TR(z+-8.70),9.79,8.58),L(x+9.67,B(y+6.01)-5)-4.49,L(B(x+-4.68)-4,y+-8.46)-1.78,don(x+-6.66,y+4.27,z+6.62,4.38,8.19))";
        let expected = Ok((input, String::from("Sequence([Return(Function { name: Union, args: [Function { name: Torus, args: [Function { name: Mod, args: [BinaryOp(Add(Variable(\"x\"), Scalar(-8.22))), Scalar(4.46)] }, Function { name: Mod, args: [BinaryOp(Add(Variable(\"y\"), Scalar(3.88))), Scalar(4.36)] }, Function { name: Triangle, args: [BinaryOp(Add(Variable(\"z\"), Scalar(5.17)))] }, Scalar(4.19), Scalar(9.74)] }, BinaryOp(Sub(Function { name: Box3, args: [BinaryOp(Add(Variable(\"x\"), Scalar(8.88))), BinaryOp(Add(Variable(\"y\"), Scalar(3.14))), BinaryOp(Add(Variable(\"z\"), Scalar(-7.53))), Scalar(6.72), Scalar(2.08), Scalar(8.98)] }, Scalar(3.77))), BinaryOp(Sub(Function { name: Box3, args: [BinaryOp(Add(Variable(\"x\"), Scalar(-0.14))), Function { name: Mod, args: [BinaryOp(Add(Variable(\"y\"), Scalar(-2.22))), Scalar(4.17)] }, BinaryOp(Add(Variable(\"z\"), Scalar(-2.84))), Scalar(1.88), Scalar(3.59), Scalar(6.38)] }, Scalar(0.57))), BinaryOp(Sub(Function { name: Length, args: [BinaryOp(Add(Variable(\"x\"), Scalar(4.15))), Function { name: Triangle, args: [BinaryOp(Add(Variable(\"y\"), Scalar(-4.79)))] }, Function { name: Mod, args: [BinaryOp(Add(Variable(\"z\"), Scalar(9.16))), Scalar(-4.84)] }] }, Scalar(0.16))), Function { name: Torus, args: [BinaryOp(Sub(Function { name: Abs, args: [BinaryOp(Add(Variable(\"x\"), Scalar(-0.87)))] }, Scalar(4.0))), BinaryOp(Sub(Function { name: Abs, args: [BinaryOp(Add(Variable(\"y\"), Scalar(-3.58)))] }, Scalar(3.0))), Function { name: Triangle, args: [BinaryOp(Add(Variable(\"z\"), Scalar(-8.7)))] }, Scalar(9.79), Scalar(8.58)] }, BinaryOp(Sub(Function { name: Length, args: [BinaryOp(Add(Variable(\"x\"), Scalar(9.67))), BinaryOp(Sub(Function { name: Abs, args: [BinaryOp(Add(Variable(\"y\"), Scalar(6.01)))] }, Scalar(5.0)))] }, Scalar(4.49))), BinaryOp(Sub(Function { name: Length, args: [BinaryOp(Sub(Function { name: Abs, args: [BinaryOp(Add(Variable(\"x\"), Scalar(-4.68)))] }, Scalar(4.0))), BinaryOp(Add(Variable(\"y\"), Scalar(-8.46)))] }, Scalar(1.78))), Function { name: Torus, args: [BinaryOp(Add(Variable(\"x\"), Scalar(-6.66))), BinaryOp(Add(Variable(\"y\"), Scalar(4.27))), BinaryOp(Add(Variable(\"z\"), Scalar(6.62))), Scalar(4.38), Scalar(8.19)] }] })])")));
        assert_eq!(
            program.map(|e| format!("{e:?}")).parse_peek(input),
            expected
        );
    }

    #[test]
    fn g67() {
        let input = "s=10; @1{a=sin(y),b=sin(x),c=sin(z),d=x,e=s+1,} SM(a,b,c,d,e)-5";
        let expected =Ok((input, String::from("Sequence([Assign { var: \"s\", rhs: Scalar(10.0) }, Assign { var: \"a\", rhs: Function { name: Sin, args: [Variable(\"y\")] } }, Assign { var: \"b\", rhs: Function { name: Sin, args: [Variable(\"x\")] } }, Assign { var: \"c\", rhs: Function { name: Sin, args: [Variable(\"z\")] } }, Assign { var: \"d\", rhs: Variable(\"x\") }, Assign { var: \"e\", rhs: BinaryOp(Add(Variable(\"s\"), Scalar(1.0))) }, Return(BinaryOp(Sub(Function { name: Smoothstep, args: [Variable(\"a\"), Variable(\"b\"), Variable(\"c\"), Variable(\"d\"), Variable(\"e\")] }, Scalar(5.0))))])")));
        assert_eq!(
            program.map(|e| format!("{e:?}")).parse_peek(input),
            expected
        );
    }

    #[test]
    fn pathways() {
        let input = "s=10,[x,z]=r0(x,z),[y,z]=r1(z,y),[y,x]=r0(y,x),@xyz{$m=mod($,1)-.5,}b=bx3(xm,ym,zm,.45)-.05,t=[0,2,3,1],i=1,n=(a=i++)=>nz(z,x,y,.01,a,a==1?2:1)*t[a]*100,@yxz{$+=n(),}@xz{$b=mod($,s*2)-s,}rG(b,bx2(bx2(xb,zb,s),TR((y+2)/40)*40,1,2.2)-.2,.3)-.1";
        let expected = Ok((input, String::from("Sequence([Assign { var: \"s\", rhs: Scalar(10.0) }, AssignArray { vars: [\"x\", \"z\"], rhs: Function { name: Rot0, args: [Variable(\"x\"), Variable(\"z\")] } }, AssignArray { vars: [\"y\", \"z\"], rhs: Function { name: Rot1, args: [Variable(\"z\"), Variable(\"y\")] } }, AssignArray { vars: [\"y\", \"x\"], rhs: Function { name: Rot0, args: [Variable(\"y\"), Variable(\"x\")] } }, Assign { var: \"xm\", rhs: BinaryOp(Sub(Function { name: Mod, args: [Variable(\"x\"), Scalar(1.0)] }, Scalar(0.5))) }, Assign { var: \"ym\", rhs: BinaryOp(Sub(Function { name: Mod, args: [Variable(\"y\"), Scalar(1.0)] }, Scalar(0.5))) }, Assign { var: \"zm\", rhs: BinaryOp(Sub(Function { name: Mod, args: [Variable(\"z\"), Scalar(1.0)] }, Scalar(0.5))) }, Assign { var: \"b\", rhs: BinaryOp(Sub(Function { name: Box3, args: [Variable(\"xm\"), Variable(\"ym\"), Variable(\"zm\"), Scalar(0.45)] }, Scalar(0.05))) }, Return(Variable(\"t\"))])")));
        assert_eq!(
            program.map(|e| format!("{e:?}")).parse_peek(input),
            expected
        );
    }

    #[test]
    fn quanta() {
        let input = "s=20,[x,z]=r0(x,z),[y,x]=r1(y,x),z+=17,y+=27,i=0,z+=ri(Z(x/s))*70,@xz{$-=nz(x,y,z,.1,i++)*5*i,$i=Z($/s),$=mod($,s)-s/2,}i=ri(xi,zi),j=ri(xi,floor(y/5)),d=i>.1?rU(L(x,z)-1*i-.5*(cos(y/4)+1),bx2(L(x,z)-(cos(floor(y/4))+1)*2,mod(y,4)-2,.1,.2)-.05,1):L(x,mod(y,5)-2.5,z)-G(j,0)*2";
        let expected = Ok((input, String::from("Sequence([Assign { var: \"s\", rhs: Scalar(20.0) }, AssignArray { vars: [\"x\", \"z\"], rhs: Function { name: Rot0, args: [Variable(\"x\"), Variable(\"z\")] } }, AssignArray { vars: [\"y\", \"x\"], rhs: Function { name: Rot1, args: [Variable(\"y\"), Variable(\"x\")] } }, Assign { var: \"z\", rhs: BinaryOp(Add(Variable(\"z\"), Scalar(17.0))) }, Assign { var: \"y\", rhs: BinaryOp(Add(Variable(\"y\"), Scalar(27.0))) }, Assign { var: \"i\", rhs: Scalar(0.0) }, Assign { var: \"z\", rhs: BinaryOp(Add(Variable(\"z\"), Variable(\"ri\"))) }])")));
        assert_eq!(
            program.map(|e| format!("{e:?}")).parse_peek(input),
            expected
        );
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
            input,
            String::from("Sequence([Assign { var: \"p\", rhs: BinaryOp(Sub(Function { name: Abs, args: [BinaryOp(Sub(Variable(\"y\"), Scalar(18.0)))] }, Scalar(13.0))) }, Assign { var: \"n\", rhs: BinaryOp(Mul(Function { name: ValueNoise, args: [Variable(\"x\"), Variable(\"y\"), Variable(\"z\"), Scalar(0.4), Scalar(0.0), Scalar(2.0)] }, Scalar(2.0))) }, Assign { var: \"q\", rhs: BinaryOp(Sub(Function { name: Mod, args: [Variable(\"p\"), BinaryOp(Add(Scalar(12.0), BinaryOp(Mul(Variable(\"n\"), Variable(\"z\")))))] }, Scalar(1.8))) }])"),
        ));
        assert_eq!(
            program.map(|e| format!("{e:?}")).parse_peek(input),
            expected
        );

        let input = "a = 1.0, a++";
        let expected = Ok((
            input,
            String::from("Sequence([Assign { var: \"a\", rhs: Scalar(1.0) }, Assign { var: \"a\", rhs: BinaryOp(Add(Variable(\"a\"), Scalar(1.0))) }])"),
        ));
        assert_eq!(
            program.map(|e| format!("{e:?}")).parse_peek(input),
            expected
        );

        let input = "@xyz{$=B($)-6,} L(x,y,z)-5";
        let expected = Ok((
            input,
            String::from("Sequence([Assign { var: \"x\", rhs: BinaryOp(Sub(Function { name: Abs, args: [Variable(\"x\")] }, Scalar(6.0))) }, Assign { var: \"y\", rhs: BinaryOp(Sub(Function { name: Abs, args: [Variable(\"y\")] }, Scalar(6.0))) }, Assign { var: \"z\", rhs: BinaryOp(Sub(Function { name: Abs, args: [Variable(\"z\")] }, Scalar(6.0))) }, Return(BinaryOp(Sub(Function { name: Length, args: [Variable(\"x\"), Variable(\"y\"), Variable(\"z\")] }, Scalar(5.0))))])")),
        );
        assert_eq!(
            program.map(|e| format!("{e:?}")).parse_peek(input),
            expected
        );
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

    // #[test]
    // fn scalar_test() {
    //     let input = "3.0";
    //     let expected = Ok(("", String::from("Scalar(3.0)")));
    //     assert_eq!(scalar.map(|e| format!("{e:?}")).parse_peek(input), expected);

    //     let input = " 12.1";
    //     let expected = Ok(("", String::from("Scalar(12.1)")));
    //     assert_eq!(scalar.map(|e| format!("{e:?}")).parse_peek(input), expected);

    //     let input = "537.23 ";
    //     let expected = Ok(("", String::from("Scalar(537.23)")));
    //     assert_eq!(scalar.map(|e| format!("{e:?}")).parse_peek(input), expected);

    //     let input = "  24.456     ";
    //     let expected = Ok(("", String::from("Scalar(24.456)")));
    //     assert_eq!(scalar.map(|e| format!("{e:?}")).parse_peek(input), expected);
    // }

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
