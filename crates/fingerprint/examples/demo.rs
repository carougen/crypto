// crates/fingerprint/examples/demo.rs

// On importe la lib fingerprint
use fingerprint::simple_hash;

fn main() {
    // Paramètres de test
    let a = 3;
    let b = 5;
    let x = 7;
    let m = 11;

    // Appel de la fonction
    let h = simple_hash(a, b, x, m);
    println!("simple_hash({}, {}, {}, {}) = {}", a, b, x, m, h);
}
