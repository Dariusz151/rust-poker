use itertools::Itertools;

use crate::utils::card::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Hands {
    None,
    HighCard(Figure),
    OnePair(Figure),
    TwoPairs(Figure, Figure),
    ThreeOfAKind(Figure),
    Straight(Figure),
    Flush(Suit),
    FullHouse(Figure),
    FourOfAKind(Figure),
    StraightFlush(Card),
    RoyalFlush,
}

#[derive(Debug, Clone)]
pub struct CardComparer {}

impl CardComparer {
    // pub fn classificate_cards(cards: &Vec<Card>) -> Hands {
    //     let card: Card = Card {
    //         suit: Suit::Hearts,
    //         figure: Figure::Ace,
    //     };
    //     Hands::HighCard(card)
    // }

    pub fn get_duplicated_card(figures: &mut Vec<Figure>) -> Vec<Figure> {
        let mut already_seen = vec![];
        figures.retain(|item| match already_seen.contains(item) {
            true => true,
            _ => {
                already_seen.push(item.clone());
                false
            }
        });

        figures.to_vec()
    }

    pub fn remove_duplicated_card(figures: &mut Vec<Figure>) -> Vec<Figure> {
        let mut already_seen = vec![];
        figures.retain(|item| match already_seen.contains(item) {
            true => false,
            _ => {
                already_seen.push(item.clone());
                true
            }
        });

        figures.to_vec()
    }

    pub fn descending_pairs(cards: &mut Vec<Card>) -> Vec<Figure> {
        let mut figures: Vec<Figure> = cards.iter().map(|x| x.figure).collect();
        let mut duplicated_cards: Vec<Figure> = CardComparer::get_duplicated_card(&mut figures);
        let mut removed_repeating_duplicates: Vec<Figure> = CardComparer::remove_duplicated_card(&mut duplicated_cards);

        removed_repeating_duplicates.sort_by(|a,b| b.cmp(&a));

        return removed_repeating_duplicates
    }

    
    pub fn check_one_pair(cards: &mut Vec<Card>) -> Hands {

        let descending_pairs: Vec<Figure> = CardComparer::descending_pairs(cards);

        if descending_pairs.len() > 0 {
            return Hands::OnePair(*&descending_pairs[0])
        }
        Hands::None
    }

    pub fn check_two_pairs(cards: &mut Vec<Card>) -> Hands {

        let descending_pairs: Vec<Figure> = CardComparer::descending_pairs(cards);

        if descending_pairs.len() > 1 {
            return Hands::TwoPairs(*&descending_pairs[0], *&descending_pairs[1])
        }
        Hands::None
    }

    pub fn check_high_card(cards: &mut Vec<Card>) -> Hands {
        cards.sort_by(|a,b| b.figure.cmp(&a.figure));
        Hands::HighCard(*&cards[0].figure)
    }
}


#[cfg(test)]
mod card_comparer_tests {
    use super::*;

    fn push_all_cards(card1: Card, card2: Card, card3: Card, card4: Card, card5: Card, card6: Card, card7: Card) -> Vec<Card> {
        let mut cards: Vec<Card> = Vec::new();
        cards.push(card1);
        cards.push(card2);
        cards.push(card3);
        cards.push(card4);
        cards.push(card5);
        cards.push(card6);
        cards.push(card7);

        cards
    }


    // CHECK TWO PAIRS

    #[test]
    fn check_two_pairs_two_pairs_2_and_6_should_return_hands_two_pairs() {
        let mut card1: Card = Card {suit: Suit::Hearts, figure: Figure::Two};
        let mut card2: Card = Card {suit: Suit::Hearts, figure: Figure::Three};
        let mut card3: Card = Card {suit: Suit::Hearts, figure: Figure::Two};
        let mut card4: Card = Card {suit: Suit::Hearts, figure: Figure::Six};
        let mut card5: Card = Card {suit: Suit::Hearts, figure: Figure::Six};
        let mut card6: Card = Card {suit: Suit::Hearts, figure: Figure::King};
        let mut card7: Card = Card {suit: Suit::Hearts, figure: Figure::Four};

        let mut cards: Vec<Card> = push_all_cards(card1, card2, card3, card4, card5, card6, card7);

        assert_eq!(
            CardComparer::check_two_pairs(&mut cards), 
            Hands::TwoPairs(Figure::Six, Figure::Two)
        );
    }

    #[test]
    fn check_two_pairs_three_4_and_three_10_should_return_hands_two_pairs() {
        let card1: Card = Card {suit: Suit::Hearts, figure: Figure::Four};
        let card2: Card = Card {suit: Suit::Hearts, figure: Figure::Four};
        let card3: Card = Card {suit: Suit::Hearts, figure: Figure::Four};
        let card4: Card = Card {suit: Suit::Hearts, figure: Figure::Ten};
        let card5: Card = Card {suit: Suit::Hearts, figure: Figure::Ten};
        let card6: Card = Card {suit: Suit::Hearts, figure: Figure::Ten};
        let card7: Card = Card {suit: Suit::Hearts, figure: Figure::Four};

        let mut cards: Vec<Card> = push_all_cards(card1, card2, card3, card4, card5, card6, card7);

        assert_eq!(
            CardComparer::check_two_pairs(&mut cards), 
            Hands::TwoPairs(Figure::Ten, Figure::Four)
        );
    }

    #[test]
    fn check_two_pairs_four_Kings_and_two_Aces_should_return_hands_two_pairs() {
        let card1: Card = Card {suit: Suit::Hearts, figure: Figure::King};
        let card2: Card = Card {suit: Suit::Hearts, figure: Figure::Ace};
        let card3: Card = Card {suit: Suit::Hearts, figure: Figure::King};
        let card4: Card = Card {suit: Suit::Hearts, figure: Figure::King};
        let card5: Card = Card {suit: Suit::Hearts, figure: Figure::Ace};
        let card6: Card = Card {suit: Suit::Hearts, figure: Figure::King};
        let card7: Card = Card {suit: Suit::Hearts, figure: Figure::Four};

        let mut cards: Vec<Card> = push_all_cards(card1, card2, card3, card4, card5, card6, card7);

        assert_eq!(
            CardComparer::check_two_pairs(&mut cards), 
            Hands::TwoPairs(Figure::Ace, Figure::King)
        );
    }

    // CHECK ONE PAIR

    #[test]
    fn check_one_pair_only_one_pair_of_4_should_return_hands_one_pair() {
        let card1: Card = Card {suit: Suit::Hearts, figure: Figure::Four};
        let card2: Card = Card {suit: Suit::Hearts, figure: Figure::Three};
        let card3: Card = Card {suit: Suit::Hearts, figure: Figure::Two};
        let card4: Card = Card {suit: Suit::Hearts, figure: Figure::Six};
        let card5: Card = Card {suit: Suit::Hearts, figure: Figure::Ten};
        let card6: Card = Card {suit: Suit::Hearts, figure: Figure::King};
        let card7: Card = Card {suit: Suit::Hearts, figure: Figure::Four};

        let mut cards: Vec<Card> = push_all_cards(card1, card2, card3, card4, card5, card6, card7);

        assert_eq!(
            CardComparer::check_one_pair(&mut cards), 
            Hands::OnePair(Figure::Four)
        );
    }
    
    #[test]
    fn check_one_pair_four_Kings_should_return_hands_one_pair_kings() {
        let card1: Card = Card {suit: Suit::Hearts, figure: Figure::Two};
        let card2: Card = Card {suit: Suit::Hearts, figure: Figure::King};
        let card3: Card = Card {suit: Suit::Hearts, figure: Figure::King};
        let card4: Card = Card {suit: Suit::Hearts, figure: Figure::Five};
        let card5: Card = Card {suit: Suit::Hearts, figure: Figure::King};
        let card6: Card = Card {suit: Suit::Hearts, figure: Figure::King};
        let card7: Card = Card {suit: Suit::Hearts, figure: Figure::Four};

        let mut cards: Vec<Card> = push_all_cards(card1, card2, card3, card4, card5, card6, card7);

        assert_eq!(
            CardComparer::check_one_pair(&mut cards), 
            Hands::OnePair(Figure::King)
        );
    }

    #[test]
    fn check_one_pair_three_Kings_and_two_Aces_should_return_hands_one_pair_ace() {
        let card1: Card = Card {suit: Suit::Hearts, figure: Figure::King};
        let card2: Card = Card {suit: Suit::Hearts, figure: Figure::King};
        let card3: Card = Card {suit: Suit::Hearts, figure: Figure::Three};
        let card4: Card = Card {suit: Suit::Hearts, figure: Figure::King};
        let card5: Card = Card {suit: Suit::Hearts, figure: Figure::Ace};
        let card6: Card = Card {suit: Suit::Hearts, figure: Figure::Ace};
        let card7: Card = Card {suit: Suit::Hearts, figure: Figure::Queen};

        let mut cards: Vec<Card> = push_all_cards(card1, card2, card3, card4, card5, card6, card7);

        assert_eq!(
            CardComparer::check_one_pair(&mut cards), 
            Hands::OnePair(Figure::Ace)
        );
    }

    #[test]
    fn check_one_pair_two_Aces_and_three_Kings_should_return_hands_one_pair_ace() {
        let card1: Card = Card {suit: Suit::Hearts, figure: Figure::Ace};
        let card2: Card = Card {suit: Suit::Hearts, figure: Figure::Ace};
        let card3: Card = Card {suit: Suit::Hearts, figure: Figure::Six};
        let card4: Card = Card {suit: Suit::Hearts, figure: Figure::Five};
        let card5: Card = Card {suit: Suit::Hearts, figure: Figure::King};
        let card6: Card = Card {suit: Suit::Hearts, figure: Figure::King};
        let card7: Card = Card {suit: Suit::Hearts, figure: Figure::King};

        let mut cards: Vec<Card> = push_all_cards(card1, card2, card3, card4, card5, card6, card7);

        assert_eq!(
            CardComparer::check_one_pair(&mut cards), 
            Hands::OnePair(Figure::Ace)
        );
    }

    #[test]
    fn check_one_pair_two_Aces_and_two_6_and_two_queens_should_return_hands_one_pair_ace() {
        let card1: Card = Card {suit: Suit::Hearts, figure: Figure::Queen};
        let card2: Card = Card {suit: Suit::Hearts, figure: Figure::Queen};
        let card3: Card = Card {suit: Suit::Hearts, figure: Figure::Six};
        let card4: Card = Card {suit: Suit::Hearts, figure: Figure::Five};
        let card5: Card = Card {suit: Suit::Hearts, figure: Figure::Six};
        let card6: Card = Card {suit: Suit::Hearts, figure: Figure::Ace};
        let card7: Card = Card {suit: Suit::Hearts, figure: Figure::Ace};

        let mut cards: Vec<Card> = push_all_cards(card1, card2, card3, card4, card5, card6, card7);

        assert_eq!(
            CardComparer::check_one_pair(&mut cards), 
            Hands::OnePair(Figure::Ace)
        );
    }

    #[test]
    fn check_one_pair_four_10_and_two_aces_should_return_hands_one_pair_ace() {
        let card1: Card = Card {suit: Suit::Hearts, figure: Figure::Ten};
        let card2: Card = Card {suit: Suit::Hearts, figure: Figure::Ten};
        let card3: Card = Card {suit: Suit::Hearts, figure: Figure::Six};
        let card4: Card = Card {suit: Suit::Hearts, figure: Figure::Ten};
        let card5: Card = Card {suit: Suit::Hearts, figure: Figure::Ten};
        let card6: Card = Card {suit: Suit::Hearts, figure: Figure::Ace};
        let card7: Card = Card {suit: Suit::Hearts, figure: Figure::Ace};

        let mut cards: Vec<Card> = push_all_cards(card1, card2, card3, card4, card5, card6, card7);

        assert_eq!(
            CardComparer::check_one_pair(&mut cards), 
            Hands::OnePair(Figure::Ace)
        );
    }

    // CHECK HIGH CARD

    #[test]
    fn check_high_card_ace_should_be_highest_card() {
        let card1: Card = Card {suit: Suit::Hearts, figure: Figure::Ten};
        let card2: Card = Card {suit: Suit::Hearts, figure: Figure::Four};
        let card3: Card = Card {suit: Suit::Hearts, figure: Figure::Six};
        let card4: Card = Card {suit: Suit::Hearts, figure: Figure::Seven};
        let card5: Card = Card {suit: Suit::Hearts, figure: Figure::Queen};
        let card6: Card = Card {suit: Suit::Hearts, figure: Figure::Ace};
        let card7: Card = Card {suit: Suit::Hearts, figure: Figure::Three};

        let mut cards: Vec<Card> = push_all_cards(card1, card2, card3, card4, card5, card6, card7);

        assert_eq!(
            CardComparer::check_high_card(&mut cards), 
            Hands::HighCard(Figure::Ace)
        );
    }
}