use super::{player::Player, rank::Rank, suit::Suit};
use crate::game_logic::card::Card;
use crate::game_logic::piles::*;
use crate::game_logic::side::Side;

use rand::{seq::SliceRandom, thread_rng, RngCore};

#[derive(Debug)]
pub struct SpeedTable {
    middle_piles: SideIndexedPile,
    active_piles: SideIndexedPile,
    player_hands: PlayerHands,
    player_piles: PlayerIndexedPile,
}

#[derive(Debug, PartialEq)]
pub enum SpeedError {
    IllegalMoveError,
}

fn draw_cards(deck: &mut Vec<Card>, i: usize) -> Vec<Card> {
    deck.drain(0..i).collect()
}

impl SpeedTable {
    fn new_set_rng(rng: &mut dyn RngCore) -> SpeedTable {
        let mut deck: Vec<Card> = Vec::new();
        for suit in Suit::iter() {
            for rank in Rank::iter() {
                deck.push(Card { suit, rank });
            }
        }

        deck.shuffle(rng);

        let middle_piles = SideIndexedPile(draw_cards(&mut deck, 7), draw_cards(&mut deck, 7));
        let player_piles = PlayerIndexedPile(draw_cards(&mut deck, 19), draw_cards(&mut deck, 19));
        let active_piles = SideIndexedPile(Vec::new(), Vec::new());
        let player_hands = PlayerHands([None; 4], [None; 4]);

        SpeedTable {
            middle_piles,
            active_piles,
            player_piles,
            player_hands,
        }
    }

    pub fn new() -> SpeedTable {
        SpeedTable::new_set_rng(&mut thread_rng())
    }

    pub fn flip_middle_cards(&mut self) {
        match (
            self.middle_piles[Side::LEFT].last(),
            self.middle_piles[Side::RIGHT].last(),
        ) {
            (Some(_), Some(_)) => {
                self.active_piles[Side::LEFT].push(self.middle_piles[Side::LEFT].pop().unwrap());
                self.active_piles[Side::RIGHT].push(self.middle_piles[Side::RIGHT].pop().unwrap());
            }
            _ => todo!(), // Shuffle middle cards and redistribute
        }
    }

    pub fn player_draw_card(&mut self, player: Player) -> Result<(), SpeedError> {
        let first_empty_index = self.player_hands[player].iter().position(|x| x.is_none());
        if let Some(i) = first_empty_index {
            self.player_hands[player][i] = Some(self.player_piles[player].pop().unwrap());
            Ok(())
        } else {
            Err(SpeedError::IllegalMoveError)
        }
    }

    pub fn place_card(
        &mut self,
        player: Player,
        side: Side,
        hand_index: usize,
    ) -> Result<(), SpeedError> {
        let card_to_place = self.player_hands[player][hand_index];
        let card_place_on = self.active_piles[side].last();

        match (card_to_place, card_place_on) {
            (Some(card_to_place), Some(card_place_on)) => {
                if card_to_place.is_adjacent_card(card_place_on) {
                    self.active_piles[side].push(card_to_place);
                    self.player_hands[player][hand_index] = None;
                    Ok(())
                } else {
                    Err(SpeedError::IllegalMoveError)
                }
            }
            _ => Err(SpeedError::IllegalMoveError),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::prelude::*;
    use rand_chacha::ChaCha8Rng;

    #[test]
    fn test_table_init() {
        let speedtable = SpeedTable::new();

        assert_eq!(speedtable.player_piles[Player::PLAYER1].len(), 19);
        assert_eq!(speedtable.player_piles[Player::PLAYER2].len(), 19);

        assert_eq!(speedtable.middle_piles[Side::LEFT].len(), 7);
        assert_eq!(speedtable.middle_piles[Side::RIGHT].len(), 7);

        assert_eq!(speedtable.active_piles[Side::LEFT].len(), 0);
        assert_eq!(speedtable.active_piles[Side::RIGHT].len(), 0);

        assert_eq!(
            speedtable.player_hands[Player::PLAYER1]
                .iter()
                .all(|x| x.is_none()),
            true
        );
        assert_eq!(
            speedtable.player_hands[Player::PLAYER2]
                .iter()
                .all(|x| x.is_none()),
            true
        );
    }

    #[test]
    fn test_flip_middle() {
        let mut table = SpeedTable::new();

        let last_middle_left = table.middle_piles[Side::LEFT].get(6).unwrap().to_owned();
        let second_last_middle_left = table.middle_piles[Side::LEFT].get(5).unwrap().to_owned();

        let last_middle_right = table.middle_piles[Side::RIGHT].get(6).unwrap().to_owned();
        let second_last_middle_right = table.middle_piles[Side::RIGHT].get(5).unwrap().to_owned();

        table.flip_middle_cards();

        assert_eq!(
            table.active_piles[Side::LEFT].last().unwrap().to_owned(),
            last_middle_left
        );
        assert_eq!(
            table.middle_piles[Side::LEFT].last().unwrap().to_owned(),
            second_last_middle_left
        );

        assert_eq!(
            table.active_piles[Side::RIGHT].last().unwrap().to_owned(),
            last_middle_right
        );
        assert_eq!(
            table.middle_piles[Side::RIGHT].last().unwrap().to_owned(),
            second_last_middle_right
        );
    }

    #[test]
    fn test_player_draw_card() {
        let mut table = SpeedTable::new();
        let player_hand = [
            Some(Card {
                rank: Rank::Ace,
                suit: Suit::Spades,
            }),
            None,
            Some(Card {
                rank: Rank::King,
                suit: Suit::Hearts,
            }),
            Some(Card {
                rank: Rank::Jack,
                suit: Suit::Diamonds,
            }),
        ];

        table.player_hands = PlayerHands(player_hand.clone(), player_hand.clone());

        let card_to_draw_player1 = table.player_piles[Player::PLAYER1][18];
        let card_to_draw_player2 = table.player_piles[Player::PLAYER2][18];

        let _ = table.player_draw_card(Player::PLAYER1);
        let _ = table.player_draw_card(Player::PLAYER2);

        assert_eq!(
            table.player_hands[Player::PLAYER1][1].unwrap(),
            card_to_draw_player1
        );
        assert_eq!(
            table.player_hands[Player::PLAYER2][1].unwrap(),
            card_to_draw_player2
        )
    }

    #[test]
    fn test_place_card() {
        let mut rng = ChaCha8Rng::seed_from_u64(0);
        let mut table = SpeedTable::new_set_rng(&mut rng);

        while let Ok(()) = table.player_draw_card(Player::PLAYER1) {}
        while let Ok(()) = table.player_draw_card(Player::PLAYER2) {}

        table.flip_middle_cards();

        assert_eq!(
            table.place_card(Player::PLAYER1, Side::RIGHT, 1),
            Err(SpeedError::IllegalMoveError)
        );
        assert_eq!(table.place_card(Player::PLAYER1, Side::LEFT, 1), Ok(()));
    }
}
