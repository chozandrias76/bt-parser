pub mod parsing;
#[cfg(test)]
mod tests {
    use parsing::comment_line::comment_line;
    use super::*;

    fn print_comments(comments: &Vec<String>) {
        for comment in comments {
            println!("{}", comment);
        }
    }

    fn assert_result_range_equal(
        actual: &Vec<String>,
        expected: &Vec<String>,
        range: std::ops::Range<usize>,
    ) {
        assert_eq!(
            actual[range.clone()],
            expected[range.clone()],
            "actual: {:?}",
            format!("{:?}", &actual[range.clone()])
        );
    }

    #[test]
    fn test_parse_comment_line() {
        assert_eq!(
            comment_line("// This is a comment_line\n"),
            Ok(("", " This is a comment_line"))
        );
        assert_eq!(comment_line("//Another comment_line"), Ok(("", "Another comment_line")));
    }

    #[test]
    fn test_parse_bt() {
        let input = include_str!("D:/Elden Ring Tools/EldenRingSaveTemplate-master/SL2.bt");

        let expected_comments = vec![
            r#"------------------------------------------------
--- 010 Editor v14.0 Binary Template

      File: SL2
   Authors: ClayAmore, Xenos
   Version: 
   Purpose: Reading an Elden Ring Save File
  Category: 
 File Mask: 
  ID Bytes: 
   History: 
------------------------------------------------"#.to_string(),
            r#" Items 
 Global CS::GaItem starts at (143ce0680 + 0x8)
 Length 0x1400
 Used for lookup of item ids from ga_item_handle maybe
 Ashes of War are first"#.to_string(),
            r#" Player Game Data
 CS::PlayerGameData+0x8 (7FF49FC3A6D0+0x8)
 Length 0x1B0"#.to_string(),
        ];

        let expected_typedefs = vec![
            r#"typedef struct Item {
    int32 ga_item_handle<format=hex>;
    int32 ItemID<format=hex>;
    // Additional data for type = Weapon
    if ( (ItemID != 0) && ((ItemID & 0xf0000000) == 0)) { 
        int32 unk; 
        int32 unk2;
        int32 ash_of_war_ga_item_handle<format=hex>;
        byte unk4;
    }
    // Additional data for type = Armor
    else if((ItemID != 0) && ((ItemID & 0xf0000000) == 0x10000000)) { 
        int32 unk;
        int32 unk2;
    }
};"#.to_string(),
            r#"typedef struct {
    uint32 unk;
    uint32 unk1;
    uint32 Health;
    uint32 MaxHealth;
    uint32 BaseMaxHealth;
    uint32 FP;
    uint32 MaxFP;
    uint32 BaseMaxFP;
    uint32 unk2;
    uint32 SP;
    uint32 MaxSP;
    uint32 BaseMaxSP;
    uint32 unk3;
    uint32 Vigor;
    uint32 Mind;
    uint32 Endurance;
    uint32 Strength;
    uint32 Dexterity;
    uint32 Intelligence;
    uint32 Faith;
    uint32 Arcane;
    uint32 unk4[0x3];
    uint32 Level;
    uint32 Souls;
    uint32 SoulMemory;
    byte unk5[0x28];
    wchar_t  CharacterName[0x10];
    byte unk6[0x2];
    byte Gender;
    byte ArcheType;
    byte unk7[0x3];
    byte Gift;
    byte unk8[0x54];
    wchar_t Password[0x9];
    wchar_t GroupPassword1[0x9];
    wchar_t GroupPassword2[0x9];
    wchar_t GroupPassword3[0x9];
    wchar_t GroupPassword4[0x9];
    wchar_t GroupPassword5[0x9];
} PlayerGameData <size=0x1B0>;"#.to_string(),
        ];

        let (remaining_input, (actual_comments, actual_typedefs)) = parsing::parse_bt(input).unwrap();

        assert_eq!(remaining_input, "");
        assert_result_range_equal(&actual_comments, &expected_comments, 0..1);
        assert_result_range_equal(&actual_comments, &expected_comments, 1..2);
        assert_result_range_equal(&actual_comments, &expected_comments, 2..3);

        assert_result_range_equal(&actual_typedefs, &expected_typedefs, 0..1);
        assert_result_range_equal(&actual_typedefs, &expected_typedefs, 1..2);

        // Print comments and typedefs for debugging
        print_comments(&actual_comments);
        print_comments(&actual_typedefs);
    }
}
