use serde::{Deserialize, Serialize};

use crate::game_logic::{Player, PlayerView};

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub enum ServerAction {
    SetBoard,
    NormalMove,
    GameWon,
    GameLost,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct ServerMessage {
    pub action: ServerAction,
    pub player_view: PlayerView,
}
