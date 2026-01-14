//! # 힌지 손실 (Hinge Loss)
//!
//! `hng_loss` 함수는 기계 학습의 분류 문제에 사용되는 손실 함수인
//! 힌지 손실을 계산합니다.
//!
//! ## 공식
//!
//! 실제 값과 예측 값 쌍(`y_true` 및 `y_pred` 벡터로 표시)에 대해
//! 힌지 손실은 다음과 같이 계산됩니다:
//!
//! - 손실 = `max(0, 1 - y_true * y_pred)`.
//!
//! `total_loss`를 총 요소 수로 나누어 평균 손실을 반환합니다.
//!
pub fn hng_loss(y_true: &[f64], y_pred: &[f64]) -> f64 {
    let mut total_loss: f64 = 0.0;
    for (p, a) in y_pred.iter().zip(y_true.iter()) {
        let loss = (1.0 - a * p).max(0.0);
        total_loss += loss;
    }
    total_loss / (y_pred.len() as f64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hinge_loss() {
        let predicted_values: Vec<f64> = vec![-1.0, 1.0, 1.0];
        let actual_values: Vec<f64> = vec![-1.0, -1.0, 1.0];
        assert_eq!(
            hng_loss(&predicted_values, &actual_values),
            0.6666666666666666
        );
    }
}
