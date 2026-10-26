mod comune;
use comune::{file_base, presenta, sostituisci};
use sim_core::dati::{DatiGioco, FileDati};
use sim_core::{Comando, Partita};

/// Licenziamento a reintegro (posizione 2): il ministro del Lavoro della destra (6 ± scarto) è lontano.
fn reintegro() -> FileDati {
    sostituisci(
        file_base(),
        "disposizioni",
        "partenza = \"tutele_crescenti\"",
        "partenza = \"reintegro\"",
    )
}

/// La disposizione della proposta di riforma del ministro del Lavoro, se ne ha una.
fn proposta_di_riforma(p: &Partita) -> Option<String> {
    let v = p.vista();
    v.ministeri
        .iter()
        .find(|m| m.dicastero == "lavoro_welfare")?
        .proposta
        .as_ref()?
        .disposizione
        .clone()
}

#[test]
fn ministro_lontano_da_una_disposizione_propone_una_riforma() {
    let mut trovata = false;
    for seme in 1..=20 {
        let v = Partita::nuova(DatiGioco::da_testi(&reintegro()).unwrap(), seme).vista();
        let m = v
            .ministeri
            .iter()
            .find(|m| m.dicastero == "lavoro_welfare")
            .unwrap();
        if let Some(p) = m.proposta.as_ref().filter(|p| p.disposizione.is_some()) {
            assert!(
                p.motivo.ends_with("lontana dalla sua posizione"),
                "{}",
                p.motivo
            );
            assert_eq!(p.importo, 0);
            trovata = true;
        }
    }
    assert!(trovata);
}

#[test]
fn accogliere_la_proposta_presenta_la_legge() {
    let mut trovata = false;
    for seme in 1..=20 {
        let mut p = Partita::nuova(DatiGioco::da_testi(&reintegro()).unwrap(), seme);
        let Some(d) = proposta_di_riforma(&p) else {
            continue;
        };
        p.comanda(Comando::Accogli {
            dicastero: "lavoro_welfare".into(),
        })
        .unwrap();
        assert_eq!(p.vista().calendario[0].valori[0].disposizione, d);
        trovata = true;
    }
    assert!(trovata);
}

#[test]
fn proposta_di_sblocco_del_prerequisito() {
    let f = sostituisci(
        file_base(),
        "disposizioni",
        "partenza = \"attuale\"\nlegge = \"L. 199/2025, art. 1 c. 400\"",
        "partenza = \"ridotto\"\nlegge = \"L. 199/2025, art. 1 c. 400\"",
    );
    let mut trovata = false;
    for seme in 1..=20 {
        let v = Partita::nuova(DatiGioco::da_testi(&f).unwrap(), seme).vista();
        let m = v
            .ministeri
            .iter()
            .find(|m| m.dicastero == "sanita")
            .unwrap();
        if let Some(p) = &m.proposta
            && p.motivo.starts_with("sblocca")
        {
            assert_eq!(
                (p.disposizione.as_deref(), p.gradino.as_deref()),
                (Some("privato_accreditato"), Some("attuale"))
            );
            trovata = true;
        }
    }
    assert!(trovata);
}

#[test]
fn calendario_pieno_rifiuta_accogli_e_tiene_la_proposta() {
    let mut trovata = false;
    for seme in 1..=20 {
        let mut p = Partita::nuova(DatiGioco::da_testi(&reintegro()).unwrap(), seme);
        if proposta_di_riforma(&p).is_none() {
            continue;
        }
        for (d, g) in [
            ("ticket", "alto"),
            ("obbligo_scolastico", "18"),
            ("leva_militare", "obbligatoria"),
        ] {
            presenta(&mut p, &[(d, g)]).unwrap();
        }
        let e = p
            .comanda(Comando::Accogli {
                dicastero: "lavoro_welfare".into(),
            })
            .unwrap_err();
        assert!(e.to_string().contains("calendario pieno"), "{e}");
        assert!(proposta_di_riforma(&p).is_some());
        trovata = true;
    }
    assert!(trovata);
}
