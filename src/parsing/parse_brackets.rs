use nom::IResult;

pub fn parse_brackets(input: &str) -> IResult<&str, &str> {
    let mut stack = Vec::new();
    let mut start_index = None;
    let mut contents = "";
    let mut rest = "";
    let mut is_nested = false;
    let mut open_bracket_idx = None;
    let mut closed_bracket_idx = None;

    for (index, character) in input.char_indices() {
        match character {
            '{' => {
                open_bracket_idx = Some(index);
                if stack.is_empty() {
                    start_index = Some(index);
                }
                stack.push(character);
                is_nested = false;
            }
            '}' => {
                closed_bracket_idx = Some(index);
                if let Some(_) = stack.pop() {
                    if stack.is_empty() {
                        if let Some(start) = start_index {
                            // Capture the expression excluding the outermost brackets
                            rest = &input[index + character.len_utf8()..];
                            let including_brackets_range = start..index + 1;
                            contents = input[including_brackets_range].trim();
                            break;
                        }
                        start_index = None;
                    }
                } else {
                    // Handle unbalanced brackets: early return or error
                    return Err(nom::Err::Error(nom::error_position!(
                        rest,
                        nom::error::ErrorKind::Fail
                    )));
                }
                is_nested = true;
            }
            _ => {
                if open_bracket_idx.is_none() && closed_bracket_idx.is_none() {
                    return Err(nom::Err::Error(nom::error_position!(
                        rest,
                        nom::error::ErrorKind::Fail
                    )));
                }
            }
        }
    }

    if stack.is_empty() {
        let rest = if rest.starts_with(";") {
            let range = 1..rest.len();
            rest[range].trim()
        } else {
            rest
        };
        Ok((rest.trim_start(), contents))
    } else {
        // Handle unbalanced brackets: early return or error
        Err(nom::Err::Error(nom::error_position!(
            input,
            nom::error::ErrorKind::Fail
        )))
    }
}

#[cfg(test)]
mod tests_parse_nested_brackets {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse_nested_brackets1() {
        let input = r#"{
        x += i;
    }"#;
        let expected_rest = r#""#;
        let expected_result = r#"{
        x += i;
    }"#;

        let (rest, result) = parse_brackets(input).unwrap();

        assert_eq!(rest, expected_rest);
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_parse_nested_brackets4() {
        let input = r#"a{}"#;
        let result = parse_brackets(input);
        assert!(
            result.is_err(),
            "Expected error due to characters before brackets: {:?}",
            result
        );
    }

    #[test]
    fn test_parse_nested_brackets5() {
        let input = r#"{a"#;
        let result = parse_brackets(input);
        assert!(
            result.is_err(),
            "Expected error due to unterminated brackets: {:?}",
            result
        );
    }

    #[test]
    fn test_parse_nested_brackets6() {
        let input = r#"a}"#;
        let result = parse_brackets(input);
        assert!(
            result.is_err(),
            "Expected error due to unmatched brackets: {:?}",
            result
        );
    }

    #[test]
    fn test_parse_nested_brackets7() {
        let input = r#"a"#;
        let result = parse_brackets(input);
        assert!(
            result.is_err(),
            "Expected error due to missing brackets: {:?}",
            result
        );
    }

    #[test]
    fn test_parse_nested_brackets8() {
        let input = r#"{a}
        {b}"#;
        let expected_rest = r#"{b}"#;
        let expected_result = r#"{a}"#;

        let (rest, result) = parse_brackets(input).unwrap();

        assert_eq!(rest, expected_rest);
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_parse_nested_brackets9() {
        let input = r#"{a};"#;
        let expected_rest = r#""#;
        let expected_result = r#"{a}"#;

        let (rest, result) = parse_brackets(input).unwrap();

        assert_eq!(rest, expected_rest);
        assert_eq!(result, expected_result);
    }
}
