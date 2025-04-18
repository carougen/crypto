//! Crate `fingerprint`: Reed–Solomon encoding examples.
//!
//! This library provides functions for polynomial evaluation and Lagrange interpolation
//! over a finite field F_p.

/// Evaluate the polynomial f(X) = sum_{i=0..} message[i] * X^i mod p.
///
/// # Arguments
/// * `message` - slice of coefficients [m0, m1, ..., m_{k-1}]
/// * `p` - prime modulus
/// * `x` - evaluation point
///
/// # Returns
/// Value of f(x) mod p.
pub fn evaluate_polynomial(message: &[u64], p: u64, x: u64) -> u64 {
    let mut result = 0;
    let mut xi = 1;

    for &coeff in message {
        // term = coeff * x^i mod p
        let term = (coeff * xi) % p;
        result = (result + term) % p;
        // update xi to x^{i+1}
        xi = (xi * x) % p;
    }

    result
}

/// Compute modular inverse of a modulo p using Fermat's little theorem.
/// Requires p to be prime.
///
/// # Arguments
/// * `a` - value to invert
/// * `p` - prime modulus
///
/// # Returns
/// Multiplicative inverse of a mod p.
pub fn mod_inv(a: u64, p: u64) -> u64 {
    // a^(p-2) mod p
    a.pow((p - 2) as u32) % p
}

/// Evaluate the Lagrange interpolation polynomial for `message` at point `x` over field p,
/// using anchor points `xs` (length k).
///
/// # Arguments
/// * `message` - slice of values at anchor points [m0, m1, ..., m_{k-1}]
/// * `xs` - slice of anchor points [x0, x1, ..., x_{k-1}]
/// * `p` - prime modulus
/// * `x` - evaluation point
///
/// # Returns
/// Interpolated value f(x) mod p.
pub fn evaluate_lagrange(message: &[u64], xs: &[u64], p: u64, x: u64) -> u64 {
    let k = message.len();
    let mut result = 0;

    for i in 0..k {
        let mut num = 1;
        let mut den = 1;

        for j in 0..k {
            if i != j {
                // num *= (x - xs[j]) mod p
                num = (num * ((x + p - xs[j]) % p)) % p;
                // den *= (xs[i] - xs[j]) mod p
                den = (den * ((xs[i] + p - xs[j]) % p)) % p;
            }
        }

        // Compute Lagrange basis polynomial li(x) = num / den mod p
        let li = (num * mod_inv(den, p)) % p;
        // Add contribution m_i * li(x)
        result = (result + message[i] * li) % p;
    }

    result
}

/// Generate the RS codeword by evaluating the polynomial at all points in F_p.
///
/// # Arguments
/// * `message` - slice of coefficients
/// * `p` - prime modulus
///
/// # Returns
/// A Vec<u64> of length p containing f(0), f(1), ..., f(p-1).
pub fn codeword_polynomial(message: &[u64], p: u64) -> Vec<u64> {
    (0..p).map(|x| evaluate_polynomial(message, p, x)).collect()
}

/// Generate the RS codeword via Lagrange interpolation at all points in F_p.
///
/// # Arguments
/// * `message` - slice of values at anchor points
/// * `xs` - slice of anchor points
/// * `p` - prime modulus
///
/// # Returns
/// A Vec<u64> of length p containing f(0), f(1), ..., f(p-1).
pub fn codeword_lagrange(message: &[u64], xs: &[u64], p: u64) -> Vec<u64> {
    (0..p).map(|x| evaluate_lagrange(message, xs, p, x)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate_polynomial_known() {
        let p = 11;
        let a = vec![2, 1, 1]; // message a
        // f(x) = 2 + x + x^2
        assert_eq!(evaluate_polynomial(&a, p, 0), 2);
        assert_eq!(evaluate_polynomial(&a, p, 1), 4 % p);
        assert_eq!(evaluate_polynomial(&a, p, 2), 8 % p);
    }

    #[test]
    fn test_codeword_polynomial_full() {
        let p = 11;
        let a = vec![2, 1, 1];
        let expected_a = vec![2, 4, 8, 3, 0, 10, 0, 3, 8, 4, 2];
        let cw_a = codeword_polynomial(&a, p);
        assert_eq!(cw_a, expected_a);

        let b = vec![2, 1, 0];
        let expected_b = vec![2, 3, 4, 5, 6, 7, 8, 9, 10, 0, 1];
        let cw_b = codeword_polynomial(&b, p);
        assert_eq!(cw_b, expected_b);
    }

    #[test]
    fn test_codeword_lagrange_same() {
        let p = 11;
        let xs = vec![0, 1, 2];
        let a = vec![2, 1, 1];
        let expected_a = vec![2, 1, 1, 2, 4, 7, 0, 5, 0, 7, 4];
        let cw_a = codeword_lagrange(&a, &xs, p);
        assert_eq!(cw_a, expected_a);

        let b = vec![2, 1, 0];
        let expected_b = vec![2, 1, 0, 10, 9, 8, 7, 6, 5, 4, 3];
        let cw_b = codeword_lagrange(&b, &xs, p);
        assert_eq!(cw_b, expected_b);
    }
}
