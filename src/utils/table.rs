use crate::utils::card::Card;
use crate::utils::player::Player;

#[derive(Debug)]
pub struct Table {
    pub flop: Option<(Card, Card, Card)>,
    pub turn: Option<Card>,
    pub river: Option<Card>,
    pub pot: u32,
    pub to_call: u32,
}

impl Table {
    pub fn new() -> Table {
        Table {
            flop: None,
            turn: None,
            river: None,
            pot: 0,
            to_call: 0,
        }
    }

    pub fn is_clean(&self) -> bool {
        self.flop == None && self.flop == None && self.flop == None && self.pot == 0
    }

    pub fn clean(&mut self) {
        self.flop = None;
        self.turn = None;
        self.river = None;
        self.pot = 0;
        self.to_call = 0;
    }

    pub fn set_pot(&mut self, cash_to_pot: u32) {
        self.pot += cash_to_pot;
    }

    pub fn compare_cards<'a>(&mut self, players: &'a mut Vec<&'a mut Player>) -> &'a mut Player {
        players.pop().unwrap() // only for return purpose
    }

    pub fn collect_reward(&mut self, player: &mut Player) {
        player.money = self.pot;
    }
}
