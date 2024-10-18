# build the crates
build:
    cargo build

# start the docs dev server
docs:
    cd docs && bun run dev

# run the kittynode cli with the given args
kittynode *args:
    target/debug/kittynode {{args}}

# start the tauri dev app
tauri:
    cd apps/kittynode-gui/kittynode-tauri && cargo tauri dev

# build the tauri app for macOS
tauri-build-apple:
    cd apps/kittynode-gui/kittynode-tauri && cargo tauri build --target aarch64-apple-darwin

# build the tauri app for Linux
tauri-build-linux:
    cd apps/kittynode-gui/kittynode-tauri && cargo tauri build --target x86_64-unknown-linux-gnu

# build the tauri app for windows:
tauri-build-windows:
    cd apps/kittynode-gui/kittynode-tauri && cargo tauri build --target x86_64-pc-windows-msvc

# run the tests
test:
    cargo nextest run

# run the tests with coverage
test-coverage:
    cargo llvm-cov nextest

# run the tests without capturing output
test-no-capture:
    cargo nextest run --no-capture
