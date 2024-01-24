use crate::card::Value;
use crate::card::Card;
use crate::hand;
use std::fmt;
use std::cmp::Ordering;

#[derive(Clone)]
pub struct Hand {
  pub cards: Vec<Card>,
}

impl Hand {
  pub fn get_cards(&self) -> &Vec<Card> {
    return &self.cards
  }

  pub fn new_hand(card1: Card, card2: Card) -> hand::Hand {
    return Hand {
      cards: Vec::from([card1, card2]),
    }
  }

  pub fn add_card(&mut self, card: Card) {
    self.cards.push(card);
  }


  pub fn get_hand_val(&self) -> Vec<u32> {
    let mut output: Vec<u32> = Vec::from([<u32>::into(0)]);

    for card in self.cards.iter() {
      for i in 0..output.len() {
        println!("{}", card.get_card_val());
        output[i] += card.get_card_val();
      }
    }
    
    let num_aces = self.cards.iter().filter(|card| match card.value {
      Value::ACE => true,
      _ => false,
    }).count();

    // match num_aces {
    //   num_aces if num_aces > 0 => {
        let mut indicie = 10;
        let mut output_len = output.len();
        for _ in 0..num_aces {
          for val in 0..output_len {
            output.push(output[val] + indicie);
          }
          indicie += 10;
        }
    //   },
    //   _ => {}
    // }
    
    return output;
  }

  pub fn get_hand_val_str(&self) -> String {
    let val = self.get_hand_val();
    return val.into_iter().map(|i| i.to_string()).collect::<Vec<String>>().join(", ");
  }

  pub fn get_status(&self) -> Ordering {
    let vals = self.get_hand_val();
    return match vals.iter().max() {
      Some(val) => val.cmp(&21),
      None => Ordering::Greater
    }
  }
}

impl fmt::Display for Hand {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        for card in &self.cards {
            write!(fmt, "{} ", card.to_string())
                .expect("Something went wrong");
        }
        Ok(())
    }
}
