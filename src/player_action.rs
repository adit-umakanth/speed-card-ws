use serde::{Deserialize, Serialize};

use crate::game_logic::Side;

#[derive(Debug, Serialize, Deserialize)]
pub enum PlayerAction {
    DrawCard,
    Flip,
    PlaceCard(usize, Side),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serde() {
        let action: Result<PlayerAction, serde_json::Error> =
            serde_json::from_str("{\"PlaceCard\":[2,\"RIGHT\"]}");

        println!("{:#?}", action);
    }
}
