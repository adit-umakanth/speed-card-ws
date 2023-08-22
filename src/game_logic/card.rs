use crate::game_logic::rank::Rank;
use crate::game_logic::suit::Suit;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

impl Card {
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

    #[test]
    fn test_card_equal() {
        assert_eq!(
            Card {
                suit: Suit::Spades,
                rank: Rank::Ace
            } == Card {
                suit: Suit::Spades,
                rank: Rank::Ace
            },
            true
        );
    }

    #[test]
    fn test_card_diff_suit() {
        assert_eq!(
            Card {
                suit: Suit::Clubs,
                rank: Rank::Ten
            } == Card {
                suit: Suit::Diamonds,
                rank: Rank::Ten
            },
            false
        );
    }

    #[test]
    fn test_card_diff_rank() {
        assert_eq!(
            Card {
                suit: Suit::Spades,
                rank: Rank::Ace
            } == Card {
                suit: Suit::Spades,
                rank: Rank::King
            },
            false
        )
    }
}
