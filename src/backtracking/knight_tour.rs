//! 이 모듈은 나이트 투어 문제의 구현을 포함합니다.
//!
//! 나이트 투어는 체스판의 모든 칸을 나이트가 정확히 한 번씩 방문하는 것을 목표로 하는 고전적인 체스 문제입니다.

/// 지정된 위치에서 시작하는 나이트 투어를 찾습니다.
///
/// # 인수
///
/// * `size_x` - 체스판의 너비입니다.
/// * `size_y` - 체스판의 높이입니다.
/// * `start_x` - 시작 위치의 x 좌표입니다.
/// * `start_y` - 시작 위치의 y 좌표입니다.
///
/// # 반환 값
///
/// 투어가 발견되면 투어 행렬을 반환하고, 그렇지 않으면 None을 반환합니다.
/// 반환되는 투어 행렬은 본질적으로 `KnightTour` 구조체의 `board` 필드인
/// `Vec<Vec<usize>>`입니다. 이는 나이트가 체스판에서 수행한 이동 순서를 나타내며,
/// 각 셀에는 나이트가 해당 칸을 방문한 순서가 포함됩니다.
pub fn find_knight_tour(
    size_x: usize,
    size_y: usize,
    start_x: usize,
    start_y: usize,
) -> Option<Vec<Vec<usize>>> {
    let mut tour = KnightTour::new(size_x, size_y);
    tour.find_tour(start_x, start_y)
}

/// 나이트 투어 문제를 구현하는 KnightTour 구조체를 나타냅니다.
struct KnightTour {
    board: Vec<Vec<usize>>,
}

impl KnightTour {
    /// 체스판에서 나이트가 가능한 이동
    const MOVES: [(isize, isize); 8] = [
        (2, 1),
        (1, 2),
        (-1, 2),
        (-2, 1),
        (-2, -1),
        (-1, -2),
        (1, -2),
        (2, -1),
    ];

    /// 주어진 체스판 크기로 새 KnightTour 인스턴스를 생성합니다.
    /// # 인수
    ///
    /// * `size_x` - 체스판의 너비입니다.
    /// * `size_y` - 체스판의 높이입니다.
    ///
    /// # 반환 값
    ///
    /// 새 KnightTour 인스턴스입니다.
    fn new(size_x: usize, size_y: usize) -> Self {
        let board = vec![vec![0; size_x]; size_y];
        KnightTour { board }
    }

    /// 체스판의 너비를 반환합니다.
    fn size_x(&self) -> usize {
        self.board.len()
    }

    /// 체스판의 높이를 반환합니다.
    fn size_y(&self) -> usize {
        self.board[0].len()
    }

    /// 주어진 위치로 이동하는 것이 안전한지 확인합니다.
    ///
    /// # 인수
    ///
    /// * `x` - 위치의 x 좌표입니다.
    /// * `y` - 위치의 y 좌표입니다.
    ///
    /// # 반환 값
    ///
    /// 위치로 이동하는 것이 안전한지 여부를 나타내는 부울 값입니다.
    fn is_safe(&self, x: isize, y: isize) -> bool {
        x >= 0
            && y >= 0
            && x < self.size_x() as isize
            && y < self.size_y() as isize
            && self.board[x as usize][y as usize] == 0
    }

    /// 나이트 투어 문제를 재귀적으로 해결합니다.
    ///
    /// # 인수
    ///
    /// * `x` - 나이트의 현재 x 좌표입니다.
    /// * `y` - 나이트의 현재 y 좌표입니다.
    /// * `move_count` - 현재 이동 횟수입니다.
    ///
    /// # 반환 값
    ///
    /// 해결책을 찾았는지 여부를 나타내는 부울 값입니다.
    fn solve_tour(&mut self, x: isize, y: isize, move_count: usize) -> bool {
        if move_count == self.size_x() * self.size_y() {
            return true;
        }
        for &(dx, dy) in &Self::MOVES {
            let next_x = x + dx;
            let next_y = y + dy;

            if self.is_safe(next_x, next_y) {
                self.board[next_x as usize][next_y as usize] = move_count + 1;

                if self.solve_tour(next_x, next_y, move_count + 1) {
                    return true;
                }
                // 백트래킹
                self.board[next_x as usize][next_y as usize] = 0;
            }
        }

        false
    }

    /// 지정된 위치에서 시작하는 나이트 투어를 찾습니다.
    ///
    /// # 인수
    ///
    /// * `start_x` - 시작 위치의 x 좌표입니다.
    /// * `start_y` - 시작 위치의 y 좌표입니다.
    ///
    /// # 반환 값
    ///
    /// 투어가 발견되면 투어 행렬을 반환하고, 그렇지 않으면 None을 반환합니다.
    fn find_tour(&mut self, start_x: usize, start_y: usize) -> Option<Vec<Vec<usize>>> {
        if !self.is_safe(start_x as isize, start_y as isize) {
            return None;
        }

        self.board[start_x][start_y] = 1;

        if !self.solve_tour(start_x as isize, start_y as isize, 1) {
            return None;
        }

        Some(self.board.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_find_knight_tour {
        ($($name:ident: $tc:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (size_x, size_y, start_x, start_y, expected) = $tc;
                if expected.is_some() {
                    assert_eq!(expected.clone().unwrap()[start_x][start_y], 1)
                }
                assert_eq!(find_knight_tour(size_x, size_y, start_x, start_y), expected);
            }
        )*
        }
    }
    test_find_knight_tour! {
        test_knight_tour_5x5: (5, 5, 0, 0, Some(vec![
            vec![1, 6, 15, 10, 21],
            vec![14, 9, 20, 5, 16],
            vec![19, 2, 7, 22, 11],
            vec![8, 13, 24, 17, 4],
            vec![25, 18, 3, 12, 23],
        ])),
        test_knight_tour_6x6: (6, 6, 0, 0, Some(vec![
            vec![1, 16, 7, 26, 11, 14],
            vec![34, 25, 12, 15, 6, 27],
            vec![17, 2, 33, 8, 13, 10],
            vec![32, 35, 24, 21, 28, 5],
            vec![23, 18, 3, 30, 9, 20],
            vec![36, 31, 22, 19, 4, 29],
        ])),
        test_knight_tour_8x8: (8, 8, 0, 0, Some(vec![
            vec![1, 60, 39, 34, 31, 18, 9, 64],
            vec![38, 35, 32, 61, 10, 63, 30, 17],
            vec![59, 2, 37, 40, 33, 28, 19, 8],
            vec![36, 49, 42, 27, 62, 11, 16, 29],
            vec![43, 58, 3, 50, 41, 24, 7, 20],
            vec![48, 51, 46, 55, 26, 21, 12, 15],
            vec![57, 44, 53, 4, 23, 14, 25, 6],
            vec![52, 47, 56, 45, 54, 5, 22, 13],
        ])),
        test_no_solution: (5, 5, 2, 1, None::<Vec<Vec<usize>>>),
        test_invalid_start_position: (8, 8, 10, 10, None::<Vec<Vec<usize>>>),
    }
}
