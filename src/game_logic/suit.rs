#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Suit {
    Diamonds,
    Spades,
    Clubs,
    Hearts,
}

impl Suit {
    pub fn iter() -> impl Iterator<Item = Suit> {
        [Suit::Diamonds, Suit::Spades, Suit::Clubs, Suit::Hearts]
            .iter()
            .copied()
    }
}
