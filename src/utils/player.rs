use std::io;

use crate::utils::card::Card;

pub const STARTING_MONEY: u32 = 15000;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Status {
    Bet(u32),
    Pass,
    Check,
    Lost,
}

#[derive(Debug, Clone)]
pub struct Player {
    pub name: String,
    pub hand: Option<(Card, Card)>,
    pub money: u32,
    pub status: Status,
}

impl Player {
    pub fn new(name: String) -> Player {
        println!("[Player] {} joined the game.", name);

        Player {
            name,
            hand: None,
            money: STARTING_MONEY,
            status: Status::Check,
        }
    }

    pub fn show_cards(&self) {
        if let Some((first, second)) = &self.hand {
            println!("First card: {:?} \nSecond card: {:?}", first, second);
        } else {
            panic!("Could not show the cards");
        }
    }

    pub fn bet(&mut self, cash_amount: u32) {
        println!("{} is beting {} dollars.", self.name, cash_amount);
        if cash_amount < self.money {
            self.money -= cash_amount;
            self.status = Status::Bet(cash_amount);
        } 
    }

    pub fn make_decision(&mut self, to_call: &mut u32) {
        println!("Player {} makes action", self.name);
        println!("Your cards are: {:?}", self.hand);
        
        let mut decision = String::new();

        'outer: loop {
            println!("What is your decision?\nBET\tCHECK\tPASS");
            io::stdin().read_line(&mut decision).expect("Failed to read line");
            match &*decision.trim().to_uppercase() {
                "BET" => {
                    'inner: loop {
                        println!("How much do you want to BET? ");
                        let mut bet_value = 0u32.to_string();
                        io::stdin().read_line(&mut bet_value).expect("Failed to read line");
                        let bet = bet_value.trim().parse::<u32>().unwrap();
                        if bet > self.money { 
                            println!("You BET more than you have!");
                        } else {
                            if bet != 0 { 
                                if bet <= *to_call{
                                    println!("BET must be higher than {}!", to_call);
                                    println!("You sure you want to BET? Y / N");
                                    let mut bet_decision = String::new();
                                    io::stdin().read_line(&mut bet_decision).expect("Failed to read line");
                                    if bet_decision.trim() == "N" { 
                                        continue 'outer 
                                    }
                                }else {
                                    self.status = Status::Bet(bet);
                                    *to_call = bet;
                                    break 'outer;
                                }  
                            }
                        }
                    }
                }
                "CHECK" => {
                    if let Status::Bet(x) = self.status{
                        if x == *to_call { 
                            break 'outer; 
                        } else {
                            println!("You can't CHECK");
                        }
                    } else {
                        println!("You can't CHECK");
                    }
                }
                "PASS" => {
                    self.status = Status::Pass;
                    break 'outer;
                }
                "CALL" => {
                    if self.money < *to_call {
                        println!("You can't CALL, you don't have that much money!");
                        continue 'outer;
                    }
                    self.status = Status::Bet(*to_call);
                    break 'outer;
                }
                _ => {
                    println!("Incorrect decision!");
                }
            }
        }
    }
}