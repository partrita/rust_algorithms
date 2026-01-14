# 알고리즘: Rust

이 프로젝트는 `Rust`로 구현된 일반적인 알고리즘을 보여주는 것을 목표로 하며, 관용적인 코드와 일반성에 중점을 둡니다.

## 프로젝트 구조

프로젝트는 다음과 같이 구성됩니다:

`src/`
  - `my_algo_category/` (내_알고리즘_카테고리)
    - `mod.rs`
    - `my_algorithm.rs` (내_알고리즘.rs)
    - `some_other_algorithm.rs` (다른_알고리즘.rs)
  - `some_other_algo_category/` (다른_알고리즘_카테고리)
    - ...


`mod.rs`에는 내보내기가 포함됩니다:

```rust
mod my_algorithm; // 내_알고리즘 모듈

pub use self::my_algorithm::my_algorithm; // 내_알고리즘::내_알고리즘 공개 사용
```

`my_algorithm.rs`에는 알고리즘과 관련 테스트가 포함됩니다:

```rust
pub fn my_algorithm() { // 내_알고리즘 함수 공개
    // ...
}

#[cfg(test)] // 테스트 구성
mod tests { // 테스트 모듈
    #[test] // 테스트 어트리뷰트
    fn my_test() { // 내_테스트 함수
        // ...
    }
}
```

## PR 제출 전

약어를 사용하지 **마십시오**: `DFS`는 `depth_first_search`여야 합니다.

다음을 실행했는지 확인하십시오
  * `cargo test`
  * `cargo fmt`
  * `cargo clippy --all -- -D warnings`

  이것으로 끝입니다!
