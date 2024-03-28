use crate::ast::*;
use crate::lexer::{AssignOp, Lexer, Op, Token};

pub fn parse(i: &mut &str) -> Statement {
    let mut lexer = Lexer::new(i);
    sequence(&mut lexer)
}

fn sequence(lexer: &mut Lexer) -> Statement {
    let mut statements = Vec::new();
    loop {
        let s = statement(lexer);
        statements.push(s);
        match lexer.peek() {
            Token::Semicolon | Token::Comma => {
                lexer.next();
            }
            Token::Eof => break,
            t => panic!("bad token: {:?}", t),
        }
    }
    Statement::Sequence(statements)
}

fn statement(lexer: &mut Lexer) -> Statement {
    let lhs = lexer.next();
    match lhs {
        Token::Variable(ref var) => {
            let op = lexer.peek();
            match op {
                Token::Assign(_) => assign(lhs, lexer),
                Token::Operator(op) if op == Op::Inc || op == Op::Dec => {
                    op_statement(var.to_string(), op, lexer)
                }
                _ => {
                    let e = expr(Some(lhs), lexer);
                    Statement::Return(Box::new(e))
                }
            }
        }
        Token::LBracket => {
            let vars = var_list(lexer);
            let token = lexer.next();
            assert_eq!(token, Token::Assign(AssignOp::Number));
            if let Token::LBracket = lexer.peek() {
                lexer.next();
                let mut rhs = Vec::new();
                loop {
                    if lexer.peek() == Token::RBracket {
                        lexer.next();
                        break;
                    }
                    rhs.push(expr(None, lexer));
                    match lexer.next() {
                        Token::Comma => {}
                        Token::RBracket => break,
                        t => panic!("bad token: {:?}", t),
                    }
                }
                Statement::AssignFromArray { vars, rhs }
            } else {
                let rhs = expr(None, lexer);
                Statement::AssignToArray {
                    vars,
                    rhs: Box::new(rhs),
                }
            }
        }
        _ => {
            let e = expr(Some(lhs), lexer);
            Statement::Return(Box::new(e))
        }
    }
}

fn assign(lhs: Token, lexer: &mut Lexer) -> Statement {
    let v = match lhs {
        Token::Variable(x) => x,
        // We have already peeked the token, so this should never happen.
        _ => unreachable!(),
    };
    let token = lexer.next();
    let mut rhs = Box::new(expr(None, lexer));
    match token {
        Token::Assign(a) => {
            let var = Box::new(Expr::Variable(v.clone()));
            rhs = match a {
                AssignOp::Number => rhs,
                AssignOp::Add => Box::new(Expr::BinaryOp(BinOp::Add(var, rhs))),
                AssignOp::Sub => Box::new(Expr::BinaryOp(BinOp::Sub(var, rhs))),
                AssignOp::Mul => Box::new(Expr::BinaryOp(BinOp::Mul(var, rhs))),
                AssignOp::Div => Box::new(Expr::BinaryOp(BinOp::Div(var, rhs))),
            }
        }
        t => panic!("bad token: {:?}", t),
    }
    Statement::Assign {
        var: v.clone(),
        rhs,
    }
}

fn op_statement(var: String, op: Op, lexer: &mut Lexer) -> Statement {
    match op {
        Op::Inc => {
            _ = lexer.next();
            Statement::Assign {
                var: var.to_string(),
                rhs: Box::new(Expr::BinaryOp(BinOp::Add(
                    Box::new(Expr::Variable(var.to_string())),
                    Box::new(Expr::Number(1.0)),
                ))),
            }
        }
        Op::Dec => {
            _ = lexer.next();
            Statement::Assign {
                var: var.to_string(),
                rhs: Box::new(Expr::BinaryOp(BinOp::Sub(
                    Box::new(Expr::Variable(var.to_string())),
                    Box::new(Expr::Number(1.0)),
                ))),
            }
        }
        _ => unreachable!("op: {:?} slipped through", op),
    }
}

fn var_list(lexer: &mut Lexer) -> Vec<String> {
    let mut vars = Vec::new();
    loop {
        let mut token = lexer.next();
        if let Token::Variable(v) = token {
            vars.push(v);
            token = lexer.next();
            if token == Token::RBracket {
                break;
            }
            assert_eq!(token, Token::Comma);
        }
    }
    vars
}

fn expr(token: Option<Token>, lexer: &mut Lexer) -> Expr {
    expr_bp(token, lexer, 0)
}

fn expr_bp(token: Option<Token>, lexer: &mut Lexer, min_bp: u8) -> Expr {
    use Op::*;
    let token = if let Some(t) = token { t } else { lexer.next() };
    let mut lhs = match token {
        Token::ScalarVal(it) => Expr::Number(it),
        Token::LParen => {
            let lhs = expr(None, lexer);
            assert_eq!(lexer.next(), Token::RParen);
            lhs
        }
        Token::Operator(op) => {
            let r_bp = prefix_binding_power(Token::Operator(op));
            let rhs = expr_bp(None, lexer, r_bp);
            match op {
                Sub => Expr::Negate(Box::new(rhs)),
                Add => rhs,
                _ => panic!("bad op: {:?}", op),
            }
        }
        Token::Function(name) => {
            assert_eq!(lexer.next(), Token::LParen);
            let mut args = Vec::new();
            loop {
                if lexer.peek() == Token::LBracket {
                    lexer.next();
                }
                if lexer.peek() == Token::RParen {
                    lexer.next();
                    break;
                }
                args.push(expr(None, lexer));
                match lexer.next() {
                    Token::RParen => break,
                    Token::Comma => {
                        if lexer.peek() == Token::RParen {
                            lexer.next();
                            break;
                        }
                    }
                    Token::RBracket => {
                        if lexer.peek() == Token::Comma {
                            lexer.next();
                        }
                    }
                    t => panic!("bad token: {:?}", t),
                }
            }
            Expr::Function { name, args }
        }
        Token::Variable(name) => Expr::Variable(name),
        t => panic!("bad token: {:?}", t),
    };

    loop {
        let op = match lexer.peek() {
            Token::Eof => break,
            t => t,
        };
        if let Some(l_bp) = postfix_binding_power(op.clone()) {
            if l_bp < min_bp {
                break;
            }
            if let Expr::Variable(v) = lhs {
                lhs = if let Token::Operator(Inc) = lexer.next() {
                    Expr::Assign(AssignExpr::Inc(v))
                } else {
                    Expr::Assign(AssignExpr::Dec(v))
                };
            }
            continue;
        };

        if let Some((l_bp, r_bp)) = infix_binding_power(op.clone()) {
            if l_bp < min_bp {
                break;
            }
            lexer.next();
            lhs = match op {
                Token::Operator(op) => {
                    let rhs = expr_bp(None, lexer, r_bp);
                    match op {
                        Add => Expr::BinaryOp(BinOp::Add(Box::new(lhs), Box::new(rhs))),
                        Sub => Expr::BinaryOp(BinOp::Sub(Box::new(lhs), Box::new(rhs))),
                        Mul => Expr::BinaryOp(BinOp::Mul(Box::new(lhs), Box::new(rhs))),
                        Div => Expr::BinaryOp(BinOp::Div(Box::new(lhs), Box::new(rhs))),
                        Pow => Expr::BinaryOp(BinOp::Pow(Box::new(lhs), Box::new(rhs))),
                        Eq => Expr::BinaryOp(BinOp::Eq(Box::new(lhs), Box::new(rhs))),
                        NotEq => Expr::BinaryOp(BinOp::NotEq(Box::new(lhs), Box::new(rhs))),
                        Greater => Expr::BinaryOp(BinOp::Greater(Box::new(lhs), Box::new(rhs))),
                        GreaterEq => Expr::BinaryOp(BinOp::GreaterEq(Box::new(lhs), Box::new(rhs))),
                        Less => Expr::BinaryOp(BinOp::Less(Box::new(lhs), Box::new(rhs))),
                        LessEq => Expr::BinaryOp(BinOp::LessEq(Box::new(lhs), Box::new(rhs))),
                        And => Expr::BinaryOp(BinOp::And(Box::new(lhs), Box::new(rhs))),
                        Or => Expr::BinaryOp(BinOp::Or(Box::new(lhs), Box::new(rhs))),
                        _ => panic!("bad op: {:?}", op),
                    }
                }
                Token::Then => {
                    let mhs = expr_bp(None, lexer, 0);
                    assert_eq!(lexer.next(), Token::Else);
                    let rhs = expr_bp(None, lexer, r_bp);
                    Expr::TernaryOp(Box::new(lhs), Box::new(mhs), Box::new(rhs))
                }
                t => panic!("bad token: {:?}", t),
            };
            continue;
        }

        break;
    }
    lhs
}

fn prefix_binding_power(op: Token) -> u8 {
    use Op::*;
    match op {
        Token::Operator(Add) | Token::Operator(Sub) => 17,
        Token::Operator(Inc) | Token::Operator(Dec) => 19,
        _ => panic!("bad op: {:?}", op),
    }
}

fn infix_binding_power(op: Token) -> Option<(u8, u8)> {
    use Op::*;
    let res = match op {
        Token::Then => (2, 1),
        Token::Operator(Or) => (3, 4),
        Token::Operator(And) => (5, 6),
        Token::Operator(Eq) | Token::Operator(NotEq) => (8, 7),
        Token::Operator(Greater)
        | Token::Operator(Less)
        | Token::Operator(GreaterEq)
        | Token::Operator(LessEq) => (9, 10),
        Token::Operator(Add) | Token::Operator(Sub) => (11, 12),
        Token::Operator(Mul) | Token::Operator(Div) => (13, 14),
        Token::Operator(Pow) => (16, 15),
        _ => return None,
    };
    Some(res)
}

fn postfix_binding_power(op: Token) -> Option<u8> {
    use Op::*;
    match op {
        Token::Operator(Inc) | Token::Operator(Dec) => Some(21),
        _ => None,
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn show() {
        let mut i = "[x,y,z] = [y, z, x]";
        let s = parse(&mut i);
        dbg!(s);
    }

    #[test]
    fn expr_tests() {
        let mut i = "-+-+-1 + +2 * 3";
        let s = parse(&mut i);
        dbg!(s);

        let mut i = "3 > 2 ? 4 + 1 : G(1,x,3)";
        let s = parse(&mut i);
        dbg!(s);
    }

    #[test]
    fn statement_tests() {
        let mut i = "sin(U(x,y,z *2))";
        let s = parse(&mut i);
        dbg!(s);

        let mut i = "x += y / 2; U(x,y,z)";
        let s = parse(&mut i);
        dbg!(s);

        let mut i = "x >= y && y < z";
        let s = parse(&mut i);
        dbg!(s);

        let mut i = "s = z ** 2";
        let s = parse(&mut i);
        dbg!(s);

        let mut i = "[a, b,c] = 2 * tan(x)";
        let s = parse(&mut i);
        dbg!(s);

        let mut i = "t = G(z++)";
        let s = parse(&mut i);
        dbg!(s);

        let mut i = "i++; j--";
        let s = parse(&mut i);
        dbg!(s);
    }

    #[test]
    fn no_more_ray() {
        let mut i = "U(L(x+28,y-10,z+8)-12, don(x-cl(x,-15,15),y-18,z-20,10,3), bx3(x-20,y-20,z+20,8)-10, L(x+3,y-16)-4)";
        let expected = String::from("Sequence([Return(Function { name: Union, args: [BinaryOp(Sub(Function { name: Length, args: [BinaryOp(Add(Variable(\"x\"), Number(28.0))), BinaryOp(Sub(Variable(\"y\"), Number(10.0))), BinaryOp(Add(Variable(\"z\"), Number(8.0)))] }, Number(12.0))), Function { name: Torus, args: [BinaryOp(Sub(Variable(\"x\"), Function { name: Clamp, args: [Variable(\"x\"), Negate(Number(15.0)), Number(15.0)] })), BinaryOp(Sub(Variable(\"y\"), Number(18.0))), BinaryOp(Sub(Variable(\"z\"), Number(20.0))), Number(10.0), Number(3.0)] }, BinaryOp(Sub(Function { name: Box3, args: [BinaryOp(Sub(Variable(\"x\"), Number(20.0))), BinaryOp(Sub(Variable(\"y\"), Number(20.0))), BinaryOp(Add(Variable(\"z\"), Number(20.0))), Number(8.0)] }, Number(10.0))), BinaryOp(Sub(Function { name: Length, args: [BinaryOp(Add(Variable(\"x\"), Number(3.0))), BinaryOp(Sub(Variable(\"y\"), Number(16.0)))] }, Number(4.0)))] })])");
        let s = parse(&mut i);
        let result = format!("{s:?}");
        assert_eq!(result, expected);
    }

    #[test]
    fn random_python() {
        let mut i = "U(don(mod(x+-8.22,4.46),mod(y+3.88,4.36),TR(z+5.17),4.19,9.74),bx3(x+8.88,y+3.14,z+-7.53,6.72,2.08,8.98)-3.77,bx3(x+-0.14,mod(y+-2.22,4.17),z+-2.84,1.88,3.59,6.38)-0.57,L(x+4.15,TR(y+-4.79),mod(z+9.16,-4.84))-0.16,don(B(x+-0.87)-4,B(y+-3.58)-3,TR(z+-8.70),9.79,8.58),L(x+9.67,B(y+6.01)-5)-4.49,L(B(x+-4.68)-4,y+-8.46)-1.78,don(x+-6.66,y+4.27,z+6.62,4.38,8.19))";
        let expected = String::from("Sequence([Return(Function { name: Union, args: [Function { name: Torus, args: [Function { name: Mod, args: [BinaryOp(Add(Variable(\"x\"), Negate(Number(8.22)))), Number(4.46)] }, Function { name: Mod, args: [BinaryOp(Add(Variable(\"y\"), Number(3.88))), Number(4.36)] }, Function { name: Triangle, args: [BinaryOp(Add(Variable(\"z\"), Number(5.17)))] }, Number(4.19), Number(9.74)] }, BinaryOp(Sub(Function { name: Box3, args: [BinaryOp(Add(Variable(\"x\"), Number(8.88))), BinaryOp(Add(Variable(\"y\"), Number(3.14))), BinaryOp(Add(Variable(\"z\"), Negate(Number(7.53)))), Number(6.72), Number(2.08), Number(8.98)] }, Number(3.77))), BinaryOp(Sub(Function { name: Box3, args: [BinaryOp(Add(Variable(\"x\"), Negate(Number(0.14)))), Function { name: Mod, args: [BinaryOp(Add(Variable(\"y\"), Negate(Number(2.22)))), Number(4.17)] }, BinaryOp(Add(Variable(\"z\"), Negate(Number(2.84)))), Number(1.88), Number(3.59), Number(6.38)] }, Number(0.57))), BinaryOp(Sub(Function { name: Length, args: [BinaryOp(Add(Variable(\"x\"), Number(4.15))), Function { name: Triangle, args: [BinaryOp(Add(Variable(\"y\"), Negate(Number(4.79))))] }, Function { name: Mod, args: [BinaryOp(Add(Variable(\"z\"), Number(9.16))), Negate(Number(4.84))] }] }, Number(0.16))), Function { name: Torus, args: [BinaryOp(Sub(Function { name: Abs, args: [BinaryOp(Add(Variable(\"x\"), Negate(Number(0.87))))] }, Number(4.0))), BinaryOp(Sub(Function { name: Abs, args: [BinaryOp(Add(Variable(\"y\"), Negate(Number(3.58))))] }, Number(3.0))), Function { name: Triangle, args: [BinaryOp(Add(Variable(\"z\"), Negate(Number(8.7))))] }, Number(9.79), Number(8.58)] }, BinaryOp(Sub(Function { name: Length, args: [BinaryOp(Add(Variable(\"x\"), Number(9.67))), BinaryOp(Sub(Function { name: Abs, args: [BinaryOp(Add(Variable(\"y\"), Number(6.01)))] }, Number(5.0)))] }, Number(4.49))), BinaryOp(Sub(Function { name: Length, args: [BinaryOp(Sub(Function { name: Abs, args: [BinaryOp(Add(Variable(\"x\"), Negate(Number(4.68))))] }, Number(4.0))), BinaryOp(Add(Variable(\"y\"), Negate(Number(8.46))))] }, Number(1.78))), Function { name: Torus, args: [BinaryOp(Add(Variable(\"x\"), Negate(Number(6.66)))), BinaryOp(Add(Variable(\"y\"), Number(4.27))), BinaryOp(Add(Variable(\"z\"), Number(6.62))), Number(4.38), Number(8.19)] }] })])");
        let s = parse(&mut i);
        let result = format!("{s:?}");
        assert_eq!(result, expected);
    }

    #[test]
    fn g67() {
        let mut i = "s=10; @1{a=sin(y),b=sin(x),c=sin(z),d=x,e=s+1,} SM(a,b,c,d,e)-5";
        let expected =String::from("Sequence([Assign { var: \"s\", rhs: Number(10.0) }, Assign { var: \"a\", rhs: Function { name: Sin, args: [Variable(\"y\")] } }, Assign { var: \"b\", rhs: Function { name: Sin, args: [Variable(\"x\")] } }, Assign { var: \"c\", rhs: Function { name: Sin, args: [Variable(\"z\")] } }, Assign { var: \"d\", rhs: Variable(\"x\") }, Assign { var: \"e\", rhs: BinaryOp(Add(Variable(\"s\"), Number(1.0))) }, Return(BinaryOp(Sub(Function { name: Smoothstep, args: [Variable(\"a\"), Variable(\"b\"), Variable(\"c\"), Variable(\"d\"), Variable(\"e\")] }, Number(5.0))))])");
        let s = parse(&mut i);
        let result = format!("{s:?}");
        assert_eq!(result, expected);
    }

    #[test]
    fn quanta() {
        let mut i = "s=20,[x,z]=r0(x,z),[y,x]=r1(y,x),z+=17,y+=27,i=0,z+=ri(Z(x/s))*70,@xz{$-=nz(x,y,z,.1,i++)*5*i,$i=Z($/s),$=mod($,s)-s/2,}i=ri(xi,zi),j=ri(xi,floor(y/5)),d=i>.1?rU(L(x,z)-1*i-.5*(cos(y/4)+1),bx2(L(x,z)-(cos(floor(y/4))+1)*2,mod(y,4)-2,.1,.2)-.05,1):L(x,mod(y,5)-2.5,z)-G(j,0)*2";
        let expected = String::from("Sequence([Assign { var: \"s\", rhs: Number(20.0) }, AssignToArray { vars: [\"x\", \"z\"], rhs: Function { name: Rot0, args: [Variable(\"x\"), Variable(\"z\")] } }, AssignToArray { vars: [\"y\", \"x\"], rhs: Function { name: Rot1, args: [Variable(\"y\"), Variable(\"x\")] } }, Assign { var: \"z\", rhs: BinaryOp(Add(Variable(\"z\"), Number(17.0))) }, Assign { var: \"y\", rhs: BinaryOp(Add(Variable(\"y\"), Number(27.0))) }, Assign { var: \"i\", rhs: Number(0.0) }, Assign { var: \"z\", rhs: BinaryOp(Add(Variable(\"z\"), BinaryOp(Mul(Function { name: Hash, args: [Function { name: Floor, args: [BinaryOp(Div(Variable(\"x\"), Variable(\"s\")))] }] }, Number(70.0))))) }, Assign { var: \"x\", rhs: BinaryOp(Sub(Variable(\"x\"), BinaryOp(Mul(BinaryOp(Mul(Function { name: ValueNoise, args: [Variable(\"x\"), Variable(\"y\"), Variable(\"z\"), Number(0.1), Assign(Inc(\"i\"))] }, Number(5.0))), Variable(\"i\"))))) }, Assign { var: \"xi\", rhs: Function { name: Floor, args: [BinaryOp(Div(Variable(\"x\"), Variable(\"s\")))] } }, Assign { var: \"x\", rhs: BinaryOp(Sub(Function { name: Mod, args: [Variable(\"x\"), Variable(\"s\")] }, BinaryOp(Div(Variable(\"s\"), Number(2.0))))) }, Assign { var: \"z\", rhs: BinaryOp(Sub(Variable(\"z\"), BinaryOp(Mul(BinaryOp(Mul(Function { name: ValueNoise, args: [Variable(\"x\"), Variable(\"y\"), Variable(\"z\"), Number(0.1), Assign(Inc(\"i\"))] }, Number(5.0))), Variable(\"i\"))))) }, Assign { var: \"zi\", rhs: Function { name: Floor, args: [BinaryOp(Div(Variable(\"z\"), Variable(\"s\")))] } }, Assign { var: \"z\", rhs: BinaryOp(Sub(Function { name: Mod, args: [Variable(\"z\"), Variable(\"s\")] }, BinaryOp(Div(Variable(\"s\"), Number(2.0))))) }, Assign { var: \"i\", rhs: Function { name: Hash, args: [Variable(\"xi\"), Variable(\"zi\")] } }, Assign { var: \"j\", rhs: Function { name: Hash, args: [Variable(\"xi\"), Function { name: Floor, args: [BinaryOp(Div(Variable(\"y\"), Number(5.0)))] }] } }, Assign { var: \"d\", rhs: TernaryOp(BinaryOp(Greater(Variable(\"i\"), Number(0.1))), Function { name: RoundMin, args: [BinaryOp(Sub(BinaryOp(Sub(Function { name: Length, args: [Variable(\"x\"), Variable(\"z\")] }, BinaryOp(Mul(Number(1.0), Variable(\"i\"))))), BinaryOp(Mul(Number(0.5), BinaryOp(Add(Function { name: Cos, args: [BinaryOp(Div(Variable(\"y\"), Number(4.0)))] }, Number(1.0))))))), BinaryOp(Sub(Function { name: Box2, args: [BinaryOp(Sub(Function { name: Length, args: [Variable(\"x\"), Variable(\"z\")] }, BinaryOp(Mul(BinaryOp(Add(Function { name: Cos, args: [Function { name: Floor, args: [BinaryOp(Div(Variable(\"y\"), Number(4.0)))] }] }, Number(1.0))), Number(2.0))))), BinaryOp(Sub(Function { name: Mod, args: [Variable(\"y\"), Number(4.0)] }, Number(2.0))), Number(0.1), Number(0.2)] }, Number(0.05))), Number(1.0)] }, BinaryOp(Sub(Function { name: Length, args: [Variable(\"x\"), BinaryOp(Sub(Function { name: Mod, args: [Variable(\"y\"), Number(5.0)] }, Number(2.5))), Variable(\"z\")] }, BinaryOp(Mul(Function { name: Intersect, args: [Variable(\"j\"), Number(0.0)] }, Number(2.0)))))) }])");
        let s = parse(&mut i);
        let result = format!("{s:?}");
        assert_eq!(result, expected);
    }

    #[test]
    fn sponge() {
        let mut i = "k(r,-U(@xyz{bx2($,$$,9),}))";
        let mut i_str: &str = &crate::expand::expand(i);
        let tokens = crate::lexer::lex(&mut i_str).expect("lexer failed");
        dbg!(&tokens);
        let s = parse(&mut i);
        dbg!(s);
    }

    #[test]
    fn temple() {
        let mut i = "d=99, [y,z]=r1(y,z), f=y+B(nz(x,z,1,.0,3))*5, @5{ [x,y,z]=[y,z,x], [x,z]=r0(x,z), [x,z]=r1(x,z), @xyz{$=sB($,2)-3,} d=rU(d, don(y,z,x,5,.5+$*.2), 1), } rU(f, L(d,nz(x,y,z,.5,1))-.1, .5)";
        let s = parse(&mut i);
        dbg!(s);
    }
}
