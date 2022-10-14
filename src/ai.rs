use std::vec;

use rand::seq::SliceRandom;

use crate::board::Board;


#[derive(PartialEq)]
pub enum Difficulty { Easy, Medium, Hard}

#[derive(Default)]
pub struct Ai {
    piece_1: char,
    piece_2: char
}

impl Ai {
    /// Place a piece [`char`] on the `board` with a minimax depth of `Difficulty`
    pub fn think(&mut self, board: &mut Board, difficulty: &Difficulty, piece: char) {
        self.piece_1 = if piece == 'X' {'O'} else {'X'};
        self.piece_2 = piece;

        // Calc best column to place in
        let (mut col, _) = self.minimax(&board, match difficulty {Difficulty::Easy => 2, Difficulty::Medium => 4, Difficulty::Hard => 5 }, i128::MIN, i128::MAX, true);

        match col {
            Some(col) => {board.place(col, self.piece_2);},
            None => {
                println!("AI found no column");
                let cols = board.get_valid_columns();
                if cols.len() > 0 {
                    col = Some(*cols.choose(&mut rand::thread_rng()).unwrap());
                    board.place(col.unwrap(), self.piece_2);
                }
            }
        } 
    }

    /// Returns an [`i32`] weight of the piece [`char`] on a given slice [`Vec<char>`] of the `Board` 
    fn evaluate_slice(&self, slice: Vec<char>, piece: char) -> i32 {
        let mut score = 0;

        if slice.iter().filter(|&n| *n == piece).count() == 4 {
            score += 100;
        } else if slice.iter().filter(|&n| *n == piece).count() == 3 && slice.iter().filter(|&n| *n == ' ').count() == 1 {
            score += 5;
        } else if slice.iter().filter(|&n| *n == piece).count() == 2 && slice.iter().filter(|&n| *n == ' ').count() == 2 {
            score += 2;
        }
        if slice.iter().filter(|&n| *n == self.piece_2).count() == 3 && slice.iter().filter(|&n| *n == ' ').count() == 1 {
            score -= 4;
        }
        
        score
    }

    /// Returns an [`i32`] of the score of a piece [`char`] 
    fn score_position(&self, board: &Board, piece: char) -> i32 {
        let mut score: i32 = 0;

        // Score center column
        let center_array = &board.rows()[3];
        let center_count: i32 = center_array.iter().filter(|&n| *n == piece).count().try_into().unwrap();
        score += center_count * 3;

        // Score Horizontal
        for r in 0..6 {
            let row_array = &board.rows()[r];
            for c in 0..4 {
                let window = &row_array[c..c+4];
                score += self.evaluate_slice(window.to_vec(), piece)
            }
        }

        // Score Vertical
        for c in 0..7 {
            let col_array = &board.columns()[c];
            for r in 0..3 {
                let window = &col_array[r..r+4];
                score += self.evaluate_slice(window.to_vec(), piece)
            }
        }

        // Score posiive sloped diagonal
        for r in 0..3 {
            for c in 0..4 {
                let mut window: Vec<char> = vec![];
                for i in 0..4 {
                    window.push(board[(r+i,c+i)] );
                }
                score += self.evaluate_slice(window, piece)
            }
        }

        for r in 0..3 {
            for c in 0..4 {
                let mut window: Vec<char> = vec![];
                for i in 0..4 {
                    window.push(board[(r+3-i,c+i)] );
                }
                score += self.evaluate_slice(window, piece)
            }
        }

        score
    }

    /// Return a [`bool`] of if the given `Board` is in winning state 
    fn is_terminal_node(&self, board: &Board) -> bool {
        board.has_won(self.piece_1) || board.has_won(self.piece_2)
    }

    /// Minimax is based on the fact that it takes into account all the possible moves that the player can take at any given time 
    /// during the game. This enables the algorithm to minimize the opponent's advantage while simultaneously maximize the 
    /// "Ai"'s advantage at every turn the Ai gets to play.
    /// 
    /// Takes the `Board` to use, along with the depth [`usize`], pruning alpha/beta values, and whether to maximize the gain
    /// for the Ai
    fn minimax(&self, board: &Board, depth: usize, alpha: i128, beta: i128, maximizing_player: bool) -> (Option<usize>, i128) {
        let valid_locations = board.get_valid_columns();
        let is_terminal = self.is_terminal_node(&board);
        if depth == 0 || is_terminal {
            if is_terminal {
                if board.has_won(self.piece_2) {
                    return (None, i128::MAX);
                } else if board.has_won(self.piece_1) {
                    return (None, i128::MIN);
                } else {
                    return (None, 0);
                }
            } else {
                return (None, self.score_position(&board, self.piece_2).into())
            }
        }

        if maximizing_player {
            let mut value = i128::MIN;
            let mut column_to_use = *valid_locations.choose(&mut rand::thread_rng()).expect("How did we get here");
            for column in valid_locations {
                let mut board_copy = board.clone();
                board_copy.place(column, self.piece_2);
                let new_score = self.minimax(&board_copy, depth - 1, alpha, beta, false).1;
                if new_score > value {
                    value = new_score.into();
                    column_to_use = column;
                }
                let alpha = alpha.max(value);
                if alpha >= beta {
                    break;
                }
            }
            return (Some(column_to_use.try_into().unwrap()), value)
        } else {
            let mut value = i128::MAX;
            let mut column_to_use = *valid_locations.choose(&mut rand::thread_rng()).expect("How did we get here");
            for column in valid_locations {
                let mut board_copy = board.clone();
                board_copy.place(column, self.piece_1);
                let new_score = self.minimax(&board_copy, depth - 1, alpha, beta, true).1;
                if new_score < value {
                    value = new_score.into();
                    column_to_use = column;
                }
                let beta = beta.min(value);
                if alpha >= beta {
                    break;
                }
            }
            return (Some(column_to_use), value)
        }
    }
}