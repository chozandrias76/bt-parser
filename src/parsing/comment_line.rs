use nom::{
    bytes::complete::{tag, take_while},
    character::complete::multispace0,
    error::context,
    sequence::tuple,
    IResult,
};

pub fn comment_line(input: &str) -> IResult<&str, (&str, &str)> {
    let mut parser = context(
        "comment line",
        tuple((
            tuple((multispace0, tag("//"))),
            take_while(|c| c != '\n'),
        )),
    );
    parser(input).map(|(next_input, result)| {
        let (comment, text) = result;
        let (_, tag) = comment;
        (next_input, (tag, text.trim()))
    })
}

#[cfg(test)]
mod comment_line_tests {
    use super::*;

    #[test]
    fn test_comment_line1() {
        let input = "// This is a comment\n";
        let result = comment_line(input);
        assert_eq!(result, Ok(("\n", ("//", "This is a comment"))));
    }

    #[test]
    fn test_comment_line2() {
        let input = "// This is a comment";
        let result = comment_line(input);
        assert_eq!(result, Ok(("", ("//", "This is a comment"))));
    }

    #[test]
    fn test_comment_line3() {
        let input = "//This is a comment\n typedef struct {\n";
        let result = comment_line(input);
        assert_eq!(result, Ok(("\n typedef struct {\n", ("//", "This is a comment"))));
    }
}
