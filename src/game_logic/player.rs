#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Player {
    PLAYER1,
    PLAYER2,
}

impl Player {
    pub fn opponent(&self) -> Player {
        match self {
            Player::PLAYER1 => Player::PLAYER2,
            Player::PLAYER2 => Player::PLAYER1,
        }
    }
}
