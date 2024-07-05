pub mod typedef_member;
pub mod typedef_members;
use crate::types::nested::Nested;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete::multispace0,
    combinator::{opt, peek, recognize},
    error::{context, VerboseError},
    multi::{many0, many1, many1_count},
    sequence::tuple,
    IResult,
};

fn parse_typedef_keyword(input: &str) -> IResult<&str, &str> {
    tag("typedef")(input.trim_start())
}

fn parse_struct_keyword(input: &str) -> IResult<&str, &str> {
    tag("struct")(input.trim_start())
}

fn parse_name(input: &str) -> IResult<&str, &str> {
    take_while(|c: char| c.is_alphanumeric() || c == '_')(input)
}

type UniversalEol<'a> = IResult<&'a str, Nested, nom::error::Error<&'a str>>;

fn parse_typedef_args(input: &str) -> UniversalEol {
    let mut parser = context(
        "argument structure",
        tuple((
            context("opening arg tag", tag("(")),
            context(
                "arguments",
                many1(tuple((
                    multispace0,
                    take_while(|c: char| c.is_alphanumeric()),
                    multispace0,
                    take_while(|c: char| c.is_alphanumeric()),
                    alt((tag(","), tag(")"))),
                ))),
            ),
        )),
    );
    parser(input.trim_start()).map(|(input, values)| {
        let nested_texts = values
            .1
            .into_iter()
            .map(|r| Nested::List(vec![Nested::Text(r.1.into()), Nested::Text(r.3.into())]))
            .collect::<Vec<Nested>>();
        (input, Nested::List(nested_texts))
    })
}

fn parse_typedef_name(input: &str) -> UniversalEol {
    let mut parser = context(
        "typedef name",
        tuple((parse_name, multispace0, peek(tag("{")))),
    );
    parser(input.trim_start()).map(|(input, values)| (input, Nested::Text(values.0.into())))
}

fn parse_typedef_up_to_bracket(input: &str) -> UniversalEol {
    let mut parser = context(
        "typedef up to bracket",
        tuple((multispace0, peek(tag("{")))),
    );
    parser(input.trim_start()).map(|(input, _)| (input, Nested::Text("".into())))
}

pub fn typedef_line(input: &str) -> IResult<&str, Nested> {
    let mut parser = tuple((
        parse_typedef_keyword,
        peek(multispace0),
        parse_struct_keyword,
        peek(multispace0),
        alt((
            parse_typedef_name,
            parse_typedef_up_to_bracket,
            parse_typedef_args,
        )),
    ));
    parser(input).map(|(input, values)| {
        let nested_texts = vec![
            Nested::Text(values.0.into()),
            // Ignore multi spaces
            Nested::Text(values.2.into()),
            // Ignore multi spaces
            values.4,
        ];
        (input.trim_start(), Nested::List(nested_texts))
    })
}

#[cfg(test)]
mod typedef_args_tests {
    use crate::types::nested::Nested;

    use super::*;

    #[test]
    fn test_parse_typedef_args1() {
        let input = r#"(int size, int size2)"#;
        let result = parse_typedef_args(input);
        assert_eq!(
            result,
            Ok((
                "",
                Nested::List(vec![
                    Nested::List(vec!["int".into(), "size".into()]),
                    Nested::List(vec!["int".into(), "size2".into()]),
                ])
                .into()
            ))
        );
    }
}

#[cfg(test)]
mod typedef_name_tests {
    use crate::types::nested::Nested;

    use super::*;

    #[test]
    fn test_parse_typedef_name1() {
        let input = r#"Foo {"#;
        let result = parse_typedef_name(input);
        assert_eq!(
            result,
            Ok((
                "{",
                "Foo".into()
            ))
        );
    }

    #[test]
    fn test_parse_typedef_name2() {
        let input = r#"Bar"#;
        match input.matches('{').count() {
            0 => (),
            _ => panic!("test input should not contain '{{' in order to pass this test"),
        }
        let result = parse_typedef_name(input);
        assert!(
            result.is_err()
        );
    }
}
#[cfg(test)]
mod typedef_line_tests {
    use crate::types::nested::Nested;

    use super::*;

    #[test]
    fn test_parse_typedef_line1() {
        let input = r#"typedef struct Foo {
} PlayerGameData <size=0x1B0>;
"#;
        let result = typedef_line(input);
        assert_eq!(
            result,
            Ok((
                r#"{
} PlayerGameData <size=0x1B0>;
"#,
                vec!["typedef".into(), "struct".into(), "Foo".into()].into()
            ))
        );
    }

    #[test]
    fn test_parse_typedef_line2() {
        let input = r#"typedef struct {
  wchar_t  CharacterName[0x10];
} PlayerGameData <size=0x1B0>;
"#;
        let result = typedef_line(input);
        assert_eq!(
            result,
            Ok((
                r#"{
  wchar_t  CharacterName[0x10];
} PlayerGameData <size=0x1B0>;
"#,
                vec!["typedef".into(), "struct".into(), "".into()].into()
            ))
        );
    }

    #[test]
    fn test_parse_typedef_line3() {
        let input = r#"typedef struct (int size, int size2) {
  EquipInventoryDataEntry CharacterName[size];
} EquipInventoryData;
"#;
        let result = typedef_line(input);
        assert_eq!(
            result,
            Ok((
                r#"{
  EquipInventoryDataEntry CharacterName[size];
} EquipInventoryData;
"#,
                Nested::List(vec![
                    Nested::Text("typedef".into()),
                    Nested::Text("struct".into()),
                    Nested::List(vec![
                        Nested::List(vec!["int".into(), "size".into()]),
                        Nested::List(vec!["int".into(), "size2".into()])
                    ]),
                ])
            ))
        );
    }
}
