use crate::hand::Hand;
use std::fmt;

pub struct Dealer {
  hand: Hand,
}

impl Dealer {
  pub fn new(hand: Hand) -> Self {
    return Dealer {
      hand
    }
  }
}

impl fmt::Display for Dealer {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{} ", self.hand.get_cards()[0].to_string()).unwrap();
        write!(fmt, "[██]").unwrap();
        Ok(())
    }
}