//! # KL 발산 손실 함수 (KL divergence Loss Function)
//!
//! `actual` 및 `predicted` 벡터로 표현되는 실제 확률 분포와 예측 확률 분포 쌍에 대해 KL 발산 손실은 다음과 같이 계산됩니다:
//!
//! `L = Σ(actual[i] * ln(actual[i]/predicted[i]))` (벡터 범위의 모든 `i`에 대해)
//!
//! 여기서 `ln`은 자연 로그 함수이고, `Σ`는 벡터의 모든 요소에 대한 합계를 나타냅니다.
//!
//! ## KL 발산 손실 함수 구현
//!
//! 이 구현은 f64 값 벡터에 대한 두 개의 참조 `actual`과 `predicted`를 사용하고 그 사이의 KL 발산 손실을 반환합니다.
//!
pub fn kld_loss(actual: &[f64], predicted: &[f64]) -> f64 {
    // 요소 중 하나라도 0인 경우를 처리하기 위한 엡실론
    let eps = 0.00001f64;
    let loss: f64 = actual
        .iter()
        .zip(predicted.iter())
        .map(|(&a, &p)| (a + eps) * ((a + eps) / (p + eps)).ln())
        .sum();
    loss
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kld_loss() {
        let test_vector_actual = vec![1.346112, 1.337432, 1.246655];
        let test_vector = vec![1.033836, 1.082015, 1.117323];
        assert_eq!(
            kld_loss(&test_vector_actual, &test_vector),
            0.7752789394328498
        );
    }
}
