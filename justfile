test:
  cargo nextest run

covtest:
  cargo llvm-cov nextest

kittynode *args:
  ./target/debug/kittynode {{args}}

tauri:
  @cd ./crates/kittynode_gui && bun run tauri dev

release-tauri:
  @cd ./crates/kittynode_gui && bun run tauri build --target aarch64-apple-darwin

release-changelog:
  git cliff --bump -o

release-crates:
  cargo publish -p kittynode_core && cargo publish -p kittynode
