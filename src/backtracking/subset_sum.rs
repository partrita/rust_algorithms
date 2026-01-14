//! 이 모듈은 주어진 정수 집합의 부분 집합 중 목표 값과 합이 같은 부분 집합이
//! 존재하는지 확인하는 기능을 제공합니다. 구현에는 재귀적 백트래킹 접근 방식이 사용됩니다.

/// 주어진 집합의 부분 집합 중 목표 값과 합이 같은 부분 집합이 존재하는지 확인합니다.
pub fn has_subset_with_sum(set: &[isize], target: isize) -> bool {
    backtrack(set, set.len(), target)
}

fn backtrack(set: &[isize], remaining_items: usize, target: isize) -> bool {
    // 필요한 합을 가진 부분 집합을 찾았습니다.
    if target == 0 {
        return true;
    }
    // 처리할 요소가 더 이상 없습니다.
    if remaining_items == 0 {
        return false;
    }
    // 마지막 요소를 포함하거나 제외하는 부분 집합을 찾을 수 있는지 확인합니다.
    backtrack(set, remaining_items - 1, target)
        || backtrack(set, remaining_items - 1, target - set[remaining_items - 1])
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! has_subset_with_sum_tests {
        ($($name:ident: $test_case:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (set, target, expected) = $test_case;
                    assert_eq!(has_subset_with_sum(set, target), expected);
                }
            )*
        }
    }

    has_subset_with_sum_tests! {
        test_small_set_with_sum: (&[3, 34, 4, 12, 5, 2], 9, true),
        test_small_set_without_sum: (&[3, 34, 4, 12, 5, 2], 30, false),
        test_consecutive_set_with_sum: (&[1, 2, 3, 4, 5, 6], 10, true),
        test_consecutive_set_without_sum: (&[1, 2, 3, 4, 5, 6], 22, false),
        test_large_set_with_sum: (&[5, 10, 12, 13, 15, 18, -1, 10, 50, -2, 3, 4], 30, true),
        test_empty_set: (&[], 0, true),
        test_empty_set_with_nonzero_sum: (&[], 10, false),
        test_single_element_equal_to_sum: (&[10], 10, true),
        test_single_element_not_equal_to_sum: (&[5], 10, false),
        test_negative_set_with_sum: (&[-7, -3, -2, 5, 8], 0, true),
        test_negative_sum: (&[1, 2, 3, 4, 5], -1, false),
        test_negative_sum_with_negatives: (&[-7, -3, -2, 5, 8], -4, true),
        test_negative_sum_with_negatives_no_solution: (&[-7, -3, -2, 5, 8], -14, false),
        test_even_inputs_odd_target: (&[2, 4, 6, 2, 8, -2, 10, 12, -24, 8, 12, 18], 3, false),
    }
}
