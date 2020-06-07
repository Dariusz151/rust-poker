use croupier::{Player, Croupier, Deck, Table};

fn main() {
    println!("Hello in Poker Rust!");
    
    let mut deck: Deck = Croupier::new_deck();
    let mut table: Table = Croupier::new_table();
    Croupier::shuffle_cards(&mut deck);
    let mut players: Vec<Player> = vec![Player::new(String::from("Mateusz"))
                                       ,Player::new(String::from("Dariusz"))];

    Croupier::deal_cards(&mut deck, &mut players, &mut table);
}