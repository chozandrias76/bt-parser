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

fn operator(input: &str) -> IResult<&str, &str> {
    ws(alt((
        tag(Expression::And.to_str()),
        tag(Expression::Or.to_str()),
        tag(Expression::Equals.to_str()),
        tag(Expression::NotEquals.to_str()),
        tag(Expression::GreaterThan.to_str()),
        tag(Expression::LessThan.to_str()),
        tag(Expression::BitAnd.to_str()),
    )))(input)
    .map(|(i, o)| (i.trim(), o.trim()))
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

fn expression(input: &str) -> IResult<&str, Vec<&str>> {
    alt((expression_a, expression_b))(input)
}

fn logical_and(input: &str) -> IResult<&str, Vec<Vec<&str>>> {
    separated_list1(ws(tag("&&")), expression)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decimal_number() {
        assert_eq!(decimal_number("0"), Ok(("", "0")));
        assert_eq!(decimal_number("0 "), Ok(("", "0")));
        assert_eq!(decimal_number(" 0"), Ok(("", "0")));
        assert_eq!(decimal_number(" 0 "), Ok(("", "0")));
    }

    #[test]
    fn test_expected_decimal_number_failures() {
        assert!(decimal_number("0x0").is_err());
        assert!(decimal_number("0x0 ").is_err());
        assert!(decimal_number(" 0x0").is_err());
        assert!(decimal_number(" 0x0 ").is_err());
        assert!(decimal_number("0f0").is_err());
    }

    #[test]
    fn test_hex_number() {
        assert_eq!(hex_number("0xf0000000"), Ok(("", "0xf0000000")));
        assert_eq!(hex_number("0xf0000000 "), Ok(("", "0xf0000000")));
        assert_eq!(hex_number(" 0xf0000000"), Ok(("", "0xf0000000")));
        assert_eq!(hex_number(" 0xf0000000 "), Ok(("", "0xf0000000")));
    }

    #[test]
    fn test_identifier() {
        assert_eq!(identifier("ItemID"), Ok(("", "ItemID")));
        assert_eq!(identifier("ItemID "), Ok(("", "ItemID")));
        assert_eq!(identifier(" ItemID"), Ok(("", "ItemID")));
        assert_eq!(identifier(" ItemID "), Ok(("", "ItemID")));
    }

    #[test]
    fn test_operator() {
        assert_eq!(operator("!="), Ok(("", "!=")));
        assert_eq!(operator("!= "), Ok(("", "!=")));
        assert_eq!(operator(" !="), Ok(("", "!=")));
        assert_eq!(operator(" != "), Ok(("", "!=")));
    }

    #[test]
    fn test_number() {
        assert_eq!(number("0"), Ok(("", "0")));
        assert_eq!(number("0 "), Ok(("", "0")));
        assert_eq!(number(" 0"), Ok(("", "0")));
        assert_eq!(number(" 0 "), Ok(("", "0")));
        assert_eq!(number("0xf0000000"), Ok(("", "0xf0000000")));
        assert_eq!(number("0xf0000000 "), Ok(("", "0xf0000000")));
        assert_eq!(number(" 0xf0000000"), Ok(("", "0xf0000000")));
        assert_eq!(number(" 0xf0000000 "), Ok(("", "0xf0000000")));
    }

    #[test]
    fn test_expression_a() {
        assert_eq!(
            expression_a("ItemID != 0"),
            Ok(("", vec!["ItemID", "!=", "0"]))
        );
        assert_eq!(
            expression_a("ItemID != 0 "),
            Ok(("", vec!["ItemID", "!=", "0"]))
        );
        assert_eq!(
            expression_a(" ItemID != 0"),
            Ok(("", vec!["ItemID", "!=", "0"]))
        );
        assert_eq!(
            expression_a(" ItemID != 0 "),
            Ok(("", vec!["ItemID", "!=", "0"]))
        );
        Expression::all_expressions().iter().for_each(|op| {
            assert_eq!(
                expression_a(&format!("ItemID {} 0", op)),
                Ok(("", vec!["ItemID", op, "0"]))
            )
        });
        assert_eq!(
            expression_a("FooBar == 0x0"),
            Ok(("", vec!["FooBar", "==", "0x0"]))
        );
    }

    #[test]
    fn test_expression_b() {
        assert_eq!(
            expression_b("(ItemID != 0)"),
            Ok(("", vec!["ItemID", "!=", "0"]))
        );
        assert_eq!(
            expression_b("(ItemID != 0) "),
            Ok(("", vec!["ItemID", "!=", "0"]))
        );
    }
}
