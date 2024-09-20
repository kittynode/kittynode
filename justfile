test:
  cargo nextest run

test-no-capture:
  cargo nextest run --no-capture

covtest:
  cargo llvm-cov nextest

kittynode *args:
  ./target/debug/kittynode {{args}}

tauri:
  @cd ./crates/kittynode_gui && bun run tauri dev

release-tauri-apple:
  @cd ./crates/kittynode_gui && bun run tauri build --target aarch64-apple-darwin

release-tauri-linux:
  @cd ./crates/kittynode_gui && bun run tauri build --target x86_64-unknown-linux-gnu

release-changelog:
  git cliff --bump -o

release-crates:
  cargo publish -p kittynode_core && cargo publish -p kittynode
