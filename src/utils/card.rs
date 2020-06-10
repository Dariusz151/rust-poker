use enum_iterator::IntoEnumIterator;

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
// TODO: implement Display