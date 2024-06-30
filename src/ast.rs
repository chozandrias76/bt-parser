use nom::{error::Error, IResult, InputLength};

#[derive(Debug, PartialEq)]
pub enum Expr {
    Identifier(String),
    Literal(i32),
    FunctionCall { name: String, args: Vec<Expr> },
    BinaryOp { left: Box<Expr>, op: Expression, right: Box<Expr> },
    Parens(Box<Expr>),
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    And,
    Or,
    Equals,
    NotEquals,
    GreaterThan,
    LessThan,
    BitAnd,
    GreaterThanOrEqualTo,
    LessThanOrEqualTo,
}

impl Expression {
    pub fn to_str(&self) -> &'static str {
        match self {
            Self::And => "&&",
            Self::Or => "||",
            Self::Equals => "==",
            Self::NotEquals => "!=",
            Self::GreaterThan => ">",
            Self::LessThan => "<",
            Self::BitAnd => "&",
            Self::GreaterThanOrEqualTo => ">=",
            Self::LessThanOrEqualTo => "<=",
        }
    }

    pub fn variants() -> &'static [Self] {
        use Expression::*;
        &[
            And,
            Or,
            Equals,
            NotEquals,
            GreaterThan,
            LessThan,
            BitAnd,
        ]
    }

    pub fn all_expressions() -> Vec<&'static str> {
        vec![
            Expression::And.to_str(), Expression::Or.to_str(), Expression::Equals.to_str(), Expression::NotEquals.to_str(), Expression::GreaterThan.to_str(), Expression::LessThan.to_str(), Expression::BitAnd.to_str(),
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
            Expression::Equals => Expression::Equals,
            Expression::NotEquals => Expression::NotEquals,
            Expression::GreaterThan => Expression::GreaterThan,
            Expression::LessThan => Expression::LessThan,
            Expression::BitAnd => Expression::BitAnd,
            Expression::GreaterThanOrEqualTo => Expression::GreaterThanOrEqualTo,
            Expression::LessThanOrEqualTo => Expression::LessThanOrEqualTo,
        }
    }
}