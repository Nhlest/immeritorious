pub struct Card {
  pub name: String
}

impl Card {
  pub fn new(name: &str) -> Self {
    Self {
      name: name.to_owned()
    }
  }
}

