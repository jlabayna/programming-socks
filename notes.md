# Ch 1

## Hello, World!
Style:
- Rust prefers snakecase for files

Function anatomy:
```rust
fn main() {

}
```
- `main` runs first
- Parameters go in the parentheses
- Function body must be wrapped by curly braces

Formatting:
- `rustfmt`

Notes on code:
```rust
    println!("Hello, world!");
```
- Style:
  - Four spaces instead of tab
- `println!` is a macro, as indicated by `!`
- Most lines require a semicolon

Compilation:
- Required
- Compiled programs do not need Rust installed

## Cargo

[documentation](https://doc.rust-lang.org/cargo/)

### What?

Cargo is Rust build system and package manager.

Creating a new project:
```bash
$ cargo new hello_cargo
$ cd hello_cargo
```
- Creates a directory with the project name
- Creates `Cargo.toml` and `src/main.rs`
- Initializes a git repo with a `.gitignore`
  - Git files not generated if run in an existing git repo
  - Override non-generation with `cargo new --vcs=git`
    - Substitute `git` with whatever version control you want

TOML: Cargo's config format
```toml
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2021"

[dependencies]
```
- `[package]` section. Cargo needs:
  - name
  - version
  - edition (of rust)
- `[dependencies]` section for dependencies
  - Note that packages of code are referred to as "crates"

Organization:
- `src` for source
- top-level for README, license, config, and other non-code related
- To convert a pre-existing project to a cargo project:
  - Move source to `src` and create `Cargo.toml`

### Building and Running a Project

Commands:
- `cargo build`
- `cargo run`
- `cargo check`

```bash
$ cargo build
```
- Creates an executable in `target/debug/hello_cargo`
- On first run, creates `Cargo.lock`:
  - Tracks exact versions of dependencies
  - Managed by Cargo

```bash
$ cargo run
```
- Compiles and runs project
  - Only recompiles if source was modified

```bash
$ cargo check
```
- Quickly check if code will compile w/o producing executable

Building for release:
```bash
$ cargo build --release
```
- Optimizes code but takes time
