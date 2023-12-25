![Numerology icon](/client/img/numerology.png "Numerology icon")

# Full Stack Version of Life Path Number Calculator

Find out your life path number and description with this simple app.

You can check out [the original app, a JavaScript-based client](https://github.com/krondorl/life-path-number).

## What is a life path number?

A life path number is a number associated with intuitive meanings.

This result can help someone:

- Increase self-knowledge
- Find out strengths and weaknesses
- Think about life task/life mission

## Motivation

I aim to deepen my Rust language knowledge. In this project, I rewrote the client application into a full stack application, in which the Rust part acts as the backend server.

## Features

- Monorepo structure
- JavaScript frontend ([Vite](https://vitejs.dev/))
- Rust backend ([Axum](https://crates.io/crates/axum))
- JSON data
- REST API

## Technical Requirements

- Node.js >= v20.9.0
- Rust >= v1.74.1

## How to use

First, you need to have [Node.js](https://nodejs.org/en) and [Rust](https://www.rust-lang.org/tools/install) installed on your computer.

Open two terminal tabs:

- Windows: run `Windows Terminal`, `cmd`, or `PowerShell`.
- Mac: run `Terminal`.
- Linux: run `Terminal`.

Then follow these steps:

1. In the `client` folder, run: `npm install`
1. In the `server` directory, execute: `cargo run`
1. In the `client` folder, run: `npm run dev`
1. Open a browser at `http://localhost:5173/`

### Running examples

This is how it looks to run the client and server from the built-in console/terminal in `VS Code` on `Windows 11`.

```
PS F:\Dev\lpn-fullstack\client> npm run dev

> client@0.1.0 dev
> vite

  VITE v5.0.10  ready in 764 ms

  ➜  Local:   http://localhost:5173/
  ➜  Network: use --host to expose
  ➜  press h + enter to show help
```

```
PS F:\Dev\lpn-fullstack\server> cargo run
   Compiling server v0.1.0 (F:\Dev\lpn-fullstack\server)
    Finished dev [unoptimized + debuginfo] target(s) in 20.65s
     Running `target\debug\server.exe`

Life path number calculation

Data is loaded. Starting server...

listening on 127.0.0.1:8080
```

## Making a build

1. In the `client` folder, run: `npm run build`
1. In the `server` directory, execute: `cargo build --release`

## License

Please see the description in [LICENSE](LICENSE).

## Release history

I published the first version in December 2023.
