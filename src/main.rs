mod game_logic;
mod player_action;
use player_action::*;
mod server_message;
use server_message::*;

use anyhow::Result;
use futures_util::{
    future::{join, select, Either},
    SinkExt, StreamExt,
};
use game_logic::{Player, SpeedError, SpeedTable};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{tungstenite::Message, WebSocketStream};

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

    start_game(p1, p2).await?;

    Ok(())
}

async fn start_game(mut p1: PlayerConnection, mut p2: PlayerConnection) -> Result<()> {
    let mut table = SpeedTable::new();
    send_player_message(
        Player::PLAYER1,
        &mut p1,
        &mut p2,
        &table,
        ServerAction::SetBoard,
        ServerAction::SetBoard,
    )
    .await;

    loop {
        let (player_move, player) = wait_for_player_move(&mut p1, &mut p2).await?;

        let move_result = match player_move {
            PlayerAction::DrawCard => table.player_draw_card(player),
            PlayerAction::Flip => table.flip_middle_cards(),
            PlayerAction::PlaceCard(hand_index, side) => table.place_card(player, side, hand_index),
        };

        if move_result.is_ok() {
            send_player_message(
                Player::PLAYER1,
                &mut p1,
                &mut p2,
                &table,
                ServerAction::NormalMove,
                ServerAction::NormalMove,
            )
            .await;
            continue;
        };

        let (player_connection, other_player_connection) = match player {
            Player::PLAYER1 => (&mut p1, &mut p2),
            Player::PLAYER2 => (&mut p2, &mut p1),
        };

        let (player_action, other_player_action) = match move_result.unwrap_err() {
            SpeedError::GameWon => (ServerAction::GameWon, ServerAction::GameLost),
            _ => (ServerAction::NormalMove, ServerAction::NormalMove),
        };

        send_player_message(
            player,
            player_connection,
            other_player_connection,
            &table,
            player_action,
            other_player_action,
        )
        .await;
    }
}

async fn connect_player(listener: &TcpListener) -> Result<PlayerConnection> {
    let (stream, _) = listener.accept().await?;
    let player_stream = tokio_tungstenite::accept_async(stream).await?;
    Ok(player_stream)
}

async fn send_player_message(
    moved_player: Player,
    player_connection: &mut PlayerConnection,
    other_player_connection: &mut PlayerConnection,
    table: &SpeedTable,
    player_action: ServerAction,
    other_player_action: ServerAction,
) {
    let (_, _) = join(
        player_connection.send(Message::Text(
            serde_json::to_string(&ServerMessage {
                action: player_action,
                player_view: table.get_player_view(moved_player),
            })
            .unwrap(),
        )),
        other_player_connection.send(Message::Text(
            serde_json::to_string(&ServerMessage {
                action: other_player_action,
                player_view: table.get_player_view(moved_player.opponent()),
            })
            .unwrap(),
        )),
    )
    .await;
}

async fn wait_for_player_move(
    p1: &mut PlayerConnection,
    p2: &mut PlayerConnection,
) -> Result<(PlayerAction, Player)> {
    match select(p1.next(), p2.next()).await {
        Either::Left(m) => Ok((
            serde_json::from_str(&m.0.unwrap()?.into_text()?)?,
            Player::PLAYER1,
        )),
        Either::Right(m) => Ok((
            serde_json::from_str(&m.0.unwrap()?.into_text()?)?,
            Player::PLAYER2,
        )),
    }
}

#[cfg(test)]
mod tests {
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
