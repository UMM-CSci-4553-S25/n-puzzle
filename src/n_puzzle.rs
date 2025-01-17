use pathfinding::{
    matrix::{Matrix, MatrixFormatError},
    prelude::Weights,
};

use std::{fmt::Display, iter::once, num::NonZeroU8};

pub type Board = Matrix<Option<NonZeroU8>>;

pub type Pos = (usize, usize);

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct NPuzzle {
    board: Board,
    blank_position: Pos,
}

impl NPuzzle {
    pub fn new(
        size: usize,
        start_items: impl IntoIterator<Item = NonZeroU8>,
        blank_position: Pos,
    ) -> Result<Self, MatrixFormatError> {
        let blank_index = size * blank_position.0 + blank_position.1;
        let mut board_items = start_items.into_iter().map(Some);
        let mut board_pieces = board_items
            // We can't take ownership of the `board_items` iterator here, so we need to get a reference
            // to it so we can access it again in the `extend` call below.
            .by_ref()
            .take(blank_index)
            .chain(once(None))
            .collect::<Vec<_>>();

        board_pieces.extend(board_items.take(size * size - blank_index - 1));

        let board = Matrix::square_from_vec(board_pieces)?;
        Ok(NPuzzle {
            board,
            blank_position,
        })
    }

    pub fn successors(&self) -> Vec<NPuzzle> {
        // println!("{self}");
        self.board
            // Get all the positions the blank can legally move to
            .neighbours(self.blank_position, false)
            // For each new blank position, create a new `NPuzzle`
            .map(|new_blank_position| self.move_blank(new_blank_position))
            .collect::<Vec<_>>()
    }

    pub fn move_blank(&self, new_blank_position: Pos) -> Self {
        assert_ne!(self.blank_position, new_blank_position);
        let mut new_board = self.board.clone();
        new_board.swap(self.blank_position, new_blank_position);
        NPuzzle {
            board: new_board,
            blank_position: new_blank_position,
        }
    }

    pub fn successors_with_costs(&self) -> Vec<(NPuzzle, usize)> {
        self.successors()
            .into_iter()
            .map(|s| (s, 1))
            .collect::<Vec<_>>()
    }

    pub fn num_incorrect(&self) -> usize {
        // Just counting tiles that are out of place
        self.board
            .items()
            .enumerate()
            .filter(|(index, (_, value))| {
                value.is_some_and(|value| *index != (value.get() - 1) as usize)
            })
            .count()
    }

    pub fn taxicab_distance(&self) -> usize {
        self.board
            .items()
            .filter_map(|(pos, value)| value.map(|v| (pos, v.get() as usize - 1)))
            .map(|((x, y), value)| {
                let (x_val, y_val) = (value / self.board.rows(), value % self.board.rows());
                // println!("Tile at ({x}, {y}) should be at ({x_val}, {y_val})");
                x.abs_diff(x_val) + y.abs_diff(y_val)
            })
            .sum()
    }

    pub fn success(&self) -> bool {
        self.num_incorrect() == 0
    }
}

impl Display for NPuzzle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.board {
            for col in row {
                match col {
                    Some(col) => write!(f, "{col:>2} ")?,
                    None => write!(f, "-- ")?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::num::NonZeroU8;

    use super::NPuzzle;

    #[test]
    fn heuristics_when_solved() {
        let puzzle = NPuzzle::new(4, (1..16).map(|v| NonZeroU8::new(v).unwrap()), (3, 3)).unwrap();
        assert_eq!(puzzle.num_incorrect(), 0);
        assert_eq!(puzzle.taxicab_distance(), 0);
    }

    #[test]
    fn heuristics_when_not_solved() {
        let puzzle = NPuzzle::new(
            3,
            [1, 3, 7, 2, 6, 5, 4, 8].map(|v| NonZeroU8::new(v).unwrap()),
            (0, 0),
        )
        .unwrap();
        assert_eq!(puzzle.num_incorrect(), 6);
        assert_eq!(puzzle.taxicab_distance(), 8);
    }

    #[test]
    fn center_successors() {
        let puzzle = NPuzzle::new(
            3,
            [7, 8, 5, 3, 1, 4, 6, 2].map(|v| NonZeroU8::new(v).unwrap()),
            (1, 1),
        )
        .unwrap();
        let successors = puzzle.successors();
        assert_eq!(successors.len(), 4);
        assert!(successors.iter().any(|s| s.blank_position == (0, 1)));
        assert!(successors.iter().any(|s| s.blank_position == (1, 0)));
        assert!(successors.iter().any(|s| s.blank_position == (1, 2)));
        assert!(successors.iter().any(|s| s.blank_position == (2, 1)));
    }

    #[test]
    fn corner_successors() {
        let puzzle = NPuzzle::new(
            3,
            [7, 8, 5, 3, 1, 4, 6, 2].map(|v| NonZeroU8::new(v).unwrap()),
            (0, 2),
        )
        .unwrap();
        let successors = puzzle.successors();
        assert_eq!(successors.len(), 2);
        assert!(successors.iter().any(|s| s.blank_position == (0, 1)));
        assert!(successors.iter().any(|s| s.blank_position == (1, 2)));
    }
}
