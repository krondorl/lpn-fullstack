![Numerology icon](/client/img/numerology.png "Numerology icon")

# Full Stack Version of Life Path Number Calculator

Find out your life path number and description with this simple app.

You can check out [the original app, a JavaScript-based client](https://github.com/krondorl/life-path-number).

## App screenshots

![App screenshot in browser](/docs/lpn-app-screen-in-browser.png "App screenshot in browser")

## Motivation

I aim to deepen my Rust language knowledge. In this project, I rewrote the client application into a full stack application, in which the Rust part acts as the backend server.

## What Is a Life Path Number?

A life path number is a number associated with intuitive meanings.

This result can help someone:

- Increase self-knowledge
- Find out strengths and weaknesses
- Think about life task/life mission

## Algorithm

1. The calculation starts with the birth date.
1. We need to sum all numbers.
1. We sum the numbers recursively until we get a single digit number.

```
Example date

1900-02-18

(1 + 9 + 0 + 0) + (0 + 2) + (1 + 8)
10 + 2 + 9
21
3

So the life path number is 3.

```

## Features

- Monorepo structure
- JavaScript frontend ([Vite](https://vitejs.dev/))
- Rust backend ([Axum](https://crates.io/crates/axum))
- JSON data
- REST API

## Technical Requirements

- Node.js >= v20.9.0
- Rust >= v1.74.1

## How to Use

### Run with Docker

To run the complete projects, [you will need Docker Desktop running.](https://www.docker.com/products/docker-desktop/)

1. Open a Terminal
1. Go to the source folder
1. Execute the command: `docker compose up`
1. Open browser at http://localhost:3050

### Run with Node.js and Rust

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

### Running Examples

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

## REST API

Only `GET` method is available for calculation.

Example

```
Request

GET http://localhost:8080/api/lpn-calc/2000-10-10

Response

{
  "lpn": 4,
  "role": "builder",
  "positive": "loyal, reliable, determined, disciplined",
  "negative": "stubborn, bossy, dominant, too focused on details"
}

```

Test response using [Postman app](https://www.postman.com/downloads/).

![GET request in Postman](/docs/get-endpoint-postman.png "GET request in Postman")

## Making a Build

1. In the `client` folder, run: `npm run build`
1. In the `server` directory, execute: `cargo build --release`

## License

Please see the description in [LICENSE](LICENSE).

## Release History

I published the first version in December 2023.
