mod game_logic;
mod player_action;
mod server_message;

fn main() {
    println!("Hello, world!");
    println!("{:#?}", game_logic::SpeedTable::new())
}
