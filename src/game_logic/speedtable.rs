use super::suit::Suit;
use crate::game_logic::card::Card;

use rand::{seq::SliceRandom, thread_rng, RngCore};

#[derive(Debug)]
pub struct SpeedTable {
    middle_piles: [Vec<Card>; 2],
    active_piles: [Vec<Card>; 2],
    player_hands: [[Option<Card>; 4]; 2],
    player_piles: [Vec<Card>; 2],
}

fn draw_cards(deck: &mut Vec<Card>, i: usize) -> [Vec<Card>; 2] {
    [deck.drain(0..i).collect(), deck.drain(0..i).collect()]
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

        let middle_piles = draw_cards(&mut deck, 7);
        let player_piles = draw_cards(&mut deck, 19);
        let active_piles = [Vec::new(), Vec::new()];
        let player_hands = [[None; 4]; 2];

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
        match (self.middle_piles[0].last(), self.middle_piles[1].last()) {
            (Some(_), Some(_)) => {
                self.active_piles[0].push(self.middle_piles[0].pop().unwrap());
                self.active_piles[1].push(self.middle_piles[1].pop().unwrap());
            }
            _ => todo!(), // Shuffle middle cards and redistribute
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::prelude::*;
    use rand_chacha::ChaCha8Rng;

    #[test]
    fn test_flip_middle() {
        let mut rng = ChaCha8Rng::seed_from_u64(0);
        let mut table = SpeedTable::new_set_rng(&mut rng);
        println!("{:?}", table);
        table.flip_middle_cards();
        println!("{:?}", table);
    }
}
