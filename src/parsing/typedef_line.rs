use crate::types::nested::Nested;
use nom::{
    bytes::complete::{tag, take_while},
    character::complete::multispace0,
    combinator::{opt, recognize},
    sequence::tuple,
    IResult,
};

fn parse_typedef_keyword(input: &str) -> IResult<&str, &str> {
    tag("typedef")(input)
}

fn parse_struct_keyword(input: &str) -> IResult<&str, &str> {
    tag("struct")(input)
}

fn parse_name(input: &str) -> IResult<&str, &str> {
    take_while(|c: char| c.is_alphanumeric() || c == '_')(input)
}

pub fn typedef_line(input: &str) -> IResult<&str, Nested> {
    let mut parser = tuple((
        parse_typedef_keyword,
        multispace0,
        parse_struct_keyword,
        multispace0,
        opt(parse_name),
    ));
    parser(input).map(|(input, values)| {
        let name = match values.4 {
            Some(name) => name.into(),
            None => "".into(),
        };
        let nested_texts = vec![
            Nested::Text(values.0.into()),
            // Ignore multi spaces
            Nested::Text(values.2.into()),
            // Ignore multi spaces
            Nested::Text(name),
        ];
        (input, Nested::List(nested_texts))
    })
}

#[cfg(test)]
mod tests {
    use crate::types::nested::Nested;

    use super::*;

    #[test]
    fn test_parse_typedef_line1() {
        let input = r#"typedef struct {
} PlayerGameData <size=0x1B0>;
"#;
        let result = typedef_line(input);
        assert_eq!(
            result,
            Ok((
                r#"{
} PlayerGameData <size=0x1B0>;
"#,
                vec!["typedef".into(), "struct".into(), "".into()].into()
            ))
        );
    }
}
