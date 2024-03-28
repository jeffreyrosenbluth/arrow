use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub enum Statement {
    Assign { var: String, rhs: Box<Expr> },
    AssignToArray { vars: Vec<String>, rhs: Box<Expr> },
    AssignFromArray { vars: Vec<String>, rhs: Vec<Expr> },
    Sequence(Vec<Statement>),
    Return(Box<Expr>),
    Empty,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum Expr {
    Number(f32),
    BinaryOp(BinOp),
    Negate(Box<Expr>),
    Function { name: FunctionName, args: Vec<Expr> },
    Variable(String),
    TernaryOp(Box<Expr>, Box<Expr>, Box<Expr>),
    Assign(AssignExpr),
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum BinOp {
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Eq(Box<Expr>, Box<Expr>),
    NotEq(Box<Expr>, Box<Expr>),
    Greater(Box<Expr>, Box<Expr>),
    GreaterEq(Box<Expr>, Box<Expr>),
    Less(Box<Expr>, Box<Expr>),
    LessEq(Box<Expr>, Box<Expr>),
    And(Box<Expr>, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),
    Pow(Box<Expr>, Box<Expr>),
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum AssignExpr {
    Inc(String),
    Dec(String),
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub enum FunctionName {
    Sin,
    Cos,
    Acos,
    Asin,
    Tan,
    Atan,
    Atan2,
    Sinh,
    Cosh,
    Tanh,
    Asinh,
    Acosh,
    Atanh,
    Exp,
    Exp2,
    Log,
    Log2,
    Pow,
    Sqrt,
    Abs,
    Sign,
    Floor,
    Trunc,
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
    Rot0,
    Rot1,
    Rot,
    Triangle,
    Corner,
    SmoothAbs,
    PolySmoothAbs,
    SmoothClamp,
    PolySmoothClamp,
    RoundMax,
    RoundMin,
    Round,
    FakeSine,
    Hash,
}
