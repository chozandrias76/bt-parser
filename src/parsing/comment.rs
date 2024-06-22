use nom::{
    bytes::complete::tag,
    character::complete::{line_ending, not_line_ending},
    combinator::opt,
    IResult,
};

pub fn comment(input: &str) -> IResult<&str, &str> {
    let (input, _) = tag("//")(input)?;
    let (input, comment) = not_line_ending(input)?;
    let (input, _) = opt(line_ending)(input)?;
    Ok((input, comment))
}
