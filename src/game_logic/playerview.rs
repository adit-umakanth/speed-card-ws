use serde::Serialize;

use super::card::Card;

/*
 * A boolean field indicates whether a card should be rendered in the player view.
 * A false value means that a card does not exist for that specific spot.
 */
#[derive(Serialize)]
pub struct PlayerView {
    pub player_hand: [Option<Card>; 4],
    pub active_cards: [Option<Card>; 2],
    pub opponent_hand: [bool; 4],
    pub opponent_pile: bool,
    pub middle_piles: [bool; 2],
}
