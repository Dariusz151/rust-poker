use crate::utils::card::Card;

pub const STARTING_MONEY: u32 = 15000;

#[derive(Debug, Copy, Clone)]
pub enum Decision {
    Bet(Result<u32, &'static str>),
    Pass,
    Check,
}

#[derive(Debug, Clone)]
pub struct Player {
    pub name: String,
    pub hand: Option<(Card, Card)>,
    pub money: u32,
    pub actual_decision: Decision,
}

impl Player {
    pub fn new(name: String) -> Player {
        println!("[Player] {} joined the game.", name);

        Player {
            name,
            hand: None,
            money: STARTING_MONEY,
            actual_decision: Decision::Check,
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
            self.actual_decision = Decision::Bet(Ok(cash_amount));
        } else {
            self.actual_decision = Decision::Bet(Err("Can't bet more money than you have!"));
        }
    }

    pub fn check(&mut self) {
        self.actual_decision = Decision::Check;
    }

    pub fn pass(&mut self) {
        self.actual_decision = Decision::Pass;
    }

    pub fn check_decision(&self) -> Decision {
        let decision: Decision = self.actual_decision;

        decision
    }
}