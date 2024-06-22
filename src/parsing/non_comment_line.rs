use nom::{
    character::complete::{line_ending, not_line_ending},
    combinator::{opt, recognize},
    sequence::tuple,
    IResult,
};

pub fn non_comment_line(input: &str) -> IResult<&str, &str> {
    recognize(tuple((not_line_ending, opt(line_ending))))(input)
}
