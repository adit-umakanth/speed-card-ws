use super::{player::Player, rank::Rank, suit::Suit, PlayerView};
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

use SpeedError as SE;

/// Possible events that could arise apart from a simple card movement.
/// Many of these should not be permitted from the client side.
#[derive(Debug, PartialEq)]
pub enum SpeedError {
    NoCardToDraw,    // Client shouldn't allow
    HandAlreadyFull, // Client shouldn't allow
    NoCardToPlace,   // Client shouldn't allow
    NoCardToPlaceOn, // Client shouldn't allow
    GameWon,
    NotAdjacentCard,
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

    /// Move the top cards on the middle piles onto the active piles.
    /// This is done on request by both players when they think they have no more cards to play.
    pub fn flip_middle_cards(&mut self) -> Result<(), SpeedError> {
        if self.middle_piles[Side::LEFT].last().is_none()
            || self.middle_piles[Side::RIGHT].last().is_none()
        {
            let mut combined_pile = Vec::new();
            combined_pile.append(&mut self.middle_piles[Side::LEFT]);
            combined_pile.append(&mut self.middle_piles[Side::RIGHT]);
            combined_pile.append(&mut self.active_piles[Side::LEFT]);
            combined_pile.append(&mut self.active_piles[Side::RIGHT]);
            combined_pile.shuffle(&mut thread_rng());
            self.middle_piles[Side::LEFT] =
                combined_pile.drain(0..combined_pile.len() / 2).collect();
            self.middle_piles[Side::RIGHT].append(&mut combined_pile);
        }
        self.active_piles[Side::LEFT].push(self.middle_piles[Side::LEFT].pop().unwrap());
        self.active_piles[Side::RIGHT].push(self.middle_piles[Side::RIGHT].pop().unwrap());

        Ok(())
    }

    fn get_first_empty_hand_idx(&self, player: Player) -> Option<usize> {
        self.player_hands[player].iter().position(|x| x.is_none())
    }

    fn check_for_win(&self, player: Player) -> bool {
        self.player_piles[player].len() == 0
            && self.player_hands[player].iter().all(|x| x.is_none())
    }

    pub fn player_draw_card(&mut self, player: Player) -> Result<(), SpeedError> {
        let first_empty_index = self
            .get_first_empty_hand_idx(player)
            .ok_or(SE::HandAlreadyFull)?;

        let card_to_draw = self.player_piles[player].pop().ok_or(SE::NoCardToDraw)?;
        self.player_hands[player][first_empty_index] = Some(card_to_draw);
        Ok(())
    }

    pub fn place_card(
        &mut self,
        player: Player,
        side: Side,
        hand_index: usize,
    ) -> Result<(), SpeedError> {
        let card_to_place = self.player_hands[player][hand_index].ok_or(SE::NoCardToPlace)?;
        let card_place_on = self.active_piles[side].last().ok_or(SE::NoCardToPlaceOn)?;

        if card_to_place.is_adjacent_card(card_place_on) {
            self.active_piles[side].push(card_to_place);
            self.player_hands[player][hand_index] = None;

            if self.check_for_win(player) {
                return Err(SE::GameWon);
            };

            return Ok(());
        } else {
            return Err(SE::NotAdjacentCard);
        }
    }

    pub fn get_player_view(&self, player: Player) -> PlayerView {
        let opponent_hand = self.player_hands[player.opponent()].map(|x| x.is_some());
        PlayerView {
            player_hand: self.player_hands[player],
            active_cards: [
                self.active_piles[Side::LEFT].last().copied(),
                self.active_piles[Side::RIGHT].last().copied(),
            ],
            opponent_hand,
            opponent_pile: self.player_piles[player.opponent()].len() != 0,
            middle_piles: [
                !self.middle_piles[Side::LEFT].is_empty(),
                !self.middle_piles[Side::RIGHT].is_empty(),
            ],
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
            Err(SE::NotAdjacentCard)
        );
        assert_eq!(table.place_card(Player::PLAYER1, Side::LEFT, 1), Ok(()));
    }

    #[test]
    fn test_middle_reshuffle_equal() {
        let mut table = SpeedTable::new();
        for _ in 0..7 {
            table.flip_middle_cards();
        }

        assert_eq!(table.middle_piles[Side::LEFT].len(), 0);
        assert_eq!(table.middle_piles[Side::RIGHT].len(), 0);

        table.flip_middle_cards();

        assert_eq!(table.active_piles[Side::LEFT].len(), 1);
        assert_eq!(table.active_piles[Side::RIGHT].len(), 1);
        assert_eq!(table.middle_piles[Side::LEFT].len(), 6);
        assert_eq!(table.middle_piles[Side::RIGHT].len(), 6);
    }

    #[test]
    fn test_middle_reshuffle_unequal() {
        let mut rng = ChaCha8Rng::seed_from_u64(0);
        let mut table = SpeedTable::new_set_rng(&mut rng);

        while let Ok(()) = table.player_draw_card(Player::PLAYER1) {}
        while let Ok(()) = table.player_draw_card(Player::PLAYER2) {}

        table.flip_middle_cards();

        assert_eq!(table.place_card(Player::PLAYER1, Side::LEFT, 1), Ok(()));

        for _ in 0..6 {
            table.flip_middle_cards();
        }

        table.flip_middle_cards();

        assert_eq!(table.active_piles[Side::LEFT].len(), 1);
        assert_eq!(table.active_piles[Side::RIGHT].len(), 1);
        assert_eq!(table.middle_piles[Side::LEFT].len(), 6);
        assert_eq!(table.middle_piles[Side::RIGHT].len(), 7);

        for _ in 0..6 {
            table.flip_middle_cards();
        }

        assert_eq!(table.middle_piles[Side::LEFT].len(), 0);
        assert_eq!(table.middle_piles[Side::RIGHT].len(), 1);

        table.flip_middle_cards();

        assert_eq!(table.active_piles[Side::LEFT].len(), 1);
        assert_eq!(table.active_piles[Side::RIGHT].len(), 1);
        assert_eq!(table.middle_piles[Side::LEFT].len(), 6);
        assert_eq!(table.middle_piles[Side::RIGHT].len(), 7);
    }
}
