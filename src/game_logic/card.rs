use crate::game_logic::suit::Suit;

#[derive(Clone, Copy, Debug)]
pub struct Card {
    pub rank: u8,
    pub suit: Suit,
}

impl Card {
    pub fn is_adjacent_card(&self, other: &Card) -> bool {
        true
    }
}
