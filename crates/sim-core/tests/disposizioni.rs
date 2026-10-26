#![expect(
    clippy::unwrap_used,
    reason = "nei test un unwrap che fallisce e' il fallimento voluto"
)]

mod comune;
use comune::{dati_base, file_base, sostituisci};
use sim_core::Errore;
use sim_core::dati::DatiGioco;
use sim_core::disposizioni::{Norma, TipoDisposizione};

fn rifiuto(file: &str, da: &str, a: &str) -> String {
    let e = DatiGioco::da_testi(&sostituisci(file_base(), file, da, a)).unwrap_err();
    assert!(matches!(e, Errore::Disposizioni(_)), "{e}");
    e.to_string()
}

#[test]
fn catalogo_base_si_carica() {
    let d = dati_base();
    let tutte = d.disposizioni();
    assert_eq!(tutte.len(), 46);
    let conta = |n: Norma| tutte.iter().filter(|x| x.norma == n).count();
    assert_eq!(
        (
            conta(Norma::Ordinaria),
            conta(Norma::Costituzionale),
            conta(Norma::Elettorale)
        ),
        (35, 2, 9)
    );
    let fiscali = [
        "forfettario",
        "cuneo_fiscale",
        "detrazione_edilizia",
        "accise",
        "incentivo_investimenti",
        "decontribuzione",
        "credito_zes",
    ];
    for x in tutte {
        let atteso = !fiscali.contains(&x.id.as_str()) && x.norma != Norma::Costituzionale;
        assert_eq!(x.referendabile, atteso, "{}", x.id);
    }
    let n = &tutte[d.disposizione("nucleare").unwrap()];
    let a = &n.in_arrivo[0];
    assert_eq!(
        (n.partenza.as_str(), a.gradino.as_str(), a.anno, a.trimestre),
        ("vietato", "consentito", 2027, 4)
    );
    let r = &tutte[d.disposizione("reddito_minimo").unwrap()];
    assert_eq!(r.dopo_abrogazione.as_deref(), Some("non_istituito"));
    assert_eq!(
        tutte[d.disposizione("decontribuzione").unwrap()]
            .perimetro
            .len(),
        8
    );
}

#[test]
fn disposizioni_elettorali_dal_menu_e_dallo_scenario() {
    let d = dati_base();
    let s = &d.disposizioni()[d.disposizione("soglia_lista").unwrap()];
    let ids: Vec<&str> = s.gradini.iter().map(|g| g.id.as_str()).collect();
    assert_eq!(ids, ["0", "300", "400", "500"]);
    assert_eq!(
        (s.partenza.as_str(), s.precedente.as_str(), s.legge.as_str()),
        ("300", "300", "L. 180/2026")
    );
    let p = &d.disposizioni()[d.disposizione("premio").unwrap()];
    assert_eq!(
        (p.partenza.as_str(), p.precedente.as_str(), p.tipo),
        ("coalizione", "nessuno", TipoDisposizione::Regime)
    );
    assert!(p.gradini.iter().all(|g| g.posizione.is_none()));
    let t = &d.disposizioni()[d.disposizione("tetto").unwrap()];
    assert_eq!(
        (t.partenza.as_str(), t.precedente.as_str()),
        ("5500_5650", "nessuno")
    );
}

#[test]
fn gradino_sconosciuto_rifiutato_col_nome() {
    let m = rifiuto("disposizioni", "partenza = \"67\"", "partenza = \"66,5\"");
    assert!(
        m.contains("eta_pensionabile: gradino sconosciuto 66,5"),
        "{m}"
    );
}

#[test]
fn leva_sconosciuta_rifiutata_col_nome() {
    let m = rifiuto(
        "disposizioni",
        "leve = { eta_pensione = 62.0 }",
        "leve = { eta_pensionabile = 62.0 }",
    );
    assert!(
        m.contains("eta_pensionabile: leva sconosciuta eta_pensionabile"),
        "{m}"
    );
}

#[test]
fn effetto_fuori_dalla_riga_del_dicastero_rifiutato() {
    let m = rifiuto(
        "disposizioni",
        "nome = \"reintegro\", posizione = 2.0, saldo = 0.0, effetti = [{ grandezza = \"regole_lavoro\"",
        "nome = \"reintegro\", posizione = 2.0, saldo = 0.0, effetti = [{ grandezza = \"qualita_scuola\"",
    );
    assert!(
        m.contains("licenziamento: qualita_scuola non è nella riga del dicastero lavoro_welfare"),
        "{m}"
    );
}

#[test]
fn perimetro_su_grandezza_nazionale_rifiutato() {
    let m = rifiuto(
        "disposizioni",
        "nome = \"nessuno stanziamento\", posizione = 5.0, saldo = -0.1, effetti = [{ grandezza = \"investimenti_privati\"",
        "nome = \"nessuno stanziamento\", posizione = 5.0, saldo = -0.1, effetti = [{ grandezza = \"sostegno_imprese\"",
    );
    assert!(
        m.contains("credito_zes: perimetro su sostegno_imprese nazionale"),
        "{m}"
    );
}

#[test]
fn effetto_oltre_meta_intervallo_rifiutato() {
    // regole_lavoro va da -5 a 5: un'intensità oltre 5 farebbe uscire Δ dal tetto del salvataggio.
    let m = rifiuto(
        "disposizioni",
        "nome = \"reintegro\", posizione = 2.0, saldo = 0.0, effetti = [{ grandezza = \"regole_lavoro\", intensita = -0.3",
        "nome = \"reintegro\", posizione = 2.0, saldo = 0.0, effetti = [{ grandezza = \"regole_lavoro\", intensita = -5.1",
    );
    assert!(
        m.contains("licenziamento: effetto su regole_lavoro fuori intervallo"),
        "{m}"
    );
}

#[test]
fn regione_del_perimetro_sconosciuta_rifiutata() {
    let m = rifiuto(
        "disposizioni",
        "perimetro = { abruzzo = 1.0",
        "perimetro = { abruzo = 1.0",
    );
    assert!(
        m.contains("decontribuzione: regione sconosciuta abruzo"),
        "{m}"
    );
}

#[test]
fn legge_elettorale_precedente_fuori_menu_rifiutata() {
    let f = sostituisci(
        file_base(),
        "scenario",
        "[legge_elettorale_precedente]\nsoglia_lista_pb = 300",
        "[legge_elettorale_precedente]\nsoglia_lista_pb = 250",
    );
    let e = DatiGioco::da_testi(&f).unwrap_err();
    assert!(
        matches!(e, Errore::LeggeElettorale(_))
            && e.to_string()
                .contains("legge elettorale precedente: soglia_lista_pb"),
        "{e}"
    );
}
