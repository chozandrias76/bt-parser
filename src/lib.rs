pub mod parsing;

use nom::
    IResult
;
use parsing::any_line::any_line;


pub fn parse_bt(input: &str) -> IResult<&str, Vec<String>> {
    let mut comments = Vec::new();
    let mut current_group = Vec::new();
    let mut remaining_input = input;
    let mut last_input_length = remaining_input.len();

    while let Ok((input, line)) = any_line(remaining_input) {
        if let Some(comment) = line {
            current_group.push(comment.to_string());
        } else {
            if !current_group.is_empty() {
                comments.push(current_group.join("\n"));
                current_group = Vec::new();
            }
        }
        if input.len() == last_input_length {
            break; // Exit loop if no progress is made
        }
        remaining_input = input;
        last_input_length = remaining_input.len();
    }

    if !current_group.is_empty() {
        comments.push(current_group.join("\n"));
    }

    Ok((remaining_input, comments))
}

#[cfg(test)]
mod tests {
    use parsing::comment::comment;

    use super::*;

    #[test]
    fn test_parse_comment() {
        assert_eq!(
            comment("// This is a comment\n"),
            Ok(("", " This is a comment"))
        );
        assert_eq!(
            comment("//Another comment"),
            Ok(("", "Another comment"))
        );
    }

    #[test]
    fn test_parse_bt() {
        let input = include_str!("D:/Elden Ring Tools/EldenRingSaveTemplate-master/SL2.bt");

        let expected_comments = vec![
            "------------------------------------------------\n--- 010 Editor v14.0 Binary Template\n\n      File: SL2\n   Authors: ClayAmore, Xenos\n   Version: \n   Purpose: Reading an Elden Ring Save File\n  Category: \n File Mask: \n  ID Bytes: \n   History: \n------------------------------------------------".to_string(),
            " Items \n Global CS::GaItem starts at (143ce0680 + 0x8)\n Length 0x1400\n Used for lookup of item ids from ga_item_handle maybe\n Ashes of War are first".to_string(),
            " Player Game Data\n CS::PlayerGameData+0x8 (7FF49FC3A6D0+0x8)\n Length 0x1B0".to_string(),
        ];
        let actual_comments = parse_bt(input).unwrap().1;
        assert_eq!(actual_comments[0..1], expected_comments[0..1]);
        assert_eq!(actual_comments[1..2], expected_comments[1..2]);
        assert_eq!(actual_comments[2..3], expected_comments[2..3]);
    }
}
