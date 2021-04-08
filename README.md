## Dev

```
cargo clippy --all-features -- -D clippy::all
cargo +nightly clippy --all-features -- -D clippy::all

cargo fmt -- --check

cargo build-all-features
cargo test-all-features -- --nocapture
```
