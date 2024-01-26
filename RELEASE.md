## Release workflow

Releases are fully automated with `release-plz`, `cargo-dist`, and `git-cliff`:

1. Feature PR is merged into main.
2. The feature PR merge to main triggers the `release-plz` workflow, which opens a release PR with the version bumps and changelog.
3. The release PR merge to main triggers the `release-plz` workflow (again), this time pushing a git tag for the release.
4. The git tag pushed to the repo triggers the `cargo-dist` workflow, which compiles for target platforms, releases to target distribution centers (crates.io, homebrew, etc.), and publishes all release artifacts into a GitHub release along with installation scripts.

## CICD workflow

#### Lightweight tests on PR

We only run lightweight jobs on PR (lints and unit tests), for example:

- cargo fmt
- crate-ci/typos
- clippy
- unit tests + codecov
- semantic PR title

#### Heavy tests on (unstable) main

We run heavier tests on main to check if we are stable and ready to cut a release. For example:

- heavier integration / e2e tests
- performance tests
