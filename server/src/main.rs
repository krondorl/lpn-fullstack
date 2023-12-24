// Made by Adam Burucs in 2023.
//
// License:
// Attribution-NonCommercial-NoDerivatives 4.0 International
// (CC BY-NC-ND 4.0)

use axum::{
    extract::Path,
    http::{self, HeaderValue, Method},
    response::Json,
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use std::fs;
use sum::sum_recursive;
use tower_http::cors::CorsLayer;
mod sum;

#[derive(Serialize, Deserialize, Debug)]
struct LifePath {
    lpn: u8,
    role: String,
    positive: String,
    negative: String,
}

fn calculate_lp_number(birth_time: String) -> Result<u32, String> {
    if birth_time.len() == 10 {
        let split_birth_time_parts = birth_time.split('-');
        let mut birth_concat = String::new();
        for part in split_birth_time_parts {
            birth_concat.push_str(part);
        }
        match birth_concat.parse::<u32>() {
            Ok(val) => Ok(sum_recursive(val)),
            Err(_) => Err(String::from("Error: converting to number.")),
        }
    } else {
        Err(String::from("Error: wrong birth time."))
    }
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

async fn send_lpn(Path(birth_date): Path<String>) -> Json<LifePath> {
    let lpn = LifePath {
        lpn: 99,
        role: String::from("test role"),
        positive: String::from("calm, collected, clever"),
        negative: String::from("bossy"),
    };
    Json(lpn)
}

async fn run_backend() {
    let app = Router::new()
        .route("/api/lpn-calc/:birth_date", get(send_lpn))
        .layer(
            CorsLayer::new()
                .allow_headers([http::header::CONTENT_TYPE])
                .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
                .allow_methods([Method::GET]),
        );
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

#[tokio::main]
async fn main() {
    let data = read_data();
    match data {
        Ok(val) => {
            let _ok_data = val;
            println!("Data is loaded. Starting server...");
            println!();
            run_backend().await;
        }
        Err(e) => println!("{e}"),
    }
}

#[cfg(test)]
mod tests {
    use crate::calculate_lp_number;

    #[test]
    fn calculate_lp_number_test() {
        assert_eq!(calculate_lp_number(String::from("1984-12-17")), Ok(6));
        assert_eq!(calculate_lp_number(String::from("2000-01-01")), Ok(4));
        assert_eq!(
            calculate_lp_number(String::from("11")),
            Err(String::from("Error: wrong birth time."))
        );
    }
}
