// N의 K번째 약수
// 아이디어는 [N, 1] 범위의 각 숫자를 확인하고 N을 완전히 나누는 K번째 숫자를 출력하는 것입니다.

pub fn kth_factor(n: i32, k: i32) -> i32 {
    let mut factors: Vec<i32> = Vec::new();
    let k = (k as usize) - 1;
    for i in 1..=n {
        if n % i == 0 {
            factors.push(i);
        }
        if let Some(number) = factors.get(k) {
            return *number;
        }
    }
    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(kth_factor(12, 3), 3);
    }

    #[test]
    fn test_2() {
        assert_eq!(kth_factor(7, 2), 7);
    }

    #[test]
    fn test_3() {
        assert_eq!(kth_factor(4, 4), -1);
    }

    #[test]
    fn test_4() {
        assert_eq!(kth_factor(950, 5), 19);
    }
}
