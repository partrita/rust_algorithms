/// 음이 아닌 정수 `n`이 주어졌을 때 올바른 형식의 괄호 조합을 모두 생성합니다.
///
/// 이 함수는 백트래킹을 사용하여 올바른 형식의 괄호에 대한 모든 가능한 조합을 생성합니다.
/// 결과 조합은 문자열 벡터로 반환됩니다.
///
/// # 인수
///
/// * `n` - 괄호 쌍의 수를 나타내는 음이 아닌 정수입니다.
pub fn generate_parentheses(n: usize) -> Vec<String> {
    let mut result = Vec::new();
    if n > 0 {
        generate("", 0, 0, n, &mut result);
    }
    result
}

/// 괄호를 재귀적으로 생성하기 위한 헬퍼 함수입니다.
///
/// 이 함수는 올바른 형식의 괄호 조합을 만들기 위해 재귀적으로 호출됩니다.
/// 지금까지 추가된 열린 괄호와 닫힌 괄호의 수를 추적하고 유효한 경우 새 괄호를 추가합니다.
///
/// # 인수
///
/// * `current` - 현재 빌드 중인 괄호 문자열입니다.
/// * `open_count` - 현재 문자열의 열린 괄호 수입니다.
/// * `close_count` - 현재 문자열의 닫힌 괄호 수입니다.
/// * `n` - 생성할 총 괄호 쌍의 수입니다.
/// * `result` - 생성된 조합을 저장하는 벡터에 대한 변경 가능한 참조입니다.
fn generate(
    current: &str,
    open_count: usize,
    close_count: usize,
    n: usize,
    result: &mut Vec<String>,
) {
    if current.len() == (n * 2) {
        result.push(current.to_string());
        return;
    }

    if open_count < n {
        let new_str = current.to_string() + "(";
        generate(&new_str, open_count + 1, close_count, n, result);
    }

    if close_count < open_count {
        let new_str = current.to_string() + ")";
        generate(&new_str, open_count, close_count + 1, n, result);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! generate_parentheses_tests {
        ($($name:ident: $test_case:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (n, expected_result) = $test_case;
                    assert_eq!(generate_parentheses(n), expected_result);
                }
            )*
        };
    }

    generate_parentheses_tests! {
        test_generate_parentheses_0: (0, Vec::<String>::new()),
        test_generate_parentheses_1: (1, vec!["()"]),
        test_generate_parentheses_2: (2, vec!["(())", "()()"]),
        test_generate_parentheses_3: (3, vec!["((()))", "(()())", "(())()", "()(())", "()()()"]),
        test_generate_parentheses_4: (4, vec!["(((())))", "((()()))", "((())())", "((()))()", "(()(()))", "(()()())", "(()())()", "(())(())", "(())()()", "()((()))", "()(()())", "()(())()", "()()(())", "()()()()"]),
    }
}
