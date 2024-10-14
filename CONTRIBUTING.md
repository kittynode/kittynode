# Contributing

## Development

- We use [just](https://github.com/casey/just) for development tasks. Install just.
- This is a monorepo, you can run `just tauri` to run the GUI, or `just docs` to run the docs. The just commands work everywhere. See the [justfile](./justfile) for more details.

## Packages

What packages are in this repo? The main packages are:

- `docs` -> this publishes to GitHub Pages
- `crates/kittynode-cli` -> this publishes to crates.io
- `crates/kittynode-core` -> this publishes to crates.io
- `crates/kittynode-gui` -> this publishes to GitHub Releases

Everything in `crates` shares the same version number. Hmm, then how can they be released independently? Let's say we improve the CLI with a bug fix but we don't bump the core library.

## Releases

> Main consideration to address: how do we handle multi-package releases? (maybe not too hard, but still)

- Commits to main:
  - Creates release PRs automatically.
  - Uses git-cliff to output version bumps and changelogs to the release branch PR.
  - Create release branch named `release/vX.Y.Z`.
- Merging a release branch:
  - Runs tests.
  - If successful, tags the repo with `vX.Y.Z`.
- The pushed tag triggers the build and release workflow:
  - Build the crates and release to crates.io.
  - Build the Tauri GUI and release to GitHub with changelogs.
