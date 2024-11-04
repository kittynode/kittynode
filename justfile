# start the axum server
axum:
  cargo run -p kittynode-axum

# build the crates
build:
  cargo build

# start the docs dev server
docs:
  pnpm -F docs dev

# install dev tools
install-dev-tools:
  cargo install cargo-edit cargo-llvm-cov cargo-nextest tauri-cli

# run the kittynode cli with the given args
kittynode *args='':
  @if [ -z "{{args}}" ]; then target/debug/kittynode help; else target/debug/kittynode {{args}}; fi

# lint the javascript code
lint-js:
  pnpm -F kittynode-frontend format-lint

# lint and fix the javascript code
lint-js-fix:
  pnpm -F kittynode-frontend format-lint:fix

# lint the rust code
lint-rs:
  cargo clippy --all-targets --all-features -- -D warnings && cargo fmt --all -- --check

# start the desktop app
tauri:
  cargo tauri dev

# start the ios app
tauri-ios:
  cargo tauri ios dev

# init the ios app
tauri-ios-init:
  cargo tauri ios init

# build the tauri app for macOS
tauri-build-apple:
  cargo tauri build --target aarch64-apple-darwin

# build the tauri app for Linux
tauri-build-linux:
  cargo tauri build --target x86_64-unknown-linux-gnu

# run the tests
test:
  cargo nextest run

# run the tests with coverage
test-coverage:
  cargo llvm-cov nextest

# run the tests without capturing output
test-no-capture:
  cargo nextest run --no-capture
