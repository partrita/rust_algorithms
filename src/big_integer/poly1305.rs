use num_bigint::BigUint;
use num_traits::Num;
use num_traits::Zero;

macro_rules! hex_uint {
    ($a:literal) => {
        BigUint::from_str_radix($a, 16).unwrap()
    };
}

/**
 * Poly1305 메시지 인증 코드:
 * 이 구현은 RFC8439를 기반으로 합니다.
 * 사용 중인 Big Integer 라이브러리는 비일정 시간 연산으로 인해
 * 암호화 애플리케이션에 적합하지 않을 수 있습니다.
*/
pub struct Poly1305 {
    p: BigUint,
    r: BigUint,
    s: BigUint,
    /// 누산기
    pub acc: BigUint,
}

impl Default for Poly1305 {
    fn default() -> Self {
        Self::new()
    }
}

impl Poly1305 {
    pub fn new() -> Self {
        Poly1305 {
            p: hex_uint!("3fffffffffffffffffffffffffffffffb"), // 2^130 - 5
            r: Zero::zero(),
            s: Zero::zero(),
            acc: Zero::zero(),
        }
    }
    pub fn clamp_r(&mut self) {
        self.r &= hex_uint!("0ffffffc0ffffffc0ffffffc0fffffff");
    }
    pub fn set_key(&mut self, key: &[u8; 32]) {
        self.r = BigUint::from_bytes_le(&key[..16]);
        self.s = BigUint::from_bytes_le(&key[16..]);
        self.clamp_r();
    }
    /// 16바이트 길이의 메시지 블록을 처리합니다. 메시지가 충분히 길지 않으면,
    /// `msg` 배열을 0으로 채우지만 `msg_bytes`는 원래 청크 길이(바이트 단위)로 설정합니다.
    /// 사용 예는 `basic_tv1`을 참조하십시오.
    pub fn add_msg(&mut self, msg: &[u8; 16], msg_bytes: u64) {
        let mut n = BigUint::from_bytes_le(msg);
        n.set_bit(msg_bytes * 8, true);
        self.acc += n;
        self.acc *= &self.r;
        self.acc %= &self.p;
    }
    /// 결과는 16바이트 길이가 보장됩니다.
    pub fn get_tag(&self) -> Vec<u8> {
        let result = &self.acc + &self.s;
        let mut bytes = result.to_bytes_le();
        bytes.resize(16, 0);
        bytes
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt::Write;
    fn get_tag_hex(tag: &[u8]) -> String {
        let mut result = String::new();
        for &x in tag {
            write!(result, "{x:02x}").unwrap();
        }
        result
    }
    #[test]
    fn basic_tv1() {
        let mut mac = Poly1305::default();
        let key: [u8; 32] = [
            0x85, 0xd6, 0xbe, 0x78, 0x57, 0x55, 0x6d, 0x33, 0x7f, 0x44, 0x52, 0xfe, 0x42, 0xd5,
            0x06, 0xa8, 0x01, 0x03, 0x80, 0x8a, 0xfb, 0x0d, 0xb2, 0xfd, 0x4a, 0xbf, 0xf6, 0xaf,
            0x41, 0x49, 0xf5, 0x1b,
        ];
        let mut tmp_buffer = [0_u8; 16];
        mac.set_key(&key);
        mac.add_msg(b"Cryptographic Fo", 16);
        mac.add_msg(b"rum Research Gro", 16);
        tmp_buffer[..2].copy_from_slice(b"up");
        mac.add_msg(&tmp_buffer, 2);
        let result = mac.get_tag();
        assert_eq!(
            get_tag_hex(result.as_slice()),
            "a8061dc1305136c6c22b8baf0c0127a9"
        );
    }
}
