mod card;
mod hand;
mod deck;
mod dealer;
use std::io;
use std::cmp::Ordering;

fn main() {
    let mut deck = deck::Deck::new_deck();

    println!("{:24}{}", "Deck length is:", deck.get_deck().len());

    deck.shuffle_self();

    // let mut hand = deck.deal_hand();
    let mut hand = hand::Hand {
        cards: [card::Card {
            value: card::Value::TWO,
            suit: card::Suit::CLUB,
        },
        card::Card {
            value: card::Value::ACE,
            suit: card::Suit::CLUB,
        },
        card::Card {
            value: card::Value::ACE,
            suit: card::Suit::CLUB,
        }].to_vec()
    };
    let deal_hand = dealer::Dealer::new(deck.deal_hand());

    loop {
        println!("{:24}{}", "The dealer has:", deal_hand.to_string());

        println!("{:24}{}", "Your cards are:", hand.to_string());
        println!("{:24}{}", "Value of the hand is:", hand.get_hand_val_str());

        match hand.get_status() {
            Ordering::Less => {
                println!("Would you like to hit or stand? h/S");
                let mut yes_no = String::new();

                io::stdin()
                    .read_line(&mut yes_no)
                    .expect("Failed to read line");

                match yes_no {
                    _ if yes_no.trim() == "N" => break,
                    _ => hand.add_card(deck.deal_card())
                }
            },
            Ordering::Equal => {
                println!("You have 21!");
                break;
            }
            Ordering::Greater => {
                println!("Bust!");
                break;
            }
        }
    }


}