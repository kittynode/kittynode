# build the crates
build:
  cargo build

# start the docs dev server
docs:
  pnpm -F docs dev --open

# generate the kittynode-core docs
docs-rs:
  cargo doc -p kittynode-core

# install icons
icons:
  cargo tauri icon assets/kittynode-logo-app.png
  cargo tauri icon assets/kittynode-logo-square.png --ios-color '#A181A7' -o tmp
  mv tmp/ios/* packages/gui/src-tauri/gen/apple/Assets.xcassets/AppIcon.appiconset
  rm -rf tmp

# install or update dev tools
install-dev-tools:
  cargo install cargo-edit cargo-llvm-cov cargo-nextest just tauri-cli

# start the ios app on a physical device
ios:
  cargo tauri ios dev --force-ip-prompt -vvv

# make an ios build
ios-build:
  cargo tauri ios build

# init the ios app
ios-init:
  cargo tauri ios init
  cp -R packages/gui/src-tauri/gen-overrides/gen/* packages/gui/src-tauri/gen
  just icons

# start the ios app on a virtual device
ios-virtual:
  cargo tauri ios dev 'iPhone 16'

# run the kittynode cli with the given args
kittynode *args='':
  @if [ -z "{{args}}" ]; then target/debug/kittynode help; else target/debug/kittynode {{args}}; fi

# lint the javascript code
lint-js:
  pnpm -F docs -F gui format-lint

# lint and fix the javascript code
lint-js-fix:
  pnpm -F docs -F gui format-lint:fix

# lint the rust code
lint-rs:
  cargo clippy --all-targets --all-features -- -D warnings && cargo fmt --all -- --check

# add a shadcn component
shadcn-add *args='':
  cd packages/gui && pnpm dlx shadcn-svelte@next add {{args}} && pnpm format-lint:fix

# update shadcn components
shadcn-update:
  echo "todo"

# start the desktop app
tauri:
  cargo tauri dev

# build the tauri app for macOS
tauri-build-apple:
  cargo tauri build --target aarch64-apple-darwin

# build the tauri app for Linux
tauri-build-linux:
  cargo tauri build --target x86_64-unknown-linux-gnu

# run the unit tests
test:
  cargo nextest run

# run the unit and integration tests
test-all:
  cargo nextest run -- --include-ignored

# run the unit tests with coverage
test-coverage:
  cargo llvm-cov nextest

# run the unit and integration tests with coverage
test-coverage-all:
  cargo llvm-cov nextest -- --include-ignored

# update all toolchains and dependencies
update-all:
  rustup update
  just install-dev-tools
  cargo upgrade
  pnpm self-update
  pnpm up -r -L

# start the web server
web:
  cargo run -p kittynode-web
