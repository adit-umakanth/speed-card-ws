use std::ops::{Index, IndexMut};

use crate::game_logic::card::Card;
use crate::game_logic::player::Player;

#[derive(Debug)]
pub struct PlayerHands(pub [Option<Card>; 4], pub [Option<Card>; 4]);

impl Index<Player> for PlayerHands {
    type Output = [Option<Card>; 4];

    fn index(&self, player: Player) -> &Self::Output {
        match player {
            Player::PLAYER1 => &self.0,
            Player::PLAYER2 => &self.1,
        }
    }
}

impl IndexMut<Player> for PlayerHands {
    fn index_mut(&mut self, player: Player) -> &mut Self::Output {
        match player {
            Player::PLAYER1 => &mut self.0,
            Player::PLAYER2 => &mut self.1,
        }
    }
}
