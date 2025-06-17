//! # Adam (Adaptive Moment Estimation) 옵티마이저
//!
//! `Adam (Adaptive Moment Estimation)` 옵티마이저는 경사 하강법 및 기계 학습에서 사용되는 적응형 학습률 알고리즘으로,
//! 심층 학습 문제를 해결하기 위한 신경망 훈련 등에 사용됩니다. 메모리 효율적인 빠른 수렴 속도를 자랑하며,
//! 기울기 이력을 기반으로 각 모델 매개변수에 대해 개별적으로 학습률을 설정하고 반복적으로 업데이트합니다.
//!
//! ## 알고리즘:
//!
//! 주어진 값:
//!   - α: 학습률
//!   - (β_1, β_2): 모멘트 추정값에 대한 지수 감쇠율
//!   - ϵ: 0으로 나누는 것을 방지하기 위한 작은 값
//!   - g_t: 시간 단계 t에서의 기울기
//!   - m_t: 시간 단계 t에서의 기울기에 대한 편향된 1차 모멘트 추정값
//!   - v_t: 시간 단계 t에서의 기울기에 대한 편향된 2차 원시 모멘트 추정값
//!   - θ_t: 시간 단계 t에서의 모델 매개변수
//!   - t: 시간 단계
//!
//! 필요 조건:
//!   θ_0
//!
//! 초기화:
//!   m_0 <- 0
//!   v_0 <- 0
//!   t <- 0
//!
//! while θ_t가 수렴하지 않을 때까지 반복:
//!   m_t = β_1 * m_{t−1} + (1 − β_1) * g_t
//!   v_t = β_2 * v_{t−1} + (1 − β_2) * g_t^2
//!   m_hat_t = m_t / 1 - β_1^t
//!   v_hat_t = v_t / 1 - β_2^t
//!   θ_t = θ_{t-1} − α * m_hat_t / (sqrt(v_hat_t) + ϵ)
//!
//! ## 참고 자료:
//!   - Adam: 확률적 최적화를 위한 방법 (Diederik P. Kingma 및 Jimmy Ba 저):
//!       - [https://arxiv.org/abs/1412.6980]
//!   - PyTorch Adam 옵티마이저:
//!       - [https://pytorch.org/docs/stable/generated/torch.optim.Adam.html#torch.optim.Adam]
//!
pub struct Adam {
    learning_rate: f64, // 알파: 반복 최적화를 위한 초기 단계 크기
    betas: (f64, f64),  // 베타: 모멘트 추정값에 대한 지수 감쇠율
    epsilon: f64,       // 엡실론: 0으로 나누는 것을 방지
    m: Vec<f64>,        // m: 기울기 벡터의 편향된 1차 모멘트 추정값
    v: Vec<f64>,        // v: 기울기 벡터의 편향된 2차 원시 모멘트 추정값
    t: usize,           // t: 시간 단계
}

impl Adam {
    pub fn new(
        learning_rate: Option<f64>,
        betas: Option<(f64, f64)>,
        epsilon: Option<f64>,
        params_len: usize,
    ) -> Self {
        Adam {
            learning_rate: learning_rate.unwrap_or(1e-3), // 일반적인 기본 학습률
            betas: betas.unwrap_or((0.9, 0.999)),         // 일반적인 기본 감쇠율
            epsilon: epsilon.unwrap_or(1e-8),             // 일반적인 기본 엡실론
            m: vec![0.0; params_len], // 1차 모멘트 벡터 요소는 모두 0으로 초기화
            v: vec![0.0; params_len], // 2차 모멘트 벡터 요소는 모두 0으로 초기화
            t: 0,                     // 시간 단계는 0으로 초기화
        }
    }

    pub fn step(&mut self, gradients: &[f64]) -> Vec<f64> {
        let mut model_params = vec![0.0; gradients.len()];
        self.t += 1;

        for i in 0..gradients.len() {
            // 편향된 1차 모멘트 추정값과 2차 원시 모멘트 추정값 업데이트
            self.m[i] = self.betas.0 * self.m[i] + (1.0 - self.betas.0) * gradients[i];
            self.v[i] = self.betas.1 * self.v[i] + (1.0 - self.betas.1) * gradients[i].powf(2f64);

            // 편향 보정된 1차 모멘트 추정값과 2차 원시 모멘트 추정값 계산
            let m_hat = self.m[i] / (1.0 - self.betas.0.powi(self.t as i32));
            let v_hat = self.v[i] / (1.0 - self.betas.1.powi(self.t as i32));

            // 모델 매개변수 업데이트
            model_params[i] -= self.learning_rate * m_hat / (v_hat.sqrt() + self.epsilon);
        }
        model_params // 업데이트된 모델 매개변수 반환
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adam_init_default_values() {
        let optimizer = Adam::new(None, None, None, 1);

        assert_eq!(optimizer.learning_rate, 0.001);
        assert_eq!(optimizer.betas, (0.9, 0.999));
        assert_eq!(optimizer.epsilon, 1e-8);
        assert_eq!(optimizer.m, vec![0.0; 1]);
        assert_eq!(optimizer.v, vec![0.0; 1]);
        assert_eq!(optimizer.t, 0);
    }

    #[test]
    fn test_adam_init_custom_lr_value() {
        let optimizer = Adam::new(Some(0.9), None, None, 2);

        assert_eq!(optimizer.learning_rate, 0.9);
        assert_eq!(optimizer.betas, (0.9, 0.999));
        assert_eq!(optimizer.epsilon, 1e-8);
        assert_eq!(optimizer.m, vec![0.0; 2]);
        assert_eq!(optimizer.v, vec![0.0; 2]);
        assert_eq!(optimizer.t, 0);
    }

    #[test]
    fn test_adam_init_custom_betas_value() {
        let optimizer = Adam::new(None, Some((0.8, 0.899)), None, 3);

        assert_eq!(optimizer.learning_rate, 0.001);
        assert_eq!(optimizer.betas, (0.8, 0.899));
        assert_eq!(optimizer.epsilon, 1e-8);
        assert_eq!(optimizer.m, vec![0.0; 3]);
        assert_eq!(optimizer.v, vec![0.0; 3]);
        assert_eq!(optimizer.t, 0);
    }

    #[test]
    fn test_adam_init_custom_epsilon_value() {
        let optimizer = Adam::new(None, None, Some(1e-10), 4);

        assert_eq!(optimizer.learning_rate, 0.001);
        assert_eq!(optimizer.betas, (0.9, 0.999));
        assert_eq!(optimizer.epsilon, 1e-10);
        assert_eq!(optimizer.m, vec![0.0; 4]);
        assert_eq!(optimizer.v, vec![0.0; 4]);
        assert_eq!(optimizer.t, 0);
    }

    #[test]
    fn test_adam_init_all_custom_values() {
        let optimizer = Adam::new(Some(1.0), Some((0.001, 0.099)), Some(1e-1), 5);

        assert_eq!(optimizer.learning_rate, 1.0);
        assert_eq!(optimizer.betas, (0.001, 0.099));
        assert_eq!(optimizer.epsilon, 1e-1);
        assert_eq!(optimizer.m, vec![0.0; 5]);
        assert_eq!(optimizer.v, vec![0.0; 5]);
        assert_eq!(optimizer.t, 0);
    }

    #[test]
    fn test_adam_step_default_params() {
        let gradients = vec![-1.0, 2.0, -3.0, 4.0, -5.0, 6.0, -7.0, 8.0];

        let mut optimizer = Adam::new(None, None, None, 8);
        let updated_params = optimizer.step(&gradients);

        assert_eq!(
            updated_params,
            vec![
                0.0009999999900000003,
                -0.000999999995,
                0.0009999999966666666,
                -0.0009999999975,
                0.000999999998,
                -0.0009999999983333334,
                0.0009999999985714286,
                -0.00099999999875
            ]
        );
    }

    #[test]
    fn test_adam_step_custom_params() {
        let gradients = vec![9.0, -8.0, 7.0, -6.0, 5.0, -4.0, 3.0, -2.0, 1.0];

        let mut optimizer = Adam::new(Some(0.005), Some((0.5, 0.599)), Some(1e-5), 9);
        let updated_params = optimizer.step(&gradients);

        assert_eq!(
            updated_params,
            vec![
                -0.004999994444450618,
                0.004999993750007813,
                -0.004999992857153062,
                0.004999991666680556,
                -0.004999990000020001,
                0.004999987500031251,
                -0.004999983333388888,
                0.004999975000124999,
                -0.0049999500004999945
            ]
        );
    }

    #[test]
    fn test_adam_step_empty_gradients_array() {
        let gradients = vec![];

        let mut optimizer = Adam::new(None, None, None, 0);
        let updated_params = optimizer.step(&gradients);

        assert_eq!(updated_params, vec![]);
    }

    #[ignore]
    #[test]
    fn test_adam_step_iteratively_until_convergence_with_default_params() {
        const CONVERGENCE_THRESHOLD: f64 = 1e-5;
        let gradients = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];

        let mut optimizer = Adam::new(None, None, None, 6);

        let mut model_params = vec![0.0; 6];
        let mut updated_params = optimizer.step(&gradients);

        while (updated_params
            .iter()
            .zip(model_params.iter())
            .map(|(x, y)| x - y)
            .collect::<Vec<f64>>())
        .iter()
        .map(|&x| x.powi(2))
        .sum::<f64>()
        .sqrt()
            > CONVERGENCE_THRESHOLD
        {
            model_params = updated_params;
            updated_params = optimizer.step(&gradients);
        }

        assert!(updated_params < vec![CONVERGENCE_THRESHOLD; 6]);
        assert_ne!(updated_params, model_params);
        assert_eq!(
            updated_params,
            vec![
                -0.0009999999899999931,
                -0.0009999999949999929,
                -0.0009999999966666597,
                -0.0009999999974999929,
                -0.0009999999979999927,
                -0.0009999999983333263
            ]
        );
    }

    #[ignore]
    #[test]
    fn test_adam_step_iteratively_until_convergence_with_custom_params() {
        const CONVERGENCE_THRESHOLD: f64 = 1e-7;
        let gradients = vec![7.0, -8.0, 9.0, -10.0, 11.0, -12.0, 13.0];

        let mut optimizer = Adam::new(Some(0.005), Some((0.8, 0.899)), Some(1e-5), 7);

        let mut model_params = vec![0.0; 7];
        let mut updated_params = optimizer.step(&gradients);

        while (updated_params
            .iter()
            .zip(model_params.iter())
            .map(|(x, y)| x - y)
            .collect::<Vec<f64>>())
        .iter()
        .map(|&x| x.powi(2))
        .sum::<f64>()
        .sqrt()
            > CONVERGENCE_THRESHOLD
        {
            model_params = updated_params;
            updated_params = optimizer.step(&gradients);
        }

        assert!(updated_params < vec![CONVERGENCE_THRESHOLD; 7]);
        assert_ne!(updated_params, model_params);
        assert_eq!(
            updated_params,
            vec![
                -0.004999992857153061,
                0.004999993750007814,
                -0.0049999944444506185,
                0.004999995000005001,
                -0.004999995454549587,
                0.004999995833336807,
                -0.004999996153849113
            ]
        );
    }
}
