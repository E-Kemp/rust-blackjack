use strum_macros::EnumIter;
use std::fmt;

#[derive(Clone)]
pub struct Card {
  pub suit: Suit,
  pub value: Value,
}

impl Card {
  pub fn get_card_val(&self) -> u32 {
    return match self.value {
      Value::KING | Value::QUEEN | Value::JACK | Value::TEN => 10,
      Value::NINE => 9,
      Value::EIGHT => 8,
      Value::SEVEN => 7,
      Value::SIX => 6,
      Value::FIVE => 5,
      Value::FOUR => 4,
      Value::THREE => 3,
      Value::TWO => 2,
      Value::ACE => 1
    }
  }
}

impl fmt::Display for Card {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "[{}{}]", self.value.to_string(), self.suit.to_string())
            .expect("Something went wrong");
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, EnumIter)]
pub enum Suit {
  CLUB,
  DIAMOND,
  HEART,
  SPADE
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Suit::CLUB => write!(f, "♣"),
            Suit::DIAMOND => write!(f, "♦"),
            Suit::HEART => write!(f, "♥"),
            Suit::SPADE => write!(f, "♠")
        }
    }
}

#[derive(Clone, Copy, Debug, EnumIter, PartialEq)]
pub enum Value {
  ACE,
  KING,
  QUEEN,
  JACK,
  TEN,
  NINE,
  EIGHT,
  SEVEN,
  SIX,
  FIVE,
  FOUR,
  THREE,
  TWO
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::ACE => write!(f, "A"),
            Value::KING => write!(f, "K"),
            Value::QUEEN => write!(f, "Q"),
            Value::JACK => write!(f, "A"),
            Value::TEN => write!(f, "T"),
            Value::NINE => write!(f, "9"),
            Value::EIGHT => write!(f, "8"),
            Value::SEVEN => write!(f, "7"),
            Value::SIX => write!(f, "6"),
            Value::FIVE => write!(f, "5"),
            Value::FOUR => write!(f, "4"),
            Value::THREE => write!(f, "3"),
            Value::TWO => write!(f, "2")
        }
    }
}