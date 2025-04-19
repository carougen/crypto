
# Chapter 2 – Algebraic Techniques for Succinct Verification

Algebraic encoding maps discrete data to **low‑degree polynomials**, enabling **succinct** probabilistic checks via random evaluation. Two central algorithms illustrate this:

1. **Reed–Solomon Fingerprinting**
2. **Freivalds’ Matrix Verification**



## 🧬 Low‑Degree Extension via Lagrange Interpolation

**Challenge**: To check equality of large data (vectors, matrices) by probing a single or few coordinates, we need an **algebraic extension** from native data points to evaluations over a larger field.

- **Data Vector**  
  $\mathbf{a} = (a_0, \dots, a_{n-1}) \in \mathbb{F}_p^n$.

- **Goal**  
  Construct polynomial $f_{\mathbf{a}}(x)$ of degree $<n$ such that  
  $$f_{\mathbf{a}}(i) = a_i,\quad i=0,1,\dots,n-1.$$

### ✂️ Lagrange Basis Polynomials

For each $i\in\{0,\dots,n-1\}$, define
$$
\ell_i(x)
=
\prod_{\substack{0\le j<n\\j\neq i}}
\frac{x - j}{i - j}.
$$
Properties:

- $\ell_i(i) = 1$
- $\ell_i(j) = 0$ for $j \neq i$

**Low‑Degree Extension**:
$$
f_{\mathbf{a}}(x)
=
\sum_{i=0}^{n-1} a_i\,\ell_i(x).
$$

This mapping $\mathbf{a}\mapsto f_{\mathbf{a}}$ is **bijective** between vectors in $\mathbb{F}_p^n$ and polynomials of degree $<n$.



## 📝 Reed–Solomon Fingerprinting

**Problem**: Verify $\mathbf{a}=\mathbf{b}$ with $\ll n$ communication.

1. **Setup**: Agree on prime $p > \frac{n}{\varepsilon}$ to bound error by $\varepsilon$.
2. **Prover** computes random $r\in\mathbb{F}_p$ and fingerprint  
   $$
   v = f_{\mathbf{a}}(r).
   $$
3. **Prover → Verifier**: send $(r,v)$.
4. **Verifier** computes $f_{\mathbf{b}}(r)$ and accepts iff $f_{\mathbf{b}}(r)=v$.

**Soundness**: If $\mathbf{a}\neq\mathbf{b}$, then $g(x)=f_{\mathbf{a}}(x)-f_{\mathbf{b}}(x)$ is nonzero degree $<n$, so  
$$
\Pr[g(r)=0]\le \frac{n-1}{p}\le \varepsilon.
$$

---

## 🔍 Freivalds’ Algorithm for Matrix Products

**Problem**: Given $A,B,C\in\mathbb{F}_p^{n\times n}$, verify $AB=C$ in $O(n^2)$.

1. Sample $\mathbf{r}\xleftarrow{\$}\mathbb{F}_p^n$ uniformly.
2. Compute $\mathbf{u}=B\,\mathbf{r}$ and then
   $$\mathbf{v}=A\,\mathbf{u},\quad\mathbf{w}=C\,\mathbf{r}.$$
3. **Accept** iff $\mathbf{v}=\mathbf{w}$.

- **Runtime**: $O(n^2)$ for two matrix–vector products.
- **Soundness**: If $C\neq AB$, then $\mathbf{v}-\mathbf{w}$ defines a nonzero polynomial in each entry of $\mathbf{r}$ ⇒ error $\le\frac{n-1}{p}$.

Repeated trials amplify soundness exponentially.

---

## 🌟 Synthesis

By **extending data** via Lagrange interpolation into low‑degree polynomials and performing **randomized checks** on evaluations, we reduce global correctness problems to **single-instance** tests. These algebraic foundations form the bedrock of modern interactive and non‑interactive proof systems.
