#[derive(Debug, Clone)]
pub enum Expr {
    Scalar(f32),
    BinaryOp(BinOp),
    UnaryOp,
    Paren(Box<Expr>),
    Assign { lhs: Box<Expr>, rhs: Box<Expr> },
    Function { name: FunctionName, args: Vec<Expr> },
    Variable(String),
}

#[derive(Debug, Clone)]
pub enum BinOp {
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
}

#[derive(Debug, Clone)]
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
}
