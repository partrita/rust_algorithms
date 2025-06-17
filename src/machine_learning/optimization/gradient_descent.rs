/// 경사 하강법 최적화 (Gradient Descent Optimization)
///
/// 경사 하강법은 함수의 최솟값을 찾는 데 사용되는 반복적인 최적화 알고리즘입니다.
/// 함수의 값이 가장 가파르게 감소하는 방향으로 매개변수(이 경우 벡터 `x`의 요소)를 업데이트하여 작동합니다.
/// 이는 현재 지점에서 함수의 기울기를 현재 지점에서 빼서 달성됩니다. 학습률은 단계 크기를 제어합니다.
///
/// 단일 매개변수(일변량)에 대한 방정식은 다음과 같습니다:
/// x_{k+1} = x_k - 학습률 * 함수_미분(x_k)
///
/// 다변량 함수의 경우 각 매개변수로 확장됩니다:
/// x_{k+1} = x_k - 학습률 * 함수_기울기(x_k)
///
/// # 인수
///
/// * `derivative_fn` - 주어진 지점에서 목적 함수의 기울기를 계산하는 함수입니다.
/// * `x` - 최적화할 초기 매개변수 벡터입니다.
/// * `learning_rate` - 각 반복의 단계 크기입니다.
/// * `num_iterations` - 최적화를 실행할 반복 횟수입니다.
///
/// # 반환 값
///
/// 최적화된 매개변수 벡터 `x`에 대한 참조입니다.

pub fn gradient_descent(
    derivative_fn: impl Fn(&[f64]) -> Vec<f64>,
    x: &mut Vec<f64>,
    learning_rate: f64,
    num_iterations: i32,
) -> &mut Vec<f64> {
    for _ in 0..num_iterations {
        let gradient = derivative_fn(x);
        for (x_k, grad) in x.iter_mut().zip(gradient.iter()) {
            *x_k -= learning_rate * grad;
        }
    }

    x
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_gradient_descent_optimized() {
        fn derivative_of_square(params: &[f64]) -> Vec<f64> {
            params.iter().map(|x| 2. * x).collect()
        }

        let mut x: Vec<f64> = vec![5.0, 6.0];
        let learning_rate: f64 = 0.03;
        let num_iterations: i32 = 1000;

        let minimized_vector =
            gradient_descent(derivative_of_square, &mut x, learning_rate, num_iterations);

        let test_vector = [0.0, 0.0];

        let tolerance = 1e-6;
        for (minimized_value, test_value) in minimized_vector.iter().zip(test_vector.iter()) {
            assert!((minimized_value - test_value).abs() < tolerance);
        }
    }

    #[test]
    fn test_gradient_descent_unoptimized() {
        fn derivative_of_square(params: &[f64]) -> Vec<f64> {
            params.iter().map(|x| 2. * x).collect()
        }

        let mut x: Vec<f64> = vec![5.0, 6.0];
        let learning_rate: f64 = 0.03;
        let num_iterations: i32 = 10;

        let minimized_vector =
            gradient_descent(derivative_of_square, &mut x, learning_rate, num_iterations);

        let test_vector = [0.0, 0.0];

        let tolerance = 1e-6;
        for (minimized_value, test_value) in minimized_vector.iter().zip(test_vector.iter()) {
            assert!((minimized_value - test_value).abs() >= tolerance);
        }
    }
}
