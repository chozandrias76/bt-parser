use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::{char, multispace0},
    combinator::recognize,
    error::context,
    sequence::{preceded, tuple},
    IResult,
};

pub fn conditional_line(input: &str) -> IResult<&str, &str> {
    let mut parser = context(
        "conditional_line",
        recognize(tuple((
            alt((tag("if"), tag("else if"))),
            preceded(multispace0, char('(')),
            take_while1(|c: char| c != '{'),
        ))),
    );
    parser(input).map(|(input, s)| (input, s.trim_end()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse_conditional1() {
        let input_if_else = r#"if ( (ItemID != 0) && ((ItemID & 0xf0000000) == 0)) { 
          int32 unk; 
          int32 unk2;
          int32 ash_of_war_ga_item_handle<format=hex>;
          byte unk4;
      }
      else if((ItemID != 0) && ((ItemID & 0xf0000000) == 0x10000000)) { 
          int32 unk;
          int32 unk2;
      }"#;
        let expected_rest = r#"{ 
          int32 unk; 
          int32 unk2;
          int32 ash_of_war_ga_item_handle<format=hex>;
          byte unk4;
      }
      else if((ItemID != 0) && ((ItemID & 0xf0000000) == 0x10000000)) { 
          int32 unk;
          int32 unk2;
      }"#;

        assert_eq!(
            conditional_line(input_if_else),
            Ok((
                expected_rest,
                "if ( (ItemID != 0) && ((ItemID & 0xf0000000) == 0))"
            ))
        );
    }

    #[test]
    fn test_parse_conditional2() {
        let input_if_else = r#"else if((ItemID != 0) && ((ItemID & 0xf0000000) == 0x10000000)) {
          int32 unk;
          int32 unk2;
      }"#;
        let expected_rest = r#"{
          int32 unk;
          int32 unk2;
      }"#;

        assert_eq!(
            conditional_line(input_if_else),
            Ok((
                expected_rest,
                "else if((ItemID != 0) && ((ItemID & 0xf0000000) == 0x10000000))"
            ))
        );
    }
}
