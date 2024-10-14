# build the crates
build:
  cargo build

# run the tests with coverage
covtest:
  cargo llvm-cov nextest

# start the docs dev server
docs:
  @cd packages/docs && bun run dev

# run the kittynode cli with the given args
kittynode *args:
  target/debug/kittynode {{args}}

# publish the crates
publish-crates:
  cargo publish -p kittynode-core && cargo publish -p kittynode-cli

# start the tauri dev app
tauri:
  @cd crates/kittynode-tauri && cargo tauri dev

# build the tauri app for macOS
tauri-build-apple:
  @cd crates/kittynode-tauri && cargo tauri build --target aarch64-apple-darwin

# build the tauri app for Linux
tauri-build-linux:
  @cd crates/kittynode-tauri && cargo tauri build --target x86_64-unknown-linux-gnu

# run the tests
test:
  cargo nextest run

# run the tests without capturing output
test-no-capture:
  cargo nextest run --no-capture

# write the changelog
write-changelog:
  git cliff --bump -o
