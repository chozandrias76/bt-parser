use nom::IResult;

pub fn parse_nested_parens(input: &str) -> IResult<&str, Vec<&str>> {
    let mut stack = Vec::new();
    let mut start_index = None;
    let mut nested_expressions = Vec::new();
    let mut rest = "";
    let mut is_nested = false;
    let mut open_paren_idx = None;
    let mut closed_paren_idx = None;

    for (index, character) in input.char_indices() {
        match character {
            '(' => {
                open_paren_idx = Some(index);
                if stack.is_empty() {
                    start_index = Some(index);
                }
                stack.push(character);
                is_nested = false;
            }
            ')' => {
                closed_paren_idx = Some(index);
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
            _ => {
                if open_paren_idx.is_none() && closed_paren_idx.is_none() {
                    return Err(nom::Err::Error(nom::error_position!(
                        rest,
                        nom::error::ErrorKind::Fail
                    )));
                }
            }
        }
    }

    if stack.is_empty() && is_nested {
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

    #[test]
    fn test_parse_nested_parens4() {
        let input = r#"a()"#;
        let result = parse_nested_parens(input);
        assert!(
            result.is_err(),
            "Expected error due to characters before parenthesis: {:?}",
            result
        );
    }

    #[test]
    fn test_parse_nested_parens5() {
        let input = r#"(a"#;
        let result = parse_nested_parens(input);
        assert!(
            result.is_err(),
            "Expected error due to unterminated parenthesis: {:?}",
            result
        );
    }

    #[test]
    fn test_parse_nested_parens6() {
        let input = r#"a)"#;
        let result = parse_nested_parens(input);
        assert!(
            result.is_err(),
            "Expected error due to unmatched parenthesis: {:?}",
            result
        );
    }

    #[test]
    fn test_parse_nested_parens7() {
        let input = r#"a"#;
        let result = parse_nested_parens(input);
        assert!(
            result.is_err(),
            "Expected error due to missing parenthesis: {:?}",
            result
        );
    }
}
