mod comune;
use comune::{dati_base, file_base, sostituisci};
use sim_core::Errore;
use sim_core::dati::DatiGioco;

fn rifiuto(file: &str, da: &str, a: &str) -> String {
    let e = DatiGioco::da_testi(&sostituisci(file_base(), file, da, a)).unwrap_err();
    assert!(matches!(e, Errore::Atti(_)), "{e}");
    e.to_string()
}

#[test]
fn catalogo_base_si_carica() {
    let d = dati_base();
    assert_eq!(d.atti().len(), 40);
    for i in 0..d.dicasteri().len() {
        assert_eq!(d.atti_del_dicastero(i).count(), 4);
    }
    let c = d
        .atti()
        .iter()
        .find(|a| a.id == "sanita_case_comunita")
        .unwrap();
    assert_eq!(c.turni, 4);
}

#[test]
fn dicastero_con_tre_atti_rifiutato_col_nome() {
    let m = rifiuto(
        "atti",
        "dicastero = \"sanita\"\nvelocita = \"lento\"\nlato = \"destra\"",
        "dicastero = \"giustizia\"\nvelocita = \"lento\"\nlato = \"destra\"",
    );
    assert!(m.contains("sanita ha 3 atti"), "{m}");
}

#[test]
fn casella_doppia_rifiutata_col_nome() {
    let m = rifiuto(
        "atti",
        "dicastero = \"sanita\"\nvelocita = \"lento\"\nlato = \"sinistra\"",
        "dicastero = \"sanita\"\nvelocita = \"lento\"\nlato = \"destra\"",
    );
    assert!(
        m.contains("sanita: due atti nella casella lento destra"),
        "{m}"
    );
}

#[test]
fn posizione_dal_lato_sbagliato_rifiutata() {
    let m = rifiuto(
        "atti",
        "lato = \"sinistra\"\nposizione = 3.0\nturni = 1\ncosto = 1.0\neffetti = [\n  { grandezza = \"qualita_servizio_sanitario\", intensita = 8.0",
        "lato = \"sinistra\"\nposizione = 5.0\nturni = 1\ncosto = 1.0\neffetti = [\n  { grandezza = \"qualita_servizio_sanitario\", intensita = 8.0",
    );
    assert!(m.contains("sanita_liste_straordinario: posizione 5"), "{m}");
}

#[test]
fn effetto_su_grandezza_sconosciuta_rifiutato_col_nome() {
    let m = rifiuto(
        "atti",
        "grandezza = \"qualita_servizio_sanitario\", intensita = 8.0",
        "grandezza = \"qualita_servizio_sanitaro\", intensita = 8.0",
    );
    assert!(m.contains("qualita_servizio_sanitaro"), "{m}");
}

#[test]
fn effetto_fuori_dalla_riga_del_dicastero_rifiutato() {
    let m = rifiuto(
        "atti",
        "grandezza = \"qualita_servizio_sanitario\", intensita = 8.0",
        "grandezza = \"qualita_scuola\", intensita = 8.0",
    );
    assert!(
        m.contains("qualita_scuola non è nella riga del dicastero sanita"),
        "{m}"
    );
}

#[test]
fn intensita_e_durata_enormi_rifiutate_col_nome() {
    let m = rifiuto(
        "atti",
        "grandezza = \"qualita_servizio_sanitario\", intensita = 8.0",
        "grandezza = \"qualita_servizio_sanitario\", intensita = 8000.0",
    );
    assert!(
        m.contains("sanita_liste_straordinario: intensita fuori intervallo"),
        "{m}"
    );
    let m = rifiuto(
        "atti",
        "intensita = 8.0, ritardo = 1, forma = \"impulso\", durata = 2.0",
        "intensita = 8.0, ritardo = 1, forma = \"impulso\", durata = 2000.0",
    );
    assert!(
        m.contains("sanita_liste_straordinario: durata fuori intervallo"),
        "{m}"
    );
}
