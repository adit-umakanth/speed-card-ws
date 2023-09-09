use serde::Serialize;

use crate::game_logic::{Player, PlayerView};

#[derive(Clone, Copy, Serialize)]
pub enum ServerAction {
    SetBoard,
    PlayerMove,
    OpponentMove,
    IllegalMove,
    GameWon,
}

#[derive(Serialize)]
pub struct ServerMessage {
    pub action: ServerAction,
    pub player_view: PlayerView,
}
