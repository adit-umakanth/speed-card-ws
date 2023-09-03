use super::card::Card;

/*
 * A boolean field indicates whether a card should be rendered in the player view.
 * A false value means that a card does not exist for that specific spot.
 */
pub struct PlayerView {
    player_hand: [Option<Card>; 4],
    active_cards: [Option<Card>; 2],
    opponent_hand: [bool; 4],
    opponent_pile: bool,
    middle_piles: [bool; 2],
}
