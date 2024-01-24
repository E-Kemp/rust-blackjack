use strum::IntoEnumIterator;
use std::fmt;
use crate::card::Card;
use crate::hand::Hand;
use rand::thread_rng;
use rand::seq::SliceRandom;

use crate::card;

pub struct Deck {
  deck: Vec<card::Card>
}

impl Deck {
  pub fn new_deck() -> Deck {
    let mut deck: Vec<card::Card> = Vec::new();
    for suit in card::Suit::iter() {
      for value in card::Value::iter() {
        deck.push(card::Card {
          suit,
          value
        })
      }
    }

    return Deck {
      deck,
    }
  }

  pub fn get_deck(&self) -> &Vec<card::Card> {
    &self.deck
  }

  pub fn deal_card(&mut self) -> Card {
    return match self.deck.pop() {
        Some(card) => card,
        None => panic!("Deck pop failed!")
    }
  }

  pub fn deal_hand(&mut self) -> Hand {
    let card1 = self.deal_card();
    let card2 = self.deal_card();
    return Hand::new_hand(card1, card2);
  }
  
  pub fn shuffle_self(&mut self) {
    self.deck.shuffle(&mut thread_rng());
  }
}

impl fmt::Display for Deck {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        for card in &self.deck {
            write!(fmt, "{}\n", card.to_string())
                .expect("Something went wrong");
        }
        Ok(())
    }
}
