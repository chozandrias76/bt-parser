use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_while, take_while1},
    character::complete::{char, multispace0, not_line_ending},
    combinator::recognize,
    multi::many0_count,
    sequence::{delimited, preceded, tuple},
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

fn parse_braced_block(input: &str) -> IResult<&str, &str> {
    recognize(tuple((
        char('{'),
        many0_count(alt((
            delimited(char('{'), take_until("}"), char('}')),
            take_while1(|c| c != '{' && c != '}'),
        ))),
        char('}'),
    )))(input)
}

fn parse_remaining_block(input: &str) -> IResult<&str, &str> {
    recognize(tuple((
        parse_braced_block,
        multispace0,
        take_while(|c: char| c != ';'),
        char(';'),
    )))(input)
}

pub fn typedef_line(input: &str) -> IResult<&str, &str> {
    recognize(tuple((
        parse_typedef_keyword,
        multispace0,
        parse_struct_keyword,
        multispace0,
        parse_name,
        multispace0,
        parse_remaining_block,
    )))(input)
}
