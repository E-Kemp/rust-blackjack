use crate::hand::Hand;
use colored::Colorize;
use std::fmt;

#[derive(Clone)]
pub struct Dealer {
   pub hand: Hand,
}

impl Dealer {
    pub fn new(hand: Hand) -> Self {
        return Dealer { hand };
    }

    pub fn should_hit(&self) -> bool {
        return self.hand.get_hand_val() < 17
    }
}

impl fmt::Display for Dealer {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{} ", self.hand.get_cards()[0].to_string()).unwrap();
        write!(fmt, "{}", "[--]".black().on_white()).unwrap();
        Ok(())
    }
}

