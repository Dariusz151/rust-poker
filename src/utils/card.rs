use enum_iterator::IntoEnumIterator;
use std::fmt;

#[derive(Debug, PartialEq, Copy, Clone, IntoEnumIterator)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

#[derive(Debug, PartialEq, Copy, Clone, IntoEnumIterator)]
pub enum Figure {
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
    pub suit: Suit,
    pub figure: Figure,
}

impl fmt::Display for Figure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Figure::Ace => write!(f, "A"),
            Figure::Two => write!(f, "2"),
            Figure::Three => write!(f, "3"),
            Figure::Four => write!(f, "4"),
            Figure::Five => write!(f, "5"),
            Figure::Six => write!(f, "6"),
            Figure::Seven => write!(f, "7"),
            Figure::Eight => write!(f, "8"),
            Figure::Nine => write!(f, "9"),
            Figure::Ten => write!(f, "10"),
            Figure::Jack => write!(f, "J"),
            Figure::Queen => write!(f, "Q"),
            Figure::King => write!(f, "K"),
        }
    }
}
impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Suit::Spades => write!(f, "♠"),
            Suit::Hearts => write!(f, "♥"),
            Suit::Diamonds => write!(f, "♦"),
            Suit::Clubs => write!(f, "♣"),
        }
    }
}
impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.figure, self.suit)
    }
}