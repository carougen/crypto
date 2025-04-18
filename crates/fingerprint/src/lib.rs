//! Crate `fingerprint` : Reed–Solomon & hashing examples

/// Calcule (a + b*x) % m
pub fn simple_hash(a: u64, b: u64, x: u64, m: u64) -> u64 {
    (a.wrapping_mul(x).wrapping_add(b)) % m
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_simple_hash() {
        assert_eq!(simple_hash(3, 5, 7, 11), (3*7+5)%11);
    }
}