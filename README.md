# Rustworthy

This repository is used as a scratchpad to learn Rust.

## Getting Started

Following the rich traditions of software development, I start my Rust adventure by
running a "Hello, World!" program in Rust.

Rust's [Get Started](https://www.rust-lang.org/learn/get-started) explains how to
install Rust and use Cargo, Rust's de facto standard build tool and package manager.

1. **Install Rust**: Since I use WSL2 for all my personal projects, I go with:

    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```

    This installs both Rust and Cargo.

2. **Create Hello World package**: Cargo has the `new` command to create a fully
   functional package:

   ```bash
   cargo new hello_world
   ```

   This creates a `main.rs` source file and the `Cargo.toml` manifest file.

3. **Build and execute the package**: Cargo's `build` command build the package.

    ```bash
    cargo build
    ```

    This creates the package lock file `Cargo.lock` and the executables under a
    `target` directory.

    ```bash
    ./target/debug/hello_world
    Hello, World!
    ```

4. **Commit this Hello World package to the repository**: Finally commit the source
    file, the manifest file and the lock file to the repository for future reference.

**Note**: I ommitted some intermediate steps in the above description as it is covered
in the [Cargo Book](https://doc.rust-lang.org/cargo/index.html)'s
[First Steps with Cargo](https://doc.rust-lang.org/cargo/getting-started/first-steps.html).
Rust also has the [Learn Rust](https://www.rust-lang.org/learn) documentation, which
I expect to use as a reference over time.

## Setting up Github Actions

Next I start setting up pipeline jobs using Github Actions. I will build upon this
pipeline to support the future usecases as I grow wiser about both Rust and Github
Actions.

To start with, I will run the pipeline on pushes to my sidebranch `setup/hello_world`.
The first job consists of checking out the code, setting up the rust toolchain, then
building and running the application. With Github Actions, this is simple enough to do
in a few minutes. The complete workflow is [here](.github/workflows/hello_world.yml).

[_side-note: I had previously used Github Actions for a few months, so I am reasonably
familiar with it. More recently, I have had to use Gitlab CI instead for about a year.
I have had some irritations with Github Actions' UX in the past, but when compared to
Gitlab CI's UX, Github Actions has started feeling like heaven._]
