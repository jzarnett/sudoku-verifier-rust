#![feature(decl_macro)]
#[macro_use]
extern crate rocket;

use async_std::task;
use rand::Rng;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};
use tokio::time::Duration;

const PUZZLE_SIZE: usize = 9;
const NANOS_TO_MS: u32 = 1000000;

#[derive(Deserialize)]
struct Sudoku {
    content: [[u8; PUZZLE_SIZE]; PUZZLE_SIZE]
}

#[derive(Serialize)]
struct Response {
    body: String
}

fn check_puzzle(p: Sudoku) -> bool {
    let mut rows = vec![[0u32; (PUZZLE_SIZE + 1)]; PUZZLE_SIZE];
    let mut cols = vec![[0u32; (PUZZLE_SIZE + 1)]; PUZZLE_SIZE];
    let mut grid = vec![[[0u32; (PUZZLE_SIZE + 1)]; 3]; 3];

    for i in 0..PUZZLE_SIZE {
        for j in 0..PUZZLE_SIZE {
            let num = p.content[i][j] as usize;
            if num <= 0 || num > PUZZLE_SIZE {
                return false;
            }

            if rows[i][num] < 1 {
                rows[i][num] = rows[i][num] + 1
            } else {
                return false;
            }
            if cols[j][num] < 1 {
                cols[j][num] = cols[j][num] + 1
            } else {
                return false;
            }

            if grid[i / 3][j / 3][num] < 1 {
                grid[i / 3][j / 3][num] = grid[i / 3][j / 3][num] + 1
            } else {
                return false;
            }
        }
    }
    return true;
}

#[post("/verify", format = "application/json", data = "<sudoku>")]
async fn verify_sudoku(sudoku: Json<Sudoku>) -> String {
    sleep_time().await;

    return if check_puzzle(sudoku.into_inner()) {
        String::from("1")
    } else {
        String::from("0")
    }
}

async fn sleep_time() {
    let mut rng = rand::rngs::OsRng::default();
    let sleep_time = rng.gen_range(25..250);
    task::sleep(Duration::new(0, sleep_time * NANOS_TO_MS)).await;
}

#[rocket::main]
async fn main() {
    rocket::ignite()
        .mount("/", routes![verify_sudoku])
        .launch()
        .await;
}