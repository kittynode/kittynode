> [!WARNING]
> The `v0.1.*` version means this is considered alpha software and the API will probably undergo breaking changes.

# stn

A command line tool for managing Taiko nodes.

Supported platforms:

- Apple Silicon macOS (aarch64-apple-darwin)
- Intel macOS (x86_64-apple-darwin)
- x64 Linux (x86_64-unknown-linux-gnu)
- x64 MUSL Linux (x86_64-unknown-linux-musl)
- x64 Windows (x86_64-pc-windows-msvc)

# Installation

Install `stn` with a single command:

```sh
curl -LsSf https://github.com/d1onys1us/stn/releases/latest/download/stn-installer.sh | sh
```

# Quickstart

1. Install a Taiko node: `stn install`.
2. Configure your Taiko node: `stn config`.
3. Start your Taiko node: `stn up`.

# Usage

Execute `stn help` for the full list of commands.

# Roadmap

Some cool stuff planned ahead:

- Adding support for a "setup wizard"
- Managing more node configs in ~/.stn (even aliasing eth-docker and managing those nodes, making stn like kubectl)
- Adding support for Taiko proposers / provers / SGX setup
- Adding support for other L2 nodes

If you have some other cool ideas feel free to open an issue! ヽ(・∀・)ﾉ
