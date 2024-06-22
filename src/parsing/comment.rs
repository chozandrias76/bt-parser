use nom::{
    IResult,
    bytes::complete::tag,
    character::complete::{not_line_ending, line_ending},
    combinator::opt,
};

pub fn comment(input: &str) -> IResult<&str, &str> {
    let (input, _) = tag("//")(input)?;
    let (input, comment) = not_line_ending(input)?;
    let (input, _) = opt(line_ending)(input)?;
    Ok((input, comment))
}
