# Full Stack Version of Life Path Number Calculator

You can find out your life path number and description with this simple app.

You can check out [the original app, which is a JavaScript based client](https://github.com/krondorl/life-path-number).

## Motivation

My aim is to deepen my Rust language knowledge. In this project I rewrite the client application into a full stack application, in which the Rust part acts as the backend server.

## Features

- Monorepo structure
- JavaScript frontend ([Vite](https://vitejs.dev/))
- Rust backend ([Axum](https://crates.io/crates/axum))
- JSON data
- REST API

## Technical Requirements

- Node.js
- npm
- Rust

## How to use

First you need to have [Node.js](https://nodejs.org/en) and [Rust](https://www.rust-lang.org/tools/install) installed on your computer.

1. In `client` folder run: `npm install`
1. In `server` directory execute: `cargo run`
1. In `client` folder run: `npm run dev`
1. Open a browser at `http://localhost:5173/`

## Making a build

1. In `client` folder run: `npm run build`
1. In `server` directory execute: `cargo build --release`

## License

Please see the description in [LICENSE](LICENSE).

## Release history

First version published in December, 2023.
