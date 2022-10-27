# Changelog

## Unreleased

These are the changes that have not been released yet. The release notes have not yet been written.

**FEATURE ENCHANCEMENTS:**

- Re-add configuration file `podmod.conf`.
    - Specify kernel module parameters in configuration file.
    - Specify container image build parameters in configuration file.
    - Tag images with module version from configuration file.

- Add `--no-prune` option to `build`.

- Add `--idempotent` option to `build`, `load`, and `unload`.
    - Use in systemd unit.

- Add `#[derive(Debug)]` to all structs in `main.rs`.

- Document systemd unit file in `README.md`.

## v0.3.5 (2022-10-18)

**FEATURE ENCHANCEMENTS:**

- Prune old images after building module.

**BUG FIXES:**

- `module_is_supported()` function checking for wrong path.

## v0.3.4 (2022-10-18)

**FEATURE ENCHANCEMENTS:**

- Add systemd unit service to load modules on boot.

- Add `CHANGELOG.md` into RPM package.

- Add note on Podman in Podman to installation instructions.

- Add optional `--test` flag in build instructions.

**BUG FIXES:**

- Add missing checks for subcommands.

## v0.3.3 (2022-10-17)

**WORKFLOW CHANGES:**

- Remove Copr integration webhook from GitHub repository.

- Add GitHub Action to publish package to both [crates.io](https://crates.io) and Copr.

**BUG FIXES:**

- Update `README.md` for Rust rewrite (forgotten in last release).
