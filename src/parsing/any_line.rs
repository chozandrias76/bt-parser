use nom::{
    IResult,
    error::{Error, ErrorKind},
};

use super::{comment::comment, non_comment_line::non_comment_line, typedef_line::typedef_line};

pub fn any_line(input: &str) -> IResult<&str, (Option<String>, Option<String>)> {
    if let Ok((input, comment_str)) = comment(input) {
        let mut remaining_input = input;
        let mut comments = vec![comment_str];
        while let Ok((input, next_comment)) = comment(remaining_input) {
            comments.push(next_comment);
            remaining_input = input;
        }
        let full_comment = comments.join("\n");
        Ok((remaining_input, (Some(full_comment), None)))
    } else if let Ok((input, typedef)) = typedef_line(input) {
        Ok((input, (None, Some(typedef.to_string()))))
    } else if let Ok((input, _)) = non_comment_line(input) {
        Ok((input, (None, None)))
    } else {
        Err(nom::Err::Error(Error::new(input, ErrorKind::Tag)))
    }
}
