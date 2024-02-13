use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub enum Statement {
    Assign { var: String, rhs: Box<Expr> },
    AssignArray { vars: Vec<String>, rhs: Box<Expr> },
    Sequence(Vec<Statement>),
    Return(Box<Expr>),
    Empty,
}

#[derive(Debug, Clone, Serialize)]
pub enum Expr {
    Scalar(f32),
    BinaryOp(BinOp),
    Paren(Box<Expr>),
    Function { name: FunctionName, args: Vec<Expr> },
    Variable(String),
}

#[derive(Debug, Clone, Serialize)]
pub enum BinOp {
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Eq(Box<Expr>, Box<Expr>),
}

#[derive(Debug, Clone, Serialize)]
pub enum FunctionName {
    Sin,
    Cos,
    Tan,
    Atan2,
    Exp,
    Exp2,
    Log,
    Log2,
    Pow,
    Sqrt,
    Abs,
    Sign,
    Floor,
    Ceil,
    Fract,
    Mod,
    Min,
    Max,
    Clamp,
    Mix,
    Smoothstep,
    Length,
    Distance,
    Dot,
    Cross,
    Normalize,
    Union,
    Intersect,
    AddMul,
    ValueNoise,
    Torus,
    Box2,
    Box3,
    Floors,
    Rot0,
    Rot1,
    Triangle,
    Corner,
    SmoothAbs,
    SmoothClamp,
}

#[derive(Debug, Clone, Serialize)]
pub enum Function3Name {}
