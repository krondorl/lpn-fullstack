![Numerology icon](/client/img/numerology.png "Numerology icon")

# Full Stack Version of Life Path Number Calculator

You can find out your life path number and description with this simple app.

You can check out [the original app, which is a JavaScript based client](https://github.com/krondorl/life-path-number).

## Motivation

My aim is to deepen my Rust language knowledge. In this project I rewrote the client application into a full stack application, in which the Rust part acts as the backend server.

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

First you need to have [Node.js](https://nodejs.org/en) and [Rust](https://www.rust-lang.org/tools/install) installed on your computer.

Open two terminal tabs:

- Windows: run `Windows Terminal` or `cmd` or `powershell`.
- Mac: run `Terminal`.
- Linux: run `Terminal`.

Then follow these steps:

1. In `client` folder run: `npm install`
1. In `server` directory execute: `cargo run`
1. In `client` folder run: `npm run dev`
1. Open a browser at `http://localhost:5173/`

### Running examples

This is how it looks running the client and server from built in console / terminal in `VS Code` on `Windows 11`.

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

1. In `client` folder run: `npm run build`
1. In `server` directory execute: `cargo build --release`

## License

Please see the description in [LICENSE](LICENSE).

## Release history

First version published in December, 2023.
