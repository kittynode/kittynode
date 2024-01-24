# Release workflow

Releases are fully automated:

1. Feature PR is merged into main
2. Merges to main trigger `release-plz` which opens a release PR with version bumps and changelog
3. Merging release PR will push a git tag and trigger `cargo-dist` to run
4. `cargo-dist` compiles for target platforms and releases to crates.io, then generates the GitHub release with changes and installer commands
