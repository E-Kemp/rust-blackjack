use crate::card::Card;
use crate::card::Value;
use crate::hand;
use colored::Colorize;
use std::cmp::Ordering;
use std::fmt;

#[derive(Clone)]
pub struct Hand {
    pub cards: Vec<Card>,
}

impl Hand {
    pub fn get_cards(&self) -> &Vec<Card> {
        return &self.cards;
    }

    pub fn new_hand(card1: Card, card2: Card) -> hand::Hand {
        return Hand {
            cards: Vec::from([card1, card2]),
        };
    }

    pub fn add_card(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn get_all_hand_val(&self) -> Vec<u32> {
        let mut output: Vec<u32> = Vec::from([<u32>::into(0)]);

        for card in self.cards.iter() {
            output[0] += card.get_card_val();
        }

        let num_aces = self
            .cards
            .iter()
            .filter(|card| match card.value {
                Value::ACE => true,
                _ => false,
            })
            .count();

        let mut indicie = 10;
        for _ in 0..num_aces {
            output.push(output[0] + indicie);
            indicie += 10;
        }

        return output;
    }

    pub fn get_hand_val(&self) -> u32 {
        let vals = self.get_all_hand_val();
        for iter in vals.iter().rev() {
            if *iter <= 21 {
                return *iter;
            }
        }
        return vals[0];
    }

    pub fn get_hand_val_str(&self) -> String {
        let vals = self.get_all_hand_val();
        return vals
            .iter()
            .map(|val| {
                if *val > 21 {
                    return val.to_string().red().to_string();
                } else {
                    return val.to_string().green().to_string();
                }
            })
            .collect::<Vec<String>>()
            .join(", ");
    }

    pub fn get_status(&self) -> Ordering {
        let val = self.get_hand_val();
        return val.cmp(&21);
    }
}

impl fmt::Display for Hand {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        for card in &self.cards {
            write!(fmt, "{} ", card.to_string()).expect("Something went wrong");
        }
        Ok(())
    }
}
