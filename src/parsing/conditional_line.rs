use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_until1, take_while1},
    character::complete::{char, line_ending, multispace0, not_line_ending, space0},
    combinator::peek,
    error::context,
    multi::{many0, many1},
    sequence::{preceded, tuple},
    IResult,
};

use crate::{ast::Expression, types::nested::Nested};
use crate::ast::Expression::{And, Or, Equals, NotEquals, GreaterThan, LessThan, BinaryAnd};

fn parse_nested_parens(input: &str) -> IResult<&str, Vec<&str>> {
    let mut stack = Vec::new();
    let mut start_index = None;
    let mut nested_expressions = Vec::new();
    let mut rest = "";
    let mut is_nested = false;

    for (index, character) in input.char_indices() {
        match character {
            '(' => {
                if stack.is_empty() {
                    start_index = Some(index);
                }
                stack.push(character);
                is_nested = false;
            }
            ')' => {
                if let Some(_) = stack.pop() {
                    if stack.is_empty() {
                        if let Some(start) = start_index {
                            // Capture the expression excluding the outermost parentheses
                            rest = &input[index + character.len_utf8()..];
                            let including_parens_range = start..index + 1;
                            nested_expressions.push(input[including_parens_range].trim());
                        }
                        start_index = None;
                    }
                } else {
                    // Handle unbalanced parentheses: early return or error
                    return Err(nom::Err::Error(nom::error_position!(
                        rest,
                        nom::error::ErrorKind::Fail
                    )));
                }
                is_nested = true;
            }
            _ => {}
        }
    }

    if stack.is_empty() && is_nested{
        Ok((rest.trim_start(), nested_expressions))
    } else {
        // Handle unbalanced parentheses: early return or error
        Err(nom::Err::Error(nom::error_position!(
            input,
            nom::error::ErrorKind::Fail
        )))
    }
}

#[cfg(test)]
mod tests_parse_nested_parens {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse_nested_parens1() {
        let input = r#"( (ItemID != 0) && ((ItemID & 0xf0000000) == 0) )"#;
        let expected_rest = r#""#;
        let expected_result = r#"( (ItemID != 0) && ((ItemID & 0xf0000000) == 0) )"#;

        let (rest, result) = parse_nested_parens(input).unwrap();

        assert_eq!(rest, expected_rest);
        assert_eq!(result, vec![expected_result]);
    }

    #[test]
    fn test_parse_nested_parens2() {
        let input = r#"( ItemID != 0 )"#;
        let expected_rest = r#""#;
        let expected_result = r#"( ItemID != 0 )"#;

        let (rest, result) = parse_nested_parens(input).unwrap();

        assert_eq!(rest, expected_rest);
        assert_eq!(result, vec![expected_result]);
    }

    #[test]
    fn test_parse_nested_parens3() {
        let input = r#"( (ItemID != 0) ) {
    }"#;
        let expected_rest = r#"{
    }"#;
        let expected_result = r#"( (ItemID != 0) )"#;

        let (rest, result) = parse_nested_parens(input).unwrap();

        assert_eq!(rest, expected_rest);
        assert_eq!(result, vec![expected_result]);
    }
}

pub fn conditional_line(input: &str) -> IResult<&str, Nested> {
    let mut parser = context(
        "conditional_line",
        tuple((
            alt((tag("if"), tag("else if"))),
            peek(preceded(multispace0, char('('))),
            take_while1(|c: char| c != '{'),
        )),
    );
    parser(input).map(|(input, s)| {
        let (if_else_or_else_if, _paren, inside_parens) = s;
        let (_rest, nested_parens) = parse_nested_parens(inside_parens.trim()).unwrap();
        let nested_parens_as_nested = nested_parens
            .iter()
            .map(|s| return Nested::Text(s.to_string()))
            .collect();
        return (
            input,
            Nested::List(vec![
                if_else_or_else_if.trim_end().into(),
                nested_parens_as_nested,
            ]),
        );
    })
}

#[cfg(test)]
mod conditional_line_tests {
    use crate::vec_nested;

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
            Ok((
                expected_rest,
                Nested::List(vec![
                    Nested::Text("if".into()),
                    Nested::List(vec![
                        Nested::Text("( (ItemID != 0) && ((ItemID & 0xf0000000) == 0))".into()),
                    ]),
                ]) // "if ( (ItemID != 0) && ((ItemID & 0xf0000000) == 0))".into()
            ))
        );
    }

    #[test]
    fn test_conditional_line2() {
        let input_if_else = r#"else if((ItemID != 0) && ((ItemID & 0xf0000000) == 0x10000000)) {
          int32 unk;
          int32 unk2;
      }"#;
        let expected_result = r#"else if"#;
        let expected_rest = r#"{
          int32 unk;
          int32 unk2;
      }"#;
        let (rest, result) = conditional_line(input_if_else).unwrap();

        assert_eq!(rest, expected_rest);
        assert_eq!(
            result,
            vec_nested![
                expected_result.into(),
                vec_nested!["((ItemID != 0) && ((ItemID & 0xf0000000) == 0x10000000))"]
            ]
            .into()
        );
    }
}
