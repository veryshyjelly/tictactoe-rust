use std::collections::HashMap;

pub const X: &str = "X";
pub const O: &str = "O";

fn cell(i: i32, j: i32) -> String {
    format!("c{}", 3 * i + j)
}

pub fn player(board: &HashMap<String, String>) -> String {
    let mut x_count = 0;
    let mut o_count = 0;

    for (_, val) in board {
        if *val == X {
            x_count += 1;
        } else if *val == O {
            o_count += 1;
        }
    }

    if x_count > o_count {
        return O.to_string();
    } else {
        return X.to_string();
    }
}

pub fn actions(board: &HashMap<String, String>) -> HashMap<String, bool> {
    let mut res: HashMap<String, bool> = HashMap::new();

    for i in 0..3 {
        for j in 0..3 {
            if board[&cell(i, j)] == "" {
                res.insert(cell(i, j), true);
            }
        }
    }

    res
}

pub fn result(board: &HashMap<String, String>, action: String) -> HashMap<String, String> {
    if board[&action] != "" {
        panic!("Invalid Move")
    }

    let mut res: HashMap<String, String> = HashMap::new();
    for (k, v) in board {
        res.insert(k.to_string(), v.to_string());
    }

    res.insert(action, player(board));

    res
}

pub fn winner(board: &HashMap<String, String>) -> String {
    for i in 0..3 {
        if (board[&cell(i, 0)] == board[&cell(i, 1)])
            && (board[&cell(i, 1)] == board[&cell(i, 2)])
            && (board[&cell(i, 0)] != "")
        {
            return board[&cell(i, 0)].to_string();
        }
    }

    for i in 0..3 {
        if board[&cell(0, i)] == board[&cell(1, i)]
            && board[&cell(1, i)] == board[&cell(2, i)]
            && board[&cell(0, i)] != ""
        {
            return board[&cell(0, i)].to_string();
        }
    }

    if (board[&cell(0, 0)] == board[&cell(1, 1)])
        && (board[&cell(1, 1)] == board[&cell(2, 2)])
        && (board[&cell(0, 0)] != "")
    {
        return board[&cell(0, 0)].to_string();
    }

    if (board[&cell(0, 2)] == board[&cell(1, 1)])
        && (board[&cell(1, 1)] == board[&cell(2, 0)])
        && (board[&cell(2, 0)] != "")
    {
        return board[&cell(2, 0)].to_string();
    }

    String::from("")
}

pub fn terminal(board: &HashMap<String, String>) -> bool {
    let win = winner(board);
    if win != "" {
        return true;
    }

    for (_, val) in board {
        if val == "" {
            return false;
        }
    }

    true
}

pub fn utility(board: &HashMap<String, String>) -> i32 {
    let win = winner(board);
    if win == X {
        return 1;
    } else if win == O {
        return -1;
    }

    0
}
