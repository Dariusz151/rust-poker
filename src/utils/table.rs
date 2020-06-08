use crate::utils::card::Card;

#[derive(Debug)]
pub struct Table {
    pub flop: Option<(Card, Card, Card)>,
    pub turn: Option<Card>,
    pub river: Option<Card>,
    pub pot: u32,
}

impl Table {
    pub fn new() -> Table {
        Table {
            flop: None,
            turn: None,
            river: None,
            pot: 0,
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
    }

    pub fn set_pot(&mut self, cash_to_pot: u32) {
        self.pot += cash_to_pot;
    }
}
