test:
    cargo nextest run

covtest:
    cargo llvm-cov nextest

kittynode *args:
    ./target/debug/kittynode {{args}}

tauri:
    pnpm tauri dev
