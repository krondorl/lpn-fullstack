use axum::{response::Html, routing::get, Router};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
struct LifePath {
    lpn: u8,
    role: String,
    positive: String,
    negative: String,
}

fn read_data() -> Result<Vec<LifePath>, String> {
    let path = "./data.json";
    let data = fs::read_to_string(path);
    match data {
        Ok(val) => {
            let life_paths = serde_json::from_str(&val);
            match life_paths {
                Ok(val) => Ok(val),
                Err(_) => Err(String::from(
                    "Error: unable to convert from JSON to vector.",
                )),
            }
        }
        Err(_) => Err(String::from("Error: unable to read file")),
    }
}

async fn run_server() {
    let app = Router::new().route("/", get(handler));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

#[tokio::main]
async fn main() {
    let data = read_data();
    match data {
        Ok(val) => {
            let _ok_data = val;
            println!("Data is loaded. Starting server...");
            println!();
            run_server().await;
        }
        Err(e) => println!("{e}"),
    }
}
