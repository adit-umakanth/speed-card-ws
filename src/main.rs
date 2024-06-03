mod game_logic;
mod player_action;
use player_action::*;
mod server_message;
use server_message::*;
mod game_session;

use anyhow::Result;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::WebSocketStream;

type PlayerConnection = WebSocketStream<TcpStream>;

#[tokio::main]
async fn main() -> Result<()> {
    start_server().await
}

async fn start_server() -> Result<()> {
    let listener = TcpListener::bind(&"0.0.0.0:8080".to_string()).await?;

    let p1 = connect_player(&listener).await?;
    println!("Player 1 connected!");
    let p2 = connect_player(&listener).await?;
    println!("Player 2 connected!");

    game_session::start_game(p1, p2).await?;

    Ok(())
}

async fn connect_player(listener: &TcpListener) -> Result<PlayerConnection> {
    let (stream, _) = listener.accept().await?;
    let player_stream = tokio_tungstenite::accept_async(stream).await?;
    Ok(player_stream)
}

#[cfg(test)]
mod tests {
    use futures_util::StreamExt;
    use game_logic::{Player, SpeedTable};
    use std::{thread, time::Duration};
    use tokio_tungstenite::connect_async;

    use super::*;

    #[tokio::test]
    async fn test_websocket_server_game() -> Result<()> {
        std::thread::spawn(|| {
            let _ = main();
        });
        thread::sleep(Duration::from_secs(2));

        let (mut p1, _) = connect_async(url::Url::parse("ws://0.0.0.0:8080")?).await?;
        let (mut p2, _) = connect_async(url::Url::parse("ws://0.0.0.0:8080")?).await?;
        let table = SpeedTable::new();

        let message1 = p1.next().await.unwrap()?.into_text()?;
        let message2 = p2.next().await.unwrap()?.into_text()?;

        let empty_board_message = ServerMessage {
            player_view: table.get_player_view(Player::PLAYER1),
            action: ServerAction::SetBoard,
        };
        assert_eq!(
            empty_board_message,
            serde_json::from_str(&message1).unwrap()
        );
        assert_eq!(
            empty_board_message,
            serde_json::from_str(&message2).unwrap()
        );

        Ok(())
    }
}
