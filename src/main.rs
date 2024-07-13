use std::fs::read_to_string;

use bt_parser::parsing::comment_line::comment_line;

pub fn main() {
    let path = "D:/Elden Ring Tools/EldenRingSaveTemplate-master/SL2.bt";
    let input = read_to_string(path).unwrap();
    let (input, _) = comment_line(input.as_str()).unwrap();
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

    println!("input: {:?}", input);
}
