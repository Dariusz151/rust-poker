use enum_iterator::IntoEnumIterator;
use rand::seq::SliceRandom;
use rand::thread_rng;

#[allow(dead_code)]
const CARDS_AMOUNT: usize = 52;
const STARTING_MONEY: i32 = 15000;

#[derive(Debug, PartialEq, Copy, Clone, IntoEnumIterator)]
enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades
}

#[derive(Debug, PartialEq, Copy, Clone, IntoEnumIterator)]
enum Figure {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Card {
    suit: Suit,
    figure: Figure
}

#[derive(Debug, PartialEq, Clone)]
pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Deck {
        let mut cards: Vec<Card> = Vec::new();
        for suit in Suit::into_enum_iter() {
            for figure in Figure::into_enum_iter() {
                let card = Card {suit: *(&suit), figure: *(&figure)};
                cards.push(card);
            }
        }

        Deck { cards }
    }

    fn take_card(&mut self) -> Card {
        if let Some(c) = self.cards.pop() {
            c
        }
        else{
            panic!("Could not take card from deck!");
        }
    }
}

#[derive(Debug)]
pub struct Table {
    flop: Option<(Card, Card, Card)>,
    turn: Option <Card>,
    river: Option <Card>,
    pot: u32,
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

    pub fn clean_table(&mut self) {
        self.flop = None;
        self.turn = None;
        self.river = None;
        self.pot = 0;
    }
}

#[derive(Debug)]
pub struct Croupier {
    name: String,
}

impl Croupier {
    pub fn new(name: String) -> Croupier {
        println!("{} [Croupier] entered the poker.", name);
        
        Croupier { name }
    }

    pub fn shuffle_cards(deck: &mut Deck) {
        println!("Cards have been shuffled.");
        deck.cards.shuffle(&mut thread_rng());
    }

    pub fn deal_cards(players: &mut Vec<Player>, deck: &mut Deck){
        for p in players{
            if let None = p.hand {
                p.hand = Some((deck.take_card(), deck.take_card()));
            }
            else{
                println!("Cards have been dealt.");
            } ;
        };
    }
}

#[derive(Debug)]
pub struct Player {
    name: String,
    hand: Option<(Card,Card)>,
    money: i32,
}

impl Player {
    pub fn new(name: String) -> Player {
        println!("[Player] {} joined the game.", name);

        Player {
            name,
            hand: None,
            money: STARTING_MONEY
        }
    }

    pub fn show_cards(&self){
        if let Some((first, second)) = &self.hand {
            println!("First card: {:?} Second card: {:?}", first, second);
        }
        else{
            panic!("Could not show the cards");
        }
    }
}


/*
** TESTS
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_new_deck() {
        // arrange
        // act
        let deck = Deck::new();
        // assert
        assert_eq!(deck.cards.len(), CARDS_AMOUNT);
    }

    #[test]
    fn shuffling_cards() {
        // arrange
        let deck_original: Deck = Deck::new();
        let mut deck_cloned: Deck = deck_original.clone();
        // act
        Croupier::shuffle_cards(&mut deck_cloned);
        // assert
        assert_ne!(deck_original, deck_cloned);
    }

    
    #[test]
    fn take_card() {
        // arrange
        let mut deck = Deck::new();
        let deck_len: usize = deck.cards.len();
        // act
        let _last_card: Card = deck.take_card();
        // assert
        assert_eq!(deck_len - 1, deck.cards.len());
    }
}