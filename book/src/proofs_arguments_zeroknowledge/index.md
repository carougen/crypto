# Proofs, Arguments & Zero-Knowledge

This section is dedicated to **_Proofs, Arguments, and Zero-Knowledge_** by Justin Thaler — a foundational book in modern cryptography focused on formal definitions, proof systems, and interactive arguments.


## 📘 Goals

This notebook serves as a **companion** to the book, including:

- ✍️ **Chapter-by-chapter notes**
- 🧠 **Key definitions and theorems**
- 🧪 **Rust implementations** of important constructs
- 📎 **Code examples** linked with real cryptographic behavior


## 🔧 Code Organization

Each major concept from the book is implemented as a separate crate in [`crates/proofs_arguments_zeroknowledge/`](../../../crates/proofs_arguments_zeroknowledge/):

- `fingerprint` → Reed–Solomon encoding & Lagrange interpolation
- `freivalds` → Freivalds' probabilistic matrix verifier
- _(more to come...)_

All examples can be run using `cargo run --example` and are documented inline.


## 🧭 Navigation

Use the menu to the left to explore the content chapter by chapter.

Each chapter contains:

- Key insights and intuitive explanations
- Rust examples with inline code or links to the crate
- References to the book’s definitions and theorems
