//! 백트래킹을 사용한 스도쿠 해결기의 Rust 구현입니다.
//!
//! 이 모듈은 백트래킹 알고리즘을 사용하여 스도쿠 퍼즐을 해결하는 기능을 제공합니다.
//!
//! 참고: [스도쿠 백트래킹 - GeeksForGeeks](https://www.geeksforgeeks.org/sudoku-backtracking-7/)

/// 스도쿠 퍼즐을 해결합니다.
///
/// 9x9 그리드로 표현된 부분적으로 채워진 스도쿠 퍼즐이 주어지면, 이 함수는 백트래킹 알고리즘을 사용하여
/// 퍼즐을 해결하려고 시도합니다.
///
/// 해결책이 존재하면 해결된 스도쿠 보드를 반환하고, 해결책이 없으면 `None`을 반환합니다.
pub fn sudoku_solver(board: &[[u8; 9]; 9]) -> Option<[[u8; 9]; 9]> {
    let mut solver = SudokuSolver::new(*board);
    if solver.solve() {
        Some(solver.board)
    } else {
        None
    }
}

/// 스도쿠 퍼즐 해결사를 나타냅니다.
struct SudokuSolver {
    /// 9x9 그리드로 표현된 스도쿠 보드입니다.
    board: [[u8; 9]; 9],
}

impl SudokuSolver {
    /// 주어진 보드로 새 스도쿠 퍼즐 해결사를 만듭니다.
    fn new(board: [[u8; 9]; 9]) -> SudokuSolver {
        SudokuSolver { board }
    }

    /// 스도쿠 보드에서 빈 셀을 찾습니다.
    ///
    /// 빈 셀의 좌표 `(행, 열)`을 반환하거나, 모든 셀이 채워져 있으면 `None`을 반환합니다.
    fn find_empty_cell(&self) -> Option<(usize, usize)> {
        // 보드에서 빈 셀을 찾습니다 (모든 셀이 채워져 있으면 None을 반환).
        for row in 0..9 {
            for column in 0..9 {
                if self.board[row][column] == 0 {
                    return Some((row, column));
                }
            }
        }

        None
    }

    /// 주어진 값이 스도쿠 규칙에 따라 특정 셀에 배치될 수 있는지 확인합니다.
    ///
    /// 값이 셀에 배치될 수 있으면 `true`를 반환하고, 그렇지 않으면 `false`를 반환합니다.
    fn is_value_valid(&self, coordinates: (usize, usize), value: u8) -> bool {
        let (row, column) = coordinates;

        // 보드에 추가할 값이 셀에 허용되는 값인지 확인합니다.
        // 행을 통해 확인합니다.
        for current_column in 0..9 {
            if self.board[row][current_column] == value {
                return false;
            }
        }

        // 열을 통해 확인합니다.
        for current_row in 0..9 {
            if self.board[current_row][column] == value {
                return false;
            }
        }

        // 셀의 3x3 블록을 통해 확인합니다.
        let start_row = row / 3 * 3;
        let start_column = column / 3 * 3;

        for current_row in start_row..start_row + 3 {
            for current_column in start_column..start_column + 3 {
                if self.board[current_row][current_column] == value {
                    return false;
                }
            }
        }

        true
    }

    /// 백트래킹을 사용하여 스도쿠 퍼즐을 재귀적으로 해결합니다.
    ///
    /// 해결책이 발견되면 `true`를 반환하고, 그렇지 않으면 `false`를 반환합니다.
    fn solve(&mut self) -> bool {
        let empty_cell = self.find_empty_cell();

        if let Some((row, column)) = empty_cell {
            for value in 1..=9 {
                if self.is_value_valid((row, column), value) {
                    self.board[row][column] = value;
                    if self.solve() {
                        return true;
                    }
                    // 현재 구성으로 보드를 해결할 수 없는 경우 백트래킹합니다.
                    self.board[row][column] = 0;
                }
            }
        } else {
            // 보드가 완성된 경우
            return true;
        }

        // 현재 구성으로 보드를 해결할 수 없는 경우 false를 반환합니다.
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_sudoku_solver {
        ($($name:ident: $board:expr, $expected:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let result = sudoku_solver(&$board);
                    assert_eq!(result, $expected);
                }
            )*
        };
    }

    test_sudoku_solver! {
        test_sudoku_correct: [
            [3, 0, 6, 5, 0, 8, 4, 0, 0],
            [5, 2, 0, 0, 0, 0, 0, 0, 0],
            [0, 8, 7, 0, 0, 0, 0, 3, 1],
            [0, 0, 3, 0, 1, 0, 0, 8, 0],
            [9, 0, 0, 8, 6, 3, 0, 0, 5],
            [0, 5, 0, 0, 9, 0, 6, 0, 0],
            [1, 3, 0, 0, 0, 0, 2, 5, 0],
            [0, 0, 0, 0, 0, 0, 0, 7, 4],
            [0, 0, 5, 2, 0, 6, 3, 0, 0],
        ], Some([
            [3, 1, 6, 5, 7, 8, 4, 9, 2],
            [5, 2, 9, 1, 3, 4, 7, 6, 8],
            [4, 8, 7, 6, 2, 9, 5, 3, 1],
            [2, 6, 3, 4, 1, 5, 9, 8, 7],
            [9, 7, 4, 8, 6, 3, 1, 2, 5],
            [8, 5, 1, 7, 9, 2, 6, 4, 3],
            [1, 3, 8, 9, 4, 7, 2, 5, 6],
            [6, 9, 2, 3, 5, 1, 8, 7, 4],
            [7, 4, 5, 2, 8, 6, 3, 1, 9],
        ]),

        test_sudoku_incorrect: [
            [6, 0, 3, 5, 0, 8, 4, 0, 0],
            [5, 2, 0, 0, 0, 0, 0, 0, 0],
            [0, 8, 7, 0, 0, 0, 0, 3, 1],
            [0, 0, 3, 0, 1, 0, 0, 8, 0],
            [9, 0, 0, 8, 6, 3, 0, 0, 5],
            [0, 5, 0, 0, 9, 0, 6, 0, 0],
            [1, 3, 0, 0, 0, 0, 2, 5, 0],
            [0, 0, 0, 0, 0, 0, 0, 7, 4],
            [0, 0, 5, 2, 0, 6, 3, 0, 0],
        ], None::<[[u8; 9]; 9]>,
    }
}
