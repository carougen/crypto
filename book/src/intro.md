# Introduction

**Crypto** is a hands‑on companion to modern cryptography textbooks, built entirely in Rust. Instead of just reading about algorithms, you can explore self‑contained Rust crates that implement and demonstrate each concept, from error‑correcting codes to probabilistic proofs.

## 🚀 Project Goals

- **Bridge Theory and Practice**  
  Turn textbook pseudocode into real, tested Rust libraries.

- **Learn by Example**  
  Run and modify examples to see how Reed–Solomon encoding, Freivalds’ algorithm, zero‑knowledge proofs, and more really work.

- **Organized by Book**  
  Each crate lives under a folder named for the source text (e.g. _Proofs, Arguments, and Zero‑Knowledge_), letting you browse code chapter by chapter.

## 📚 What’s Inside

- **`fingerprint`**  
  Reed–Solomon encoding & Lagrange interpolation examples (from _Proofs, Arguments, and Zero‑Knowledge_).

- **`freivalds-matrix`**  
  Freivalds’ probabilistic matrix multiplication verifier.

- _(…and more crates to come, each tied to a classic crypto book.)_

Each crate includes:
1. A minimal `Cargo.toml`.
2. A `src/lib.rs` implementing the core routines.
3. An `examples/` directory showing how to generate codewords, test proofs, or verify matrices.

## 🛠️ Getting Started

1. **Clone the repo**
   ```bash
   git clone https://github.com/carougen/crypto.git
   cd crypto
   ```

2. **Build all crates**
   ```bash
   cargo build --all
   ```

3. **Run some examples**
    - Reed–Solomon by polynomial evaluation:
      ```bash
      cargo run --example rs_polynomial --package fingerprint
      ```
    - Reed–Solomon by Lagrange interpolation:
      ```bash
      cargo run --example rs_lagrange --package fingerprint
      ```
    - Freivalds’ matrix verifier:
      ```bash
      cargo run --example freivalds --package freivalds
      ```

4. **Run the test suite**
    - All crates at once:
      ```bash
      cargo test --all
      ```
    - Or per crate:
      ```bash
      cargo test --package fingerprint
      cargo test --package freivalds-matrix
      ```

## 🤝 Contributing

Contributions are welcome! To add a new chapter or book:

1. Create a new folder under `crates/<book-slug>/…`
2. Add your crate with `Cargo.toml`, `src/lib.rs`, and `examples/`
3. Update the `[workspace]` members in the root `Cargo.toml`
4. Send a pull request!

---

Dive in to see theory come alive in Rust & happy coding!
