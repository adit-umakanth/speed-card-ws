use std::ops::{Index, IndexMut};

use crate::game_logic::card::Card;
use crate::game_logic::side::Side;

#[derive(Debug)]
pub struct SideIndexedPile(pub Vec<Card>, pub Vec<Card>);

impl Index<Side> for SideIndexedPile {

    type Output = Vec<Card>;

    fn index(&self, side: Side) -> &Self::Output {
        match side {
            Side::LEFT => &self.0,
            Side::RIGHT => &self.1
        }
    }
}

impl IndexMut<Side> for SideIndexedPile {
    fn index_mut(&mut self, side: Side) -> &mut Self::Output {
        match side {
            Side::LEFT => &mut self.0,
            Side::RIGHT => &mut self.1
        }
    }
}