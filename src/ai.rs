use std::cmp::{max, min};
use std::collections::HashMap;

pub mod tictactoe;
use tictactoe::*;

pub fn minmax(board: &HashMap<String, String>) -> String {
    if terminal(board) {
        return String::from("");
    }

    return if player(board) == X {
        let (mut max_action_value, mut max_action) = (-1, String::from(""));
        for (action, _) in actions(&board) {
            let v = min_value(&result(&board, action.to_string()));
            if v > max_action_value {
                max_action = action;
                max_action_value = v;
            }
        }
        max_action
    } else {
        let (mut min_action_value, mut min_action) = (1, String::from(""));
        for (action, _) in actions(board) {
            let v = min_value(&result(board, action.to_string()));
            if v < min_action_value {
                min_action = action;
                min_action_value = v;
            }
        }
        min_action
    };
}

fn max_value(board: &HashMap<String, String>) -> i32 {
    if terminal(board) {
        return utility(board);
    }
    let mut v = i32::MIN;
    for (action, _) in actions(&board) {
        v = max(v, min_value(&result(board, action)));
    }
    v
}

fn min_value(board: &HashMap<String, String>) -> i32 {
    if terminal(&board) {
        return utility(&board);
    }

    let mut v = i32::MAX;
    for (action, _) in actions(&board) {
        v = min(v, max_value(&result(board, action)));
    }
    v
}
