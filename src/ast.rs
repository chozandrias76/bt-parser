use nom::{error::Error, IResult, InputLength};

#[derive(Debug, PartialEq)]
pub enum Expr {
    Identifier(String),
    Literal(i32),
    FunctionCall {
        name: String,
        args: Vec<Expr>,
    },
    BinaryOp {
        left: Box<Expr>,
        op: Expression,
        right: Box<Expr>,
    },
    Parens(Box<Expr>),
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    And,
    Or,
    Not,
    Equals,
    NotEquals,
    GreaterThan,
    LessThan,
    BinaryAnd,
    GreaterThanOrEqualTo,
    LessThanOrEqualTo,
    Add,
    Subtract,
    Multiply,
    Divide,
    BinaryInvert,
    BinaryXor,
    Modulus,
    Increment,
    Decrement,
    Ternary,
    BinaryShiftLeft,
    BinaryShiftRight,
}

impl Expression {
    pub fn to_str(&self) -> &'static str {
        match self {
            Self::And => "&&",
            Self::Or => "||",
            Self::Not => "!",
            Self::Equals => "==",
            Self::NotEquals => "!=",
            Self::GreaterThan => ">",
            Self::LessThan => "<",
            Self::BinaryAnd => "&",
            Self::GreaterThanOrEqualTo => ">=",
            Self::LessThanOrEqualTo => "<=",
            Self::Add => "+",
            Self::Subtract => "-",
            Self::Multiply => "*",
            Self::Divide => "/",
            Self::BinaryInvert => "~",
            Self::BinaryXor => "^",
            Self::Modulus => "%",
            Self::Increment => "++",
            Self::Decrement => "--",
            Self::Ternary => "?:",
            Self::BinaryShiftLeft => "<<",
            Self::BinaryShiftRight => ">>",
        }
    }

    pub fn variants() -> &'static [Self] {
        use Expression::*;
        &[
            And,
            Or,
            Not,
            Equals,
            NotEquals,
            GreaterThan,
            LessThan,
            BinaryAnd,
            GreaterThanOrEqualTo,
            LessThanOrEqualTo,
            Add,
            Subtract,
            Multiply,
            Divide,
            BinaryInvert,
            BinaryXor,
            Modulus,
            Increment,
            Decrement,
            Ternary,
            BinaryShiftLeft,
            BinaryShiftRight,
        ]
    }

    pub fn all_expressions() -> Vec<&'static str> {
        vec![
            Expression::And.to_str(),
            Expression::Or.to_str(),
            Expression::Not.to_str(),
            Expression::Equals.to_str(),
            Expression::NotEquals.to_str(),
            Expression::GreaterThan.to_str(),
            Expression::LessThan.to_str(),
            Expression::BinaryAnd.to_str(),
            Expression::GreaterThanOrEqualTo.to_str(),
            Expression::LessThanOrEqualTo.to_str(),
            Expression::Add.to_str(),
            Expression::Subtract.to_str(),
            Expression::Multiply.to_str(),
            Expression::Divide.to_str(),
            Expression::BinaryInvert.to_str(),
            Expression::BinaryXor.to_str(),
            Expression::Modulus.to_str(),
            Expression::Increment.to_str(),
            Expression::Decrement.to_str(),
            Expression::Ternary.to_str(),
            Expression::BinaryShiftLeft.to_str(),
            Expression::BinaryShiftRight.to_str(),
        ]
    }
}

impl InputLength for Expression {
    fn input_len(&self) -> usize {
        self.to_str().len()
    }
}

impl Clone for Expression {
    fn clone(&self) -> Self {
        match self {
            Expression::And => Expression::And,
            Expression::Or => Expression::Or,
            Expression::Not => Expression::Not,
            Expression::Equals => Expression::Equals,
            Expression::NotEquals => Expression::NotEquals,
            Expression::GreaterThan => Expression::GreaterThan,
            Expression::LessThan => Expression::LessThan,
            Expression::BinaryAnd => Expression::BinaryAnd,
            Expression::GreaterThanOrEqualTo => Expression::GreaterThanOrEqualTo,
            Expression::LessThanOrEqualTo => Expression::LessThanOrEqualTo,
            Expression::Add => Expression::Add,
            Expression::Subtract => Expression::Subtract,
            Expression::Multiply => Expression::Multiply,
            Expression::Divide => Expression::Divide,
            Expression::BinaryInvert => Expression::BinaryInvert,
            Expression::BinaryXor => Expression::BinaryXor,
            Expression::Modulus => Expression::Modulus,
            Expression::Increment => Expression::Increment,
            Expression::Decrement => Expression::Decrement,
            Expression::Ternary => Expression::Ternary,
            Expression::BinaryShiftLeft => Expression::BinaryShiftLeft,
            Expression::BinaryShiftRight => Expression::BinaryShiftRight,
        }
    }
}
