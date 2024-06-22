pub mod any_line;
pub mod comment_line;
pub mod non_comment_line;
pub mod typedef_line;
pub mod conditional_line;

use nom::IResult;

pub fn parse_bt(input: &str) -> IResult<&str, (Vec<String>, Vec<String>)> {
    let mut comments = Vec::new();
    let mut typedefs = Vec::new();
    let mut remaining_input = input;
    let mut last_input_length = remaining_input.len();

    while let Ok((input, (comment, typedef))) = any_line::any_line(remaining_input) {
        if let Some(comment) = comment {
            comments.push(comment);
        }
        if let Some(typedef) = typedef {
            typedefs.push(typedef);
        }
        if input.len() == last_input_length {
            break; // Exit loop if no progress is made
        }
        remaining_input = input;
        last_input_length = remaining_input.len();
    }

    Ok((remaining_input, (comments, typedefs)))
}
