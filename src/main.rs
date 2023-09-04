mod game_logic;
mod player_action;
use player_action::*;
mod server_message;

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
type Reciever = SplitStream<WebSocketStream<TcpStream>>;

#[tokio::main]
async fn main() -> Result<()> {
    let mut table = SpeedTable::new();
    table.flip_middle_cards();
    table.player_draw_card(Player::PLAYER1);
    table.player_draw_card(Player::PLAYER1);
    table.player_draw_card(Player::PLAYER1);
    table.player_draw_card(Player::PLAYER1);

    table.player_draw_card(Player::PLAYER2);
    table.player_draw_card(Player::PLAYER2);
    table.player_draw_card(Player::PLAYER2);
    table.player_draw_card(Player::PLAYER2);

    let addr = "127.0.0.1:8080".to_string();
    let listener = TcpListener::bind(&addr).await?;

    let (mut p1_tx, mut p1_rx) = connect_player(&listener).await?;
    println!("Player 1 connected!");
    let (mut p2_tx, mut p2_rx) = connect_player(&listener).await?;
    println!("Player 2 connected!");

    send_player_view(&mut p1_tx, &mut p2_tx, &table).await;

    loop {
        let (player_move, player) = wait_for_player_move(&mut p1_rx, &mut p2_rx).await?;
        println!("{:#?} by {:#?}", player_move, player);

        match player_move {
            PlayerAction::DrawCard => {
                table.player_draw_card(player);
            }
            PlayerAction::Flip => {
                table.flip_middle_cards();
            }
            PlayerAction::PlaceCard(hand_index, side) => {
                table.place_card(player, side, hand_index);
            }
        }
        send_player_view(&mut p1_tx, &mut p2_tx, &table).await;
    }

    Ok(())
}

async fn connect_player(listener: &TcpListener) -> Result<(Sender, Reciever)> {
    let (stream, _) = listener.accept().await?;
    let player_stream = tokio_tungstenite::accept_async(stream).await?;
    Ok(player_stream.split())
}

async fn send_player_view(p1: &mut Sender, p2: &mut Sender, table: &SpeedTable) {
    let (_, _) = join(
        p1.send(Message::Text(
            serde_json::to_string(&table.get_player_view(Player::PLAYER1)).unwrap(),
        )),
        p2.send(Message::Text(
            serde_json::to_string(&table.get_player_view(Player::PLAYER2)).unwrap(),
        )),
    )
    .await;
}

async fn wait_for_player_move(
    p1: &mut Reciever,
    p2: &mut Reciever,
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
