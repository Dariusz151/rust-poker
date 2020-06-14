use itertools::Itertools;

use crate::utils::card::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Hands {
    None,
    HighCard(Card),
    OnePair(Figure),
    TwoPairs(Figure, Figure),
    ThreeOfAKind(Card),
    Straight(Card),
    Flush(Card),
    FullHouse(Card),
    FourOfAKind(Card),
    StraightFlush(Card),
    RoyalFlush,
}

#[derive(Debug, Clone)]
pub struct CardComparer {}

impl CardComparer {
    pub fn classificate_cards(cards: &Vec<Card>) -> Hands {
        let card: Card = Card {
            suit: Suit::Hearts,
            figure: Figure::Ace,
        };
        Hands::HighCard(card)
    }

    // pub fn high_card(cards: &Vec<Card>) -> Hands {
    //     let mut unique_figures = Vec::new();
    //     for (_i, card) in cards.iter().enumerate() {
    //         if unique_figures.contains(&card.figure) {
    //             return true;
    //         }
    //         unique_figures.push(card.figure);
    //     }
    //     return false;
    // }

    pub fn check_one_pair(cards: &mut Vec<Card>) -> Hands {
        //TODO: implement this as trait + create a function !!
        let mut already_seen = vec![];
        cards.retain(|item| match already_seen.contains(item) {
            true => true,
            _ => {
                already_seen.push(item.clone());
                false
            }
        });

        let mut already_seen = vec![];
        cards.retain(|item| match already_seen.contains(item) {
            true => false,
            _ => {
                already_seen.push(item.clone());
                true
            }
        });

        cards.sort_by(|a,b| b.figure.cmp(&a.figure));

        if cards.len() > 0 {
            let card1_figure = &cards[0].figure;
            return Hands::OnePair(*card1_figure)
        }
        Hands::None
    }

    pub fn check_two_pairs(cards: &mut Vec<Card>) -> Hands {
        let mut already_seen = vec![];
        cards.retain(|item| match already_seen.contains(item) {
            true => true,
            _ => {
                already_seen.push(item.clone());
                false
            }
        });

        let mut already_seen = vec![];
        cards.retain(|item| match already_seen.contains(item) {
            true => false,
            _ => {
                already_seen.push(item.clone());
                true
            }
        });

        cards.sort_by(|a,b| b.figure.cmp(&a.figure));

        if cards.len() > 1 {
            let card1_figure = &cards[0].figure;
            let card2_figure = &cards[1].figure;
            return Hands::TwoPairs(*card1_figure, *card2_figure)
        }
        Hands::None
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

    #[test]
    fn card_comparer_only_one_pair_of_4_should_return_hands_one_pair() {
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
    fn card_comparer_three_Kings_should_return_hands_one_pair_king() {
        let card1: Card = Card {suit: Suit::Hearts, figure: Figure::Two};
        let card2: Card = Card {suit: Suit::Hearts, figure: Figure::Three};
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
    fn card_comparer_four_Kings_should_return_hands_one_pair_kings() {
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
    fn card_comparer_three_Kings_and_two_Aces_should_return_hands_one_pair_ace() {
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
    fn card_comparer_two_Aces_and_three_Kings_should_return_hands_one_pair_ace() {
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
    fn card_comparer_two_Aces_and_two_6_and_two_queens_should_return_hands_one_pair_ace() {
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
    fn card_comparer_four_10_and_two_aces_should_return_hands_one_pair_ace() {
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

    // #[test]
    // fn card_comparer_two_pairs_on_input_should_return_hands_two_pairs() {
    //     let mut card1: Card = Card {suit: Suit::Hearts, figure: Figure::Two};
    //     let mut card2: Card = Card {suit: Suit::Hearts, figure: Figure::Three};
    //     let mut card3: Card = Card {suit: Suit::Hearts, figure: Figure::Two};
    //     let mut card4: Card = Card {suit: Suit::Hearts, figure: Figure::Six};
    //     let mut card5: Card = Card {suit: Suit::Hearts, figure: Figure::Six};
    //     let mut card6: Card = Card {suit: Suit::Hearts, figure: Figure::King};
    //     let mut card7: Card = Card {suit: Suit::Hearts, figure: Figure::Four};

    //     let mut cards: Vec<Card> = push_all_cards(card1, card2, card3, card4, card5, card6, card7);

    //     let mut hands = CardComparer::check_two_pairs(&mut cards); 
    //     println!("Hands: {:?}", &hands);

    //     assert_eq!(hands, Hands::TwoPairs(Figure::Two, Figure::Six));
    // }
}