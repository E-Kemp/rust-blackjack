mod card;
mod dealer;
mod deck;
mod hand;
use colored::Colorize;
use std::cmp::Ordering;
use std::io;
use std::process::ExitCode;

fn main() -> ExitCode {
    let mut deck = deck::Deck::new_deck();

    println!("{:24}{}", "Deck length is:", deck.get_deck().len());

    deck.shuffle_self();

    let mut hand = deck.deal_hand();

    let mut dealer = dealer::Dealer::new(deck.deal_hand());

    // Blackjack check
    if hand.get_status() == Ordering::Equal {
        return result(&hand, &dealer);
    }

    loop {
        println!("{:24}{}", "The dealer has:", dealer.to_string());
        println!("{:24}{}", "Your cards are:", hand.to_string());
        println!("{:24}{}", "Value of your hand is:", hand.get_hand_val_str());

        match hand.get_status() {
            Ordering::Less => {
                println!("Would you like to [H]it or [s]tand? H/s");
                let mut hit_stand = String::new();

                io::stdin()
                    .read_line(&mut hit_stand)
                    .expect("Failed to read line");

                match hit_stand {
                    _ if hit_stand.trim() == "s" => break,
                    _ => hand.add_card(deck.deal_card()),
                }
            }
            Ordering::Equal => {
                println!("You have 21!");
                break;
            }
            Ordering::Greater => {
                println!("Bust!");
                return ExitCode::FAILURE;
            }
        }
    }

    println!(
        "{:24}{}\n\n{}",
        "Value of your hand is:",
        hand.get_hand_val_str(),
        "Dealer's turn!"
    );

    loop {
        println!("{:24}{}", "Dealer's cards are:", dealer.hand.to_string());
        println!(
            "{:24}{}",
            "Value of the dealer:",
            dealer.hand.get_hand_val_str()
        );
        if dealer.should_hit() {
            println!("\n{}", "Dealer hits!");
            dealer.hand.add_card(deck.deal_card());
        } else {
            match dealer.hand.get_status() {
                Ordering::Less | Ordering::Equal => return result(&hand, &dealer),
                Ordering::Greater => {
                    println!("Dealer Bust, you win!");
                    return ExitCode::SUCCESS;
                }
            }
        }
    }
}

fn result(hand: &hand::Hand, dealer: &dealer::Dealer) -> ExitCode {
    let (result, dealer_res, player_res, result_msg) =
        match dealer.hand.get_hand_val().cmp(&hand.get_hand_val()) {
            Ordering::Less => (
                Ordering::Less,
                format!("{}", dealer.hand.get_hand_val().to_string().red()),
                format!("{}", hand.get_hand_val().to_string().green()),
                "You win!".green(),
            ),
            Ordering::Equal => (
                Ordering::Equal,
                format!("{}", dealer.hand.get_hand_val().to_string().yellow()),
                format!("{}", hand.get_hand_val().to_string().yellow()),
                "Push!".yellow(),
            ),
            Ordering::Greater => (
                Ordering::Greater,
                format!("{}", dealer.hand.get_hand_val().to_string().green()),
                format!("{}", hand.get_hand_val().to_string().red()),
                "You lose!".red(),
            ),
        };
    println!(
        "{}\n{:12}{:12}: {}\n{:12}{:12}: {}\n{}",
        "Results:",
        "Dealer:",
        dealer_res,
        dealer.hand.to_string(),
        "You:",
        player_res,
        hand.to_string(),
        result_msg
    );
    match result {
        Ordering::Greater | Ordering::Equal => return ExitCode::SUCCESS,
        Ordering::Less => return ExitCode::FAILURE,
    }
}

