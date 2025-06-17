//! 이 모듈은 방향 또는 무방향 그래프에서 해밀턴 순환을 찾는 기능을 제공합니다.
//! 출처: [위키백과](https://ko.wikipedia.org/wiki/해밀턴_경로_문제)

/// 인접 행렬에서 해밀턴 순환을 찾을 때 발생할 수 있는 잠재적 오류를 나타냅니다.
#[derive(Debug, PartialEq, Eq)]
pub enum FindHamiltonianCycleError {
    /// 인접 행렬이 비어 있음을 나타냅니다.
    EmptyAdjacencyMatrix,
    /// 인접 행렬이 정사각형이 아님을 나타냅니다.
    ImproperAdjacencyMatrix,
    /// 시작 정점이 범위를 벗어났음을 나타냅니다.
    StartOutOfBound,
}

/// 인접 행렬을 사용하여 그래프를 나타냅니다.
struct Graph {
    /// 그래프를 나타내는 인접 행렬입니다.
    adjacency_matrix: Vec<Vec<bool>>,
}

impl Graph {
    /// 제공된 인접 행렬로 새 그래프를 만듭니다.
    ///
    /// # 인수
    ///
    /// * `adjacency_matrix` - 각 요소가 두 정점 간의 간선 존재(`true`) 또는 부재(`false`)를
    ///                        나타내는 정사각형 행렬입니다.
    ///
    /// # 반환 값
    ///
    /// 성공하면 그래프를 포함하는 `Result`이거나, 행렬에 문제가 있는 경우 `FindHamiltonianCycleError`입니다.
    fn new(adjacency_matrix: Vec<Vec<bool>>) -> Result<Self, FindHamiltonianCycleError> {
        // 인접 행렬이 비어 있는지 확인합니다.
        if adjacency_matrix.is_empty() {
            return Err(FindHamiltonianCycleError::EmptyAdjacencyMatrix);
        }

        // 인접 행렬이 정사각형인지 확인합니다.
        if adjacency_matrix
            .iter()
            .any(|row| row.len() != adjacency_matrix.len())
        {
            return Err(FindHamiltonianCycleError::ImproperAdjacencyMatrix);
        }

        Ok(Self { adjacency_matrix })
    }

    /// 그래프의 정점 수를 반환합니다.
    fn num_vertices(&self) -> usize {
        self.adjacency_matrix.len()
    }

    /// 해밀턴 순환 경로에 정점 `v`를 포함하는 것이 안전한지 확인합니다.
    ///
    /// # 인수
    ///
    /// * `v` - 고려 중인 정점의 인덱스입니다.
    /// * `visited` - 방문한 정점을 나타내는 벡터에 대한 참조입니다.
    /// * `path` - 탐색 중인 현재 경로에 대한 참조입니다.
    /// * `pos` - 고려 중인 현재 정점의 위치입니다.
    ///
    /// # 반환 값
    ///
    /// 경로에 `v`를 포함하는 것이 안전하면 `true`이고, 그렇지 않으면 `false`입니다.
    fn is_safe(&self, v: usize, visited: &[bool], path: &[Option<usize>], pos: usize) -> bool {
        // 현재 정점과 경로의 마지막 정점이 인접한지 확인합니다.
        if !self.adjacency_matrix[path[pos - 1].unwrap()][v] {
            return false;
        }

        // 정점이 이미 경로에 포함되었는지 확인합니다.
        !visited[v]
    }

    /// 해밀턴 순환을 재귀적으로 검색합니다.
    ///
    /// 이 함수는 `find_hamiltonian_cycle`에 의해 호출됩니다.
    ///
    /// # 인수
    ///
    /// * `path` - 탐색 중인 현재 경로를 나타내는 변경 가능한 벡터입니다.
    /// * `visited` - 방문한 정점을 나타내는 변경 가능한 벡터입니다.
    /// * `pos` - 고려 중인 현재 정점의 위치입니다.
    ///
    /// # 반환 값
    ///
    /// 해밀턴 순환이 발견되면 `true`이고, 그렇지 않으면 `false`입니다.
    fn hamiltonian_cycle_util(
        &self,
        path: &mut [Option<usize>],
        visited: &mut [bool],
        pos: usize,
    ) -> bool {
        if pos == self.num_vertices() {
            // 마지막으로 포함된 정점에서 첫 번째 정점으로 가는 간선이 있는지 확인합니다.
            return self.adjacency_matrix[path[pos - 1].unwrap()][path[0].unwrap()];
        }

        for v in 0..self.num_vertices() {
            if self.is_safe(v, visited, path, pos) {
                path[pos] = Some(v);
                visited[v] = true;
                if self.hamiltonian_cycle_util(path, visited, pos + 1) {
                    return true;
                }
                path[pos] = None;
                visited[v] = false;
            }
        }

        false
    }

    /// 지정된 정점에서 시작하여 그래프에서 해밀턴 순환을 찾으려고 시도합니다.
    ///
    /// 해밀턴 순환은 모든 정점을 정확히 한 번 방문하고 시작 정점으로 돌아갑니다.
    ///
    /// # 참고
    /// 이 구현은 가능한 모든 해밀턴 순환을 찾지 못할 수 있습니다.
    /// 유효한 순환을 하나 찾으면 중지됩니다. 여러 해밀턴 순환이 존재하는 경우
    /// 하나만 반환됩니다.
    ///
    /// # 반환 값
    ///
    /// 해밀턴 순환이 발견되면 `Ok(Some(path))`이며, 여기서 `path`는
    /// 순환의 정점 인덱스를 포함하는 벡터이고, 동일한 정점으로 시작하고 끝납니다.
    ///
    /// 해밀턴 순환이 없으면 `Ok(None)`입니다.
    fn find_hamiltonian_cycle(
        &self,
        start_vertex: usize,
    ) -> Result<Option<Vec<usize>>, FindHamiltonianCycleError> {
        // 시작 정점을 확인합니다.
        if start_vertex >= self.num_vertices() {
            return Err(FindHamiltonianCycleError::StartOutOfBound);
        }

        // 경로를 초기화합니다.
        let mut path = vec![None; self.num_vertices()];
        // 지정된 정점에서 시작합니다.
        path[0] = Some(start_vertex);

        // 방문한 벡터를 초기화합니다.
        let mut visited = vec![false; self.num_vertices()];
        visited[start_vertex] = true;

        if self.hamiltonian_cycle_util(&mut path, &mut visited, 1) {
            // 시작 정점으로 돌아가서 순환을 완료합니다.
            path.push(Some(start_vertex));
            Ok(Some(path.into_iter().map(Option::unwrap).collect()))
        } else {
            Ok(None)
        }
    }
}

/// 인접 행렬로 표현된 그래프에서 지정된 정점에서 시작하여 해밀턴 순환을 찾으려고 시도합니다.
pub fn find_hamiltonian_cycle(
    adjacency_matrix: Vec<Vec<bool>>,
    start_vertex: usize,
) -> Result<Option<Vec<usize>>, FindHamiltonianCycleError> {
    Graph::new(adjacency_matrix)?.find_hamiltonian_cycle(start_vertex)
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! hamiltonian_cycle_tests {
        ($($name:ident: $test_case:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (adjacency_matrix, start_vertex, expected) = $test_case;
                    let result = find_hamiltonian_cycle(adjacency_matrix, start_vertex);
                    assert_eq!(result, expected);
                }
            )*
        };
    }

    hamiltonian_cycle_tests! {
        test_complete_graph: (
            vec![
                vec![false, true, true, true],
                vec![true, false, true, true],
                vec![true, true, false, true],
                vec![true, true, true, false],
            ],
            0,
            Ok(Some(vec![0, 1, 2, 3, 0]))
        ),
        test_directed_graph_with_cycle: (
            vec![
                vec![false, true, false, false, false],
                vec![false, false, true, true, false],
                vec![true, false, false, true, true],
                vec![false, false, true, false, true],
                vec![true, true, false, false, false],
            ],
            2,
            Ok(Some(vec![2, 3, 4, 0, 1, 2]))
        ),
        test_undirected_graph_with_cycle: (
            vec![
                vec![false, true, false, false, true],
                vec![true, false, true, false, false],
                vec![false, true, false, true, false],
                vec![false, false, true, false, true],
                vec![true, false, false, true, false],
            ],
            2,
            Ok(Some(vec![2, 1, 0, 4, 3, 2]))
        ),
        test_directed_graph_no_cycle: (
            vec![
                vec![false, true, false, true, false],
                vec![false, false, true, true, false],
                vec![false, false, false, true, false],
                vec![false, false, false, false, true],
                vec![false, false, true, false, false],
            ],
            0,
            Ok(None::<Vec<usize>>)
        ),
        test_undirected_graph_no_cycle: (
            vec![
                vec![false, true, false, false, false],
                vec![true, false, true, true, false],
                vec![false, true, false, true, true],
                vec![false, true, true, false, true],
                vec![false, false, true, true, false],
            ],
            0,
            Ok(None::<Vec<usize>>)
        ),
        test_triangle_graph: (
            vec![
                vec![false, true, false],
                vec![false, false, true],
                vec![true, false, false],
            ],
            1,
            Ok(Some(vec![1, 2, 0, 1]))
        ),
        test_tree_graph: (
            vec![
                vec![false, true, false, true, false],
                vec![true, false, true, true, false],
                vec![false, true, false, false, false],
                vec![true, true, false, false, true],
                vec![false, false, false, true, false],
            ],
            0,
            Ok(None::<Vec<usize>>)
        ),
        test_empty_graph: (
            vec![],
            0,
            Err(FindHamiltonianCycleError::EmptyAdjacencyMatrix)
        ),
        test_improper_graph: (
            vec![
                vec![false, true],
                vec![true],
                vec![false, true, true],
                vec![true, true, true, false]
            ],
            0,
            Err(FindHamiltonianCycleError::ImproperAdjacencyMatrix)
        ),
        test_start_out_of_bound: (
            vec![
                vec![false, true, true],
                vec![true, false, true],
                vec![true, true, false],
            ],
            3,
            Err(FindHamiltonianCycleError::StartOutOfBound)
        ),
        test_complex_directed_graph: (
            vec![
                vec![false, true, false, true, false, false],
                vec![false, false, true, false, true, false],
                vec![false, false, false, true, false, false],
                vec![false, true, false, false, true, false],
                vec![false, false, true, false, false, true],
                vec![true, false, false, false, false, false],
            ],
            0,
            Ok(Some(vec![0, 1, 2, 3, 4, 5, 0]))
        ),
        single_node_self_loop: (
            vec![
                vec![true],
            ],
            0,
            Ok(Some(vec![0, 0]))
        ),
        single_node: (
            vec![
                vec![false],
            ],
            0,
            Ok(None)
        ),
    }
}
