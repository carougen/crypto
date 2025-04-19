# Chapter 1 – Introduction to Proof Systems

This chapter introduces **verifiable computing** and the models used to formalize cryptographic proofs, such as **interactive proofs**, **arguments**, and **zero-knowledge proofs**. The motivation is to enable efficient verification of computations — without having to recompute them.



## 🎯 Motivation: Delegating Computation

A verifier wants to **outsource a computation** to a powerful prover, but **verify** the result quickly.

- Traditional solution: redo the computation — inefficient.
- Modern solution: ask the prover to **prove** that the computation was correct.

This motivates the use of **proof systems**: formal protocols that allow a prover to convince a verifier that a statement is true.



## 🧾 Basic Notions

Let $x$ be a binary input string and $L$ a language.

A **proof system** for $L$ is a protocol between a **prover** $P$ and a **verifier** $V$ such that:

- **Completeness**: If $x \in L$, then $P$ can convince $V$ to accept.
- **Soundness**: If $x \notin L$, then no cheating prover can convince $V$, except with small probability.

The verifier is typically **efficient** (polynomial time), while the prover may be **unbounded** (in IPs) or polynomially bounded (in **arguments**).



## 📡 Interactive Proofs (IPs)

- **Two-party protocols** $(P, V)$
- $V$ is probabilistic and polynomial-time
- $P$ may be computationally unbounded
- The protocol consists of **multiple rounds** of communication

> We define completeness and soundness *with respect to the randomness* of $V$ and possible strategies of $P$.



## 🔐 Example: Proving Password Knowledge

Suppose Alice has a password $x$ and commits to it using a hash function $z = h(x)$.

She wants to prove knowledge of $x$ without revealing it.

- Naive approach: send $x$ → leaks secret
- Better: **zero-knowledge proof** (ZKP) for: “I know $x$ such that $h(x) = z$”

> ZKPs prove *membership* in a language without leaking the witness.



## 📦 Other Models

| Model | Description |
|-------|-------------|
| IPs   | Interactive Proofs (strong soundness, expensive prover) |
| Arguments | Sound only against efficient (poly-time) provers |
| MIPs  | Multiple independent provers |
| PCPs  | Probabilistic Checkable Proofs (static proofs, spot-checked) |
