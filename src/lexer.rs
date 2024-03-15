use winnow::{
    ascii::{alpha1, alphanumeric0, float, multispace0},
    combinator::{alt, dispatch, fail, opt, peek, preceded, repeat, terminated},
    prelude::*,
    token::{any, take},
};

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    ScalarVal(f32),
    BinOp(Binary),
    UnOp(Unary),
    TernaryOp(Ternary),
    Delimiter(Delim),
    Identifier(String),
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
    let c1 = take(1u8).and_then(alpha1).parse_next(i)?;
    let c2 = opt(alphanumeric0).parse_next(i)?;
    match c2 {
        Some(c) => Ok(Token::Identifier(format!("{}{}", c1, c))),
        None => Ok(Token::Identifier(c1.to_string())),
    }
}

mod tests {
    use super::*;
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
    fn test_lex() {
        let input = "variable0 = 1 + 2.8 * .3 - 4 / 5 % 6 ** 7";
        let expected = vec![
            Token::Identifier("variable0".to_string()),
            Token::BinOp(Binary::Assign),
            Token::ScalarVal(1.0),
            Token::BinOp(Binary::Add),
            Token::ScalarVal(2.8),
            Token::BinOp(Binary::Mul),
            Token::ScalarVal(0.3),
            Token::BinOp(Binary::Sub),
            Token::ScalarVal(4.0),
            Token::BinOp(Binary::Div),
            Token::ScalarVal(5.0),
            Token::BinOp(Binary::Mod),
            Token::ScalarVal(6.0),
            Token::BinOp(Binary::Pow),
            Token::ScalarVal(7.0),
        ];
        assert_eq!(lex.parse_peek(input), Ok(("", expected)));
    }
}
