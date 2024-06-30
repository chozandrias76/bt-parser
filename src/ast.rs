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
    GreaterThanOrEqualTo,
    LessThanOrEqualTo,
    Add,
    Subtract,
    Multiply,
    Divide,
    BinaryInvert,
    BinaryXor,
    BinaryAnd,
    BinaryOr,
    Modulus,
    Increment,
    Decrement,
    Ternary,
    BinaryShiftLeft,
    BinaryShiftRight,
    AddEquals,
    MinusEquals,
    MultiplyEquals,
    DivideEquals,
    BinaryAndEquals,
    BinaryXorEquals,
    BinaryModulusEquals,
    BinaryOrEquals,
    BinaryShiftLeftEquals,
    BinaryShiftRightEquals,
}

impl Expression {
    pub fn to_str(&self) -> &'static str {
        match self {
            // <
            Self::BinaryShiftLeftEquals => "<<=",
            Self::BinaryShiftLeft => "<<",
            Self::LessThanOrEqualTo => "<=",
            Self::LessThan => "<",
            // >
            Self::BinaryShiftRightEquals => ">>=",
            Self::BinaryShiftRight => ">>",
            Self::GreaterThanOrEqualTo => ">=",
            Self::GreaterThan => ">",
            // &
            Self::And => "&&",
            Self::BinaryAndEquals => "&=",
            Self::BinaryAnd => "&",
            // \!
            Self::NotEquals => "!=",
            Self::Not => "!",
            // |
            Self::Or => "||",
            Self::BinaryOrEquals => "|=",
            Self::BinaryOr => "|",
            // =
            Self::Equals => "==",
            // +
            Self::AddEquals => "+=",
            Self::Increment => "++",
            Self::Add => "+",
            // -
            Self::MinusEquals => "-=",
            Self::Decrement => "--",
            Self::Subtract => "-",
            // *
            Self::MultiplyEquals => "*=",
            Self::Multiply => "*",
            // ^
            Self::BinaryXorEquals => "^=",
            Self::BinaryXor => "^",
            // %
            Self::BinaryModulusEquals => "%=",
            Self::Modulus => "%",
            // /
            Self::DivideEquals => "/=",
            Self::Divide => "/",
            // special
            Self::BinaryInvert => "~",
            Self::Ternary => "?:",
           
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
            GreaterThanOrEqualTo,
            LessThanOrEqualTo,
            Add,
            Subtract,
            Multiply,
            Divide,
            BinaryInvert,
            BinaryXor,
            BinaryAnd,
            BinaryOr,
            Modulus,
            Increment,
            Decrement,
            Ternary,
            BinaryShiftLeft,
            BinaryShiftRight,
            AddEquals,
            MinusEquals,
            MultiplyEquals,
            DivideEquals,
            BinaryAndEquals,
            BinaryXorEquals,
            BinaryModulusEquals,
            BinaryOrEquals,
            BinaryShiftLeftEquals,
            BinaryShiftRightEquals,
        ]
    }

    pub fn all_expressions() -> Vec<&'static str> {
        Self::variants().iter().map(|expr| expr.to_str()).collect()

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
            Self::And => Self::And,
            Self::Or => Self::Or,
            Self::Not => Self::Not,
            Self::Equals => Self::Equals,
            Self::NotEquals => Self::NotEquals,
            Self::GreaterThan => Self::GreaterThan,
            Self::LessThan => Self::LessThan,
            Self::GreaterThanOrEqualTo => Self::GreaterThanOrEqualTo,
            Self::LessThanOrEqualTo => Self::LessThanOrEqualTo,
            Self::Add => Self::Add,
            Self::Subtract => Self::Subtract,
            Self::Multiply => Self::Multiply,
            Self::Divide => Self::Divide,
            Self::BinaryInvert => Self::BinaryInvert,
            Self::BinaryXor => Self::BinaryXor,
            Self::BinaryAnd => Self::BinaryAnd,
            Self::BinaryOr => Self::BinaryOr,
            Self::Modulus => Self::Modulus,
            Self::Increment => Self::Increment,
            Self::Decrement => Self::Decrement,
            Self::Ternary => Self::Ternary,
            Self::BinaryShiftLeft => Self::BinaryShiftLeft,
            Self::BinaryShiftRight => Self::BinaryShiftRight,
            Self::AddEquals => Self::AddEquals,
            Self::MinusEquals => Self::MinusEquals,
            Self::MultiplyEquals => Self::MultiplyEquals,
            Self::DivideEquals => Self::DivideEquals,
            Self::BinaryAndEquals => Self::BinaryAndEquals,
            Self::BinaryXorEquals => Self::BinaryXorEquals,
            Self::BinaryModulusEquals => Self::BinaryModulusEquals,
            Self::BinaryOrEquals => Self::BinaryOrEquals,
            Self::BinaryShiftLeftEquals => Self::BinaryShiftLeftEquals,
            Self::BinaryShiftRightEquals => Self::BinaryShiftRightEquals,
        }
    }
}
