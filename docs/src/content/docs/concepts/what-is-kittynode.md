---
title: What is Kittynode?
description: Learn about what Kittynode is and how it works.
---

:::note
This page describes Kittynode's current design; it doesn't reflect the current implementation progress.
:::

Kittynode is an easy to use and secure by default tool for running Ethereum nodes. Let's go over the design and features of Kittynode.

## Design

#### Easy to use and secure by default

The workflow for Kittynode is designed to make it as easy as possible for the user to get started, while making the right recommendations for them along their operator journey. For example, a user may download Kittynode and sync an Ethereum testnet node in just a few clicks to get feet wet. However, they may later decide to become an independent staker on Ethereum mainnet, which requires a lot more security checks. Kittynode will guide the user through the necessary steps to get there, while removing any initial barriers to entry.

#### Modular core library

Kittynode is architected as a backend library in Rust. This means that the desktop application is merely a consumer of the core library (using Tauri to bind commands to the core library). Kittynode supports a cross-platform desktop and mobile application as a frontend. However Kittynode can and will support a CLI, which can simply reuse the core library.

Rust was chosen for safety and performance, as well as the ability to support multiple platforms. Kittynode easily runs on Windows, MacOS, and Linux.

## Features

#### Package ecosystem

Kittynode supports a package ecosystem. Ethereum node combinations are simply packages which are executed and managed by Kittynode. Package developers can create their own packages. Kittynode supports Docker images but also supports direct binary executables. Kittynode may also support other languages and scripts in the future.

The primary design goals of the package ecosystem is to ensure:

1. Packages are securely isolated from each other.
2. Package installs and uninstalls are atomic.

#### Port manager

The port manager is a utility within Kittynode that manages the inbound and outbound ports for Kittynode packages. It is designed to be a central point where the user can configure the ports for their packages. Users can easily view and manage what ports their system is exposing to the outside world and also between packages.

#### System checker

The system checker is a utility within Kittynode that checks the system prerequisites for running a node, along with other system checks. A list of some checks are:

- Checking the available storage.
- Checking the available CPU and RAM.
- Checking the firewall and other security settings.
- Checking if WiFi is enabled or disabled.

#### Remote mobile access

Kittynode supports remote access. This means you can setup and monitor your node from a phone or desktop. This is done through Wireguard, which will allow users to monitor their node from trusted devices. The other reason why this is important is to make it easy for users to upgrade their client software, which is effectively voting on Ethereum changes.
