use croupier::{Player, Croupier, Deck, Table};

use std::io;

fn main() {
    println!("Hello in Poker Rust!");

    let mut deck: Deck = Croupier::new_deck();
    let mut table: Table = Croupier::new_table();
    let mut players: Vec<Player> = vec![Player::new(String::from("Mateusz"))
                                       ,Player::new(String::from("Dariusz"))];


    loop  {
        
        // Croupier::shuffle_cards(&mut deck);
        // Croupier::deal_cards(&mut deck, &mut players, &mut table);
        let mut guess = String::new();
        println!("Input a decision");
        
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        
        
        match guess.trim() {
            "bet"  => println!("Bet"),
            "pass"  => println!("Pass"),
            "check" => println!("Check"),
            "exit" => break,
            _ => println!("Default ")
        }
    }
}