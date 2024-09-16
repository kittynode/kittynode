> [!WARNING]  
> kittynode is under development and is not yet ready for use.

[![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/kittynode/kittynode/ci-rust.yml?branch=main&logo=GitHub%20Actions&label=ci-rust)](https://github.com/kittynode/kittynode/actions/workflows/ci-rust.yml?query=branch:main)
[![codecov](https://codecov.io/github/kittynode/kittynode/graph/badge.svg?token=TJAUBD8RPT)](https://codecov.io/github/kittynode/kittynode)

# kittynode

A tool for managing decentralized node infrastructure at home.

## Development

- We use [just](https://github.com/casey/just) for development tasks.
- For development speed we only release for aarch64-apple-darwin. But kittynode is cross-platform.
- For local signing use the key `tauri.conf.json` > `bundle` > `macOS` > `signingIdentity` with `security find-identity -v -p codesigning`. The value in `()` is fine.
