Setup the Rust environment
source "$HOME/.cargo/env"


This installed Rust 1.64.0 on my LiveLab container.

rustc -V
cargo -V


# Dependencies
oracle = { version = "0.5", features = ["chrono"] }

- https://github.com/minio/minio/issues/14498
- https://github.com/kubo/rust-oracle
