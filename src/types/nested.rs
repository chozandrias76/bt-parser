use std::fmt::{self, Debug, Display, Formatter};

pub enum Nested {
  Text(String),
  List(Vec<Nested>),
}

impl Debug for Nested {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self {
      Nested::Text(text) => write!(f, "{}", text),
      Nested::List(nested) => write!(f, "{:?}", nested),
    }
  }
}

impl Display for Nested {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self {
      Nested::Text(text) => write!(f, "{}", text),
      Nested::List(nested) => {
        let nested_texts: Vec<String> = nested.iter().map(|n| n.to_string()).collect();
        write!(f, "{}", nested_texts.join(" "))
      }
    }
  }
}

impl PartialEq for Nested {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (Nested::Text(text1), Nested::Text(text2)) => text1 == text2,
      (Nested::List(nested1), Nested::List(nested2)) => nested1 == nested2,
      _ => false,
    }
  }
}

impl std::convert::Into<Nested> for &str {
  fn into(self) -> Nested {
    Nested::Text(self.to_string())
  }
}

impl std::convert::Into<Nested> for Vec<String> {
  fn into(self) -> Nested {
    let nested_texts: Vec<Nested> = self.iter().map(|s| Nested::Text(s.clone())).collect();
    Nested::List(nested_texts)
  }
}