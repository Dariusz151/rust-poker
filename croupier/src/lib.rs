use enum_iterator::IntoEnumIterator;
use rand::seq::SliceRandom;
use rand::thread_rng;

#[allow(dead_code)]
const CARDS_AMOUNT: usize = 52;
const STARTING_MONEY: u32 = 15000;

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
    money: u32,
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
        else
        {
            panic!("Could not show the cards");
        }
    }

    pub fn bet(&mut self, cash_amount: u32)
    {
        println!("{} is beting {} dollars.", self.name, cash_amount);
        if cash_amount < self.money
        {
            self.money -= cash_amount;
            // TODO: add this cash_amount to game pot.
        }
        else
        {
            panic!("Can't bet more money than you have.");
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

    #[test]
    fn bet_less_than_have_should_subtract_from_players_money() {
        let mut player: Player = Player::new(String::from("Mateusz"));   
        player.bet(10000);
        assert_eq!(player.money, 5000);
    }

    #[test]
    #[should_panic]
    fn bet_more_than_have_should_panic() {
        let mut player: Player = Player::new(String::from("Mateusz"));
        player.bet(20000);
    }
}