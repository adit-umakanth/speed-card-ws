# Speed Card Game - WebSocket Server

This project is a server to manage players of a variant of the card game called [Speed](https://en.wikipedia.org/wiki/Spit_(card_game)). The code for the front-end client (written using Svelte) can be found [here](https://github.com/adit-umakanth/speed-card-frontend).

### Gameplay
* 1 vs 1 game
* Each player starts with 19 cards in their deck
* Player can draw and reveal top cards from deck to keep up to 4 in hand
* Two cards in the middle are flipped and revealed at the same time by players
* Players can play any card from their hand onto either middle card if the rank is one above or one below (Ace wraps around to King)
* There are no turns so reaction time and speed is crucial (hence the name of the game)
* If neither player can play a card, the middle deck is flipped at the same time once again
* Winner is the first player to discard all their cards

## Technical Implmentation

Since there are no turns and either player can make a move at any time, the WebSocket API is the ideal choice for communication between server and client:
* Players connect to server
* Server initializes a game and sends required state to both players
* Either player makes a move and it is sent to the server
* The sever validates whether the move is legal, and sends updated game state to both players
* This continues until one player finishes all their cards

## Set up and run locally
Ensure that a compatable version of [Rust](https://www.rust-lang.org/learn/get-started) is installed, and run the following commands to compile and start the server in debug mode:
```
git checkout https://github.com/adit-umakanth/speed-card-ws.git
cd speed-card-ws
cargo run
```

For the front-end component, please check out and follow the usage steps in [this repo](https://github.com/adit-umakanth/speed-card-frontend).
