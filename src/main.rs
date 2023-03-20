use std::{collections::HashMap, cell::RefCell};

use rocket::{self, routes, get, fs::FileServer, post, serde::json::Json};
use serde::{Deserialize, Serialize};

mod ai;
use ai::{minmax, tictactoe::{terminal, winner, player, result}};

#[derive(Deserialize, Serialize)]
struct Move {
    is_terminated: bool,
    winner: String,
    player: String,
    moove: i32
}

impl Move {
    fn new() -> Move {
        Move { 
            is_terminated: false, 
            winner: String::from(""),
            player: String::from(""),
            moove: 0
        }
    }
}

thread_local! (static BOARD: RefCell<HashMap<String, String>> = RefCell::new(HashMap::new()));

#[post("/updateBoard", data = "<bb>")]
async fn update_board(bb: Json<HashMap<String, String>>) {
    BOARD.with(|board| {
        let mut x = board.borrow_mut();
        for k in bb.keys() {
            x.insert(k.to_string(), bb.get(k).unwrap().to_string()); 
        }
    }) 
}

#[get("/getMove")]
async fn get_move() -> Json<Move> {
    let mut res = Move::new();

    BOARD.with(|bb| {
        let board = bb.borrow().clone();
        if terminal(&board) {
            res.moove = -1;
            let win = winner(&board);
            if win != "" {
                res.winner = win;
            } else {
                res.winner = String::from("draw");
            }
            res.is_terminated = true;
        } else {
            let moove = minmax(&board);
            res.moove = moove.chars().nth(1).unwrap().to_string().parse::<i32>().unwrap();
            res.player = player(&board);
            if terminal(&result(&board, moove.to_string())) {
                res.is_terminated = true;
                let win = winner(&result(&board, moove));
                if win != "" {
                    res.winner = win;
                } else {
                    res.winner = String::from("draw");
                }
            }
        }
    });

    Json(res)
}


#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    // let board: HashMap<String, String> = HashMap::new();

    let _rocket = rocket::build()
    .mount("/", FileServer::from("ui/"))
    .mount("/", routes![update_board, get_move])
    .launch().await?;
    Ok(()) 
}
