// Made by Adam Burucs in 2023.
//
// License:
// Attribution-NonCommercial-NoDerivatives 4.0 International
// (CC BY-NC-ND 4.0)

use axum::{
    extract::Path,
    extract::State,
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

#[derive(Clone)]
struct AppState {
    life_paths: Vec<LifePath>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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

async fn send_lpn(
    Path(birth_date): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<LifePath>, String> {
    let calculated_life_path = calculate_lp_number(birth_date);
    match calculated_life_path {
        Ok(lp_val) => {
            let life_path = state.life_paths[(lp_val - 1) as usize].clone();
            Ok(Json(life_path))
        }
        Err(_e) => Err(String::from("Error: life path number calculation error.")),
    }
}

async fn run_backend(state: AppState) {
    let app = Router::new()
        .route("/api/lpn-calc/:birth_date", get(send_lpn))
        .with_state(state)
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
    println!();
    println!("Life path number calculation");
    println!();
    let data = read_data();
    match data {
        Ok(ok_data) => {
            println!("Data is loaded. Starting server...");
            println!();
            run_backend(AppState {
                life_paths: ok_data,
            })
            .await;
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
