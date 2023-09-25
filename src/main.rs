mod game_logic;
mod player_action;
use player_action::*;
mod server_message;
use server_message::*;

use anyhow::Result;
use futures_util::{
    future::{join, select, Either, Join},
    stream::{SplitSink, SplitStream},
    Future, SinkExt, StreamExt,
};
use game_logic::{Player, SpeedTable};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{tungstenite::Message, WebSocketStream};

type Sender = SplitSink<WebSocketStream<TcpStream>, Message>;
type Receiver = SplitStream<WebSocketStream<TcpStream>>;
type PlayerConnection = (Sender, Receiver);

#[tokio::main]
async fn main() -> Result<()> {
    let addr = "0.0.0.0:8080".to_string();
    let listener = TcpListener::bind(&addr).await?;

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
        &mut p1.0,
        &mut p2.0,
        &table,
        ServerAction::SetBoard,
        ServerAction::SetBoard,
    )
    .await;

    loop {
        let (player_move, player) = wait_for_player_move(&mut p1.1, &mut p2.1).await?;

        let move_result = match player_move {
            PlayerAction::DrawCard => table.player_draw_card(player),
            PlayerAction::Flip => table.flip_middle_cards(),
            PlayerAction::PlaceCard(hand_index, side) => table.place_card(player, side, hand_index),
        };
        send_player_message(
            &mut p1.0,
            &mut p2.0,
            &table,
            ServerAction::SetBoard,
            ServerAction::SetBoard,
        )
        .await;
    }
}

async fn connect_player(listener: &TcpListener) -> Result<PlayerConnection> {
    let (stream, _) = listener.accept().await?;
    let player_stream = tokio_tungstenite::accept_async(stream).await?;
    Ok(player_stream.split())
}

async fn send_player_message(
    player: &mut Sender,
    other_player: &mut Sender,
    table: &SpeedTable,
    player_action: ServerAction,
    other_player_action: ServerAction,
) {
    let (_, _) = join(
        player.send(Message::Text(
            serde_json::to_string(&ServerMessage {
                action: player_action,
                player_view: table.get_player_view(Player::PLAYER1),
            })
            .unwrap(),
        )),
        other_player.send(Message::Text(
            serde_json::to_string(&ServerMessage {
                action: other_player_action,
                player_view: table.get_player_view(Player::PLAYER2),
            })
            .unwrap(),
        )),
    )
    .await;
}

async fn wait_for_player_move(
    p1: &mut Receiver,
    p2: &mut Receiver,
) -> Result<(PlayerAction, Player)> {
    match select(p1.next(), p2.next()).await {
        Either::Left(m) => Ok((
            serde_json::from_str(&m.0.unwrap().unwrap().into_text()?)?,
            Player::PLAYER1,
        )),
        Either::Right(m) => Ok((
            serde_json::from_str(&m.0.unwrap().unwrap().into_text()?)?,
            Player::PLAYER2,
        )),
    }
}
