# Speed Card Game - WebSocket Server

This project is a server to manage players of a variant of the card game called [Speed](https://en.wikipedia.org/wiki/Spit_(card_game)). The code for the front-end client (written using Svelte) can be found [here](https://github.com/adit-umakanth/speed-card-frontend).

## Technical Implmentation

This server uses the WebSocket API to provide low-latency communication between clients since this game is not turn-based, and players can move at any time. These moves have to be reflected in the game state as quickly as possible to both players so WebSockets are an ideal choice over HTTP request polling or similar methods.

## Set up and run locally
Ensure that a compatable version of [Rust](https://www.rust-lang.org/learn/get-started) is installed, and run the following commands to compile and start the server in debug mode:
```
git checkout https://github.com/adit-umakanth/speed-card-ws.git
cd speed-card-ws
cargo run
```

For the front-end component, please check out and follow the usage steps in [this repo](https://github.com/adit-umakanth/speed-card-frontend).
