<!-- markdownlint-disable MD013 -->

# Development Setup

## Getting Started

1. **Setting Up Rust**

   In order to work with this project you'll need to setup a working
   Rust development environment that includes `rustc` and `cargo`.

   - For `OSX` and `Linux`:

     ```bash
     curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
     ```

   - For `Windows`:

     [Download the installer] and follow the onscreen instructions.

     [Download the installer]: https://win.rustup.rs/x86_64

2. **Install `cargo-binstall`**

   Most of the workflows in this project lean heavily upon cargo binary
   packages. While we could simply `cargo install <package>`, this allows
   you to install a pre-compiled binary for your environment and avoids
   a possible lengthy compile time.

   - For `OSX` and `Linux`:

   ```bash
   curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
   ```

   - For `Windows`:

   ```bash
   Set-ExecutionPolicy Unrestricted -Scope Process; iex (iwr "https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.ps1").Content
   ```

3. **Install `just` Build Tool**

   `just` is the preferred build tool for this project, and that includes
   bootstrapping the remainder of your local environment. It can be
   installed with the following command:

   ```bash
   cargo binstall just
   ```

4. **Finish Bootstrapping Environment**

   Once you have a working rust environment you can finish setting up the
   remainder of this project locally with the following command:

   ```bash
   just init
   ```

5. Install `mdbook-journal` Locally

   Create a production build of the binary and install it to your
   crate local binary location:

   ```bash
   just install
   ```
