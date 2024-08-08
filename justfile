build-sample:
    cargo build --bin sample

build-sample-watch:
    cargo watch -x 'build --bin sample'

build-sample-release:
    cargo build --release --bin sample

run-sample:
    cargo run -p sample

run-sample-watch:
    cargo watch -x 'run --bin sample'

lint:
    cargo clippy

test:
    cargo test

