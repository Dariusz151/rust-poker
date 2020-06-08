/*
** TESTS
*/

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn shuffling_cards() {
    //     // arrange
    //     let deck_original: Deck = Deck::new();
    //     let mut deck_cloned: Deck = deck_original.clone();
    //     // act
    //     Croupier::shuffle_cards(&mut deck_cloned);
    //     // assert
    //     assert_ne!(deck_original, deck_cloned);
    // }
    // #[test]
    // fn create_new_deck() {
    //     // arrange
    //     // act
    //     let deck = Deck::new();
    //     // assert
    //     assert_eq!(deck.cards.len(), CARDS_AMOUNT);
    // }

    // #[test]
    // fn take_card() {
    //     // arrange
    //     let mut deck = Deck::new();
    //     let deck_len: usize = deck.cards.len();
    //     // act
    //     let _last_card: Card = deck.take_card();
    //     // assert
    //     assert_eq!(deck_len - 1, deck.cards.len());
    // }

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
