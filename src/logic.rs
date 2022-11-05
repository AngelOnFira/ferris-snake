use log::info;
use rand::seq::SliceRandom;
use serde_json::{json, Value};
use std::collections::{HashMap, HashSet};

use crate::{Battlesnake, Board, Game};

pub fn info() -> Value {
    info!("INFO");

    return json!({
        "apiversion": "1",
        "author": "", // TODO: Your Battlesnake Username
        "color": "#888888", // TODO: Choose color
        "head": "default", // TODO: Choose head
        "tail": "default", // TODO: Choose tail
    });
}

// start is called when your Battlesnake begins a game
pub fn start(_game: &Game, _turn: &u32, _board: &Board, _you: &Battlesnake) {
    info!("GAME START");
}

// end is called when your Battlesnake finishes a game
pub fn end(_game: &Game, _turn: &u32, _board: &Board, _you: &Battlesnake) {
    info!("GAME OVER");
}

// move is called on every turn and returns your next move
// Valid moves are "up", "down", "left", or "right"
// See https://docs.battlesnake.com/api/example-move for available data
pub fn get_move(_game: &Game, turn: &u32, board: &Board, you: &Battlesnake) -> Value {
    // Find all the unsafe places on the board
    let mut unsafe_spaces: HashSet<(u32, u32)> = board
        .snakes
        .iter()
        .map(|snake| {
            // dbg!(&snake);
            snake
                .body
                .iter()
                .map(|coord| (coord.x, coord.y))
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect();

    // Add everywhere that a snake (not us) could move to
    for snake in &board.snakes {
        if snake.id == you.id {
            continue;
        }

        let head = &snake.head;

        if head.x > 0 {
            unsafe_spaces.insert((head.x - 1, head.y));
        }

        if head.x < board.width - 1 {
            unsafe_spaces.insert((head.x + 1, head.y));
        }

        if head.y > 0 {
            unsafe_spaces.insert((head.x, head.y - 1));
        }

        if head.y < board.height - 1 {
            unsafe_spaces.insert((head.x, head.y + 1));
        }
    }

    // Add all the hazards
    for hazard in &board.hazards {
        unsafe_spaces.insert((hazard.x, hazard.y));
    }

    // Find the ones that are beside us, and put them in a vec
    let mut safe_moves = Vec::new();
    for possible_move in &[
        ("up", 0, 1),
        ("down", 0, -1),
        ("left", -1, 0),
        ("right", 1, 0),
    ] {
        let (direction, x, y) = possible_move;
        let head = &you.body[0];
        let new_x = head.x as i32 + x;
        let new_y = head.y as i32 + y;

        if new_x >= 0
            && new_x < board.width as i32
            && new_y >= 0
            && new_y < board.height as i32
            && !unsafe_spaces.contains(&(new_x as u32, new_y as u32))
        {
            safe_moves.push(direction);
        }
    }

    // Choose a random move from the safe ones
    let chosen = safe_moves.choose(&mut rand::thread_rng()).unwrap_or(&&"up");

    // TODO: Step 4 - Move towards food instead of random, to regain health and survive longer
    // let food = &board.food;

    info!("MOVE {}: {}", turn, chosen);
    return json!({ "move": chosen });
}
