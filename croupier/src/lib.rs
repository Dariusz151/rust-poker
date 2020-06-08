use enum_iterator::IntoEnumIterator;
use rand::seq::SliceRandom;
use rand::thread_rng;

#[allow(dead_code)]
const CARDS_AMOUNT: usize = 52;
const STARTING_MONEY: u32 = 15000;

#[derive(Debug, Copy, Clone)]
pub enum Decision {
    Bet(Result<u32, &'static str>),
    Pass,
    Check,
}

#[derive(Debug, PartialEq, Copy, Clone, IntoEnumIterator)]
enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
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
    figure: Figure,
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
                let card = Card {
                    suit: *(&suit),
                    figure: *(&figure),
                };
                cards.push(card);
            }
        }

        Deck { cards }
    }

    pub fn take_card(&mut self) -> Card {
        if let Some(c) = self.cards.pop() {
            c
        } else {
            panic!("Could not take card from deck!");
        }
    }
}

#[derive(Debug)]
pub struct Table {
    flop: Option<(Card, Card, Card)>,
    turn: Option<Card>,
    river: Option<Card>,
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

#[derive(Debug, Clone)]
pub struct Player {
    pub name: String,
    hand: Option<(Card, Card)>,
    money: u32,
    actual_decision: Decision,
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

#[derive(Debug)]
pub struct Croupier {
    name: String,
}

impl Croupier {
    pub fn new(name: String) -> Croupier {
        println!("{} [Croupier] entered the poker.", name);
        Croupier { name }
    }

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

#[derive(Debug)]
pub struct Round {
    pub table: Table,
    pub players: Option<Vec<Player>>,
}

impl Round {
    pub fn new() -> Round {
        let table: Table = Round::new_table();

        Round {
            table: table,
            players: None,
        }
    }

    pub fn new_table() -> Table {
        Table::new()
    }

    pub fn clean_table(&mut self) {
        self.table.clean();
    }

    pub fn add_players(&mut self, players: Vec<Player>) {
        self.players = Some(players);
    }

    pub fn start_round(&mut self, round_number: &mut u32) {
        if let Some(players) = &mut self.players {
            println!("Starting new round.");
        } else {
            println!("There is no players in this round.");
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
    fn create_new_deck() {
        // arrange
        // act
        let deck = Deck::new();
        // assert
        assert_eq!(deck.cards.len(), CARDS_AMOUNT);
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

    // #[test]
    // #[ignore]
    // fn bet_less_than_have_should_subtract_from_players_money() {
    //     let mut player: Player = Player::new(String::from("Mateusz"));
    //     let amount: u32 = STARTING_MONEY - 2000;
    //     let result: Decision = player.bet(amount).unwrap();

    //     assert_eq!(player.money, 2000);
    //     assert_eq!(result, Decision::Bet(Ok(amount)));
    // }

    // #[test]
    // #[ignore]
    // fn bet_more_than_have_should_err() {
    //     let mut player: Player = Player::new(String::from("Mateusz"));
    //     assert!(player.bet(STARTING_MONEY + 2000).is_err());
    // }

    // #[test]
    // #[ignore]
    // fn check_should_return_actual_state_check() {
    //     let mut player: Player = Player::new(String::from("Mateusz"));
    //     player.check();
    //     assert_eq!(player.actual_state, Decision::Check);
    // }

    // #[test]
    // #[ignore]
    // fn pass_should_return_actual_state_pass() {
    //     let mut player: Player = Player::new(String::from("Mateusz"));
    //     player.pass();
    //     assert_eq!(player.actual_state, Decision::Pass);
    // }
}
