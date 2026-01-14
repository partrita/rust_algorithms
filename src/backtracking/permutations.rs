//! 이 모듈은 백트래킹 알고리즘을 사용하여 주어진 정수 컬렉션의 모든 가능한
//! 고유한 순열을 생성하는 함수를 제공합니다.

/// 주어진 정수 벡터의 모든 가능한 고유한 순열을 생성합니다.
///
/// # 인수
///
/// * `nums` - 정수 벡터입니다. 중복을 효과적으로 처리하기 위해 순열을 생성하기 전에
/// 입력 벡터가 정렬됩니다.
///
/// # 반환 값
///
/// 입력 벡터의 모든 가능한 고유한 순열을 포함하는 벡터입니다.
pub fn permute(mut nums: Vec<isize>) -> Vec<Vec<isize>> {
    let mut permutations = Vec::new();
    let mut current = Vec::new();
    let mut used = vec![false; nums.len()];

    nums.sort();
    generate(&nums, &mut current, &mut used, &mut permutations);

    permutations
}

/// `permute` 함수가 고유한 순열을 재귀적으로 생성하기 위한 헬퍼 함수입니다.
///
/// # 인수
///
/// * `nums` - 정렬된 정수 슬라이스에 대한 참조입니다.
/// * `current` - 현재 순열을 담고 있는 벡터에 대한 변경 가능한 참조입니다.
/// * `used` - 어떤 요소가 사용되었는지 추적하는 벡터에 대한 변경 가능한 참조입니다.
/// * `permutations` - 생성된 모든 고유한 순열을 담고 있는 벡터에 대한 변경 가능한 참조입니다.
fn generate(
    nums: &[isize],
    current: &mut Vec<isize>,
    used: &mut Vec<bool>,
    permutations: &mut Vec<Vec<isize>>,
) {
    if current.len() == nums.len() {
        permutations.push(current.clone());
        return;
    }

    for idx in 0..nums.len() {
        if used[idx] {
            continue;
        }

        if idx > 0 && nums[idx] == nums[idx - 1] && !used[idx - 1] {
            continue;
        }

        current.push(nums[idx]);
        used[idx] = true;

        generate(nums, current, used, permutations);

        current.pop();
        used[idx] = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! permute_tests {
        ($($name:ident: $test_case:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (input, expected) = $test_case;
                    assert_eq!(permute(input), expected);
                }
            )*
        }
    }

    permute_tests! {
        test_permute_basic: (vec![1, 2, 3], vec![
            vec![1, 2, 3],
            vec![1, 3, 2],
            vec![2, 1, 3],
            vec![2, 3, 1],
            vec![3, 1, 2],
            vec![3, 2, 1],
        ]),
        test_permute_empty: (Vec::<isize>::new(), vec![vec![]]),
        test_permute_single: (vec![1], vec![vec![1]]),
        test_permute_duplicates: (vec![1, 1, 2], vec![
            vec![1, 1, 2],
            vec![1, 2, 1],
            vec![2, 1, 1],
        ]),
        test_permute_all_duplicates: (vec![1, 1, 1, 1], vec![
            vec![1, 1, 1, 1],
        ]),
        test_permute_negative: (vec![-1, -2, -3], vec![
            vec![-3, -2, -1],
            vec![-3, -1, -2],
            vec![-2, -3, -1],
            vec![-2, -1, -3],
            vec![-1, -3, -2],
            vec![-1, -2, -3],
        ]),
        test_permute_mixed: (vec![-1, 0, 1], vec![
            vec![-1, 0, 1],
            vec![-1, 1, 0],
            vec![0, -1, 1],
            vec![0, 1, -1],
            vec![1, -1, 0],
            vec![1, 0, -1],
        ]),
        test_permute_larger: (vec![1, 2, 3, 4], vec![
            vec![1, 2, 3, 4],
            vec![1, 2, 4, 3],
            vec![1, 3, 2, 4],
            vec![1, 3, 4, 2],
            vec![1, 4, 2, 3],
            vec![1, 4, 3, 2],
            vec![2, 1, 3, 4],
            vec![2, 1, 4, 3],
            vec![2, 3, 1, 4],
            vec![2, 3, 4, 1],
            vec![2, 4, 1, 3],
            vec![2, 4, 3, 1],
            vec![3, 1, 2, 4],
            vec![3, 1, 4, 2],
            vec![3, 2, 1, 4],
            vec![3, 2, 4, 1],
            vec![3, 4, 1, 2],
            vec![3, 4, 2, 1],
            vec![4, 1, 2, 3],
            vec![4, 1, 3, 2],
            vec![4, 2, 1, 3],
            vec![4, 2, 3, 1],
            vec![4, 3, 1, 2],
            vec![4, 3, 2, 1],
        ]),
    }
}
