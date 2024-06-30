use bt_parser::parsing::{comment_line::comment_line, typedef_line::typedef_line};

pub fn main() {
    let input = include_str!("D:/Elden Ring Tools/EldenRingSaveTemplate-master/SL2.bt");
    let (input, _) = comment_line(input).unwrap();
    let (input, _) = comment_line(input).unwrap();
    let (input, _) = comment_line(input).unwrap();
    let (input, _) = comment_line(input).unwrap();
    let (input, _) = comment_line(input).unwrap();
    let (input, _) = comment_line(input).unwrap();
    let (input, _) = comment_line(input).unwrap();
    let (input, _) = comment_line(input).unwrap();
    let (input, _) = comment_line(input).unwrap();
    let (input, _) = comment_line(input).unwrap();
    let (input, _) = comment_line(input).unwrap();
    let (input, _) = comment_line(input).unwrap();
    let (input, _) = comment_line(input).unwrap();
    let (input, _) = comment_line(input).unwrap();
    let (input, _) = comment_line(input).unwrap();
    let (input, _) = comment_line(input).unwrap();
    let (input, _) = comment_line(input).unwrap();
    let (_input, item_struct) = typedef_line(input).unwrap();

    println!("Item: {:?}", item_struct);
}
