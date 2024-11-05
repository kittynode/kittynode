# build the crates
build:
  cargo build

# start the docs dev server
docs:
  pnpm -F docs dev

# install icons
icons:
  cargo tauri icon assets/kittynode-logo.png && cargo tauri icon assets/kittynode-logo-no-padding.png -o tmp && mv tmp/ios/* packages/gui/src-tauri/gen/apple/Assets.xcassets/AppIcon.appiconset && rm -rf tmp

# install or update dev tools
install-dev-tools:
  cargo install cargo-edit cargo-llvm-cov cargo-nextest just tauri-cli

# run the kittynode cli with the given args
kittynode *args='':
  @if [ -z "{{args}}" ]; then target/debug/kittynode help; else target/debug/kittynode {{args}}; fi

# lint the javascript code
lint-js:
  pnpm -F gui format-lint

# lint and fix the javascript code
lint-js-fix:
  pnpm -F gui format-lint:fix

# lint the rust code
lint-rs:
  cargo clippy --all-targets --all-features -- -D warnings && cargo fmt --all -- --check

# add a shadcn component
shadcn-add *args='':
  cd packages/gui && pnpm dlx shadcn-svelte@next add {{args}} && pnpm format-lint:fix

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

# start the web server
web:
  cargo run -p kittynode-web
