use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::utils::table::Table;
use crate::utils::deck::Deck;
use crate::utils::player::Player;

#[derive(Debug)]
pub struct Round {
    pub table: Table,
    pub deck: Deck,

}

impl Round {
    pub fn new() -> Round {
        Round {
            table: Table::new(),
            deck: Deck::new(),
        }
    }

    pub fn new_round(&mut self) {
        self.table = Table::new();
        self.deck = Deck::new();
    }

    pub fn end_round(&mut self) {
        self.table.clean();
        self.deck.clean();
    }

    pub fn shuffle_cards(&mut self) {
        self.deck.cards.shuffle(&mut thread_rng());
        println!("Cards have been shuffled.");
    }

    pub fn deal_cards(&mut self, players: &mut Vec<&mut Player>) {
        // Deal cards for players
        for p in players {
            if let None = p.hand {
                p.hand = Some((self.deck.take_card(), self.deck.take_card()));
                println!("Croupier dealt cards to {}", p.name);
            } else {
                println!("Cards have already been dealt.");
            };
        }
        // Deal cards for table
        if self.table.is_clean() {
            self.table.flop = Some((self.deck.take_card(), self.deck.take_card(), self.deck.take_card()));
            self.table.turn = Some(self.deck.take_card());
            self.table.river = Some(self.deck.take_card());
        }
    }
}