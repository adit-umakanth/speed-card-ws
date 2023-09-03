use std::ops::{Index, IndexMut};

use crate::game_logic::card::Card;
use crate::game_logic::player::Player;

#[derive(Debug)]
pub struct PlayerIndexedPile(pub Vec<Card>, pub Vec<Card>);

impl Index<Player> for PlayerIndexedPile {
    type Output = Vec<Card>;

    fn index(&self, player: Player) -> &Self::Output {
        match player {
            Player::PLAYER1 => &self.0,
            Player::PLAYER2 => &self.1,
        }
    }
}

impl IndexMut<Player> for PlayerIndexedPile {
    fn index_mut(&mut self, player: Player) -> &mut Self::Output {
        match player {
            Player::PLAYER1 => &mut self.0,
            Player::PLAYER2 => &mut self.1,
        }
    }
}
