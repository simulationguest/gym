test:
    cargo test

format:
    cargo +nightly fmt

lint:
    cargo +nightly clippy
    pnpm -C webapp lint

web:
    pnpm -C webapp dev
