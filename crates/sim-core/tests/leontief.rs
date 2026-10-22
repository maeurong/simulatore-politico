use sim_core::economia::leontief::{prezzi, produzione};

fn vicino(a: f64, b: f64) -> bool {
    (a - b).abs() < 1e-9
}

#[test]
fn esempio_carbone_acciaio() {
    let a = vec![vec![0.5, 0.2], vec![0.4, 0.1]];
    let x = produzione(&a, &[7.0, 4.0]);
    assert!(
        vicino(x[0], 7.1 / 0.37) && vicino(x[1], 4.8 / 0.37),
        "{x:?}"
    );
}

#[test]
fn prezzi_unitari_quando_il_costo_chiude_ogni_colonna() {
    let a = vec![vec![0.5, 0.2], vec![0.4, 0.1]];
    let p = prezzi(&a, &[1.0 - 0.9, 1.0 - 0.3]);
    assert!(vicino(p[0], 1.0) && vicino(p[1], 1.0), "{p:?}");
}

#[test]
fn matrice_nulla_restituisce_la_domanda() {
    let a = vec![vec![0.0; 3]; 3];
    assert_eq!(produzione(&a, &[1.0, 2.0, 3.0]), vec![1.0, 2.0, 3.0]);
}
