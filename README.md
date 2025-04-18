# Crypto Notes

This repository brings together my **notes**, **Rust examples**, and **documentation** on cryptography. It is organized with a **Cargo workspace** for the Rust code and an **mdBook** for the documentation.

---

## 📂 Project Structure

```plaintext
crypto/                         # Repository root
├── README.md                   # This file
├── .gitignore                  # Git ignore rules
├── Cargo.toml                  # Cargo workspace definition
├── Cargo.lock                  # Cargo lockfile
├── crates/                     # Rust crates (one concept per crate)
│   ├── fingerprint/            # “fingerprint” library
│   │   ├── Cargo.toml
│   │   ├── src/lib.rs          # Reed–Solomon implementation
│   │   └── examples/           # Usage examples
│   ├── freivalds-matrix/       # Freivalds library
│   │   ├── Cargo.toml
│   │   └── src/lib.rs
│   │   └── examples/           # Usage examples
├── book/                       # mdBook documentation
│   ├── book.toml               # mdBook configuration
│   └── src/
│       ├── SUMMARY.md          # Table of contents
│       ├── intro.md            # Introduction & usage guide
│       ├── proofs-arguments-zero-knowledge.md
│       └── examples/           # Code snippets for the book
└── .github/                    # CI / GitHub Actions
    └── workflows/ci.yml        # Tests, build and deploy
```

---

## 🚀 Prerequisites

- **Rust** (via [rustup](https://rustup.rs/))
- **mdBook**:
  ```bash
  cargo install mdbook
  ```
- (Optional) **RustRover** or any Rust IDE (VSCode + rust-analyzer, IntelliJ Rust, etc.)

---

## 📦 Build & Tests

From the repository root:

```bash
# Compile all Rust crates
cargo build --all

# Run all tests
cargo test --all

# Generate the HTML for the book
cd book
mdbook build
```

---

## ▶️ Running a Rust Example

### fingerprint

```bash
# Run the reed-solomon with lagrange example for the fingerprint crate
cargo run -p fingerprint --example rs_lagrange
```

### freivalds

```bash
# Run tests or upcoming examples
cargo test -p freivalds --example freivalds
```

---

## 📖 Documentation & mdBook

To **serve** the book locally (with live reload):

```bash
cd book
mdbook serve
```

Open http://localhost:3000 in your browser.

---

## 🤝 Contributing

1. Fork the repository
2. Create a branch `feature/my-feature`
3. Add your code or notes
4. Submit a Pull Request

All improvements (examples, corrections, new chapters) are welcome!

---

## 📜 License

MIT
