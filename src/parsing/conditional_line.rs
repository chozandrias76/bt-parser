use nom::{
  IResult,
  bytes::complete::{tag, take_while1},
  character::complete::{char, multispace0},
  sequence::{delimited, preceded, tuple},
  combinator::recognize,
  multi::many0,
};

fn parse_if_keyword(input: &str) -> IResult<&str, &str> {
  tag("if")(input)
}

fn parse_else_if_keyword(input: &str) -> IResult<&str, &str> {
  tag("else if")(input)
}

fn parse_condition(input: &str) -> IResult<&str, &str> {
  delimited(char('('), take_while1(|c| c != ')'), char(')'))(input)
}

fn parse_braced_block(input: &str) -> IResult<&str, &str> {
  let mut level = 0;
  let mut i = 0;

  for (j, c) in input.chars().enumerate() {
      match c {
          '{' => {
              level += 1;
          }
          '}' => {
              level -= 1;
              if level == 0 {
                  i = j;
                  break;
              }
          }
          _ => {}
      }
  }

  if level == 0 {
      Ok((&input[i + 1..], &input[..=i]))
  } else {
      Err(nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Char)))
  }
}

fn parse_if_block(input: &str) -> IResult<&str, &str> {
  recognize(tuple((
      parse_if_keyword,
      multispace0,
      parse_condition,
      multispace0,
      parse_braced_block,
  )))(input)
}

fn parse_else_if_block(input: &str) -> IResult<&str, &str> {
  recognize(tuple((
      parse_else_if_keyword,
      multispace0,
      parse_condition,
      multispace0,
      parse_braced_block,
  )))(input)
}

fn parse_if_else_blocks(input: &str) -> IResult<&str, &str> {
  recognize(tuple((
      parse_if_block,
      many0(preceded(multispace0, parse_else_if_block)),
  )))(input)
}

pub fn conditional_line(input: &str) -> IResult<&str, &str> {
  parse_if_else_blocks(input)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_parse_conditional() {
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

      assert_eq!(conditional_line(input_if_else), Ok(("", input_if_else)));
  }
}
