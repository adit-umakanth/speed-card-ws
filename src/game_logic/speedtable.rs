use super::{suit::Suit, player::Player};
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

fn draw_cards(deck: &mut Vec<Card>, i: usize) -> Vec<Card> {
    deck.drain(0..i).collect()
}

impl SpeedTable {
    fn new_set_rng(rng: &mut dyn RngCore) -> SpeedTable {
        let mut deck: Vec<Card> = Vec::new();
        for suit in Suit::iter() {
            for rank in 1..14 {
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
        match (self.middle_piles[Side::LEFT].last(), self.middle_piles[Side::RIGHT].last()) {
            (Some(_), Some(_)) => {
                self.active_piles[Side::LEFT].push(self.middle_piles[Side::LEFT].pop().unwrap());
                self.active_piles[Side::RIGHT].push(self.middle_piles[Side::RIGHT].pop().unwrap());
            }
            _ => todo!(), // Shuffle middle cards and redistribute
        }
    }

    pub fn player_draw_card(&mut self, player: Player) {
        let first_empty_index = self.player_hands[player].iter().position(|x| x.is_none());
        if let Some(i) = first_empty_index {
            self.player_hands[player][i] = Some(self.player_piles[player].pop().unwrap());
        }
    }

    pub fn place_card(&mut self, player: Player, side: Side, hand_index: usize) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::prelude::*;
    use rand_chacha::ChaCha8Rng;

    #[test]
    fn test_flip_middle() {
        let mut rng = ChaCha8Rng::seed_from_u64(5);
        let mut table = SpeedTable::new_set_rng(&mut rng);
        println!("{:#?}\n", table);
        table.flip_middle_cards();
        println!("{:#?}\n", table);
        table.player_draw_card(Player::PLAYER1);
        println!("{:#?}\n", table);
    }
}
