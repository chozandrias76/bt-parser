use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::{char, multispace0, one_of, satisfy, space0},
    combinator::{map, map_res, not, opt, peek, recognize, value},
    error::Error,
    multi::{many0, many1, separated_list1},
    sequence::{delimited, pair, preceded, terminated, tuple},
    AsChar, IResult, Parser,
};

use crate::ast::Expression;

fn ws<'a, F: 'a, O>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O>
where
    F: FnMut(&'a str) -> IResult<&'a str, O>,
{
    delimited(multispace0, inner, multispace0)
}

fn identifier(input: &str) -> IResult<&str, &str> {
    ws(take_while1(|c: char| c.is_ascii_alphanumeric()))(input).map(|(i, o)| (i.trim(), o.trim()))
}

macro_rules! expression_tag {
    ($expr:expr) => {
        map(
            tuple((
                peek(tuple((
                    tag($expr.to_str()),
                    not_expression_tag,
                ))),
                tag($expr.to_str()),
            )),
            |(matched, _)| {
                println!("Expression Tag matched {:?} with {:?}", $expr, matched.0);
                matched.0
            },
        )
    };
}

#[cfg(test)]
mod test_macro_expression_tag {
    use super::*;

    #[test]
    fn test_expression_tag1() {
        Expression::variants().iter().for_each(|expr| {
            let ignored = vec![
                Expression::BinaryAnd,
                Expression::BinaryOr,
                Expression::Subtract,
                Expression::Add,
                Expression::GreaterThan,
                Expression::LessThan,
            ];
            if ignored.into_iter().any(|r| r.to_str() == expr.to_str()) {
                return;
            }
            let tag_on = format!("{}{}", expr.to_str(), expr.to_str());
            let tag_on = tag_on.as_str();
            assert!(
                expression_tag!(expr)(tag_on).is_err(),
                "Passed on {}",
                tag_on
            );
        });
    }

    #[test]
    fn test_expression_tag2() {
        Expression::variants().iter().for_each(|expr| {
            let tag_on = format!("{}", expr.to_str());
            let tag_on = tag_on.as_str();
            assert!(
                expression_tag!(expr)(tag_on).is_ok(),
                "Failed on {}",
                tag_on
            );
        });
    }
}

macro_rules! expression_tag {
    ($variant:expr) => {
        (tag($variant.to_str()))
    };
}

macro_rules! alternating_expression_tags {
    () => {
        alt((
            alt((
                expression_tag!(Expression::BinaryShiftLeftEquals),
                alt((
                    expression_tag!(Expression::BinaryShiftLeft),
                    expression_tag!(Expression::LessThanOrEqualTo),
                )),
                expression_tag!(Expression::LessThan),
            )),
            alt((
                expression_tag!(Expression::BinaryShiftRightEquals),
                alt((
                    expression_tag!(Expression::BinaryShiftRight),
                    expression_tag!(Expression::GreaterThanOrEqualTo),
                )),
                expression_tag!(Expression::GreaterThan),
            )),
            alt((
                alt((
                    expression_tag!(Expression::And),
                    expression_tag!(Expression::BinaryAndEquals),
                )),
                expression_tag!(Expression::BinaryAnd),
            )),
            alt((
                expression_tag!(Expression::NotEquals),
                expression_tag!(Expression::Not),
            )),
            alt((
                alt((
                    expression_tag!(Expression::Or),
                    expression_tag!(Expression::BinaryOrEquals),
                )),
                expression_tag!(Expression::BinaryOr),
            )),
            alt((
                expression_tag!(Expression::Equals),
                alt((
                    alt((
                        alt((
                            expression_tag!(Expression::AddEquals),
                            expression_tag!(Expression::Increment),
                        )),
                        expression_tag!(Expression::Add),
                    )),
                    alt((
                        alt((
                            expression_tag!(Expression::MinusEquals),
                            expression_tag!(Expression::Decrement),
                        )),
                        expression_tag!(Expression::Subtract),
                    )),
                    alt((
                        expression_tag!(Expression::MultiplyEquals),
                        expression_tag!(Expression::Multiply),
                    )),
                    alt((
                        expression_tag!(Expression::BinaryXorEquals),
                        expression_tag!(Expression::BinaryXor),
                    )),
                    alt((
                        expression_tag!(Expression::BinaryModulusEquals),
                        expression_tag!(Expression::Modulus),
                    )),
                    alt((
                        expression_tag!(Expression::DivideEquals),
                        expression_tag!(Expression::Divide),
                    )),
                    alt((
                        expression_tag!(Expression::Ternary),
                        expression_tag!(Expression::BinaryInvert),
                    )),
                )),
            )),
        ))
    };
}

fn operator(input: &str) -> IResult<&str, &str> {
    delimited(
        space0,
        tuple((
            alternating_expression_tags!(),
            not_expression_tag,
        )),
        space0,
    )(input)
    .map(|(i, o): (&str, (&str, ()))| (i.trim(), o.0.trim()))
}

#[cfg(test)]
mod test_operator {
    use super::*;

    #[test]
    fn test_operator1() {
        Expression::variants().iter().for_each(|expr| {
            let ignored = vec![
                Expression::BinaryAnd,
                Expression::BinaryOr,
                Expression::Subtract,
                Expression::Add,
                Expression::GreaterThan,
                Expression::LessThan,
            ];
            if ignored.into_iter().any(|r| r.to_str() == expr.to_str()) {
                return;
            }
            let tag_on = format!("{}{}", expr.to_str(), expr.to_str());
            let tag_on = tag_on.as_str();
            assert!(operator(tag_on).is_err(), "Passed on {}", tag_on);
        });
    }

    #[test]
    fn test_operator2() {
        Expression::variants().iter().for_each(|expr| {
            let tag_on = format!("{}", expr.to_str());
            let tag_on = tag_on.as_str();
            assert!(operator(tag_on).is_ok(), "Failed on {}", tag_on);
        });
    }
}

fn not_expression_tag(input: &str) -> IResult<&str, ()> {
    not(alternating_expression_tags!())(input).map(|(i, _)| {
        return (i.trim(), ());
    })
}

#[cfg(test)]
mod test_not_expression_tags {
    use super::*;

    #[test]
    fn test_not_expression_tag1() {
        let suffix = "!";
        Expression::variants().iter().for_each(|expr| {
            let tag_on = format!("{}{}", expr.to_str(), suffix);
            assert!(
                not_expression_tag(tag_on.as_str()).is_err(),
                "Passed on {}",
                tag_on
            );
        });
    }

    #[test]
    fn test_not_expression_tag2() {
        let suffix = "a";
        let tag_on = format!("{}", suffix);
        assert!(
            not_expression_tag(tag_on.as_str()).is_ok(),
            "Failed on {}",
            tag_on
        );
    }

    #[test]
    fn test_not_expression_tag3() {
        let tag_on = format!("!=");
        assert!(
            not_expression_tag(tag_on.as_str()).is_err(),
            "Passed on {}",
            tag_on
        );
    }
}

fn hex_number(input: &str) -> IResult<&str, &str> {
    tuple((
        space0,
        recognize(tuple((tag("0x"), many1(satisfy(|c| c.is_hex_digit()))))),
        opt(take_while1(|c: char| c.is_whitespace())),
    ))(input)
    .map(|(i, o)| {
        return (i.trim(), o.1.trim());
    })
}

fn decimal_number(input: &str) -> IResult<&str, &str> {
    tuple((
        space0,
        take_while1(|c: char| c.is_digit(10)),
        peek(not(many1(satisfy(|c| {
            c.is_alphabetic() || c.is_ascii_punctuation()
        })))),
    ))(input)
    .map(|(i, o)| (i.trim(), o.1.trim()))
}
fn hex_number_with_paren(input: &str) -> IResult<&str, &str> {
    tuple((
        space0,
        recognize(tuple((tag("0x"), many1(satisfy(|c| c.is_hex_digit()))))),
        opt(take_while1(|c: char| c.is_whitespace() || c != ')')),
    ))(input)
    .map(|(i, o)| {
        return (i.trim(), o.1.trim());
    })
}

fn decimal_number_with_paren(input: &str) -> IResult<&str, &str> {
    tuple((
        space0,
        take_while1(|c: char| c.is_digit(10)),
        peek(not(many1(satisfy(|c| {
            (c.is_alphabetic() || c.is_ascii_punctuation()) && c != ')'
        })))),
    ))(input)
    .map(|(i, o)| (i.trim(), o.1.trim()))
}

fn number(input: &str) -> IResult<&str, &str> {
    ws(alt((hex_number, decimal_number)))(input).map(|(i, o)| (i.trim(), o.trim()))
}

fn number_with_paren(input: &str) -> IResult<&str, &str> {
    ws(alt((
        alt((hex_number, decimal_number)),
        alt((hex_number_with_paren, decimal_number_with_paren)),
    )))(input)
    .map(|(i, o)| (i.trim(), o.trim()))
}

fn expression_a(input: &str) -> IResult<&str, Vec<&str>> {
    map(tuple((identifier, operator, number_with_paren)), |t| {
        vec![t.0, t.1, t.2]
    })(input)
    .map(|(i, o)| (i.trim(), o.into_iter().map(|f| f.trim()).collect()))
}

fn expression_b(input: &str) -> IResult<&str, Vec<&str>> {
    map(delimited(char('('), expression_a, char(')')), |v| v)(input)
        .map(|(i, o)| (i.trim(), o.into_iter().map(|f| f.trim()).collect()))
}

pub fn expression(input: &str) -> IResult<&str, Vec<&str>> {
    alt((expression_a, expression_b))(input)
}

fn logical_and(input: &str) -> IResult<&str, Vec<Vec<&str>>> {
    separated_list1(ws(tag("&&")), expression)(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse_conditional1() {
        let input = r#"(ItemID != 0) && ((ItemID & 0xf0000000) == 0)"#;
        let expected_rest = r#"&& ((ItemID & 0xf0000000) == 0)"#;
        let expected_result = vec!["ItemID","!=","0"];

        let (rest, result) = expression(input).unwrap();

        assert_eq!(rest, expected_rest);
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_parse_conditional2() {
        let input = r#"ItemID != 0"#;
        let expected_rest = r#""#;
        let expected_result = vec!["ItemID", "!=", "0"];

        let (rest, result) = expression(input).unwrap();

        assert_eq!(rest, expected_rest);
        assert_eq!(result, expected_result);
    }

}
