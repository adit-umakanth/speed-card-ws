use crate::game_logic::{Player, PlayerView};

enum ServerMessage {
    PlayerMove(PlayerView),
    OpponentMove(PlayerView),
    IllegalMove,
    GameWon(Player),
}
