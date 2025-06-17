//! 이 모듈은 미로 속의 쥐 문제의 구현을 포함합니다.
//!
//! 미로 속의 쥐 문제는 미로의 시작 위치에서 출구 위치까지의 경로를 찾는 것을
//! 목표로 하는 고전적인 알고리즘 문제입니다.

/// 미로 작업 중 발생할 수 있는 다양한 오류를 나타내는 열거형입니다.
#[derive(Debug, PartialEq, Eq)]
pub enum MazeError {
    /// 미로가 비어 있음을 나타냅니다(행이 0개).
    EmptyMaze,
    /// 시작 위치가 범위를 벗어났음을 나타냅니다.
    OutOfBoundPos,
    /// 미로의 부적절한 표현을 나타냅니다(예: 직사각형이 아닌 미로).
    ImproperMazeRepr,
}

/// 지정된 위치에서 시작하여 미로를 통과하는 경로를 찾습니다.
///
/// # 인수
///
/// * `maze` - 각 내부 벡터가 미로 그리드의 행을 나타내는 벡터의 벡터로 표현된 미로입니다.
/// * `start_x` - 시작 위치의 x 좌표입니다.
/// * `start_y` - 시작 위치의 y 좌표입니다.
///
/// # 반환 값
///
/// `Result`로서 다음 중 하나를 반환합니다:
/// - 경로가 발견되고 해결책 행렬을 포함하는 경우 `Ok(Some(solution))`
/// - 경로가 발견되지 않은 경우 `Ok(None)`
/// - 범위를 벗어난 시작 위치 또는 부적절한 미로 표현과 같은 다양한 오류 조건에 대해 `Err(MazeError)`
///
/// # 해결책 선택
///
/// 이 함수는 미리 정의된 이동 순서에 따라 발견한 첫 번째 성공적인 경로를 반환합니다.
/// 이동 순서는 `Maze` 구조체의 `MOVES` 상수에 정의되어 있습니다.
///
/// 백트래킹 알고리즘은 이 순서대로 각 방향을 탐색합니다. 여러 해결책이 존재하는 경우,
/// 알고리즘은 이 순서에 따라 찾은 첫 번째 경로를 반환합니다. 각 방향을 재귀적으로 탐색하고,
/// 유효한 이동을 표시하며, 필요한 경우 백트래킹하여 해결책을 효율적이고 일관되게 찾도록 보장합니다.
pub fn find_path_in_maze(
    maze: &[Vec<bool>],
    start_x: usize,
    start_y: usize,
) -> Result<Option<Vec<Vec<bool>>>, MazeError> {
    if maze.is_empty() {
        return Err(MazeError::EmptyMaze);
    }

    // 시작 위치 유효성 검사
    if start_x >= maze.len() || start_y >= maze[0].len() {
        return Err(MazeError::OutOfBoundPos);
    }

    // 미로 표현 유효성 검사 (필요한 경우)
    if maze.iter().any(|row| row.len() != maze[0].len()) {
        return Err(MazeError::ImproperMazeRepr);
    }

    // 유효성 검사를 통과하면 경로 찾기 진행
    let maze_instance = Maze::new(maze.to_owned());
    Ok(maze_instance.find_path(start_x, start_y))
}

/// 미로를 나타냅니다.
struct Maze {
    maze: Vec<Vec<bool>>,
}

impl Maze {
    /// 미로에서 가능한 이동을 나타냅니다.
    const MOVES: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    /// 새 Maze 인스턴스를 생성합니다.
    /// # 인수
    ///
    /// * `maze` - 각 내부 벡터가 미로 그리드의 행을 나타내는 벡터의 벡터로 표현된 미로입니다.
    ///
    /// # 반환 값
    ///
    /// 새 Maze 인스턴스입니다.
    fn new(maze: Vec<Vec<bool>>) -> Self {
        Maze { maze }
    }

    /// 미로의 너비를 반환합니다.
    ///
    /// # 반환 값
    ///
    /// 미로의 너비입니다.
    fn width(&self) -> usize {
        self.maze[0].len()
    }

    /// 미로의 높이를 반환합니다.
    ///
    /// # 반환 값
    ///
    /// 미로의 높이입니다.
    fn height(&self) -> usize {
        self.maze.len()
    }

    /// 지정된 위치에서 시작하여 미로를 통과하는 경로를 찾습니다.
    ///
    /// # 인수
    ///
    /// * `start_x` - 시작 위치의 x 좌표입니다.
    /// * `start_y` - 시작 위치의 y 좌표입니다.
    ///
    /// # 반환 값
    ///
    /// 경로가 발견되면 해결책 행렬을 반환하고, 그렇지 않으면 None을 반환합니다.
    fn find_path(&self, start_x: usize, start_y: usize) -> Option<Vec<Vec<bool>>> {
        let mut solution = vec![vec![false; self.width()]; self.height()];
        if self.solve(start_x as isize, start_y as isize, &mut solution) {
            Some(solution)
        } else {
            None
        }
    }

    /// 백트래킹을 사용하여 미로 속의 쥐 문제를 재귀적으로 해결합니다.
    ///
    /// # 인수
    ///
    /// * `x` - 현재 x 좌표입니다.
    /// * `y` - 현재 y 좌표입니다.
    /// * `solution` - 현재 해결책 행렬입니다.
    ///
    /// # 반환 값
    ///
    /// 해결책을 찾았는지 여부를 나타내는 부울 값입니다.
    fn solve(&self, x: isize, y: isize, solution: &mut [Vec<bool>]) -> bool {
        if x == (self.height() as isize - 1) && y == (self.width() as isize - 1) {
            solution[x as usize][y as usize] = true;
            return true;
        }

        if self.is_valid(x, y, solution) {
            solution[x as usize][y as usize] = true;

            for &(dx, dy) in &Self::MOVES {
                if self.solve(x + dx, y + dy, solution) {
                    return true;
                }
            }

            // 어떤 방향으로도 해결책에 도달하지 못하면 백트래킹합니다.
            solution[x as usize][y as usize] = false;
            return false;
        }
        false
    }

    /// 주어진 위치가 미로에서 유효한지 확인합니다.
    ///
    /// # 인수
    ///
    /// * `x` - 위치의 x 좌표입니다.
    /// * `y` - 위치의 y 좌표입니다.
    /// * `solution` - 현재 해결책 행렬입니다.
    ///
    /// # 반환 값
    ///
    /// 위치가 유효한지 여부를 나타내는 부울 값입니다.
    fn is_valid(&self, x: isize, y: isize, solution: &[Vec<bool>]) -> bool {
        x >= 0
            && y >= 0
            && x < self.height() as isize
            && y < self.width() as isize
            && self.maze[x as usize][y as usize]
            && !solution[x as usize][y as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_find_path_in_maze {
        ($($name:ident: $start_x:expr, $start_y:expr, $maze:expr, $expected:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let solution = find_path_in_maze($maze, $start_x, $start_y);
                    assert_eq!(solution, $expected);
                    if let Ok(Some(expected_solution)) = &solution {
                        assert_eq!(expected_solution[$start_x][$start_y], true);
                    }
                }
            )*
        }
    }

    test_find_path_in_maze! {
        maze_with_solution_5x5: 0, 0, &[
            vec![true, false, true, false, false],
            vec![true, true, false, true, false],
            vec![false, true, true, true, false],
            vec![false, false, false, true, true],
            vec![false, true, false, false, true],
        ], Ok(Some(vec![
            vec![true, false, false, false, false],
            vec![true, true, false, false, false],
            vec![false, true, true, true, false],
            vec![false, false, false, true, true],
            vec![false, false, false, false, true],
        ])),
        maze_with_solution_6x6: 0, 0, &[
            vec![true, false, true, false, true, false],
            vec![true, true, false, true, false, true],
            vec![false, true, true, true, true, false],
            vec![false, false, false, true, true, true],
            vec![false, true, false, false, true, false],
            vec![true, true, true, true, true, true],
        ], Ok(Some(vec![
            vec![true, false, false, false, false, false],
            vec![true, true, false, false, false, false],
            vec![false, true, true, true, true, false],
            vec![false, false, false, false, true, false],
            vec![false, false, false, false, true, false],
            vec![false, false, false, false, true, true],
        ])),
        maze_with_solution_8x8: 0, 0, &[
            vec![true, false, false, false, false, false, false, true],
            vec![true, true, false, true, true, true, false, false],
            vec![false, true, true, true, false, false, false, false],
            vec![false, false, false, true, false, true, true, false],
            vec![false, true, false, true, true, true, false, true],
            vec![true, false, true, false, false, true, true, true],
            vec![false, false, true, true, true, false, true, true],
            vec![true, true, true, false, true, true, true, true],
        ], Ok(Some(vec![
            vec![true, false, false, false, false, false, false, false],
            vec![true, true, false, false, false, false, false, false],
            vec![false, true, true, true, false, false, false, false],
            vec![false, false, false, true, false, false, false, false],
            vec![false, false, false, true, true, true, false, false],
            vec![false, false, false, false, false, true, true, true],
            vec![false, false, false, false, false, false, false, true],
            vec![false, false, false, false, false, false, false, true],
        ])),
        maze_without_solution_4x4: 0, 0, &[
            vec![true, false, false, false],
            vec![true, true, false, false],
            vec![false, false, true, false],
            vec![false, false, false, true],
        ], Ok(None::<Vec<Vec<bool>>>),
        maze_with_solution_3x4: 0, 0, &[
            vec![true, false, true, true],
            vec![true, true, true, false],
            vec![false, true, true, true],
        ], Ok(Some(vec![
            vec![true, false, false, false],
            vec![true, true, true, false],
            vec![false, false, true, true],
        ])),
        maze_without_solution_3x4: 0, 0, &[
            vec![true, false, true, true],
            vec![true, false, true, false],
            vec![false, true, false, true],
        ], Ok(None::<Vec<Vec<bool>>>),
        improper_maze_representation: 0, 0, &[
            vec![true],
            vec![true, true],
            vec![true, true, true],
            vec![true, true, true, true]
        ], Err(MazeError::ImproperMazeRepr),
        out_of_bound_start: 0, 3, &[
            vec![true, false, true],
            vec![true, true],
            vec![false, true, true],
        ], Err(MazeError::OutOfBoundPos),
        empty_maze: 0, 0, &[], Err(MazeError::EmptyMaze),
        maze_with_single_cell: 0, 0, &[
            vec![true],
        ], Ok(Some(vec![
                vec![true]
        ])),
        maze_with_one_row_and_multiple_columns: 0, 0, &[
            vec![true, false, true, true, false]
        ], Ok(None::<Vec<Vec<bool>>>),
        maze_with_multiple_rows_and_one_column: 0, 0, &[
            vec![true],
            vec![true],
            vec![false],
            vec![true],
        ], Ok(None::<Vec<Vec<bool>>>),
        maze_with_walls_surrounding_border: 0, 0, &[
            vec![false, false, false],
            vec![false, true, false],
            vec![false, false, false],
        ], Ok(None::<Vec<Vec<bool>>>),
        maze_with_no_walls: 0, 0, &[
            vec![true, true, true],
            vec![true, true, true],
            vec![true, true, true],
        ], Ok(Some(vec![
            vec![true, true, true],
            vec![false, false, true],
            vec![false, false, true],
        ])),
        maze_with_going_back: 0, 0, &[
            vec![true,  true,  true,  true, true,   true],
            vec![false, false, false, true, false,  true],
            vec![true,  true,  true,  true,  false, false],
            vec![true,  false, false, false, false, false],
            vec![true,  false, false, false, true, true],
            vec![true,  false, true,  true,  true,  false],
            vec![true,  false, true , false, true,  false],
            vec![true,  true,  true,  false, true,  true],
        ], Ok(Some(vec![
            vec![true,  true,  true,  true, false,  false],
            vec![false, false, false, true, false,  false],
            vec![true,  true,  true,  true,  false, false],
            vec![true,  false, false, false, false, false],
            vec![true,  false, false, false, false, false],
            vec![true,  false, true,  true,  true,  false],
            vec![true,  false, true , false, true,  false],
            vec![true,  true,  true,  false, true,  true],
        ])),
    }
}
