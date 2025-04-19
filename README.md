# Exploring Cryptography With Rust

This repository brings together my **notes**, **Rust examples**, and **documentation** on cryptography. It is organized with a **Cargo workspace** for the Rust code and an **mdBook** for the documentation.

---

## 📂 Project Structure

```plaintext
exploring-cryptography-with-rust/                         # Repository root
├── README.md                   # Project overview and usage
├── .gitignore                  # Files and folders ignored by Git
├── Cargo.toml                  # Cargo workspace definition (lists all crates)
├── Cargo.lock                  # Cargo dependency lockfile
├── crates/                     # Rust crates grouped by textbook
│   ├── proofs_arguments_zeroknowledge/
│   │   ├── fingerprint/        # Reed–Solomon encoding and interpolation
│   │   │   ├── Cargo.toml
│   │   │   ├── src/lib.rs
│   │   │   └── examples/
│   │   ├── freivalds/          # Freivalds' algorithm crate
│   │   │   ├── Cargo.toml
│   │   │   ├── src/lib.rs
│   │   │   └── examples/
│   ├── introduction_to_modern_cryptography/
│   ├── ...
│   └── serious_cryptography/
├── book/                       # mdBook documentation
│   ├── book.toml               # mdBook configuration with KaTeX enabled
│   └── src/
│       ├── SUMMARY.md          # Table of contents (links to each book)
│       ├── intro.md            # Project introduction and usage
│       ├── proofs_arguments_zeroknowledge/
│       │   ├── index.md        # Overview of the book
│       │   ├── examples/       # Code snippets for documentation
│       │   ├── chapter1.md     # Notes on the first chapter
│       │   ├── chapter2.md     # Notes on the second chapter
│       │   ├── ...
│       │   └── images/         # (Optional) diagrams, visual content
│       ├── introduction_to_modern_cryptography/
│       ├── ...
│       └── serious_cryptography/
└── .github/
    └── workflows/
        └── ci.yml              # CI pipeline: test + mdBook + deploy to GitHub Pages
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
