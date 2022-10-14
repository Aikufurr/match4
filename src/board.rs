use std::{ops::{Index, IndexMut}, vec};

use array2d::Array2D;

pub struct Board {
    cells: Array2D<char>,
    current_char: char
}

impl Default for Board {
    fn default() -> Self {
        Self {
            cells: Array2D::filled_with(' ', 6, 7),
            current_char: 'O'
        }
    }
}

impl Index<(usize, usize)> for Board {
    type Output = char;

    fn index<'a>(&'a self, _index: (usize, usize)) -> &'a char {
        &self.cells[_index]
    }
}

impl Board {
    /// Returns a [`bool`] if placing a piece [`char`] in column [`usize`] was successful
    pub fn place(&mut self, column: usize, piece: char) -> bool{
        if self.cells[(0, column)] != ' ' {
            println!("Column full");
            false
        } else {        
            for row in (0..6).rev() {
                if self.cells[(row, column)] == ' ' {
                    // Get mut cell at (row,column), dereference it, assign new piece
                    *self.cells.index_mut((row,column)) =  piece;
                    break;
                }
            }
            true
        }
    }

    /// Collects the [`Array2D`] into a [`Vec`] of rows, each of which contains a [`Vec`] of elements.
    pub fn rows(&self) -> Vec<Vec<char>> {
        self.cells.as_rows()
    }
    /// Collects the [`Array2D`] into a [`Vec`] of columns, each of which contains a [`Vec`] of elements.
    pub fn columns(&self) -> Vec<Vec<char>> {
        self.cells.as_columns()
    }

    /// Returns a copy of `Board`
    pub fn clone(&self) -> Board {
        Board {
            cells: self.cells.clone(),
            current_char: self.current_char.clone()
        }
    }

    /// Returns a [`Vec<usize>`] of columns that are not full
    pub fn get_valid_columns(&self)  -> Vec<usize> {
        let mut columns: Vec<usize> = vec![];
        for column in 0..7 {
            if self.cells[(0, column)] == ' ' {
                columns.push(column);
            }
        }
        columns
    }

    /// Returns a [`bool`] if piece [`char`] as four in a row
    pub fn has_won(&self, piece: char) -> bool {
        // Vertical Check 
        for i in 0..3 {
            for j in 0..7 {
                if self.cells[(i,j)] == piece && self.cells[(i+1,j)] == piece && self.cells[(i+2,j)] == piece && self.cells[(i+3,j)] == piece {
                    return true;
                }           
            }
        }
        // Horizontal Check
        for i in 0..6 {
            for j in 0..4 {
                if self.cells[(i,j)] == piece && self.cells[(i,j+1)] == piece && self.cells[(i,j+2)] == piece && self.cells[(i,j+3)] == piece {
                    return true;
                }           
            }
        }
        // Down-Right Check
        for i in 0..3 {
            for j in 0..4 {
                if self.cells[(i,j)] == piece && self.cells[(i+1,j+1)] == piece && self.cells[(i+2,j+2)] == piece && self.cells[(i+3,j+3)] == piece {
                    return true;
                }           
            }
        }
        // Down-Left Check
        for i in 3..6 {
            for j in 0..4 {
                if self.cells[(i,j)] == piece && self.cells[(i-1,j+1)] == piece && self.cells[(i-2,j+2)] == piece && self.cells[(i-3,j+3)] == piece {
                    return true;
                }           
            }
        }
        return false;
    }

}