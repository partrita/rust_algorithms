//! 이 모듈은 주어진 색상 수에 대해 무방향 (또는 방향) 그래프의 모든 가능한 색칠을 생성하는 기능을 제공합니다.
//! GraphColoring 구조체와 색상 할당 유효성 검사 및 모든 유효한 색칠 찾기 메서드를 포함합니다.

/// 인접 행렬에 색칠할 때 발생할 수 있는 잠재적 오류를 나타냅니다.
#[derive(Debug, PartialEq, Eq)]
pub enum GraphColoringError {
    // 인접 행렬이 비어 있음을 나타냅니다.
    EmptyAdjacencyMatrix,
    // 인접 행렬이 정사각형이 아님을 나타냅니다.
    ImproperAdjacencyMatrix,
}

/// 그래프의 모든 가능한 유효한 색칠을 생성합니다.
///
/// # 인수
///
/// * `adjacency_matrix` - 그래프의 인접 행렬을 나타내는 2D 벡터입니다.
/// * `num_colors` - 그래프를 색칠하는 데 사용할 수 있는 색상의 수입니다.
///
/// # 반환 값
///
/// * 행렬에 문제가 있는 경우 솔루션 벡터 또는 `GraphColoringError`가 포함된 `Option`을 포함하는 `Result`입니다.
pub fn generate_colorings(
    adjacency_matrix: Vec<Vec<bool>>,
    num_colors: usize,
) -> Result<Option<Vec<Vec<usize>>>, GraphColoringError> {
    Ok(GraphColoring::new(adjacency_matrix)?.find_solutions(num_colors))
}

/// 그래프 색칠 문제를 나타내는 구조체입니다.
struct GraphColoring {
    // 그래프의 인접 행렬
    adjacency_matrix: Vec<Vec<bool>>,
    // 각 정점에 할당된 현재 색상
    vertex_colors: Vec<usize>,
    // 검색 중에 발견된 정점에 대한 모든 유효한 색상 할당 벡터
    solutions: Vec<Vec<usize>>,
}

impl GraphColoring {
    /// 새 GraphColoring 인스턴스를 만듭니다.
    ///
    /// # 인수
    ///
    /// * `adjacency_matrix` - 그래프의 인접 행렬을 나타내는 2D 벡터입니다.
    ///
    /// # 반환 값
    ///
    /// * 새 GraphColoring 인스턴스 또는 행렬이 비어 있거나 정사각형이 아닌 경우 `GraphColoringError`입니다.
    fn new(adjacency_matrix: Vec<Vec<bool>>) -> Result<Self, GraphColoringError> {
        let num_vertices = adjacency_matrix.len();

        // 인접 행렬이 비어 있는지 확인합니다.
        if num_vertices == 0 {
            return Err(GraphColoringError::EmptyAdjacencyMatrix);
        }

        // 인접 행렬이 정사각형인지 확인합니다.
        if adjacency_matrix.iter().any(|row| row.len() != num_vertices) {
            return Err(GraphColoringError::ImproperAdjacencyMatrix);
        }

        Ok(GraphColoring {
            adjacency_matrix,
            vertex_colors: vec![usize::MAX; num_vertices],
            solutions: Vec::new(),
        })
    }

    /// 그래프의 정점 수를 반환합니다.
    fn num_vertices(&self) -> usize {
        self.adjacency_matrix.len()
    }

    /// 주어진 색상을 충돌 없이 정점에 할당할 수 있는지 확인합니다.
    ///
    /// # 인수
    ///
    /// * `vertex` - 색칠할 정점의 인덱스입니다.
    /// * `color` - 정점에 할당할 색상입니다.
    ///
    /// # 반환 값
    ///
    /// * 색상을 정점에 할당할 수 있으면 `true`이고, 그렇지 않으면 `false`입니다.
    fn is_color_valid(&self, vertex: usize, color: usize) -> bool {
        for neighbor in 0..self.num_vertices() {
            // 정점에서 나가는 간선과 정점으로 들어오는 간선을 확인합니다.
            if (self.adjacency_matrix[vertex][neighbor] || self.adjacency_matrix[neighbor][vertex])
                && self.vertex_colors[neighbor] == color
            {
                return false;
            }
        }
        true
    }

    /// 그래프에 대한 모든 유효한 색칠을 재귀적으로 찾습니다.
    ///
    /// # 인수
    ///
    /// * `vertex` - 현재 색칠할 정점입니다.
    /// * `num_colors` - 그래프를 색칠하는 데 사용할 수 있는 색상의 수입니다.
    fn find_colorings(&mut self, vertex: usize, num_colors: usize) {
        if vertex == self.num_vertices() {
            self.solutions.push(self.vertex_colors.clone());
            return;
        }

        for color in 0..num_colors {
            if self.is_color_valid(vertex, color) {
                self.vertex_colors[vertex] = color;
                self.find_colorings(vertex + 1, num_colors);
                self.vertex_colors[vertex] = usize::MAX;
            }
        }
    }

    /// 그래프 색칠 문제에 대한 모든 솔루션을 찾습니다.
    ///
    /// # 인수
    ///
    /// * `num_colors` - 그래프를 색칠하는 데 사용할 수 있는 색상의 수입니다.
    ///
    /// # 반환 값
    ///
    /// * 솔루션 벡터 또는 `GraphColoringError`가 포함된 `Option`을 포함하는 `Result`입니다.
    fn find_solutions(&mut self, num_colors: usize) -> Option<Vec<Vec<usize>>> {
        self.find_colorings(0, num_colors);
        if self.solutions.is_empty() {
            None
        } else {
            Some(std::mem::take(&mut self.solutions))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_graph_coloring {
        ($($name:ident: $test_case:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (adjacency_matrix, num_colors, expected) = $test_case;
                    let actual = generate_colorings(adjacency_matrix, num_colors);
                    assert_eq!(actual, expected);
                }
            )*
        };
    }

    test_graph_coloring! {
        test_complete_graph_with_3_colors: (
            vec![
                vec![false, true, true, true],
                vec![true, false, true, false],
                vec![true, true, false, true],
                vec![true, false, true, false],
            ],
            3,
            Ok(Some(vec![
                vec![0, 1, 2, 1],
                vec![0, 2, 1, 2],
                vec![1, 0, 2, 0],
                vec![1, 2, 0, 2],
                vec![2, 0, 1, 0],
                vec![2, 1, 0, 1],
            ]))
        ),
        test_linear_graph_with_2_colors: (
            vec![
                vec![false, true, false, false],
                vec![true, false, true, false],
                vec![false, true, false, true],
                vec![false, false, true, false],
            ],
            2,
            Ok(Some(vec![
                vec![0, 1, 0, 1],
                vec![1, 0, 1, 0],
            ]))
        ),
        test_incomplete_graph_with_insufficient_colors: (
            vec![
                vec![false, true, true],
                vec![true, false, true],
                vec![true, true, false],
            ],
            1,
            Ok(None::<Vec<Vec<usize>>>)
        ),
        test_empty_graph: (
            vec![],
            1,
            Err(GraphColoringError::EmptyAdjacencyMatrix)
        ),
        test_non_square_matrix: (
            vec![
                vec![false, true, true],
                vec![true, false, true],
            ],
            3,
            Err(GraphColoringError::ImproperAdjacencyMatrix)
        ),
        test_single_vertex_graph: (
            vec![
                vec![false],
            ],
            1,
            Ok(Some(vec![
                vec![0],
            ]))
        ),
        test_bipartite_graph_with_2_colors: (
            vec![
                vec![false, true, false, true],
                vec![true, false, true, false],
                vec![false, true, false, true],
                vec![true, false, true, false],
            ],
            2,
            Ok(Some(vec![
                vec![0, 1, 0, 1],
                vec![1, 0, 1, 0],
            ]))
        ),
        test_large_graph_with_3_colors: (
            vec![
                vec![false, true, true, false, true, true, false, true, true, false],
                vec![true, false, true, true, false, true, true, false, true, true],
                vec![true, true, false, true, true, false, true, true, false, true],
                vec![false, true, true, false, true, true, false, true, true, false],
                vec![true, false, true, true, false, true, true, false, true, true],
                vec![true, true, false, true, true, false, true, true, false, true],
                vec![false, true, true, false, true, true, false, true, true, false],
                vec![true, false, true, true, false, true, true, false, true, true],
                vec![true, true, false, true, true, false, true, true, false, true],
                vec![false, true, true, false, true, true, false, true, true, false],
            ],
            3,
            Ok(Some(vec![
                vec![0, 1, 2, 0, 1, 2, 0, 1, 2, 0],
                vec![0, 2, 1, 0, 2, 1, 0, 2, 1, 0],
                vec![1, 0, 2, 1, 0, 2, 1, 0, 2, 1],
                vec![1, 2, 0, 1, 2, 0, 1, 2, 0, 1],
                vec![2, 0, 1, 2, 0, 1, 2, 0, 1, 2],
                vec![2, 1, 0, 2, 1, 0, 2, 1, 0, 2],
            ]))
        ),
        test_disconnected_graph: (
            vec![
                vec![false, false, false],
                vec![false, false, false],
                vec![false, false, false],
            ],
            2,
            Ok(Some(vec![
                vec![0, 0, 0],
                vec![0, 0, 1],
                vec![0, 1, 0],
                vec![0, 1, 1],
                vec![1, 0, 0],
                vec![1, 0, 1],
                vec![1, 1, 0],
                vec![1, 1, 1],
            ]))
        ),
        test_no_valid_coloring: (
            vec![
                vec![false, true, true],
                vec![true, false, true],
                vec![true, true, false],
            ],
            2,
            Ok(None::<Vec<Vec<usize>>>)
        ),
        test_more_colors_than_nodes: (
            vec![
                vec![true, true],
                vec![true, true],
            ],
            3,
            Ok(Some(vec![
                vec![0, 1],
                vec![0, 2],
                vec![1, 0],
                vec![1, 2],
                vec![2, 0],
                vec![2, 1],
            ]))
        ),
        test_no_coloring_with_zero_colors: (
            vec![
                vec![true],
            ],
            0,
            Ok(None::<Vec<Vec<usize>>>)
        ),
        test_complete_graph_with_3_vertices_and_3_colors: (
            vec![
                vec![false, true, true],
                vec![true, false, true],
                vec![true, true, false],
            ],
            3,
            Ok(Some(vec![
                vec![0, 1, 2],
                vec![0, 2, 1],
                vec![1, 0, 2],
                vec![1, 2, 0],
                vec![2, 0, 1],
                vec![2, 1, 0],
            ]))
        ),
        test_directed_graph_with_3_colors: (
            vec![
                vec![false, true, false, true],
                vec![false, false, true, false],
                vec![true, false, false, true],
                vec![true, false, false, false],
            ],
            3,
            Ok(Some(vec![
                vec![0, 1, 2, 1],
                vec![0, 2, 1, 2],
                vec![1, 0, 2, 0],
                vec![1, 2, 0, 2],
                vec![2, 0, 1, 0],
                vec![2, 1, 0, 1],
            ]))
        ),
        test_directed_graph_no_valid_coloring: (
            vec![
                vec![false, true, false, true],
                vec![false, false, true, true],
                vec![true, false, false, true],
                vec![true, false, false, false],
            ],
            3,
            Ok(None::<Vec<Vec<usize>>>)
        ),
        test_large_directed_graph_with_3_colors: (
            vec![
                vec![false, true, false, false, true, false, false, true, false, false],
                vec![false, false, true, false, false, true, false, false, true, false],
                vec![false, false, false, true, false, false, true, false, false, true],
                vec![true, false, false, false, true, false, false, true, false, false],
                vec![false, true, false, false, false, true, false, false, true, false],
                vec![false, false, true, false, false, false, true, false, false, true],
                vec![true, false, false, false, true, false, false, true, false, false],
                vec![false, true, false, false, false, true, false, false, true, false],
                vec![false, false, true, false, false, false, true, false, false, true],
                vec![true, false, false, false, true, false, false, true, false, false],
            ],
            3,
            Ok(Some(vec![
                vec![0, 1, 2, 1, 2, 0, 1, 2, 0, 1],
                vec![0, 2, 1, 2, 1, 0, 2, 1, 0, 2],
                vec![1, 0, 2, 0, 2, 1, 0, 2, 1, 0],
                vec![1, 2, 0, 2, 0, 1, 2, 0, 1, 2],
                vec![2, 0, 1, 0, 1, 2, 0, 1, 2, 0],
                vec![2, 1, 0, 1, 0, 2, 1, 0, 2, 1]
            ]))
        ),
    }
}
