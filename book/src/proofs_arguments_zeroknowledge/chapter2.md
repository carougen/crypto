# Chapter 2 – Soundness via Fingerprinting and Freivalds’ Algorithm

This chapter explores how **randomized protocols** provide **sound verification** of algebraic claims with **high efficiency**, focusing on:

1. Equality testing of vectors via **polynomial fingerprinting**
2. Matrix product verification via **Freivalds’ algorithm**

---

## 📘 Example 1: Verifying Equality via Fingerprinting

Let $\mathbb{F}_p$ be a finite field of prime order $p$.

Let $\mathbf{a}, \mathbf{b} \in \mathbb{F}_p^n$ be two vectors.

Our goal is to verify whether $\mathbf{a} = \mathbf{b}$ using **minimal communication**, without comparing all entries.



### ✅ Protocol: Polynomial Fingerprinting

1. Define polynomials over $\mathbb{F}_p$:
   $$
   p_a(x) = \sum_{i=0}^{n-1} a_i x^i, \quad p_b(x) = \sum_{i=0}^{n-1} b_i x^i
   $$

2. Choose a random point $r \in \mathbb{F}_p$

3. Evaluate $p_a(r)$ and $p_b(r)$

4. Accept if $p_a(r) = p_b(r)$



### 🧠 Soundness Guarantee

If $\mathbf{a} \neq \mathbf{b}$, then $p_a(x) \neq p_b(x)$. Since $p_a - p_b$ is a non-zero polynomial of degree at most $d < n$, then by the **Schwartz–Zippel lemma**:

$$
\Pr[p_a(r) = p_b(r)] \leq \frac{d}{p}
$$

If $p > n^2$, this probability is less than $1/n$ — strong soundness with a single field element check.



### ⏱️ Complexity

- Prover: $O(n)$ for evaluation
- Verifier: $O(n)$ for single evaluation and comparison
- Communication: 2 elements of $\mathbb{F}_p$: $(r,p_a(r))$ so ($O(\log n)$ bits)



### 🧪 Rust Example

```rust
let r = random_in_field(p);
let eval_a = eval_poly(&a, r, p);
let eval_b = eval_poly(&b, r, p);
assert_eq!(eval_a, eval_b);
```



## 📘 Lagrange Interpolation

Given $n$ evaluation points $(x_0, y_0), \dots, (x_{n-1}, y_{n-1})$ in $\mathbb{F}_p^2$, the unique polynomial $q(x)$ of degree $< n$ satisfying $q(x_i) = y_i$ for all $i$ is:

$$
q(x) = \sum_{i=0}^{n-1} y_i \cdot \ell_i(x)
$$

with:

$$
\ell_i(x) = \prod_{j \neq i} \frac{x - x_j}{x_i - x_j}
$$

This reconstruction is fundamental for **encoding and decoding** Reed–Solomon codes and evaluating polynomials from tabulated values.

---

## 📘 Example 2: Matrix Product Verification (Freivalds' Algorithm)

Let $A, B, C \in \mathbb{F}_p^{n \times n}$.

We want to verify whether $AB = C$ without computing the full product, which costs $O(n^3)$ or $O(n^{2.373})$ (e.g., with Strassen).



### ✅ Protocol: Freivalds’ Algorithm

1. Choose a random vector $r \in \mathbb{F}_p^n$

2. Compute:
   - $u = Br$
   - $v = Cr$
   - $w = A u$

3. Accept if:
   $$
   w = v
   $$



### 🧠 Soundness Guarantee

If $AB \neq C$, define $D = AB - C \neq 0$.

Then $Dr \neq 0$ with probability $\geq 1/2$. Thus:

$$
\Pr[A(Br) = Cr] = \Pr[Dr = 0] \leq \frac{1}{2}
$$

Repeating $k$ times gives:
$$
\Pr[\text{all trials accept}] \leq \left(\frac{1}{2}\right)^k
$$



### ⏱️ Complexity

- Matrix-vector multiplication: $O(n^2)$
- Total verifier time: **$O(n^2)$**
- Communication: **None** (computation only)

This is exponentially cheaper than checking $AB = C$ directly.



### 🧪 Rust Example

```rust
let r = random_vector(n);
let u = mat_vec_mul(&b, &r);
let v = mat_vec_mul(&c, &r);
let w = mat_vec_mul(&a, &u);
assert_eq!(w, v);
```

---

## 🔗 Summary

| Problem                      | Deterministic Cost | Probabilistic Cost | Error Probability |
|------------------------------|--------------------|--------------------|-------------------|
| Vector equality              | $\Omega(n\log m)$  | $O(\log n)$        | $\le1/n$          |
| Verifying $AB=C$ (Freivalds) | $O(n^3)$           | $O(n^2)$           | $\le2^{-k}$       |


These examples show how **randomness** enables powerful probabilistic proofs with minimal effort.
