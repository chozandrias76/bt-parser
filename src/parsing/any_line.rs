use nom::{
    IResult,
    error::{Error, ErrorKind},
};

use super::{comment_line::comment_line, non_comment_line::non_comment_line, typedef_line::typedef_line};

pub fn any_line(input: &str) -> IResult<&str, (Option<String>, Option<String>)> {
    if let Ok((input, comment_str)) = comment_line(input) {
        let mut remaining_input = input;
        let mut comment_lines = vec![comment_str];
        while let Ok((input, next_comment)) = comment_line(remaining_input) {
            comment_lines.push(next_comment);
            remaining_input = input;
        }
        let full_comment = comment_lines.join("\n");
        Ok((remaining_input, (Some(full_comment), None)))
    } else if let Ok((input, typedef)) = typedef_line(input) {
        Ok((input, (None, Some(typedef.to_string()))))
    } else if let Ok((input, _)) = non_comment_line(input) {
        Ok((input, (None, None)))
    } else {
        Err(nom::Err::Error(Error::new(input, ErrorKind::Tag)))
    }
}
