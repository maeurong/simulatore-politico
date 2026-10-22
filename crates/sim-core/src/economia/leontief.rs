//! Leontief: quantità `x = (I−A)⁻¹ d` e prezzi `p = (I−Aᵀ)⁻¹ c`, per eliminazione di Gauss.
//!
//! Con `A` che passa `Struttura::valida` (costi di ogni colonna sotto 1) `I−A` è invertibile
//! (Hawkins-Simon): nessun pivot nullo.

/// Quantità prodotte che soddisfano la domanda finale: risolve `(I − A) x = d`.
pub fn produzione(a: &[Vec<f64>], domanda: &[f64]) -> Vec<f64> {
    risolvi(
        (0..a.len())
            .map(|i| (0..a.len()).map(|j| delta(i, j) - a[i][j]).collect())
            .collect(),
        domanda.to_vec(),
    )
}

/// Prezzi che coprono i costi unitari: risolve `(I − Aᵀ) p = c`.
pub fn prezzi(a: &[Vec<f64>], costo_unitario: &[f64]) -> Vec<f64> {
    risolvi(
        (0..a.len())
            .map(|i| (0..a.len()).map(|j| delta(i, j) - a[j][i]).collect())
            .collect(),
        costo_unitario.to_vec(),
    )
}

fn delta(i: usize, j: usize) -> f64 {
    if i == j { 1.0 } else { 0.0 }
}

// ponytail: Gauss denso O(n³), basta per n = 10.
fn risolvi(mut m: Vec<Vec<f64>>, mut b: Vec<f64>) -> Vec<f64> {
    let n = b.len();
    for k in 0..n {
        let pivot = (k..n).fold(k, |p, i| if m[i][k].abs() > m[p][k].abs() { i } else { p });
        m.swap(k, pivot);
        b.swap(k, pivot);
        let (sopra, sotto) = m.split_at_mut(k + 1);
        let riga_k = &sopra[k];
        for (i, riga) in (k + 1..).zip(sotto) {
            let f = riga[k] / riga_k[k];
            for (r, p) in riga[k..].iter_mut().zip(&riga_k[k..]) {
                *r -= f * p;
            }
            b[i] -= f * b[k];
        }
    }
    for k in (0..n).rev() {
        let resto: f64 = (k + 1..n).map(|j| m[k][j] * b[j]).sum();
        b[k] = (b[k] - resto) / m[k][k];
    }
    b
}
