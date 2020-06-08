use croupier::{Croupier, Decision, Deck, Player, Round};

use std::io;

fn main() {
    println!("Hello in Poker Rust!");

    let mut round: Round = Round::new();
    let mut players: Vec<Player> = vec![
        Player::new(String::from("Mateusz")),
        Player::new(String::from("Dariusz")),
    ];

    round.add_players(players.clone());

    let mut deck: Deck = Croupier::new_deck();
    Croupier::shuffle_cards(&mut deck);
    Croupier::deal_cards(&mut deck, &mut players, &mut round.table);
    let mut round_number: u32 = 0;

    'outer: loop {
        round.start_round(&mut round_number);
        let mut turn_index = 0;

        'inner: loop {
            let mut guess = String::new();
            println!("Input a decision player: {}", players[turn_index].name);
            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");

            match guess.trim() {
                "show-cards" => {
                    players[turn_index].show_cards();
                }
                "bet" => {
                    players[turn_index].bet(100);
                }
                "pass" => {
                    players[turn_index].pass();
                }
                "check" => {
                    players[turn_index].check();
                }
                "show-decision" => {
                    let decision: Decision = players[turn_index].check_decision();
                    println!("Your decision: {:?}", decision);
                }
                "end-turn" => {
                    let players_count: usize = players.len();
                    turn_index += 1;
                    if turn_index > players_count - 1 {
                        round_number += 1;
                        Croupier::show_cards(&mut round.table, &mut round_number);
                        if round_number > 3 {
                            break 'outer;
                        }
                        break 'inner;
                    }
                }
                "exit" => break 'outer,
                "end-round" => break 'inner,
                _ => println!("Default "),
            }
        }
    }
}
