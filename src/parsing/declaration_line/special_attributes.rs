use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::not_line_ending,
    error::context,
    sequence::tuple,
    IResult,
};

use crate::types::nested::Nested;

fn parse_special_attributes(input: &str) -> IResult<&str, (&str, &str)> {
    let mut parser = context(
        "special_attributes",
        tuple((
            alt((
                tag("format="),
                tag("format="),
                tag("format="),
                tag("format="),
                tag("fgcolor="),
                tag("bgcolor="),
                tag("style="),
                tag("comment="),
                tag("name="),
                tag("open="),
                tag("hidden="),
                tag("read="),
                tag("write="),
                tag("size="),
                tag("optimize="),
                tag("disasm="),
            )),
            not_line_ending,
        )),
    );
    parser(input)
}

/// Validates special attributes that would normally be present between < and >
///
/// # Arguments
/// * `input` - A string slice that holds the input
///
/// # Returns
/// * `IResult<&str, Nested>`: On success, returns `Ok` wrapping the remaining input
///
/// # Example
///
/// ```
/// use nom::IResult;
/// use bt_parser::parsing::declaration_line::special_attributes::special_attributes;
/// use bt_parser::types::nested::Nested;
///   
/// let input = "format=hex";
/// let result = special_attributes(input).unwrap();
/// assert_eq!(result, ("", Nested::List(vec![Nested::Text("format=".into()), Nested::Text("hex".into())].into())));
/// ```
///
/// # Panics
/// Panics if the input is not a valid special attribute
/// One of: [link](https://www.sweetscape.com/010editor/manual/TemplateVariables.htm)
pub fn special_attributes(input: &str) -> IResult<&str, Nested> {
    match parse_special_attributes(input) {
        Ok((rest, result)) => Ok((rest, vec![result.0.into(), result.1.into()].into())),
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod tests_special_attributes {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_special_attributes1() {
        let input = r#"read=Str("<%g %g %g>",this[0],this[1],this[2])"#;
        let expected_rest = "";
        let expected_result = vec![
            "read=".into(),
            r#"Str("<%g %g %g>",this[0],this[1],this[2])"#.into(),
        ]
        .into();

        let (rest, result) = special_attributes(input).unwrap();

        assert_eq!(rest, expected_rest);
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_special_attributes2() {
        let input = "bgcolor=(this < 0 ? cRed : cNone )";
        let expected_rest = "";
        let expected_result = vec!["bgcolor=".into(), "(this < 0 ? cRed : cNone )".into()].into();

        let (rest, result) = special_attributes(input).unwrap();

        assert_eq!(rest, expected_rest);
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_special_attributes3() {
        let input = r#"comment="This should be greater than 15.""#;
        let expected_rest = "";
        let expected_result = vec![
            "comment=".into(),
            r#""This should be greater than 15.""#.into(),
        ]
        .into();

        let (rest, result) = special_attributes(input).unwrap();

        assert_eq!(rest, expected_rest);
        assert_eq!(result, expected_result);
    }
}
