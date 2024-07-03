use nom::{
    bytes::complete::{tag, take_while1},
    character::complete::{alphanumeric1, multispace0, multispace1},
    combinator::{map, opt},
    error::context,
    sequence::{delimited, preceded, tuple},
    IResult,
};

use crate::types::nested::Nested;
use nom::error::Error;
use nom::Err;

fn type_parser(input: &str) -> IResult<&str, &str> {
    take_while1(|c: char| c.is_alphanumeric() || c == '_')(input)
}

fn identifier_parser(input: &str) -> IResult<&str, &str> {
    alphanumeric1(input)
}

fn format_parser(input: &str) -> IResult<&str, &str> {
    delimited(tag("<"), take_while1(|c: char| c != '>'), tag(">"))(input)
}

fn parse_declaration_content(input: &str) -> IResult<&str, (&str, &str, Option<&str>)> {
    map(
        tuple((
            type_parser,
            multispace1,
            identifier_parser,
            opt(preceded(multispace0, format_parser)),
        )),
        |(type_, _, identifier, special_attributes)| (type_, identifier, special_attributes),
    )(input)
}

fn parse_declaration_statement(
    input: &str,
) -> Result<(&str, (&str, &str, Option<&str>)), Err<Error<&str>>> {
    let mut parser = context(
        "declaration_statement",
        delimited(multispace0, parse_declaration_content, tag(";")),
    );
    parser(input)
}

pub fn declaration_statement(
    input: &str,
) -> Result<(&str, Nested), nom::Err<nom::error::Error<&str>>> {
    parse_declaration_statement(input).map(|(rest, result)| {
        let (type_, identifier, special_attributes) = result;
        let format_as_nested: Nested = match special_attributes.map(|f| f.into()) {
            Some(nested) => nested,
            None => Nested::Text("".into()),
        
        };
        (rest.trim(), Nested::List(vec![type_.into(), identifier.into(), format_as_nested]))
    })
}

#[cfg(test)]
mod declaration_statement_tests {
    use super::*;

    #[test]
    fn test_declaration_statement1() {
        let input = "int a;";
        let result = declaration_statement(input).unwrap();
        assert_eq!(result, ("", vec!["int".into(), "a".into(), "".into()].into()));
    }

    #[test]
    fn test_declaration_statement2() {
        let input = "int a; ";
        let result = declaration_statement(input).unwrap();
        assert_eq!(result, ("", vec!["int".into(), "a".into(), "".into()].into()));
    }

    #[test]
    fn test_declaration_statement3() {
        let input = " int a; ";
        let result = declaration_statement(input).unwrap();
        assert_eq!(result, ("", vec!["int".into(), "a".into(), "".into()].into()));
    }

    #[test]
    fn test_declaration_statement4() {
        let input = " int a;";
        let result = declaration_statement(input).unwrap();
        assert_eq!(result, ("", vec!["int".into(), "a".into(), "".into()].into()));
    }

    #[test]
    fn test_declaration_statement_with_format() {
        let input = "int a<0x%08X>;";
        let result = declaration_statement(input).unwrap();
        assert_eq!(result, ("", vec!["int".into(), "a".into(), "0x%08X".into()].into()));
    }

    #[test]
    fn test_error1() {
        let input = "// Additional data for type = Weapon";
        let result = declaration_statement(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_error2() {
        let input = "if ( (ItemID != 0) && ((ItemID & 0xf0000000) == 0)) { ";
        let result = declaration_statement(input);
        assert!(result.is_err());
    }
}