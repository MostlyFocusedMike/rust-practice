pub struct GetNumber {
  value: u8, // remember, properties are private by default
}

impl GetNumber {
  pub fn new(value: i32) -> GetNumber {
      if value < 1 || value > 100 {
          panic!("value must be between 1 and 100, got {}.", value);
      };
      GetNumber { value: value as u8}
  }

  pub fn value(&self) -> u8 {
      self.value
  }
}