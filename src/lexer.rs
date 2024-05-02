use winnow::{
    ascii::{alpha1, alphanumeric0, float, multispace0},
    combinator::{alt, dispatch, fail, opt, peek, preceded, repeat, terminated},
    prelude::*,
    token::{any, take},
};

use crate::ast::FunctionName;
use crate::expand::expand;

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
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

impl core::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Token::*;
        match self {
            ScalarVal(v) => write!(f, "{}", v),
            Variable(v) => write!(f, "{}", v),
            Function(e) => write!(f, "{:?}", e),
            a => write!(f, "{:?}", a),
        }
    }
}

impl winnow::stream::ContainsToken<Token> for Token {
    #[inline(always)]
    fn contains_token(&self, token: Token) -> bool {
        *self == token
    }
}

impl winnow::stream::ContainsToken<Token> for &'_ [Token] {
    #[inline]
    fn contains_token(&self, token: Token) -> bool {
        self.iter().any(|t| *t == token)
    }
}

impl<const LEN: usize> winnow::stream::ContainsToken<Token> for &'_ [Token; LEN] {
    #[inline]
    fn contains_token(&self, token: Token) -> bool {
        self.iter().any(|t| *t == token)
    }
}

impl<const LEN: usize> winnow::stream::ContainsToken<Token> for [Token; LEN] {
    #[inline]
    fn contains_token(&self, token: Token) -> bool {
        self.iter().any(|t| *t == token)
    }
}

pub fn lex(i: &mut &str) -> PResult<Vec<Token>> {
    preceded(multispace0, repeat(1.., terminated(token, multispace0))).parse_next(i)
}

pub struct Lexer {
    pub tokens: Vec<Token>,
    pub token_num: usize,
}

impl Lexer {
    pub fn new(input: &mut &str) -> Self {
        let mut i_str: &str = &expand(input);
        let mut tokens = lex(&mut i_str).expect("lexer failed");
        tokens.reverse();
        Lexer {
            tokens,
            token_num: 0,
        }
    }

    pub fn next(&mut self) -> Token {
        self.token_num += 1;
        self.tokens.pop().unwrap_or(Token::Eof)
    }

    pub fn peek(&mut self) -> Token {
        self.tokens.last().cloned().unwrap_or(Token::Eof)
    }
}

fn token(i: &mut &str) -> PResult<Token> {
    use Token::*;
    let single = dispatch! {peek(any);
        '0'..='9' | '.' => float.map(Token::ScalarVal),
        '(' => '('.value(LParen),
        ')' => ')'.value(RParen),
        '[' => '['.value(LBracket),
        ']' => ']'.value(RBracket),
        '{' => '{'.value(LBrace),
        '}'=> '}'.value(RBrace),
        ',' => ','.value(Comma),
        ';' => ';'.value(Semicolon),
        '+' => '+'.value(Operator(Op::Add)),
        '-' => '-'.value(Operator(Op::Sub)),
        '*' => '*'.value(Operator(Op::Mul)),
        '/' => '/'.value(Operator(Op::Div)),
        '=' => '='.value(Assign(AssignOp::Number)),
        '>' => '>'.value(Operator(Op::Greater)),
        '<' => '<'.value(Operator(Op::Less)),
        '?' => '?'.value(Then),
        ':' => ':'.value(Else),
        _ => fail,
    };
    let pow = "**".value(Operator(Op::Pow));
    let eq = "==".value(Operator(Op::Eq));
    let neq = "!=".value(Operator(Op::NotEq));
    let geq = ">=".value(Operator(Op::GreaterEq));
    let leq = "<=".value(Operator(Op::LessEq));
    let and = "&&".value(Operator(Op::And));
    let or = "||".value(Operator(Op::Or));
    let inc = "++".value(Operator(Op::Inc));
    let dec = "--".value(Operator(Op::Dec));
    let assign_add = "+=".value(Assign(AssignOp::Add));
    let assign_sub = "-=".value(Assign(AssignOp::Sub));
    let assign_mul = "*=".value(Assign(AssignOp::Mul));
    let assign_div = "/=".value(Assign(AssignOp::Div));
    alt((
        assign_add, assign_div, assign_mul, assign_sub, inc, dec, and, or, eq, neq, geq, leq, pow,
        single, identifier,
    ))
    .parse_next(i)
}

fn identifier(i: &mut &str) -> PResult<Token> {
    use FunctionName::*;
    use Token::*;
    let c1 = take(1u8).and_then(alpha1).parse_next(i)?;
    let c2 = opt(alphanumeric0).parse_next(i)?;
    let s = match c2 {
        Some(c) => format!("{}{}", c1, c),
        None => c1.to_string(),
    };
    match s.as_str() {
        "sin" => Ok(Function(Sin)),
        "cos" => Ok(Function(Cos)),
        "tan" => Ok(Function(Tan)),
        "atan2" => Ok(Function(Atan2)),
        "exp" => Ok(Function(Exp)),
        "exp2" => Ok(Function(Exp2)),
        "log" => Ok(Function(Log)),
        "log2" => Ok(Function(Log2)),
        "pow" => Ok(Function(FunctionName::Pow)),
        "sqrt" => Ok(Function(Sqrt)),
        "abs" => Ok(Function(Abs)),
        "sign" => Ok(Function(Sign)),
        "floor" => Ok(Function(Floor)),
        "ceil" => Ok(Function(Ceil)),
        "fract" => Ok(Function(Fract)),
        "FR" => Ok(Function(Fract)),
        "mod" => Ok(Function(FunctionName::Mod)),
        "min" => Ok(Function(Min)),
        "max" => Ok(Function(Max)),
        "cl" => Ok(Function(Clamp)),
        "mix" => Ok(Function(Mix)),
        "B" => Ok(Function(Abs)),
        "SM" => Ok(Function(Smoothstep)),
        "L" => Ok(Function(Length)),
        "H" => Ok(Function(Distance)),
        "A" => Ok(Function(AddMul)),
        "D" => Ok(Function(Dot)),
        "X" => Ok(Function(Cross)),
        "N" => Ok(Function(Normalize)),
        "U" => Ok(Function(Union)),
        "G" => Ok(Function(Intersect)),
        "Z" => Ok(Function(Floor)),
        "nz" => Ok(Function(ValueNoise)),
        "don" => Ok(Function(Torus)),
        "bx2" => Ok(Function(Box2)),
        "bx3" => Ok(Function(Box3)),
        "r0" => Ok(Function(Rot0)),
        "r1" => Ok(Function(Rot1)),
        "TR" => Ok(Function(Triangle)),
        "k" => Ok(Function(Corner)),
        "sB" => Ok(Function(SmoothAbs)),
        "scl" => Ok(Function(SmoothClamp)),
        "rG" => Ok(Function(RoundMax)),
        "rmax" => Ok(Function(RoundMax)),
        "rU" => Ok(Function(RoundMin)),
        "rmin" => Ok(Function(RoundMin)),
        "acos" => Ok(Function(Acos)),
        "asin" => Ok(Function(Asin)),
        "atan" => Ok(Function(Atan)),
        "sinh" => Ok(Function(Sinh)),
        "cosh" => Ok(Function(Cosh)),
        "tanh" => Ok(Function(Tanh)),
        "trunc" => Ok(Function(Trunc)),
        "asinh" => Ok(Function(Asinh)),
        "acosh" => Ok(Function(Acosh)),
        "atanh" => Ok(Function(Atanh)),
        "qB" => Ok(Function(PolySmoothAbs)),
        "sabs" => Ok(Function(SmoothAbs)),
        "round" => Ok(Function(Round)),
        "qcl" => Ok(Function(PolySmoothClamp)),
        "g" => Ok(Function(FakeSine)),
        "ri" => Ok(Function(Hash)),
        "rot" => Ok(Function(Rot)),
        _ => Ok(Variable(s.to_string())),
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn show() {
        // let mut input = "s=1;@5{@xyz{$=B($*2)-8,}s*=.5,}(L(x,y,z)-8)*s";
        // let input = "ri(xi,yi,zi)>.4&&L(xi,yi,zi)>3?L(xm,ym,zm)-2:10";
        // let input = "i=mod(floor(x/8)+floor(z/8),2),x=mod(x,8)-4,z=mod(z,8)-4,a=L(x,y,z)-1,q=L(x,z),b=max(D([1,.3],[q,y]),-5-y),a=rU(a,b,1),y+=1,a=rU(a,L(x,y*5,z)-.8,1),y+=3,a=rU(a,L(x,y*2,z)-1,.5),y+=1,a=rU(a,L(x,y*3,z)-1.7,0.1),min(a,y+.5*i*nz(x,y,z,8,0))";
        // let mut input = "variable0 = 1 + 2.8 * 3 - 4 / 5 % 6 ** 7";
        let input = "k(r,-U(@xyz{bx2($,$$,9),}))";
        let i = expand(input);
        let _ = dbg!(lex.parse_peek(&i));
    }

    #[test]
    fn test_function() {
        use Token::*;
        let input = "s = G(x,y,z); t = rmin(x, y, s)";
        let expected = vec![
            Variable("s".to_string()),
            Assign(AssignOp::Number),
            Function(FunctionName::Intersect),
            LParen,
            Variable("x".to_string()),
            Comma,
            Variable("y".to_string()),
            Comma,
            Variable("z".to_string()),
            RParen,
            Semicolon,
            Variable("t".to_string()),
            Assign(AssignOp::Number),
            Function(FunctionName::RoundMin),
            LParen,
            Variable("x".to_string()),
            Comma,
            Variable("y".to_string()),
            Comma,
            Variable("s".to_string()),
            RParen,
        ];
        assert_eq!(lex.parse_peek(input), Ok(("", expected)));
    }

    #[test]
    fn test_assign() {
        use Op::*;
        use Token::*;
        let input = "variable0 = 1 + 2.8 * .3 - 4 / 5 + 6 ** 7";
        let expected = vec![
            Variable("variable0".to_string()),
            Assign(AssignOp::Number),
            ScalarVal(1.0),
            Operator(Add),
            ScalarVal(2.8),
            Operator(Mul),
            ScalarVal(0.3),
            Operator(Sub),
            ScalarVal(4.0),
            Operator(Div),
            ScalarVal(5.0),
            Operator(Add),
            ScalarVal(6.0),
            Operator(Pow),
            ScalarVal(7.0),
        ];
        assert_eq!(lex.parse_peek(input), Ok(("", expected)));
    }
}
