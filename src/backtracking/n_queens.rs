//! 이 모듈은 N-퀸 문제를 해결하는 기능을 제공합니다.
//!
//! N-퀸 문제는 N개의 퀸을 NxN 체스판에 서로 위협하지 않도록 배치하는 고전적인
//! 체스판 퍼즐입니다. 퀸은 같은 행, 열 또는 대각선을 공유하면 서로 공격할 수 있습니다.
//!
//! 이 구현은 백트래킹 알고리즘을 사용하여 N-퀸 문제를 해결합니다.
//! 빈 체스판에서 시작하여 서로 충돌하지 않도록 다른 행에 퀸을 반복적으로 배치하려고 시도합니다.
//! 유효한 해결책이 발견되면 해결책 목록에 추가됩니다.

/// 주어진 크기에 대해 N-퀸 문제를 해결하고 해결책 벡터를 반환합니다.
///
/// # 인수
///
/// * `n` - 체스판의 크기 (NxN)입니다.
///
/// # 반환 값
///
/// N-퀸 문제에 대한 모든 해결책을 포함하는 벡터입니다.
pub fn n_queens_solver(n: usize) -> Vec<Vec<String>> {
    let mut solver = NQueensSolver::new(n);
    solver.solve()
}

/// N-퀸 문제 해결사를 나타냅니다.
struct NQueensSolver {
    // 체스판의 크기
    size: usize,
    // '.'은 빈 공간을, 'Q'는 퀸을 나타내는 체스판을 나타내는 2D 벡터
    board: Vec<Vec<char>>,
    // 모든 유효한 해결책을 저장하는 벡터
    solutions: Vec<Vec<String>>,
}

impl NQueensSolver {
    /// 주어진 크기로 새 `NQueensSolver` 인스턴스를 만듭니다.
    ///
    /// # 인수
    ///
    /// * `size` - 체스판의 크기 (N×N)입니다.
    ///
    /// # 반환 값
    ///
    /// 새 `NQueensSolver` 인스턴스입니다.
    fn new(size: usize) -> Self {
        NQueensSolver {
            size,
            board: vec![vec!['.'; size]; size],
            solutions: Vec::new(),
        }
    }

    /// N-퀸 문제를 해결하고 해결책 벡터를 반환합니다.
    ///
    /// # 반환 값
    ///
    /// N-퀸 문제에 대한 모든 해결책을 포함하는 벡터입니다.
    fn solve(&mut self) -> Vec<Vec<String>> {
        self.solve_helper(0);
        std::mem::take(&mut self.solutions)
    }

    /// 지정된 위치 (행, 열)에 퀸을 배치하는 것이 안전한지 확인합니다.
    ///
    /// # 인수
    ///
    /// * `row` - 확인할 위치의 행 인덱스입니다.
    /// * `col` - 확인할 위치의 열 인덱스입니다.
    ///
    /// # 반환 값
    ///
    /// 지정된 위치에 퀸을 배치하는 것이 안전하면 `true`이고, 그렇지 않으면 `false`입니다.
    fn is_safe(&self, row: usize, col: usize) -> bool {
        // 열 및 대각선 확인
        for i in 0..row {
            if self.board[i][col] == 'Q'
                || (col >= row - i && self.board[i][col - (row - i)] == 'Q')
                || (col + row - i < self.size && self.board[i][col + (row - i)] == 'Q')
            {
                return false;
            }
        }
        true
    }

    /// N-퀸 문제를 해결하기 위한 재귀 헬퍼 함수입니다.
    ///
    /// # 인수
    ///
    /// * `row` - 현재 처리 중인 행입니다.
    fn solve_helper(&mut self, row: usize) {
        if row == self.size {
            self.solutions
                .push(self.board.iter().map(|row| row.iter().collect()).collect());
            return;
        }

        for col in 0..self.size {
            if self.is_safe(row, col) {
                self.board[row][col] = 'Q';
                self.solve_helper(row + 1);
                self.board[row][col] = '.';
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_n_queens_solver {
        ($($name:ident: $tc:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (n, expected_solutions) = $tc;
                    let solutions = n_queens_solver(n);
                    assert_eq!(solutions, expected_solutions);
                }
            )*
        };
    }

    test_n_queens_solver! {
        test_0_queens: (0, vec![Vec::<String>::new()]),
        test_1_queen: (1, vec![vec!["Q"]]),
        test_2_queens:(2, Vec::<Vec<String>>::new()),
        test_3_queens:(3, Vec::<Vec<String>>::new()),
        test_4_queens: (4, vec![
            vec![".Q..",
                 "...Q",
                 "Q...",
                 "..Q."],
            vec!["..Q.",
                 "Q...",
                 "...Q",
                 ".Q.."],
        ]),
        test_5_queens:(5, vec![
            vec!["Q....",
                 "..Q..",
                 "....Q",
                 ".Q...",
                 "...Q."],
            vec!["Q....",
                 "...Q.",
                 ".Q...",
                 "....Q",
                 "..Q.."],
            vec![".Q...",
                 "...Q.",
                 "Q....",
                 "..Q..",
                 "....Q"],
            vec![".Q...",
                 "....Q",
                 "..Q..",
                 "Q....",
                 "...Q."],
            vec!["..Q..",
                 "Q....",
                 "...Q.",
                 ".Q...",
                 "....Q"],
            vec!["..Q..",
                 "....Q",
                 ".Q...",
                 "...Q.",
                 "Q...."],
            vec!["...Q.",
                 "Q....",
                 "..Q..",
                 "....Q",
                 ".Q..."],
            vec!["...Q.",
                 ".Q...",
                 "....Q",
                 "..Q..",
                 "Q...."],
            vec!["....Q",
                 ".Q...",
                 "...Q.",
                 "Q....",
                 "..Q.."],
            vec!["....Q",
                 "..Q..",
                 "Q....",
                 "...Q.",
                 ".Q..."],
        ]),
        test_6_queens: (6, vec![
            vec![".Q....",
                 "...Q..",
                 ".....Q",
                 "Q.....",
                 "..Q...",
                 "....Q."],
            vec!["..Q...",
                 ".....Q",
                 ".Q....",
                 "....Q.",
                 "Q.....",
                 "...Q.."],
            vec!["...Q..",
                 "Q.....",
                 "....Q.",
                 ".Q....",
                 ".....Q",
                 "..Q..."],
            vec!["....Q.",
                 "..Q...",
                 "Q.....",
                 ".....Q",
                 "...Q..",
                 ".Q...."],
        ]),
    }
}
