use std::io;

use crate::utils::player::{Status, Player};
use crate::utils::round::Round;

#[derive(Debug)]
pub struct Game {
    players: Vec<Player>,
    round: Round,
}

impl Game {
    pub fn new() -> Game {
        Game { 
            players: Vec::new(),
            round: Round::new(),
        }
    }

    pub fn add_players(&mut self) {
        println!("How many players?");
        let mut input = String::new();
        io::stdin().read_line(&mut input)
                   .expect("Failed to read line");

        let players: u32 = input.trim().parse().unwrap();
        
        for i in 0..players {
            println!("Input {} player name: ", i + 1);
            let mut player_name = String::new();
            io::stdin().read_line(&mut player_name)
                       .expect("Failed to read line");
            self.players.push(Player::new(String::from(player_name.trim())));
        }
    }   

    pub fn run(&mut self) {
        let mut round_number: u32 = 0;
        'outer: loop {
            // Takes players still in game
            let mut players_ptr: Vec<&mut Player> = self.players
                    .iter_mut()
                    .filter(|player| player.status != Status::Lost)
                    .collect();
            self.round.new_round();
            self.round.shuffle_cards();
            self.round.deal_cards(&mut players_ptr);

            let mut turn_index = 0;

            'inner: loop {
                let playern_index = turn_index % players_ptr.len();
                if players_ptr[playern_index].status != Status::Pass {
                    players_ptr[playern_index].make_decision(&mut self.round.table.to_call);
                }
            }

            self.round.end_round();
        }

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
                //     "show-cards" => {
                //         players[turn_index].show_cards();
                //     }
                //     "bet" => {
                //         players[turn_index].bet(100);
                //     }
                //     "pass" => {
                //         players[turn_index].pass();
                //     }
                //     "check" => {
                //         players[turn_index].check();
                //     }
                //     "show-decision" => {
                //         let decision: Status = players[turn_index].check_status();
                //         println!("Your decision: {:?}", decision);
                //     }
                //     "end-turn" => {
                //         let players_count: usize = players.len();
                //         turn_index += 1;
                //         if turn_index > players_count - 1 {
                //             round_number += 1;
                //             Croupier::show_cards(&mut round.table, &mut round_number);
                //             if round_number > 3 {
                //                 break 'outer;
                //             }
                //             break 'inner;
                //         }
                //     }
                //     "exit" => break 'outer,
                //     "end-round" => break 'inner,
                //     _ => println!("Default "),
                // }
        //     }
        // }

    }    
}