# Contributing

## Development

- We use [just](https://github.com/casey/just) for development tasks. Install just.
- This is a monorepo, you can run `just tauri` to run the GUI, or `just docs` to run the docs. The just commands work everywhere. See the [justfile](./justfile) for more details.

## Releases

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
