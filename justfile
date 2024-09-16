test:
  cargo nextest run

covtest:
  cargo llvm-cov nextest

kittynode *args:
  ./target/debug/kittynode {{args}}

tauri:
  @cd ./crates/kittynode_gui && bun run tauri dev

tauri-release:
  @cd ./crates/kittynode_gui && bun run tauri build --target aarch64-apple-darwin

release:
  git cliff --bump

release-o:
  git cliff --bump -o
