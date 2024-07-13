use nom::{
    bytes::complete::{tag, take_until1},
    combinator::map_res,
    error::ParseError,
    IResult,
};

fn get_remaining_input<'a>(
    input: &'a str,
    first_bracket_index: usize,
    last_bracket_index: usize,
) -> &'a str {
    let part2 = &input[last_bracket_index + 1..];

    &input[..first_bracket_index + part2.len()].trim()
}

/// A parser designed to work inside the `nom::sequence::delimited` parser, e.g.:
/// ```
/// use nom::bytes::complete::tag;
/// use nom::sequence::delimited;
/// use parse_hyperlinks::take_until_unbalanced;
///
/// let mut parser = delimited(tag("<"), take_until_unbalanced('<', '>'), tag(">"));
/// assert_eq!(parser("<<inside>inside>abc"), Ok(("abc", "<inside>inside")));
/// ```
/// It skips nested brackets until it finds an extra unbalanced closing bracket. Escaped brackets
/// like `\<` and `\>` are not considered as brackets and are not counted. This function is
/// very similar to `nom::bytes::complete::take_until(">")`, except it also takes nested brackets.
pub fn take_until_unbalanced<'a>(
    opening_bracket: char,
    closing_bracket: char,
) -> impl Fn(&'a str) -> IResult<&'a str, &'a str, nom::error::Error<&'a str>> {
    move |i: &'a str| {
        let mut index = 0;
        let mut first_bracket_index = None;
        let mut last_bracket_index = None;
        let mut bracket_counter = 0;
        let mut found_one_opening_bracket_pair = false;
        let mut found_one_closing_bracket_pair = false;
        while let Some(n) = &i[index..].find(&[opening_bracket, closing_bracket][..]) {
            index += n;
            let mut it = i[index..].chars();
            match it.next().unwrap_or_default() {
                c if c == opening_bracket => {
                    bracket_counter += 1;
                    if !found_one_opening_bracket_pair {
                        first_bracket_index = Some(index);
                    }
                    index += opening_bracket.len_utf8();
                    found_one_opening_bracket_pair = true;
                }
                c if c == closing_bracket => {
                    // Closing bracket.
                    last_bracket_index = Some(index);
                    bracket_counter -= 1;
                    index += closing_bracket.len_utf8();
                    found_one_closing_bracket_pair = true;
                }
                // Can not happen.
                _ => unreachable!(),
            };
            // We found the unmatched closing bracket.
            let _ = if bracket_counter == -1 {
                Err::<nom::error::ErrorKind, nom::Err<nom::error::Error<&str>>>(nom::Err::Error(
                    nom::error::Error::from_error_kind(i, nom::error::ErrorKind::TagClosure),
                ))
            } else {
                continue;
            };
        }

        if bracket_counter == 0 && found_one_opening_bracket_pair && found_one_closing_bracket_pair
        {
            match (first_bracket_index, last_bracket_index) {
                (Some(first_bracket_index), Some(last_bracket_index)) => {
                    if let Some(data) = i.get(index..) {
                        if data.is_empty() {
                            let output = &i[first_bracket_index + opening_bracket.len_utf8()
                                ..last_bracket_index];

                            let remaining_input: &str =
                                get_remaining_input(i, first_bracket_index, last_bracket_index);
                            return Ok((remaining_input, output.trim()));
                        }
                        match take_until1::<&str, &str, nom::error::Error<&str>>(";")(data) {
                            Ok((_remaining_input, output)) => {
                                let input = &i[first_bracket_index + opening_bracket.len_utf8()
                                    ..last_bracket_index]
                                    .trim();

                                return Ok((output.trim(), input));
                            }
                            Err(_) => {
                                return Err(nom::Err::Error(nom::error::Error::from_error_kind(
                                    i,
                                    nom::error::ErrorKind::Tag,
                                )));
                            }
                        }
                    }
                    let output =
                        &i[first_bracket_index + opening_bracket.len_utf8()..last_bracket_index];

                    let remaining_input: &str =
                        get_remaining_input(i, first_bracket_index, last_bracket_index);
                    Ok((remaining_input, output.trim()))
                }
                _ => unreachable!(),
            }
        } else {
            Err(nom::Err::Error(nom::error::Error::from_error_kind(
                i,
                nom::error::ErrorKind::Tag,
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::error::ErrorKind;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse_until_unbalanced_parens() {
        assert!(take_until_unbalanced('<', '>')("abc").is_err());
    }

    #[test]
    fn test_take_until_unmatched1() {
        assert!(take_until_unbalanced('(', ')')("abc)").is_err());
    }

    #[test]
    fn test_take_until_unmatched2() {
        assert_eq!(take_until_unbalanced('(', ')')("(abc)"), Ok(("", "abc")));
    }

    #[test]
    fn test_take_until_unmatched3() {
        assert!(take_until_unbalanced('(', ')')("{abc)").is_err());
    }

    #[test]
    fn test_take_until_unmatched4() {
        assert!(take_until_unbalanced('(', ')')("def{abc)").is_err());
    }

    #[test]
    fn test_take_until_unmatched5() {
        let result = take_until_unbalanced('{', '}')(
            "{
                anything
            } PlayerGameData <size=0x1B0>;",
        );
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.0, "PlayerGameData <size=0x1B0>");
        assert_eq!(result.1, "anything");
    }
}
