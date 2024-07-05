use nom::{
    bytes::{
        complete::tag,
        streaming::take_until,
    },
    character::complete::char,
    combinator::{not, peek, recognize},
    sequence::{preceded, tuple},
    IResult,
};

use crate::types::nested::Nested;

use super::typedef_member::typedef_member;

fn parse_typedef_to_terminator(input: &str) -> IResult<&str, &str> {
    let until_semicolon = take_until(";");
    let not_open_brace = not(peek(char('{')));

    // Combine parsers to ensure we stop at ';' and error if '{' is found before ';'
    let mut parse_logic = preceded(not_open_brace, until_semicolon);

    parse_logic(input).and_then(|(next_input, result)| {
        // Check if the result contains '{', which should not happen
        if result.contains('{') {
          Err(nom::Err::Error(nom::error_position!(
            input,
            nom::error::ErrorKind::Fail
        )))
        } else {
            Ok((next_input, result))
        }
    })
}

fn parse_statement_terminator(input: &str) -> IResult<&str, &str> {
    peek(tag(";"))(input)
}

fn trim_start_terminator(input: &str) -> &str {
    match input.chars().next() {
        Some(';') => &input[1..],
        _ => input,
    }
}

fn parse_typedef_to_end(input: &str) -> IResult<&str, &str> {
    recognize(tuple((
        parse_typedef_to_terminator,
        parse_statement_terminator,
    )))(input)
    .map(|(input, value)| {
        let rest_without_terminator = trim_start_terminator(input);
        return (rest_without_terminator, value.trim_end());
    })
}

pub fn typedef_members(input: &str) -> IResult<&str, Nested> {
    let mut members = Vec::new();
    let mut rest = input;
    loop {
        let result = typedef_member(rest);
        if result.is_err() {
            break;
        }
        let (new_rest, member) = result.unwrap();
        members.push(member);
        rest = new_rest;
    }
    match parse_typedef_to_end(rest) {
        Ok((rest, name)) => Ok((rest, Nested::List(vec![name.into(), Nested::List(members)]))),
        Err(_) => Err(nom::Err::Error(nom::error_position!(
            input,
            nom::error::ErrorKind::Fail
        ))),
    }
}

#[cfg(test)]
mod typedef_members_tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_typedef_members1() {
        // Test that a single member can be parsed
        let input = "unsigned int myInt;";
        let result = typedef_members(input);
        assert!(result.is_ok());
        let (rest, members) = result.unwrap();
        assert_eq!(rest, "");
        assert_eq!(
            members,
            Nested::List(vec![
                "myInt".into(),
                Nested::List(vec!["unsigned".into(), "int".into()])
            ])
        );
    }

    #[test]
    fn test_typedef_members2() {
        let input = "unsigned unsigned unsigned unsigned int myInt;";
        let result = typedef_members(input);
        assert!(result.is_ok());
        let (rest, members) = result.unwrap();
        assert_eq!(rest, "");
        assert_eq!(
            members,
            Nested::List(vec![
                "myInt".into(),
                Nested::List(vec![
                    "unsigned".into(),
                    "unsigned".into(),
                    "unsigned".into(),
                    "unsigned".into(),
                    "int".into()
                ])
            ])
        );
    }

    #[test]
    fn test_typedef_members3() {
        let input = "unsigned int myInt; unsigned int myInt2;";
        let result = typedef_members(input);
        assert!(result.is_ok());
        let (rest, members) = result.unwrap();
        assert_eq!(rest, " unsigned int myInt2;");
        assert_eq!(
            members,
            Nested::List(vec![
                "myInt".into(),
                Nested::List(vec!["unsigned".into(), "int".into()])
            ])
        );
    }

    #[test]
    fn test_typedef_member3() {
        let input = "struct {};";
        let result = typedef_members(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_typedef_member4() {
        let input = "struct {";
        let result = typedef_members(input);
        assert!(result.is_err());
    }
}
