use croupier::{Player, Croupier, Deck};

fn main() {
    println!("Hello in Poker Rust!");
    let mut deck: Deck = Deck::new();
    Croupier::shuffle_cards(&mut deck);
    let mut players: Vec<Player> = vec![Player::new(String::from("Mateusz"))
                                       ,Player::new(String::from("Dariusz"))];

    Croupier::deal_cards(&mut players, &mut deck);
}