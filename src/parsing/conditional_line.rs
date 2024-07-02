use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::{char, multispace0},
    combinator::peek,
    error::context,
    sequence::{preceded, tuple},
    IResult,
};

use crate::types::nested::Nested;

fn parse_conditional_line(input: &str) -> IResult<&str, (&str, char, &str)> {
    let mut parser = context(
        "conditional_line",
        tuple((
            alt((tag("if"), tag("else if"))),
            peek(preceded(multispace0, char('('))),
            take_while1(|c: char| c != '{'),
        )),
    );
    parser(input)
}

/// Parses a conditional line from the given input string.
///
/// This function is designed to parse conditional lines, specifically targeting
/// syntaxes like `if` and `else if` followed by conditions enclosed in parentheses.
/// # Example
///
/// ```
/// use nom::{IResult, error::ErrorKind};
/// use bt_parser::parsing::conditional_line::conditional_line;
/// use bt_parser::types::nested::Nested;
///
/// let input = "if ( (ItemID != 0) && ((ItemID & 0xf0000000) == 0)) {";
/// let result = conditional_line(input).unwrap();
/// assert_eq!(result, ("{", Nested::List(vec!["if".into(),"( (ItemID != 0) && ((ItemID & 0xf0000000) == 0))".into(),].into())));
/// ```
///
/// # Parameters
/// - `input`: A string slice that holds the input to be parsed.
///
/// # Returns
/// - `IResult<&str, Nested>`: On success, returns `Ok` wrapping the remaining input
///   and a tuple containing the matched keyword (`"if"` or `"else if"`),
///   and the condition string up to (but not including) the opening
///   curly brace `{`. On failure, returns an error wrapped in `Err`.
pub fn conditional_line(input: &str) -> IResult<&str, Nested> {
    parse_conditional_line(input).map(|(rest, result)| {
        let (keyword, _, condition) = result;
        (
            rest.trim(),
            vec![keyword.into(), condition.trim().into()].into(),
        )
    })
}

#[cfg(test)]
mod conditional_line_tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_conditional_line1() {
        let input_if_else = r#"if ( (ItemID != 0) && ((ItemID & 0xf0000000) == 0)) { 
          int32 unk; 
          int32 unk2;
          int32 ash_of_war_ga_item_handle<format=hex>;
          byte unk4;
      }
      else if((ItemID != 0) && ((ItemID & 0xf0000000) == 0x10000000)) { 
          int32 unk;
          int32 unk2;
      }"#;
        let expected_result = Nested::List(
            vec![
                "if".into(),
                "( (ItemID != 0) && ((ItemID & 0xf0000000) == 0))".into(),
            ]
            .into(),
        );

        let expected_rest = r#"{ 
          int32 unk; 
          int32 unk2;
          int32 ash_of_war_ga_item_handle<format=hex>;
          byte unk4;
      }
      else if((ItemID != 0) && ((ItemID & 0xf0000000) == 0x10000000)) { 
          int32 unk;
          int32 unk2;
      }"#;

        assert_eq!(
            conditional_line(input_if_else),
            Ok((expected_rest, expected_result))
        );
    }

    #[test]
    fn test_conditional_line2() {
        let input_if_else = r#"else if((ItemID != 0) && ((ItemID & 0xf0000000) == 0x10000000)) {
          int32 unk;
          int32 unk2;
      }"#;
        let expected_result = Nested::List(
            vec![
                "else if".into(),
                "((ItemID != 0) && ((ItemID & 0xf0000000) == 0x10000000))".into(),
            ]
            .into(),
        );
        let expected_rest = r#"{
          int32 unk;
          int32 unk2;
      }"#;
        let (rest, result) = conditional_line(input_if_else).unwrap();

        assert_eq!(rest, expected_rest);
        assert_eq!(result, expected_result);
    }
}
