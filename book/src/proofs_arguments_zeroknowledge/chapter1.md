# Chapter 1 – Introduction

**Verifiable Computing (VC)** enables an untrusted **prover** to convince a lightweight **verifier** that a computation was executed correctly **without** the verifier re‑running the entire task.



## 🎯 Core Objectives

- **Correctness Assurance**  
  Ensure that outsourced or untrusted computations yield correct results.
- **Verifier Efficiency**  
  Verification should be significantly cheaper (time, space, or communication) than re‑execution.
- **Robust Soundness**  
  Malicious provers cannot cheat except with negligible probability.
- **(Optional) Zero‑Knowledge**  
  Proofs may leak no additional information beyond the statement’s validity.



## ☁️ Motivation: Delegated Computation

1. **Cloud Services**  
   Clients offload heavy tasks (data analytics, ML inference) to servers.
2. **Resource Constraints**  
   Embedded devices, IoT, or blockchains cannot re‑compute large workloads.
3. **Trust Minimization**  
   Even if the server is malicious, clients gain confidence via cryptographic proofs.



## 🔍 Formal Model of a Proof System

A **proof system** for language $L$ (or NP relation $R$) is a pair $(\mathcal{P},\mathcal{V})$:

- **Prover** $\mathcal{P}(x,w)$
    - Input: instance $x$ and witness $w$ such that $(x,w)\in R$.
    - Output: proof transcript $\pi$.

- **Verifier** $\mathcal{V}(x,\pi)$
    - Input: $x$ and $\pi$.
    - Output: $\texttt{accept}$ or $\texttt{reject}$ in probabilistic polynomial time.

### 🔑 Desired Properties

- **Completeness**  
  Honest prover always convinces:
  $$
  \Pr[\mathcal{V}(x,\pi)=\texttt{accept}
  \mid \pi\!\leftarrow\!\mathcal{P}(x,w)] = 1.
  $$
- **Soundness**  
  No efficient prover can cheat on false $x$:
  $$
  \Pr[\mathcal{V}(x,\pi)=\texttt{accept}] \le \varepsilon,
  $$
  where $\varepsilon$ is negligible in $|x|$.
- **Zero‑Knowledge (optional)**  
  Exists a simulator producing transcripts indistinguishable from real, without $w$:
  $$
  \{\pi \mid \mathcal{P}(x,w)\} \approx_c \{\pi \mid \text{Sim}(x)\}.
  $$



## 🎓 From NP to Interactive Proofs

| Feature              | NP Proof (Static)  | Interactive Proof (IP)         |
|----------------------|--------------------|--------------------------------|
| Proof Transmission   | Entire witness $w$ | Sequence of messages           |
| Verifier Computation | $\mathrm{Poly}(    | x                              |)$   | $\mathrm{Poly}(|x|)$      |
| Soundness Guarantee  | Deterministic      | Probabilistic ($\varepsilon$)  |
| Information Revealed | Full witness       | Optional zero‑knowledge        |

**Insight**: By allowing randomness and interaction, verifier can sample “spot checks” to drastically reduce work.



## 🧩 Motivating Example: Cloud Data Integrity

- **Setup**: Client stores data on server : retains only a short digest.
- **Query**: Client asks server to compute $f(\text{data})$.
- **Proof**: Server returns output plus proof $\pi$.
- **Verification**: Client runs $\mathcal{V}(f(\text{data}),\pi)$ in time $\ll$ cost of $f$ itself.

This model generalizes to any function $f$ solvable by a circuit or program.
