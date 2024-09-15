test:
    cargo nextest run

covtest:
    cargo llvm-cov nextest

kittynode *args:
    ./target/debug/kittynode {{args}}

tauri:
    @cd ./crates/kittynode_gui && bun run tauri dev

release:
  git cliff --bump -o
