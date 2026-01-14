//! 이 모듈은 백트래킹 알고리즘을 사용하여 `0...n-1`에서 `k`개의 숫자로
//! 만들 수 있는 모든 가능한 조합을 생성하는 함수를 제공합니다.

/// 조합 생성 시 사용자 정의 오류 유형입니다.
#[derive(Debug, PartialEq)]
pub enum CombinationError {
    KGreaterThanN,
    InvalidZeroRange,
}

/// `0...n-1` 범위의 숫자 중 `k`개를 선택하여 만들 수 있는 모든 가능한 조합을 생성합니다.
///
/// # 인수
///
/// * `n` - 범위의 상한 (`0`부터 `n-1`까지).
/// * `k` - 각 조합의 요소 수.
///
/// # 반환 값
///
/// `0...n-1` 범위의 숫자 중 `k`개를 선택하여 만들 수 있는 모든 가능한 조합을 담은 벡터를 포함하는 `Result` 또는
/// 입력이 유효하지 않은 경우 `CombinationError`.
pub fn generate_all_combinations(n: usize, k: usize) -> Result<Vec<Vec<usize>>, CombinationError> {
    if n == 0 && k > 0 {
        return Err(CombinationError::InvalidZeroRange);
    }

    if k > n {
        return Err(CombinationError::KGreaterThanN);
    }

    let mut combinations = vec![];
    let mut current = vec![0; k];
    backtrack(0, n, k, 0, &mut current, &mut combinations);
    Ok(combinations)
}

/// 조합을 재귀적으로 생성하는 헬퍼 함수입니다.
///
/// # 인수
///
/// * `start` - 조합을 시작할 현재 숫자입니다.
/// * `n` - 범위의 상한 (`0`부터 `n-1`까지)입니다.
/// * `k` - 조합을 완성하기 위해 남은 요소의 수입니다.
/// * `index` - 조합에서 현재 채우고 있는 인덱스입니다.
/// * `current` - 현재 구성 중인 조합에 대한 변경 가능한 참조입니다.
/// * `combinations` - 모든 조합을 담고 있는 벡터에 대한 변경 가능한 참조입니다.
fn backtrack(
    start: usize,
    n: usize,
    k: usize,
    index: usize,
    current: &mut Vec<usize>,
    combinations: &mut Vec<Vec<usize>>,
) {
    if index == k {
        combinations.push(current.clone());
        return;
    }

    for num in start..=(n - k + index) {
        current[index] = num;
        backtrack(num + 1, n, k, index + 1, current, combinations);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! combination_tests {
        ($($name:ident: $test_case:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (n, k, expected) = $test_case;
                    assert_eq!(generate_all_combinations(n, k), expected);
                }
            )*
        }
    }

    combination_tests! {
        test_generate_4_2: (4, 2, Ok(vec![
            vec![0, 1],
            vec![0, 2],
            vec![0, 3],
            vec![1, 2],
            vec![1, 3],
            vec![2, 3],
        ])),
        test_generate_4_3: (4, 3, Ok(vec![
            vec![0, 1, 2],
            vec![0, 1, 3],
            vec![0, 2, 3],
            vec![1, 2, 3],
        ])),
        test_generate_5_3: (5, 3, Ok(vec![
            vec![0, 1, 2],
            vec![0, 1, 3],
            vec![0, 1, 4],
            vec![0, 2, 3],
            vec![0, 2, 4],
            vec![0, 3, 4],
            vec![1, 2, 3],
            vec![1, 2, 4],
            vec![1, 3, 4],
            vec![2, 3, 4],
        ])),
        test_generate_5_1: (5, 1, Ok(vec![
            vec![0],
            vec![1],
            vec![2],
            vec![3],
            vec![4],
        ])),
        test_empty: (0, 0, Ok(vec![vec![]])),
        test_generate_n_eq_k: (3, 3, Ok(vec![
            vec![0, 1, 2],
        ])),
        test_generate_k_greater_than_n: (3, 4, Err(CombinationError::KGreaterThanN)),
        test_zero_range_with_nonzero_k: (0, 1, Err(CombinationError::InvalidZeroRange)),
    }
}
