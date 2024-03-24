use crate::ast::*;
use crate::lexer::{AssignOp, Lexer, Op, Token};

fn assign(lexer: &mut Lexer) -> Statement {
    let lhs = lexer.next();
    let v = match lhs {
        Token::Variable(x) => x,
        _ => unreachable!(),
    };
    let token = lexer.next();
    let rhs = expr(lexer);
    match token {
        Token::Assign(AssignOp::Scalar) => Statement::Assign {
            var: v.clone(),
            rhs: Box::new(rhs),
        },
        Token::Assign(AssignOp::Add) => Statement::Assign {
            var: v.clone(),
            rhs: Box::new(Expr::BinaryOp(BinOp::Add(
                Box::new(Expr::Variable(v.clone())),
                Box::new(rhs),
            ))),
        },
        Token::Assign(AssignOp::Sub) => Statement::Assign {
            var: v.clone(),
            rhs: Box::new(Expr::BinaryOp(BinOp::Sub(
                Box::new(Expr::Variable(v.clone())),
                Box::new(rhs),
            ))),
        },
        Token::Assign(AssignOp::Mul) => Statement::Assign {
            var: v.clone(),
            rhs: Box::new(Expr::BinaryOp(BinOp::Mul(
                Box::new(Expr::Variable(v.clone())),
                Box::new(rhs),
            ))),
        },
        Token::Assign(AssignOp::Div) => Statement::Assign {
            var: v.clone(),
            rhs: Box::new(Expr::BinaryOp(BinOp::Div(
                Box::new(Expr::Variable(v.clone())),
                Box::new(rhs),
            ))),
        },
        t => panic!("bad token: {:?}", t),
    }
}

fn sequence(lexer: &mut Lexer) -> Statement {
    let mut statements = Vec::new();
    loop {
        let s = statement(lexer);
        statements.push(s);
        match lexer.peek() {
            Token::Semicolon => {
                lexer.next();
            }
            Token::Eof => break,
            t => panic!("bad token: {:?}", t),
        }
    }
    Statement::Sequence(statements)
}

fn statement(lexer: &mut Lexer) -> Statement {
    let lhs = lexer.peek();
    if let Token::Variable(_) = lhs {
        assign(lexer)
    } else {
        let e = expr(lexer);
        Statement::Return(Box::new(e))
    }
}

pub fn parse(i: &mut &str) -> Statement {
    let mut lexer = Lexer::new(i);
    sequence(&mut lexer)
}

fn expr(lexer: &mut Lexer) -> Expr {
    expr_bp(lexer, 0)
}

fn expr_bp(lexer: &mut Lexer, min_bp: u8) -> Expr {
    use Op::*;
    let mut lhs = match lexer.next() {
        Token::ScalarVal(it) => Expr::Number(it),
        Token::LParen => {
            let lhs = expr(lexer);
            assert_eq!(lexer.next(), Token::RParen);
            lhs
        }
        Token::Operator(op) => {
            let r_bp = prefix_binding_power(Token::Operator(op));
            let rhs = expr_bp(lexer, r_bp);
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
                args.push(expr(lexer));
                match lexer.next() {
                    Token::RParen => break,
                    Token::Comma => {}
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

        if let Some((l_bp, r_bp)) = infix_binding_power(op.clone()) {
            if l_bp < min_bp {
                break;
            }
            lexer.next();
            lhs = match op {
                Token::Operator(op) => {
                    let rhs = expr_bp(lexer, r_bp);
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
                    let mhs = expr_bp(lexer, 0);
                    assert_eq!(lexer.next(), Token::Else);
                    let rhs = expr_bp(lexer, r_bp);
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
        Token::Operator(Add) | Token::Operator(Sub) => 9,
        _ => panic!("bad op: {:?}", op),
    }
}

fn infix_binding_power(op: Token) -> Option<(u8, u8)> {
    use Op::*;
    let res = match op {
        Token::Then => (2, 1),
        Token::Operator(Eq) | Token::Operator(NotEq) => (4, 3),
        Token::Operator(Greater)
        | Token::Operator(Less)
        | Token::Operator(GreaterEq)
        | Token::Operator(LessEq) => (5, 6),
        Token::Operator(Add) | Token::Operator(Sub) => (7, 8),
        Token::Operator(Mul) | Token::Operator(Div) => (9, 10),
        _ => return None,
    };
    Some(res)
}

#[cfg(test)]

mod tests {
    use super::*;

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
        let mut i = "sin(U(x,y,z))";
        let s = parse(&mut i);
        dbg!(s);

        let mut i = "x += y / 2; U(x,y,z)";
        let s = parse(&mut i);
        dbg!(s);

        let mut i = "x >= y && y < z";
        let s = parse(&mut i);
        dbg!(s);
    }
}
