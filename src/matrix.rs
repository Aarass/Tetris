use std::fmt::Display;

use crate::pieces::Table;
use bevy::ecs::resource::Resource;

use crate::pieces::PieceIndicies;

#[derive(Resource)]
pub struct Matrix {
    elements: Vec<Vec<u8>>,
}

const MIN_WIDTH: usize = 7;
const MIN_HEIGHT: usize = 10;

impl Matrix {
    pub fn try_new(width: usize, height: usize) -> Option<Self> {
        if width < MIN_WIDTH {
            return None;
        }

        if height < MIN_HEIGHT {
            return None;
        };

        Some(Matrix {
            elements: vec![vec![0u8; width]; height],
        })
    }

    fn height(&self) -> usize {
        self.elements.len()
    }

    fn width(&self) -> usize {
        self.elements.first().unwrap().len()
    }

    fn get(&self, row: usize, col: usize) -> u8 {
        self.elements[row][col]
    }

    fn set(&mut self, row: usize, col: usize) {
        self.elements[row][col] = 1;
    }

    fn clear(&mut self, row: usize, col: usize) {
        self.elements[row][col] = 0;
    }
}

impl Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.height() {
            for j in 0..self.width() {
                write!(f, "{} ", self.get(i, j))?
            }
            write!(f, "\n")?
        }
        Ok(())
    }
}

pub fn check_for_colision(matrix: &Matrix, table: &Table, piece_indicies: &PieceIndicies) -> bool {
    let width = matrix.width();
    let height = matrix.height();

    for i in (0..=3).rev() {
        for j in 0..=3 {
            if table[i][j] == 0 {
                continue;
            }

            let row = piece_indicies.i + i as i32;
            let col = piece_indicies.j + j as i32;

            if row < 0 || row as usize >= height {
                return true;
            }

            if col < 0 || col as usize >= width {
                return true;
            }

            let row = row as usize;
            let col = col as usize;

            if matrix.get(row, col) == 1 {
                return true;
            }
        }
    }

    return false;
}

pub fn fix_piece(matrix: &mut Matrix, table: &Table, piece_indicies: &PieceIndicies) {
    for i in 0..=3 {
        for j in 0..=3 {
            if table[i][j] == 1 {
                matrix.set(piece_indicies.i as usize + i, piece_indicies.j as usize + j);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        consts::{COLS, ROWS},
        matrix::{Matrix, check_for_colision},
        pieces::PieceIndicies,
    };

    #[test]
    fn test_1() {
        let matrix = Matrix::try_new(COLS as usize, ROWS as usize).unwrap();
        let table = [[1, 1, 0, 0], [1, 1, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]];
        let indicies = PieceIndicies { i: 0, j: 0 };

        let collided = check_for_colision(&matrix, &table, &indicies);

        assert!(collided == false);
    }

    #[test]
    fn test_2() {
        let mut matrix = Matrix::try_new(COLS as usize, ROWS as usize).unwrap();
        let table = [[1, 1, 0, 0], [1, 1, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]];
        let indicies = PieceIndicies { i: 0, j: 1 };

        matrix.set(0, 1);

        let collided = check_for_colision(&matrix, &table, &indicies);

        assert!(collided == true);
    }

    #[test]
    fn test_3() {
        let mut matrix = Matrix::try_new(COLS as usize, ROWS as usize).unwrap();
        let table = [[1, 1, 0, 0], [1, 1, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]];
        let indicies = PieceIndicies { i: 3, j: 3 };

        matrix.set(2, 2);
        matrix.set(3, 2);
        matrix.set(4, 2);
        matrix.set(5, 2);
        matrix.set(5, 3);
        matrix.set(5, 4);
        matrix.set(5, 5);
        matrix.set(4, 5);
        matrix.set(3, 5);
        matrix.set(2, 5);
        matrix.set(2, 4);
        matrix.set(2, 3);

        let collided = check_for_colision(&matrix, &table, &indicies);

        assert!(collided == false);
    }
}
