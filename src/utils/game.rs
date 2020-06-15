use crate::utils::player::{Status, Player};
use crate::utils::round::Round;
use crate::utils::util::*;

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

        let players: u32 = read_line().trim().parse().unwrap();
        
        for i in 0..players {
            println!("Input {} player name: ", i + 1);
            self.players.push(Player::new(String::from(read_line().trim())));
        }
    }   
    
    pub fn announce_winner(winner: &Player) {
        println!("GAME WON BY {}\nCONGRATULATION!", winner.name);
    } 

    pub fn run(&mut self) {
        'game: loop {
            // Takes players still in game
            let mut players_ptr: Vec<&mut Player> = self.players
                                                        .iter_mut()
                                                        .filter(|player| player.status != Status::Lost)
                                                        .collect();
            if players_ptr.len() == 1 {
                Game::announce_winner(players_ptr[0]);
                break 'game;
            }
            self.round.new_round(&mut players_ptr);
            self.round.shuffle_cards();
            self.round.deal_cards(&mut players_ptr);
            self.round.run(&mut players_ptr);
            self.round.check_round_winner(&mut players_ptr);
            self.round.end_round();
        }// game
    }// run    
}