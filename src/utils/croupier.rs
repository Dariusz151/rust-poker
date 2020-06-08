use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::utils::deck::Deck;
use crate::utils::player::Player;
use crate::utils::table::Table;

#[derive(Debug)]
pub struct Croupier {
}

impl Croupier {
    pub fn new_deck() -> Deck {
        Deck::new()
    }

    pub fn shuffle_cards(deck: &mut Deck) {
        println!("Cards have been shuffled.");
        deck.cards.shuffle(&mut thread_rng());
    }

    pub fn deal_cards(deck: &mut Deck, players: &mut Vec<Player>, table: &mut Table) {
        // Deal cards for players
        for p in players {
            if let None = p.hand {
                p.hand = Some((deck.take_card(), deck.take_card()));
                println!("Croupier dealt cards to {}.", p.name);
            } else {
                println!("Cards have already been dealt.");
            };
        }
        // Deal cards for table
        if table.is_clean() {
            table.flop = Some((deck.take_card(), deck.take_card(), deck.take_card()));
            table.turn = Some(deck.take_card());
            table.river = Some(deck.take_card());
        }
    }

    pub fn show_cards(table: &mut Table, round_number: &mut u32) {
        match round_number {
            0 => println!("Nothing to show."),
            1 => println!("Flop: {:?}", table.flop),
            2 => println!("Turn: {:?}", table.turn),
            3 => println!("River: {:?}", table.river),
            _ => println!("Round ended."),
        };
    }
}
