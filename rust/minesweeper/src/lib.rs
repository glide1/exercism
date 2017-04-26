#[derive(Debug, Clone, Copy)]
enum BoardState {
    Mine,
    Count(u8),
    Empty,
}

impl BoardState {
    fn to_char(&self) -> char {
        match self {
            &Mine => '*',
            &Count(c) => (c + b'0') as char,
            &Empty => ' ',
        }
    }

    fn increase_count(&mut self) {
        *self = match self {
            &mut Count(n) => Count(n + 1),
            &mut Empty => Count(1),
            &mut Mine => Mine,
        }
    }
}

use BoardState::*;

pub fn annotate(board: &[&str]) -> Vec<String> {

    let mut count_board: Vec<Vec<BoardState>> = Vec::with_capacity(board.len());
    let mut mine_locations: Vec<(usize, usize)> = Vec::new();

    for i in 0..board.len() {
        let board_row = board[i];

        let mut row: Vec<BoardState> = Vec::with_capacity(board_row.len());
        let mut j = 0;

        for ch in board_row.chars() {
            match ch {
                '*' => {
                    row.push(Mine);
                    mine_locations.push((i, j))
                }
                _ => row.push(Empty),
            }
            j += 1
        }
        count_board.push(row);
    }

    for (row, col) in mine_locations {
        increase_mine_count(&mut count_board, row, col);
    }

    println!("count board: {:?}", count_board);

    count_board
        .iter()
        .fold(Vec::new(), |mut vec, row| {
            vec.push(row.iter().map(|cell| cell.to_char()).collect());
            vec
        })

}

fn increase_mine_count(board: &mut Vec<Vec<BoardState>>, row: usize, col: usize) {
    if row > 0 {
        if col > 0 {
            increase_mine_cell(board, row - 1, col - 1);
        }
        increase_mine_cell(board, row - 1, col);
        increase_mine_cell(board, row - 1, col + 1)
    }
    if col > 0 {
        increase_mine_cell(board, row, col - 1);
        increase_mine_cell(board, row + 1, col - 1)
    }
    increase_mine_cell(board, row, col + 1);

    increase_mine_cell(board, row + 1, col);
    increase_mine_cell(board, row + 1, col + 1);
}

fn increase_mine_cell(board: &mut Vec<Vec<BoardState>>, row: usize, col: usize) {
    if let Some(current_row) = board.get_mut(row) {
        if let Some(current_cell) = current_row.get_mut(col) {
            current_cell.increase_count();
        }
    }
}

