use serde::{Deserialize, Serialize};

use crate::game_logic::rank::Rank;
use crate::game_logic::suit::Suit;

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

impl Card {
    pub fn new(rank: Rank, suit: Suit) -> Card {
        Card { rank, suit }
    }

    pub fn is_adjacent_card(&self, other: &Card) -> bool {
        match self.rank.value().abs_diff(other.rank.value()) {
            1 | 12 => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Rank::*;
    use Suit::*;

    #[test]
    fn test_adjacent_cards() {
        assert_eq!(
            Card::new(Ace, Spades).is_adjacent_card(&Card::new(King, Clubs)),
            true
        );
        assert_eq!(
            Card::new(Two, Spades).is_adjacent_card(&Card::new(Three, Diamonds)),
            true
        );

        assert_eq!(
            Card::new(Queen, Hearts).is_adjacent_card(&Card::new(Jack, Hearts)),
            true
        );
    }

    #[test]
    fn test_not_adjacent_cards() {
        assert_eq!(
            Card::new(Ace, Spades).is_adjacent_card(&Card::new(Ace, Hearts)),
            false
        );
        assert_eq!(
            Card::new(Two, Diamonds).is_adjacent_card(&Card::new(Four, Diamonds)),
            false
        );
        assert_eq!(
            Card::new(Seven, Clubs).is_adjacent_card(&Card::new(King, Clubs)),
            false
        );
    }
}
