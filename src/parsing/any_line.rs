use nom::{
    error::{Error, ErrorKind},
    IResult,
};

use super::{comment::comment, non_comment_line::non_comment_line};

pub fn any_line(input: &str) -> IResult<&str, Option<&str>> {
    if let Ok((input, comment)) = comment(input) {
        Ok((input, Some(comment)))
    } else if let Ok((input, _)) = non_comment_line(input) {
        Ok((input, None))
    } else {
        Err(nom::Err::Error(Error::new(input, ErrorKind::Tag)))
    }
}
