# stn

[![GitHub Release](https://img.shields.io/github/v/release/libreth/stn?logo=github)](https://github.com/libreth/stn/releases)
[![Crates.io Version](https://img.shields.io/crates/v/stn?logo=rust)](https://crates.io/crates/stn)
[![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/libreth/stn/ci.yml?branch=main&logo=GitHub%20Actions&label=ci)](https://github.com/libreth/stn/actions/workflows/ci.yml?query=branch:main)
[![codecov](https://codecov.io/gh/libreth/stn/graph/badge.svg?token=TJAUBD8RPT)](https://codecov.io/gh/libreth/stn)

A command line tool for managing Taiko nodes.

![screenshot of cli tool](.github/readme_cli_screenshot.png)

> Screenshot of `v0.1.27`, but `stn` is getting updated all the time.

## Installing and updating

Installing and updating use the same command.

#### Cargo (recommended)

1. [Install Rust](https://www.rust-lang.org/tools/install)
2. Install or update `stn` with cargo:
   ```bash
   cargo install stn
   ```

#### Linux

Inside of a terminal:

```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/libreth/stn/releases/latest/download/stn-installer.sh | sh
```

#### Windows

Inside of a Powershell window:

```bash
irm https://github.com/libreth/stn/releases/latest/download/stn-installer.ps1 | iex
```

#### MacOS

Inside of a terminal:

```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/libreth/stn/releases/latest/download/stn-installer.sh | sh
```

## Quickstart

1. Install a Taiko node: `stn install`.
2. Configure your Taiko node: `stn config`.
3. Start your Taiko node: `stn up`.

## Usage

Execute `stn` for the full list of commands and their description.

## Contribute

If you have any feature requests or bug reports please [open an issue](https://github.com/libreth/stn/issues/new)! ヽ(・∀・)ﾉ
