# 📦 lcli

**lcli** is a simple command-line Rust application that prints a greeting message based on user input.

It uses the **Clap** argument parser to accept a `name` string and prints:

    Hello, <name>!

Example usage:

    lcli --name Alice
    # outputs: Hello, Alice!

---

## 🧠 Features

- Simple CLI built with **Clap**
- Cross-platform Rust executable
- Configurable via a `--name` argument

---

## 🚀 Installation

### From source

Clone the repository:

    git clone https://github.com/gonardfreeman/lcli.git
    cd lcli

Build the project:

    cargo build --release

The binary will be under:

    target/release/lcli

### Install via script (curl)

You can install `lcli` easily using the provided install script:

    curl -sSL https://raw.githubusercontent.com/gonardfreeman/lcli/main/install.sh | bash

This will download the latest release for your platform and place the binary in `/usr/local/bin` (or prompt for a path).

---

## 🎯 Usage

Run without arguments:

    lcli
    # Hello, shuttle!

Or specify a name:

    lcli --name Alice
    # Hello, Alice!

---

## ⚙️ Command Line Options

| Flag                 | Description                               |
|----------------------|-------------------------------------------|
| `-n`, `--name <NAME>` | Set the name to greet (default: `shuttle`) |

---

## 📦 Releases

Precompiled releases are available in the **Releases** section on GitHub (e.g., `v0.1.5`).  
[https://github.com/gonardfreeman/lcli/releases](https://github.com/gonardfreeman/lcli/releases)

---

## 🛠️ Development

To run locally in debug mode:

    cargo run -- --name <your name>

To build and optimize:

    cargo build --release

---

## 📁 Project Structure

- `src/main.rs` — primary source file  
- `Cargo.toml` — project manifest  
- `install.sh` — optional install helper script

---

## 📝 License

`lcli` is open-source and free to use under the MIT License.

You can copy or modify it freely, and redistribute under the same license.

---

## 👍 Contributing

Contributions are welcome! You can:

- open issues  
- submit pull requests  
- suggest improvements

---

## 🧩 Notes

- Consider renaming the binary in `Cargo.toml` so that the output file is named `lcli` instead of the default crate name.  
- Add documentation, examples, and tests as the project grows.
