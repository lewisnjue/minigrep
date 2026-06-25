# minigre

A small command-line search tool written in Rust while learning *The Rust Programming Language*. It behaves like a simplified `grep` and supports case-insensitive search using the `IGNORE_CASE` environment variable.

[![Release](https://github.com/lewisnjue/minigrep/actions/workflows/release.yml/badge.svg)](https://github.com/lewisnjue/minigrep/actions/workflows/release.yml)

## Features

- Search for a query string in a file
- Case-sensitive search by default
- Case-insensitive search when `IGNORE_CASE` is set
- Release automation for Linux, macOS, and Windows

## Usage

Build and run locally:

```bash
cargo run -- "pattern" example.txt
```

Example:

```bash
IGNORE_CASE=1 cargo run -- "rust" poem.txt
```

## Install

```bash
cargo install --path .
```

## Release Automation

A GitHub Actions workflow builds release binaries for:

- Linux: `x86_64-unknown-linux-gnu`
- macOS: `x86_64-apple-darwin`
- Windows: `x86_64-pc-windows-msvc`

A release is published automatically when a tag matching `v*.*.*` is pushed.

## Releases

Download published release assets from the GitHub releases page:

- https://github.com/lewisnjue/minigrep/releases

## License

This project is licensed under the MIT License.
