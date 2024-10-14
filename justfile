test:
  cargo nextest run

test-no-capture:
  cargo nextest run --no-capture

covtest:
  cargo llvm-cov nextest

docs:
  @cd ./packages/docs && bun run dev

kittynode *args:
  ./target/debug/kittynode {{args}}

tauri:
  @cd ./packages/crates/kittynode-tauri && cargo tauri dev

tauri-build-apple:
  @cd ./packages/crates/kittynode-tauri && cargo tauri build --target aarch64-apple-darwin

tauri-build-linux:
  @cd ./packages/crates/kittynode-tauri && cargo tauri build --target x86_64-unknown-linux-gnu

release-changelog:
  git cliff --bump -o

release-crates:
  cargo publish -p kittynode-core && cargo publish -p kittynode-cli
