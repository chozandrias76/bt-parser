use bt_parser::parse_bt;

pub fn main() {
  let input = include_str!("D:/Elden Ring Tools/EldenRingSaveTemplate-master/SL2.bt");
  let comments = parse_bt(input).unwrap().1;
  for comment in comments {
      println!("{}", comment);
  }
}