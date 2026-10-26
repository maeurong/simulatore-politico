mod comune;
use comune::dati_base;
use sim_core::Partita;
use sim_core::parlamento::{FormaMisura, ValoreProposto};

fn v(d: &str, g: &str) -> Vec<ValoreProposto> {
    vec![ValoreProposto {
        disposizione: d.into(),
        gradino: g.into(),
    }]
}

#[test]
fn anteprima_da_costo_effetti_leve_e_linee() {
    let p = Partita::nuova(dati_base(), 1);
    let a = p.anteprima(&v("privato_accreditato", "ampio")).unwrap();
    assert_eq!(
        (a.forma, a.proponente.as_deref()),
        (FormaMisura::Riforma, Some("sanita"))
    );
    assert!(a.costo_annuo > 0);
    assert_eq!(a.effetti.len(), 1);
    assert!(a.effetti[0].migliora && a.effetti[0].ritardo == 1);
    assert_eq!(a.linee.len(), 7);
    let e = p.anteprima(&v("eta_pensionabile", "70")).unwrap();
    assert_eq!(
        (e.leve[0].leva.as_str(), e.leve[0].da, e.leve[0].a),
        ("eta_pensione", 67.0, 70.0)
    );
}

#[test]
fn anteprima_non_cambia_la_partita() {
    let p = Partita::nuova(dati_base(), 1);
    let prima = p.salva().unwrap();
    let a = p.anteprima(&v("privato_accreditato", "ampio")).unwrap();
    assert_eq!(a, p.anteprima(&v("privato_accreditato", "ampio")).unwrap());
    assert_eq!(p.salva().unwrap(), prima);
}

#[test]
fn anteprima_rifiuta_come_la_presentazione() {
    let p = Partita::nuova(dati_base(), 1);
    let e = p.anteprima(&v("eta_pensionabile", "67")).unwrap_err();
    assert!(
        e.to_string().contains("Età pensionabile è già a 67 anni"),
        "{e}"
    );
}

#[test]
fn anteprima_della_legge_elettorale_da_i_seggi() {
    let p = Partita::nuova(dati_base(), 1);
    let a = p.anteprima(&v("soglia_lista", "0")).unwrap();
    assert_eq!(a.seggi.len(), 7);
    assert!(a.effetti.is_empty() && a.costo_annuo == 0);
}
