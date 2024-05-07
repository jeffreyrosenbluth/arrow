use core::panic;

use crate::ast::FunctionName;
use crate::expand::expand;

#[derive(Clone, Debug, PartialEq)]
pub enum TokenType {
    ScalarVal(f32),
    Operator(Op),
    Assign(AssignOp),
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Comma,
    Semicolon,
    Then,
    Else,
    Variable(String),
    Function(FunctionName),
    Eof,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Eq,
    NotEq,
    Greater,
    GreaterEq,
    Less,
    LessEq,
    And,
    Or,
    Not,
    Inc,
    Dec,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum AssignOp {
    Number,
    Add,
    Sub,
    Mul,
    Div,
}

impl core::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use TokenType::*;
        match self {
            ScalarVal(v) => write!(f, "{}", v),
            Variable(v) => write!(f, "{}", v),
            Function(e) => write!(f, "{:?}", e),
            a => write!(f, "{:?}", a),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub pos: usize,
}

impl Token {
    pub fn new(token_type: TokenType, pos: usize) -> Self {
        Self { token_type, pos }
    }
}

pub struct Lexer {
    pub tokens: Vec<Token>,
    pub token_num: usize,
}

impl Lexer {
    pub fn new(input: &mut &str) -> Self {
        let mut i_str: &str = &expand(input);
        let mut tokens = lex(&mut i_str);
        tokens.reverse();
        Lexer {
            tokens: tokens,
            token_num: 0,
        }
    }

    pub fn next(&mut self) -> Token {
        self.token_num += 1;
        self.tokens.pop().unwrap_or(Token::new(TokenType::Eof, 0))
    }

    pub fn peek(&mut self) -> Token {
        self.tokens
            .last()
            .cloned()
            .unwrap_or(Token::new(TokenType::Eof, 0))
    }
}

pub fn lex(i: &str) -> Vec<Token> {
    use TokenType::*;
    let funcs = vec![
        "sin", "cos", "tan", "atan2", "exp", "exp2", "log", "log2", "pow", "sqrt", "abs", "sign",
        "floor", "ceil", "fract", "FR", "mod", "min", "max", "cl", "mix", "B", "SM", "L", "H", "A",
        "D", "X", "N", "U", "G", "Z", "nz", "don", "bx2", "bx3", "r0", "r1", "TR", "k", "sB",
        "scl", "rG", "rmax", "rU", "rmin", "acos", "asin", "atan", "sinh", "cosh", "tanh", "trunc",
        "asinh", "acosh", "atanh", "qB", "sabs", "round", "qcl", "g", "ri", "rot",
    ];
    let mut tokens = Vec::new();
    let mut cs = i.chars().enumerate().peekable();
    while let Some((n, c)) = cs.next() {
        match c {
            '+' => {
                if cs.peek().map_or(false, |&(_, c)| c == '=') {
                    cs.next();
                    tokens.push(Token::new(Assign(AssignOp::Add), n))
                } else if cs.peek().map_or(false, |&(_, c)| c == '+') {
                    cs.next();
                    tokens.push(Token::new(Operator(Op::Inc), n))
                } else {
                    tokens.push(Token::new(Operator(Op::Add), n))
                }
            }
            '-' => {
                if cs.peek().map_or(false, |&(_, c)| c == '=') {
                    cs.next();
                    tokens.push(Token::new(Assign(AssignOp::Sub), n))
                } else if cs.peek().map_or(false, |&(_, c)| c == '-') {
                    cs.next();
                    tokens.push(Token::new(Operator(Op::Dec), n))
                } else {
                    tokens.push(Token::new(Operator(Op::Sub), n))
                }
            }
            '*' => {
                if cs.peek().map_or(false, |&(_, c)| c == '=') {
                    cs.next();
                    tokens.push(Token::new(Assign(AssignOp::Mul), n))
                } else if cs.peek().map_or(false, |&(_, c)| c == '*') {
                    cs.next();
                    tokens.push(Token::new(Operator(Op::Pow), n))
                } else {
                    tokens.push(Token::new(Operator(Op::Mul), n))
                }
            }
            '/' => {
                if cs.peek().map_or(false, |&(_, c)| c == '=') {
                    cs.next();
                    tokens.push(Token::new(Assign(AssignOp::Div), n))
                } else {
                    tokens.push(Token::new(Operator(Op::Div), n))
                }
            }
            '=' => {
                if cs.peek().map_or(false, |&(_, c)| c == '=') {
                    cs.next();
                    tokens.push(Token::new(Operator(Op::Eq), n))
                } else {
                    tokens.push(Token::new(Assign(AssignOp::Number), n))
                }
            }
            '(' => tokens.push(Token::new(LParen, n)),
            ')' => tokens.push(Token::new(RParen, n)),
            '{' => tokens.push(Token::new(LBrace, n)),
            '}' => tokens.push(Token::new(RBrace, n)),
            '[' => tokens.push(Token::new(LBracket, n)),
            ']' => tokens.push(Token::new(RBracket, n)),
            ',' => tokens.push(Token::new(Comma, n)),
            ';' => tokens.push(Token::new(Semicolon, n)),
            '>' => {
                if cs.peek().map_or(false, |&(_, c)| c == '=') {
                    cs.next();
                    tokens.push(Token::new(Operator(Op::GreaterEq), n))
                } else {
                    tokens.push(Token::new(Operator(Op::Greater), n))
                }
            }
            '<' => {
                if cs.peek().map_or(false, |&(_, c)| c == '=') {
                    cs.next();
                    tokens.push(Token::new(Operator(Op::LessEq), n))
                } else {
                    tokens.push(Token::new(Operator(Op::Less), n))
                }
            }
            '?' => tokens.push(Token::new(Then, n)),
            ':' => tokens.push(Token::new(Else, n)),
            '|' => {
                if cs.peek().map_or(false, |&(_, c)| c == '|') {
                    cs.next();
                    tokens.push(Token::new(Operator(Op::Or), n))
                } else {
                    panic!("Unknown operator")
                }
            }
            '&' => {
                if cs.peek().map_or(false, |&(_, c)| c == '&') {
                    cs.next();
                    tokens.push(Token::new(Operator(Op::And), n))
                } else {
                    panic!("Unknown operator")
                }
            }
            '!' => {
                if cs.peek().map_or(false, |&(_, c)| c == '=') {
                    cs.next();
                    tokens.push(Token::new(Operator(Op::NotEq), n))
                } else {
                    tokens.push(Token::new(Operator(Op::Not), n))
                }
            }
            c if c.is_digit(10) || c == '.' => {
                let mut number = String::new();
                number.push(c);
                while let Some(&(_, c)) = cs.peek() {
                    if c.is_digit(10) || c == '.' {
                        number.push(c);
                        cs.next();
                    } else {
                        break;
                    }
                }
                match number.parse::<f32>() {
                    Ok(v) => tokens.push(Token::new(ScalarVal(v), n)),
                    Err(_) => continue,
                }
            }
            c if c.is_alphabetic() => {
                let mut name = String::new();
                name.push(c);
                while let Some(&(_, c)) = cs.peek() {
                    if c.is_alphanumeric() {
                        name.push(c);
                        cs.next();
                    } else {
                        break;
                    }
                }
                if funcs.contains(&name.as_str()) {
                    tokens.push(Token::new(Function(get_fname(&name.as_str())), n));
                } else {
                    tokens.push(Token::new(Variable(name), n));
                }
            }
            c if c.is_whitespace() => continue,
            c => panic!("Unknown character {}", c),
        }
    }
    tokens
}

fn get_fname(func: &str) -> FunctionName {
    match func {
        "sin" => FunctionName::Sin,
        "cos" => FunctionName::Cos,
        "tan" => FunctionName::Tan,
        "atan2" => FunctionName::Atan2,
        "exp" => FunctionName::Exp,
        "exp2" => FunctionName::Exp2,
        "log" => FunctionName::Log,
        "log2" => FunctionName::Log2,
        "pow" => FunctionName::Pow,
        "sqrt" => FunctionName::Sqrt,
        "abs" => FunctionName::Abs,
        "sign" => FunctionName::Sign,
        "floor" => FunctionName::Floor,
        "ceil" => FunctionName::Ceil,
        "fract" => FunctionName::Fract,
        "FR" => FunctionName::Fract,
        "mod" => FunctionName::Mod,
        "min" => FunctionName::Min,
        "max" => FunctionName::Max,
        "cl" => FunctionName::Clamp,
        "mix" => FunctionName::Mix,
        "B" => FunctionName::Abs,
        "SM" => FunctionName::Smoothstep,
        "L" => FunctionName::Length,
        "H" => FunctionName::Distance,
        "A" => FunctionName::AddMul,
        "D" => FunctionName::Dot,
        "X" => FunctionName::Cross,
        "N" => FunctionName::Normalize,
        "U" => FunctionName::Union,
        "G" => FunctionName::Intersect,
        "Z" => FunctionName::Floor,
        "nz" => FunctionName::ValueNoise,
        "don" => FunctionName::Torus,
        "bx2" => FunctionName::Box2,
        "bx3" => FunctionName::Box3,
        "r0" => FunctionName::Rot0,
        "r1" => FunctionName::Rot1,
        "TR" => FunctionName::Triangle,
        "k" => FunctionName::Corner,
        "sB" => FunctionName::SmoothAbs,
        "scl" => FunctionName::SmoothClamp,
        "rG" => FunctionName::RoundMax,
        "rmax" => FunctionName::RoundMax,
        "rU" => FunctionName::RoundMin,
        "rmin" => FunctionName::RoundMin,
        "acos" => FunctionName::Acos,
        "asin" => FunctionName::Asin,
        "atan" => FunctionName::Atan,
        "sinh" => FunctionName::Sinh,
        "cosh" => FunctionName::Cosh,
        "tanh" => FunctionName::Tanh,
        "trunc" => FunctionName::Trunc,
        "asinh" => FunctionName::Asinh,
        "acosh" => FunctionName::Acosh,
        "atanh" => FunctionName::Atanh,
        "qB" => FunctionName::PolySmoothAbs,
        "sabs" => FunctionName::SmoothAbs,
        "round" => FunctionName::Round,
        "qcl" => FunctionName::PolySmoothClamp,
        "g" => FunctionName::FakeSine,
        "ri" => FunctionName::Hash,
        "rot" => FunctionName::Rot,
        _ => panic!("Unknown function"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn show() {
        let input = "ri(xi,yi,zi)>.4&&L(xi,yi,zi)>3?L(xm,ym,zm)-2:10";
        // let input = "i=mod(floor(x/8)+floor(z/8),2),x=mod(x,8)-4,z=mod(z,8)-4,a=L(x,y,z)-1,q=L(x,z),b=max(D([1,.3],[q,y]),-5-y),a=rU(a,b,1),y+=1,a=rU(a,L(x,y*5,z)-.8,1),y+=3,a=rU(a,L(x,y*2,z)-1,.5),y+=1,a=rU(a,L(x,y*3,z)-1.7,0.1),min(a,y+.5*i*nz(x,y,z,8,0))";
        // let input = "a *= 1 * 2 = 3 ** 4 + FR(3.4)";
        let i = expand(input);
        let _ = dbg!(lex(&i));
    }
}
