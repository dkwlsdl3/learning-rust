# Learning Rust

AI에게 구현을 전부 맡기지 않고, 직접 Rust 코드를 읽고 작성하는 감각을 만드는 개인 학습
프로젝트입니다.

## 학습 순서

1. 운동 기록 요약 CLI: 타입, 파싱, `Result`, 컬렉션, 소유권
2. 허프만 압축기: 트리, `BinaryHeap`, `Box`, 비트 처리
3. 비동기 미니 프로그램: `async`, `Send`, `Sync`, 회사 코드와 연결

## 첫 과제 실행 목표

```bash
cargo run -- fixtures/workouts.csv
```

예상 결과의 핵심 값은 다음과 같습니다.

```text
벤치프레스: 9세트, 볼륨 3200kg
스쿼트: 10세트, 볼륨 5062.5kg
최고 볼륨: 스쿼트
```

상세 요구사항은 `TODO.md`를 봅니다. AI는 기본적으로 완성 답안을 작성하지 않고 개념 설명,
테스트 설계와 코드 리뷰를 담당합니다.

## 검증

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```
