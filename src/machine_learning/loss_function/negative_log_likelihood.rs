// 음의 로그 가능도 손실 함수 (Negative Log Likelihood Loss Function)
//
// `neg_log_likelihood` 함수는 기계 학습의 분류 문제에 사용되는 손실 함수인
// 음의 로그 가능도 손실을 계산합니다.
//
// ## 공식
//
// `y_true` 및 `y_pred` 벡터로 표현되는 실제 값과 예측 값 쌍에 대해
// 음의 로그 가능도 손실은 다음과 같이 계산됩니다:
//
// - 손실 = `-y_true * log(y_pred) - (1 - y_true) * log(1 - y_pred)`.
//
// `total_loss`를 총 요소 수로 나누어 평균 손실을 반환합니다.
//
// 참고 자료:
// https://towardsdatascience.com/cross-entropy-negative-log-likelihood-and-all-that-jazz-47a95bd2e81
// http://neuralnetworksanddeeplearning.com/chap3.html
// 공식 유도:
// https://medium.com/@bhardwajprakarsh/negative-log-likelihood-loss-why-do-we-use-it-for-binary-classification-7625f9e3c944

pub fn neg_log_likelihood(
    y_true: &[f64],
    y_pred: &[f64],
) -> Result<f64, NegativeLogLikelihoodLossError> {
    // 입력값이 비어 있는지 확인합니다.
    if y_true.len() != y_pred.len() {
        return Err(NegativeLogLikelihoodLossError::InputsHaveDifferentLength);
    }
    // 실제 값과 예측 값의 길이가 같은지 확인합니다.
    if y_pred.is_empty() {
        return Err(NegativeLogLikelihoodLossError::EmptyInputs);
    }
    // 값이 0과 1 사이에 있는지 확인합니다.
    if !are_all_values_in_range(y_true) || !are_all_values_in_range(y_pred) {
        return Err(NegativeLogLikelihoodLossError::InvalidValues);
    }

    let mut total_loss: f64 = 0.0;
    for (p, a) in y_pred.iter().zip(y_true.iter()) {
        let loss: f64 = -a * p.ln() - (1.0 - a) * (1.0 - p).ln();
        total_loss += loss;
    }
    Ok(total_loss / (y_pred.len() as f64))
}

#[derive(Debug, PartialEq, Eq)]
pub enum NegativeLogLikelihoodLossError {
    InputsHaveDifferentLength,
    EmptyInputs,
    InvalidValues,
}

fn are_all_values_in_range(values: &[f64]) -> bool {
    values.iter().all(|&x| (0.0..=1.0).contains(&x))
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_with_wrong_inputs {
        ($($name:ident: $inputs:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (values_a, values_b, expected_error) = $inputs;
                    assert_eq!(neg_log_likelihood(&values_a, &values_b), expected_error);
                    assert_eq!(neg_log_likelihood(&values_b, &values_a), expected_error);
                }
            )*
        }
    }

    test_with_wrong_inputs! {
        different_length: (vec![0.9, 0.0, 0.8], vec![0.9, 0.1], Err(NegativeLogLikelihoodLossError::InputsHaveDifferentLength)),
        different_length_one_empty: (vec![], vec![0.9, 0.1], Err(NegativeLogLikelihoodLossError::InputsHaveDifferentLength)),
        value_greater_than_1: (vec![1.1, 0.0, 0.8], vec![0.1, 0.2, 0.3], Err(NegativeLogLikelihoodLossError::InvalidValues)),
        value_greater_smaller_than_0: (vec![0.9, 0.0, -0.1], vec![0.1, 0.2, 0.3], Err(NegativeLogLikelihoodLossError::InvalidValues)),
        empty_input: (vec![], vec![], Err(NegativeLogLikelihoodLossError::EmptyInputs)),
    }

    macro_rules! test_neg_log_likelihood {
        ($($name:ident: $inputs:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (actual_values, predicted_values, expected) = $inputs;
                    assert_eq!(neg_log_likelihood(&actual_values, &predicted_values).unwrap(), expected);
                }
            )*
        }
    }

    test_neg_log_likelihood! {
        set_0: (vec![1.0, 0.0, 1.0], vec![0.9, 0.1, 0.8], 0.14462152754328741),
        set_1: (vec![1.0, 0.0, 1.0], vec![0.1, 0.2, 0.3], 1.2432338162113972),
        set_2: (vec![0.0, 1.0, 0.0], vec![0.1, 0.2, 0.3], 0.6904911240102196),
        set_3: (vec![1.0, 0.0, 1.0, 0.0], vec![0.9, 0.1, 0.8, 0.2], 0.164252033486018),
    }
}
