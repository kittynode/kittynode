test:
  cargo nextest run

test-no-capture:
  cargo nextest run --no-capture

covtest:
  cargo llvm-cov nextest

kittynode *args:
  ./target/debug/kittynode {{args}}

docs:
  @cd ./docs && bun run dev

frontend:
  @cd ./crates/kittynode-gui && bun run dev

frontend-build:
  @cd ./crates/kittynode-gui && bun run build

tauri:
  @cd ./crates/kittynode-tauri && cargo tauri dev

tauri-build-apple:
  @cd ./crates/kittynode-tauri && cargo tauri build --target aarch64-apple-darwin

tauri-build-linux:
  @cd ./crates/kittynode-tauri && cargo tauri build --target x86_64-unknown-linux-gnu

release-changelog:
  git cliff --bump -o

release-crates:
  cargo publish -p kittynode-core && cargo publish -p kittynode-cli
