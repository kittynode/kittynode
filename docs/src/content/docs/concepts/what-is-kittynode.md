---
title: What is Kittynode?
description: Learn about what Kittynode is and how it works.
---

:::note
This page describes Kittynode's current design; it doesn't reflect the current implementation progress.
:::

Kittynode is a safe and easy to use tool for running Ethereum nodes. Let's go over the motivation, design and features of Kittynode.

## Motivation

#### Node operators are important

The motivation behind Kittynode starts at the user. We believe that for Ethereum to direct power away from centralized entities and back into the hands of users, users must verify the correctness of the shared database through operating a node.

Additionally, a major part of Ethereum's security is bolstered by the consensus layer. Kittynode recognizes the importance of supporting node operators, and provides a tool that is easy and secure by default, while being flexible to meet the needs of the user.

#### User freedom above all

There is a motivation that goes beyond Ethereum itself. We believe that humans have the right to operate software in a way that upholds their freedom. This is important because Kittynode is not Ethereum maximalist, it is freedom maximalist. We use Ethereum because we believe Ethereum to be the best tool to achieve this goal. However, we happily accept other decentralized networks to extend Kittynode with their own packages for node operators to utilize. This further pushes Ethereum to prove itself while offering users maximum software freedom.

## Design

#### Easy to use and secure by default

The workflow for Kittynode is designed to make it as easy as possible for the user to get started, while making the right recommendations for them along their journey. For example, a user may download Kittynode just to try syncing an Ethereum testnet node to get their feet wet. However, they may later decide to become an independent staker on Ethereum, which requires a lot more security checks. Kittynode will guide the user through the necessary steps to get there, while removing any initial barriers to entry.

#### Modular core library

Kittynode is architected as a library in Rust. This means that the desktop application is merely a consumer of the core library. To start with, Kittynode supports a desktop app as a frontend. However, Kittynode can easily be extended to support a CLI, or mobile apps.

Rust was chosen for safety and performance, as well as the ability to support multiple platforms. Kittynode easily runs on Windows, MacOS, and Linux.

## Features

#### Package ecosystem

Kittynode supports a package ecosystem. Ethereum node combinations are simply packages which are executed and managed by Kittynode. Package developers can create their own packages. Kittynode supports Docker images but also supports direct binary executables. Kittynode may also support other languages and scripts in the future.

The primary design goals of the package ecosystem is to ensure:

1. Packages are isolated from each other.
2. Package installs and uninstalls are atomic.

#### Port manager

The port manager is a utility within Kittynode that manages the inbound and outbound ports for Kittynode packages. It is designed to be a central point where the user can configure the ports for their packages.

#### System checker

The system checker is a utility within Kittynode that checks the system prerequisites for running a node, along with other system checks. A list of some checks are:

- Checking the available storage.
- Checking the available CPU and RAM.
- Checking the firewall and other security settings.
- Checking if WiFi is enabled or disabled.

#### Remote mobile access

Kittynode supports remote access. This means you can setup and monitor your node from a phone or desktop. This is done through Wireguard, which will allow users to monitor their node from trusted devices. The other reason why this is important is to make it easy for users to upgrade their client software, which is effectively voting on Ethereum changes. Giving an easy UX for users here is important.
