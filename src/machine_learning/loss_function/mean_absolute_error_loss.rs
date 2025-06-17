//! # 평균 절대 오차 손실 함수 (Mean Absolute Error Loss Function)
//!
//! `mae_loss` 함수는 기계 학습에서 사용되는 견고한 손실 함수인
//! 평균 절대 오차 손실을 계산합니다.
//!
//! ## 공식
//!
//! `actual` 및 `predicted` 벡터로 표현되는 실제 값과 예측 값 쌍에 대해
//! 평균 절대 오차 손실은 다음과 같이 계산됩니다:
//!
//! - 손실 = `절대값(실제값 - 예측값) / 요소_개수`.
//!
//! `total_loss`를 총 요소 수로 나누어 평균 손실을 반환합니다.
//!
pub fn mae_loss(predicted: &[f64], actual: &[f64]) -> f64 {
    let mut total_loss: f64 = 0.0;
    for (p, a) in predicted.iter().zip(actual.iter()) {
        let diff: f64 = p - a;
        let absolute_diff = diff.abs();
        total_loss += absolute_diff;
    }
    total_loss / (predicted.len() as f64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mae_loss() {
        let predicted_values: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0];
        let actual_values: Vec<f64> = vec![1.0, 3.0, 3.5, 4.5];
        assert_eq!(mae_loss(&predicted_values, &actual_values), 0.5);
    }
}
