---
title: What is Kittynode?
description: Learn about what Kittynode is and how it works.
---

:::note
This page describes Kittynode's current design; it doesn't reflect the current implementation progress.
:::

Kittynode is an easy-to-use and secure-by-default tool for running Ethereum nodes. Let's go over the [design](#design) and [features](#features) of Kittynode.

## Design

#### Easy-to-use and secure

The workflow for Kittynode is designed to make it as easy as possible for the user to get started, while making the right recommendations for them along their operator journey. For example, a user may download Kittynode and sync an Ethereum testnet node in just a few clicks to get feet wet. However, they may later decide to become an independent staker on Ethereum mainnet, which requires a lot more security checks. Kittynode will guide the user through the necessary steps to get there, while removing any initial barriers to entry.

#### Modular core library

Kittynode is architected as a backend library in Rust, providing several benefits:

- Reusable core: The desktop application is a consumer of this core library, with Tauri used to bind commands to the library.
- Cross-platform support: Kittynode supports a cross-platform desktop and mobile application as a frontend, along with a CLI that reuses the same core library.
- Safety and performance: Rust was chosen for its safety, performance, and cross-platform compatibility, making Kittynode easy to run on Windows, MacOS, and Linux.

## Features

#### Package ecosystem

Kittynode supports a package ecosystem. Ethereum nodes are simply packages which are executed and managed by Kittynode. Package developers can easily create their own packages, with an easy API within Kittynode. Kittynode supports Docker images but also supports direct binary executables. Kittynode may also support other languages and executable scripts in the future.

The primary design goals of the package ecosystem is to ensure:

1. Packages are securely isolated from each other.
2. Packages run the same on all systems.
3. Package installs and uninstalls are atomic.

#### Port manager

The port manager is a utility within Kittynode that manages the inbound and outbound ports for Kittynode packages. It is designed to be a central point where the user can configure the ports for their packages. Users can easily view and manage what ports their system is exposing to the outside world and also between packages.

#### System checker

The system checker is a utility within Kittynode that checks the system prerequisites for running a node, along with other system checks. A short list of some checks are:

- Checking the available storage, CPU, and RAM.
- Checking the firewall and other security settings.
- Checking if WiFi is enabled or disabled.

#### Remote mobile access

Kittynode supports remote access. This means you can setup and monitor your node from a phone or desktop. This is done via secure connections with Wireguard, which allows users to monitor their node from trusted devices. The other reason why this is important is to make it easy for users to upgrade their client software, which is effectively voting on Ethereum changes.
