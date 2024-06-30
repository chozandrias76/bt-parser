use nom::{error::Error, IResult, InputLength};

#[derive(Debug, PartialEq)]
pub enum Expr {
    Identifier(String),
    Literal(i32),
    FunctionCall { name: String, args: Vec<Expr> },
    BinaryOp { left: Box<Expr>, op: BinaryOp, right: Box<Expr> },
    Parens(Box<Expr>),
}

#[derive(Debug, PartialEq)]
pub enum BinaryOp {
    And,
    Or,
    Equals,
    NotEquals,
    GreaterThan,
    LessThan,
    BitAnd,
}

impl BinaryOp {
    pub fn to_str(&self) -> &'static str {
        match self {
            BinaryOp::And => "&&",
            BinaryOp::Or => "||",
            BinaryOp::Equals => "==",
            BinaryOp::NotEquals => "!=",
            BinaryOp::GreaterThan => ">",
            BinaryOp::LessThan => "<",
            BinaryOp::BitAnd => "&",
        }
    }

    pub fn all_operators() -> &'static [&'static str] {
        &[
            "&&", "||", "==", "!=", ">", "<", "&",
        ]
    }
}

impl InputLength for BinaryOp {
    fn input_len(&self) -> usize {
        self.to_str().len()
    }
}

impl Clone for BinaryOp {
    fn clone(&self) -> Self {
        match self {
            BinaryOp::And => BinaryOp::And,
            BinaryOp::Or => BinaryOp::Or,
            BinaryOp::Equals => BinaryOp::Equals,
            BinaryOp::NotEquals => BinaryOp::NotEquals,
            BinaryOp::GreaterThan => BinaryOp::GreaterThan,
            BinaryOp::LessThan => BinaryOp::LessThan,
            BinaryOp::BitAnd => BinaryOp::BitAnd,
        }
    }
}