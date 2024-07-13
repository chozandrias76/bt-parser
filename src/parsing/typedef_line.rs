pub mod typedef_member;
pub mod typedef_members;

use crate::types::nested::Nested;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_till, take_till1, take_while},
    character::complete::{anychar, multispace0, space0},
    combinator::{map_res, opt, peek, recognize, value},
    error::{context, Error, ErrorKind, ParseError, VerboseError, VerboseErrorKind},
    multi::many1,
    sequence::{delimited, tuple, Tuple},
    Err, IResult, Map, Parser,
};

use super::declaration_line::special_attributes::{self, special_attributes};
use crate::shared::take_until_unbalanced::take_until_unbalanced;

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

fn parse_optional_name<'a>(input: &str) -> IResult<&str, Option<&str>, nom::error::Error<&str>> {
    context(
        "optional name",
        opt(recognize(tuple((
            take_while(char::is_alphanumeric),
            // take_till(|c: char| c == ' '),
            space0::<_, nom::error::Error<_>>,
        )))),
    )(input)
}

fn parse_typedef_with_alias_name(input: &str) -> UniversalEol {
    let (input, optional_name) = parse_optional_name(input)?;
    let optional_name = optional_name.unwrap();

    let _optional_space = space0::<_, nom::error::Error<_>>(input)?;
    let (input, curly_bracket_contents) = take_until_unbalanced('{', '}')(input)?;
    let _optional_space = space0::<_, nom::error::Error<_>>(input)?;

    let (input, optional_name1) = parse_optional_name(input)?;
    let optional_name1 = optional_name1.unwrap();
    let _optional_space = space0::<_, nom::error::Error<_>>(input)?;
    let (input, special_attribute) = take_until_unbalanced('<', '>')(input)?;
    let _optional_space = space0::<_, nom::error::Error<_>>(input)?;
    let (input, special_attribute) = special_attributes(special_attribute)?;
    let name = if !(optional_name.trim().is_empty() || optional_name1.trim().is_empty()) {
        Nested::List(vec![optional_name.trim().into(), optional_name1.trim().into()].into())
    } else if optional_name.trim().is_empty() {
        Nested::Text(optional_name1.trim().into())
    } else if optional_name1.trim().is_empty() {
        Nested::Text(optional_name.trim().into())
    } else {
        panic!("Unexpected case")
    };
    Ok((
        input,
        Nested::List(vec![
            name,
            Nested::Text(curly_bracket_contents.trim_end().into()),
            special_attribute,
        ]),
    ))
}

#[cfg(test)]
mod alias_name_tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse_alias_name1() {
        let input = r#"{
    } PlayerGameData <size=0x1B0>;"#;
        let result = parse_typedef_with_alias_name(input);
        assert!(result.is_ok());
        let (input, result) = result.unwrap();
        assert_eq!(input, "");
        match result {
            Nested::List(values) => {
                assert_eq!(values.len(), 3);
                match &values[0] {
                    Nested::List(_) => panic!("Expected Nested::List"),

                    Nested::Text(value) => assert_eq!(value, "PlayerGameData"),
                }
                assert_eq!(values[1], Nested::Text("".into()));
                match &values[2] {
                    Nested::List(values) => {
                        assert_eq!(values.len(), 2);
                        assert_eq!(values[0], Nested::Text("size=".into()));
                        assert_eq!(values[1], Nested::Text("0x1B0".into()));
                    }
                    _ => panic!("Expected Nested::List"),
                }
            }
            _ => panic!("Expected Nested::List"),
        }
    }

    #[test]
    fn test_parse_alias_name2() {
        let input = r#"{
    } PlayerGameData;"#;
        let result = parse_typedef_with_alias_name(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_alias_name3() {
        let input = r#"PlayerGameData {
    };"#;
        let result = parse_typedef_with_alias_name(input);
        assert!(result.is_err());
    }
}

pub fn typedef_line(input: &str) -> IResult<&str, Nested> {
    let mut parser = tuple((
        parse_typedef_keyword,
        value((), multispace0),
        alt((parse_struct_keyword,)), // should implement other keywords
        value((), multispace0),
        alt((
            parse_typedef_with_alias_name,
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
        assert_eq!(result, Ok(("{", "Foo".into())));
    }

    #[test]
    fn test_parse_typedef_name2() {
        let input = r#"Bar"#;
        match input.matches('{').count() {
            0 => (),
            _ => panic!("test input should not contain '{{' in order to pass this test"),
        }
        let result = parse_typedef_name(input);
        assert!(result.is_err());
    }
}
#[cfg(test)]
mod typedef_line_tests {
    use crate::types::nested::Nested;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_parse_typedef_line1() {
        let input = r#"typedef struct Foo {
} PlayerGameData <size=0x1B0>;
"#;
        let result = typedef_line(input);
        assert!(result.is_ok());
        let (input, result) = result.unwrap();
        assert_eq!(input, "");
        match result {
            Nested::List(values) => {
                assert_eq!(values.len(), 3);
                assert_eq!(values[0], Nested::Text("typedef".into()));
                assert_eq!(values[1], Nested::Text("struct".into()));
                match &values[2] {
                    Nested::List(values) => {
                        assert_eq!(values.len(), 3);
                        match &values[0] {
                            Nested::List(value) => {
                                assert_eq!(value.len(), 2);
                                assert_eq!(value[0], Nested::Text("Foo".into()));
                                assert_eq!(value[1], Nested::Text("PlayerGameData".into()));
                            },
                            _ => panic!("Expected Nested::List"),
                        }
                        assert_eq!(values[1], Nested::Text("".into()));
                        match &values[2] {
                            Nested::List(values) => {
                                assert_eq!(values.len(), 2);
                                assert_eq!(values[0], Nested::Text("size=".into()));
                                assert_eq!(values[1], Nested::Text("0x1B0".into()));
                            }
                            _ => panic!("Expected Nested::List"),
                        }
                    }
                    _ => panic!("Expected Nested::List"),
                }
            }
            _ => panic!("Expected Nested::List"),
        }
    }

    #[test]
    fn test_parse_typedef_line2() {
        let input = r#"typedef struct {
  wchar_t  CharacterName[0x10];
} PlayerGameData <size=0x1B0>;
"#;
        let result = typedef_line(input);
        assert!(result.is_ok());
        let (input, result) = result.unwrap();
        assert_eq!(input, "");
        match result {
            Nested::List(values) => {
                assert_eq!(values.len(), 3);
                assert_eq!(values[0], Nested::Text("typedef".into()));
                assert_eq!(values[1], Nested::Text("struct".into()));
                match &values[2] {
                    Nested::List(values) => {
                        assert_eq!(values.len(), 3);
                        assert_eq!(values[0], Nested::Text("PlayerGameData".into()));
                        match &values[1] {
                            Nested::Text(value) => {
                                let byte_value = value.as_bytes();
                                let expected_byte_value = "wchar_t  CharacterName[0x10];".as_bytes(); 
                                assert_eq!(byte_value, expected_byte_value);
                            },
                            _ => panic!("Expected Nested::Text"),
                        }
                        match &values[2] {
                            Nested::List(values) => {
                                assert_eq!(values.len(), 2);
                                assert_eq!(values[0], Nested::Text("size=".into()));
                                assert_eq!(values[1], Nested::Text("0x1B0".into()));
                            }
                            _ => panic!("Expected Nested::List"),
                        }
                    }
                    _ => panic!("Expected Nested::List"),
                }
            }
            _ => panic!("Expected Nested::List"),
        }
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
