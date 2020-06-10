mod utils;

use crate::utils::game::Game;
use crate::utils::round::Round;
use crate::utils::player::Player;
use crate::utils::croupier::Croupier;
use crate::utils::deck::Deck;
use crate::utils::player::Status;

use std::io;

fn main() {
    println!("Hello in Poker Rust! \nWould you like to start? Y / N");

    let mut answer = String::new();
    io::stdin().read_line(&mut answer)
               .expect("Failed to read line");
    match answer.trim() {
        "Y" => (),
        "N" => { 
            println!("Thank you for your time! :)");
            std::process::exit(0);
        },
        _ => panic!("Wrong input!"),
    }
        
    let mut game: Game = Game::new();
    game.add_players();
    println!("All players are ready! Let's start the game!");
    game.run();
    println!("Thank you for your time! :)");





    // let mut round: Round = Round::new();
    // let mut players: Vec<Player> = vec![
    //     Player::new(String::from("Mateusz")),
    //     Player::new(String::from("Dariusz")),
    //];

    //round.add_players(players.clone());

    // let mut deck: Deck = Croupier::new_deck();
    // Croupier::shuffle_cards(&mut deck);
    // Croupier::deal_cards(&mut deck, &mut players, &mut round.table);
    // let mut round_number: u32 = 0;

    // 'outer: loop {
    //     //round.start_round(&mut round_number);
    //     let mut turn_index = 0;

    //     'inner: loop {
    //         let mut guess = String::new();
    //         println!("Input a decision player: {}", players[turn_index].name);
    //         io::stdin()
    //             .read_line(&mut guess)
    //             .expect("Failed to read line");

    //         match guess.trim() {
    //             "show-cards" => {
    //                 players[turn_index].show_cards();
    //             }
    //             "bet" => {
    //                 players[turn_index].bet(100);
    //             }
    //             "pass" => {
    //                 players[turn_index].pass();
    //             }
    //             "check" => {
    //                 players[turn_index].check();
    //             }
    //             "show-decision" => {
    //                 let decision: Status = players[turn_index].check_status();
    //                 println!("Your decision: {:?}", decision);
    //             }
    //             "end-turn" => {
    //                 let players_count: usize = players.len();
    //                 turn_index += 1;
    //                 if turn_index > players_count - 1 {
    //                     round_number += 1;
    //                     Croupier::show_cards(&mut round.table, &mut round_number);
    //                     if round_number > 3 {
    //                         break 'outer;
    //                     }
    //                     break 'inner;
    //                 }
    //             }
    //             "exit" => break 'outer,
    //             "end-round" => break 'inner,
    //             _ => println!("Default "),
    //         }
    //     }
    // }
}
