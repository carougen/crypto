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
├── crates/                     # Rust crates organized by textbook
│   ├── proofs_arguments_zeroknowledge/
│   │   ├── fingerprint/        # Reed–Solomon & Lagrange interpolation
│   │   │   ├── Cargo.toml
│   │   │   ├── src/lib.rs
│   │   │   └── examples/
│   │   ├── freivalds/          # Freivalds' algorithm
│   │   │   ├── Cargo.toml
│   │   │   ├── src/lib.rs
│   │   │   └── examples/
│   ├── (Other books with examples)
├── book/                       # mdBook documentation
│   ├── book.toml               # mdBook configuration
│   └── src/
│       ├── SUMMARY.md          # Table of contents
│       ├── intro.md            # Introduction & usage guide
│       ├── proofs_arguments_zeroknowledge.md
        ├── (Other books report)
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

### From "Proofs, Arguments, and Zero-Knowledge"

#### fingerprint
```bash
cargo run -p fingerprint --example rs_lagrange
cargo run -p fingerprint --example rs_polynomial
```

#### freivalds
```bash
cargo run -p freivalds --example freivalds
```

---

## 📖 Documentation & mdBook

To **serve** the book locally (with live reload):

```bash
cd book
mdbook serve
```

Then open [http://localhost:3000](http://localhost:3000) in your browser.

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
