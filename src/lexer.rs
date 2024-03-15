use winnow::{
    ascii::{alpha1, alphanumeric0, float, multispace0},
    combinator::{alt, dispatch, fail, opt, peek, preceded, repeat, terminated},
    prelude::*,
    token::{any, take},
};

use crate::ast::FunctionName;

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    ScalarVal(f32),
    BinOp(Binary),
    UnOp(Unary),
    TernaryOp(Ternary),
    Delimiter(Delim),
    Variable(String),
    Function(FunctionName),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Binary {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
    Eq,
    NotEq,
    Greater,
    GreaterEq,
    Less,
    LessEq,
    And,
    Or,
    Assign,
    AssignAdd,
    AssignSub,
    AssignMul,
    AssignDiv,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Unary {
    Neg,
    Not,
    Inc,
    Dec,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Ternary {
    Then,
    Else,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]

pub enum Delim {
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Comma,
    Semicolon,
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

fn token(i: &mut &str) -> PResult<Token> {
    use Binary::*;
    use Delim::*;
    use Ternary::*;
    use Token::*;
    let single = dispatch! {peek(any);
        // '0'..='9' => digit1.try_map(FromStr::from_str).map(Token::ScalarVal),
        '0'..='9' | '.' => float.map(Token::ScalarVal),
        '(' => '('.value(Delimiter(LParen)),
        ')' => ')'.value(Delimiter(RParen)),
        '[' => '['.value(Delimiter(LBracket)),
        ']' => ']'.value(Delimiter(RBracket)),
        '{' => '{'.value(Delimiter(LBrace)),
        '}'=> '}'.value(Delimiter(RBrace)),
        ',' => ','.value(Delimiter(Comma)),
        ';' => ';'.value(Delimiter(Semicolon)),
        '+' => '+'.value(BinOp(Add)),
        '-' => '-'.value(BinOp(Sub)),
        '*' => '*'.value(BinOp(Mul)),
        '/' => '/'.value(BinOp(Div)),
        '%' => '%'.value(BinOp(Mod)),
        '=' => '='.value(BinOp(Assign)),
        '>' => '>'.value(BinOp(Greater)),
        '<' => '<'.value(BinOp(Less)),
        '?' => '?'.value(TernaryOp(Then)),
        ':' => ':'.value(TernaryOp(Else)),
        _ => fail,
    };
    let pow = "**".value(BinOp(Pow));
    let eq = "==".value(BinOp(Binary::Eq));
    let neq = "!=".value(BinOp(Binary::NotEq));
    let geq = ">=".value(BinOp(Binary::GreaterEq));
    let leq = "<=".value(BinOp(Binary::LessEq));
    let and = "&&".value(BinOp(Binary::And));
    let or = "||".value(BinOp(Binary::Or));
    let inc = "++".value(UnOp(Unary::Inc));
    let dec = "--".value(UnOp(Unary::Dec));
    let assign_add = "+=".value(BinOp(AssignAdd));
    let assign_sub = "-=".value(BinOp(AssignSub));
    let assign_mul = "*=".value(BinOp(AssignMul));
    let assign_div = "/=".value(BinOp(AssignDiv));
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
        "pow" => Ok(Function(Pow)),
        "sqrt" => Ok(Function(Sqrt)),
        "abs" => Ok(Function(Abs)),
        "sign" => Ok(Function(Sign)),
        "floor" => Ok(Function(Floor)),
        "ceil" => Ok(Function(Ceil)),
        "fract" => Ok(Function(Fract)),
        "FR" => Ok(Function(Fract)),
        "mod" => Ok(Function(Mod)),
        "min" => Ok(Function(Min)),
        "max" => Ok(Function(Max)),
        "clamp" => Ok(Function(Clamp)),
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

mod tests {
    use super::*;
    use crate::ast::*;
    use crate::expand::expand;

    #[test]
    fn show() {
        // let mut input = "s=1;@5{@xyz{$=B($*2)-8,}s*=.5,}(L(x,y,z)-8)*s";
        let input = "ri(xi,yi,zi)>.4&&L(xi,yi,zi)>3?L(xm,ym,zm)-2:10";
        // let mut input = "variable0 = 1 + 2.8 * 3 - 4 / 5 % 6 ** 7";
        let i = expand(input);
        let _ = dbg!(lex.parse_peek(&i));
    }
    #[test]
    fn test_function() {
        use Token::*;
        let input = "s = G(x,y,z); t = rmin(x, y, s)";
        let expected = vec![
            Variable("s".to_string()),
            BinOp(Binary::Assign),
            Function(FunctionName::Intersect),
            Delimiter(Delim::LParen),
            Variable("x".to_string()),
            Delimiter(Delim::Comma),
            Variable("y".to_string()),
            Delimiter(Delim::Comma),
            Variable("z".to_string()),
            Delimiter(Delim::RParen),
            Delimiter(Delim::Semicolon),
            Variable("t".to_string()),
            BinOp(Binary::Assign),
            Function(FunctionName::RoundMin),
            Delimiter(Delim::LParen),
            Variable("x".to_string()),
            Delimiter(Delim::Comma),
            Variable("y".to_string()),
            Delimiter(Delim::Comma),
            Variable("s".to_string()),
            Delimiter(Delim::RParen),
        ];
        assert_eq!(lex.parse_peek(input), Ok(("", expected)));
    }

    #[test]
    fn test_assign() {
        use Token::*;
        let input = "variable0 = 1 + 2.8 * .3 - 4 / 5 % 6 ** 7";
        let expected = vec![
            Variable("variable0".to_string()),
            BinOp(Binary::Assign),
            ScalarVal(1.0),
            BinOp(Binary::Add),
            ScalarVal(2.8),
            BinOp(Binary::Mul),
            ScalarVal(0.3),
            BinOp(Binary::Sub),
            ScalarVal(4.0),
            BinOp(Binary::Div),
            ScalarVal(5.0),
            BinOp(Binary::Mod),
            ScalarVal(6.0),
            BinOp(Binary::Pow),
            ScalarVal(7.0),
        ];
        assert_eq!(lex.parse_peek(input), Ok(("", expected)));
    }
}
