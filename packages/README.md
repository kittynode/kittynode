Kittynode is a command center for node operators. This document gives a high level architectural overview.

- [kittynode-core](https://github.com/kittynode/kittynode/tree/main/packages/core): Core library for Kittynode written in Rust.
- [kittynode-cli](https://github.com/kittynode/kittynode/tree/main/packages/cli): A CLI frontend that binds to kittynode-core.
- [kittynode-gui](https://github.com/kittynode/kittynode/tree/main/packages/gui): A GUI frontend that binds to kittynode-core.
- [kittynode-web](https://github.com/kittynode/kittynode/tree/main/packages/web): An authenticated local web server that binds routes to kittynode-core.

## Technology used

- Core library written in Rust.
- Frontend written in Svelte.
- CLI is a cross-platform Rust binary.
- GUI is a cross-platform Tauri app that compiles the core library into the binary along with the frontend into a native webview, communicates with the core library over IPC, and is able to tap into native platform APIs.

## Development guide

See the [development guide](https://kittynode.io/development/development-guide) for more information on coding principles and how to contribute to the project.

## User facing apps

Kittynode has two user facing apps:

- A command line interface (CLI).
- A graphical user interface (GUI).

These user facing apps manage a kittynode through the core library.

## Capabilities

There are several capabilities you can add to your Kittynode which augment the threat model.

- **Read only (default)**: Kittynode is a read-only monitoring application.
- **Local only**: Kittynode can update local node infrastructure from the host machine.
- **Private onchain requests**: Kittynode can update local node infrastructure via listening to private requests submitted onchain.
- **Local HTTPS server**: Kittynode can update local node infrastructure via requests that come from the same Wireguard network (but a different machine, such as a phone); these requests are authenticated by a passkey or JWT token.

We believe the local HTTPS server over Wireguard is a convenient and secure approach. But the choice is up to you.

## A diagram

![Kittynode architecture diagram](../assets/diagrams/diagram.svg)

## Design

### Easy-to-use and secure

The workflow for Kittynode is designed to make it as easy as possible for the user to get started, while making the right recommendations for them along their operator journey. For example, a user may download Kittynode and sync an Ethereum testnet node in just a few clicks to get their feet wet.

However, they may later decide to become an independent staker on Ethereum mainnet, which requires a lot more security checks. Kittynode will guide the user through the necessary steps to get there, while keeping the low initial barrier to entry.

### Modular core library

Kittynode is architected as a backend library in Rust, providing several benefits:

- **Reusable core**: The desktop application is a consumer of this core library, with Tauri used to bind commands to the library.
- **Cross-platform support**: Kittynode supports a cross-platform desktop and mobile application as a frontend, along with a CLI that reuses the same core library.
- **Safety and performance**: Rust was chosen for its safety, performance, and cross-platform compatibility, making Kittynode easy to run on Windows, MacOS, and Linux.

### Direct container access

Kittynode doesn't use Docker CLI commands on the user's system directly. Instead it has its own module that directly interacts with the Docker engine with Bollard. This improves Kittynode's portability, security, and testability. It also allows for more flexible networking setups between Kittynode packages.

Kittynode also plans to support other free and open source container runtimes in the future, like containerd and moby.

## Features

### Package ecosystem

Kittynode supports a package ecosystem. Ethereum nodes are simply packages which are executed and managed by Kittynode. Developers can create their own packages easily with Kittynode's package API. Kittynode supports Docker images but also supports direct binary executables. Kittynode may also support other languages and executable scripts in the future.

The design goals of the package ecosystem are to ensure installs are:

- **Secure**: Packages are securely isolated from each other.
- **Consistent**: Packages behave the same on all systems.
- **Atomic**: Package installs and uninstalls are atomic, without polluting the system.

### System checker

The system checker is a utility within Kittynode that checks the system prerequisites for running a node, along with other system checks. A short list of some checks are:

- **System resources**: Checking the available storage, CPU, and RAM.
- **Network settings**: Checking the firewall and internet connectivity.
- **Security settings**: Checking file permissions and other system security settings.

The system checker is important in several areas of Kittynode. For example, when creating a validator key it is important to ensure WiFi is disabled, and file permissions are properly set on the key file.

### Remote access

Kittynode supports remote access. This means you can setup and monitor your node from a phone or desktop. This is done via secure connections with Wireguard, which allows users to monitor their node from trusted devices. This is also important so that users can easily upgrade their nodes, which is effectively voting.
