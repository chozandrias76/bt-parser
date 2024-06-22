use bt_parser::parsing::parse_bt;


pub fn main() {
  let input = include_str!("D:/Elden Ring Tools/EldenRingSaveTemplate-master/SL2.bt");
  let results = parse_bt(input).unwrap().1;
  for comment in results.0 {
      println!("{}", comment);
  }
}