set shell := ["bash", "-eu", "-o", "pipefail", "-c"]

default:
    cargo fmt --all -- --check
    cargo test --workspace

check:
    cargo check --workspace
