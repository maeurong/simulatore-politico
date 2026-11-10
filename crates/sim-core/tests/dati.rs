#![expect(
    clippy::unwrap_used,
    reason = "nei test un unwrap che fallisce e' il fallimento voluto"
)]

mod comune;
use comune::{file_base, imposta, sostituisci};
use sim_core::dati::{DatiGioco, FileDati, Ruolo};
use sim_core::errore::Errore;

fn con_dicasteri(testo: &str) -> FileDati {
    FileDati {
        dicasteri: testo.into(),
        ..file_base()
    }
}

#[test]
fn dati_base_si_caricano() {
    let d = DatiGioco::da_testi(&file_base()).unwrap();
    assert_eq!(d.dicasteri().len(), 10);
    assert!(d.dicasteri().iter().any(|x| x.nome == "Sanità"));
    assert_eq!(
        d.dicasteri().iter().filter(|x| x.regione_bersaglio).count(),
        2
    );
    assert_eq!(d.manopole()["soglia_minaccia"].valore, 40.0);
}

#[test]
fn ruolo_orfano_rifiutato_col_nome() {
    let testo = file_base()
        .dicasteri
        .replace("\"colpa_disoccupazione_trasferimenti\"", "");
    let e = DatiGioco::da_testi(&con_dicasteri(&testo)).unwrap_err();
    assert_eq!(
        e,
        Errore::RuoloOrfano {
            ruolo: Ruolo::ColpaDisoccupazioneTrasferimenti
        }
    );
    assert_eq!(
        e.to_string(),
        "ruolo senza dicastero: colpa_disoccupazione_trasferimenti"
    );
}

#[test]
fn ruolo_doppio_rifiutato_col_nome() {
    let testo = format!(
        "{}\n[[dicastero]]\nid = \"bis\"\nnome = \"Bis\"\nindicatore_servizio = \"x\"\npeso_spartizione = 1\nregione_bersaglio = false\nruoli = [\"bilancio_e_credibilita\"]\nspesa_pil = 0.0\nsalari_pil = 0.0\nacquisti_pil = 0.0\n",
        file_base().dicasteri
    );
    let e = DatiGioco::da_testi(&con_dicasteri(&testo)).unwrap_err();
    assert!(
        e.to_string()
            .starts_with("ruolo assegnato a due dicasteri: bilancio_e_credibilita")
    );
}

#[test]
fn indicatore_servizio_doppio_rifiutato() {
    let testo = format!(
        "{}\n[[dicastero]]\nid = \"bis\"\nnome = \"Bis\"\nindicatore_servizio = \"sicurezza\"\npeso_spartizione = 1\nregione_bersaglio = false\nruoli = []\nspesa_pil = 0.0\nsalari_pil = 0.0\nacquisti_pil = 0.0\n",
        file_base().dicasteri
    );
    let e = DatiGioco::da_testi(&con_dicasteri(&testo)).unwrap_err();
    assert!(matches!(
        &e,
        Errore::IndicatoreDoppio { indicatore, secondo, .. }
            if indicatore == "sicurezza" && secondo == "bis"
    ));
    assert!(
        e.to_string()
            .starts_with("indicatore di servizio assegnato a due dicasteri: sicurezza")
    );
}

#[test]
fn ruoli_tutti_coprono_ogni_variante() {
    // Il match senza `_` non compila se l'enum cresce: va aggiornato insieme a TUTTI.
    fn nella_lista(r: Ruolo) -> bool {
        match r {
            Ruolo::ColpaDisoccupazioneTrasferimenti
            | Ruolo::ColpaImposteImpiegoPubblico
            | Ruolo::BilancioECredibilita => Ruolo::TUTTI.contains(&r),
        }
    }
    for r in [
        Ruolo::ColpaDisoccupazioneTrasferimenti,
        Ruolo::ColpaImposteImpiegoPubblico,
        Ruolo::BilancioECredibilita,
    ] {
        assert!(nella_lista(r));
    }
}

#[test]
fn ruolo_sconosciuto_rifiutato_col_nome() {
    let testo = file_base()
        .dicasteri
        .replace("\"bilancio_e_credibilita\"", "\"bilancio_e_credibilta\"");
    let e = DatiGioco::da_testi(&con_dicasteri(&testo)).unwrap_err();
    assert!(matches!(e, Errore::Toml { .. }));
    assert!(e.to_string().contains("bilancio_e_credibilta"));
}

#[test]
fn dicastero_ripetuto_rifiutato() {
    let base = file_base().dicasteri;
    let primo = base.split("[[dicastero]]").nth(1).unwrap();
    let testo = format!("{base}\n[[dicastero]]{primo}");
    let e = DatiGioco::da_testi(&con_dicasteri(&testo)).unwrap_err();
    assert_eq!(
        e,
        Errore::DicasteroDoppio {
            id: "economia_finanze".into()
        }
    );
}

#[test]
fn manopola_fuori_intervallo_rifiutata_col_nome() {
    let manopole = file_base()
        .manopole
        .replacen("valore = 40.0", "valore = 90.0", 1);
    let e = DatiGioco::da_testi(&FileDati {
        manopole,
        ..file_base()
    })
    .unwrap_err();
    assert!(
        e.to_string()
            .starts_with("manopola fuori intervallo: soglia_minaccia = 90")
    );
}

#[test]
fn manopola_non_finita_rifiutata() {
    let manopole = file_base()
        .manopole
        .replacen("valore = 40.0", "valore = nan", 1);
    let e = DatiGioco::da_testi(&FileDati {
        manopole,
        ..file_base()
    })
    .unwrap_err();
    assert!(matches!(e, Errore::FuoriIntervallo { ref nome, .. } if nome == "soglia_minaccia"));
}

#[test]
fn manopola_con_massimo_infinito_rifiutata() {
    // valore dentro l'intervallo: a rifiutarla e' solo is_finite.
    let manopole = file_base().manopole.replacen("max = 50.0", "max = inf", 1);
    let e = DatiGioco::da_testi(&FileDati {
        manopole,
        ..file_base()
    })
    .unwrap_err();
    assert!(matches!(e, Errore::FuoriIntervallo { ref nome, .. } if nome == "soglia_minaccia"));
}

#[test]
fn manopola_con_intervallo_rovesciato_rifiutata() {
    let manopole = file_base().manopole.replacen("min = 30.0", "min = 60.0", 1);
    let e = DatiGioco::da_testi(&FileDati {
        manopole,
        ..file_base()
    })
    .unwrap_err();
    assert!(matches!(e, Errore::FuoriIntervallo { ref nome, .. } if nome == "soglia_minaccia"));
}

#[test]
fn soglie_dei_ministri_sono_manopole_richieste() {
    for nome in ["soglia_dimissioni", "soglia_minaccia"] {
        let f = sostituisci(
            file_base(),
            "manopole",
            &format!("[{nome}]"),
            &format!("[{nome}_x]"),
        );
        assert_eq!(
            DatiGioco::da_testi(&f).unwrap_err(),
            Errore::ManopolaMancante { nome: nome.into() }
        );
    }
}

#[test]
fn scala_servizio_nulla_rifiutata_anche_se_il_file_la_ammette() {
    let f = imposta(
        file_base(),
        "manopole",
        "[scala_servizio]\nvalore = ",
        "0.0",
    );
    let f = imposta(
        f,
        "manopole",
        "[scala_servizio]\nvalore = 0.0\nmin = ",
        "0.0",
    );
    let e = DatiGioco::da_testi(&f).unwrap_err();
    assert!(e.to_string().contains("scala_servizio"), "{e}");
}

/// `nome` a `valore`, con `min` e `max` del file allargati perché passi il controllo generico.
fn con_manopola(nome: &str, valore: &str) -> Result<DatiGioco, Errore> {
    let f = imposta(
        file_base(),
        "manopole",
        &format!("[{nome}]\nvalore = "),
        valore,
    );
    let f = imposta(
        f,
        "manopole",
        &format!("[{nome}]\nvalore = {valore}\nmin = "),
        "-1.0",
    );
    let f = imposta(
        f,
        "manopole",
        &format!("[{nome}]\nvalore = {valore}\nmin = -1.0\nmax = "),
        "1e7",
    );
    DatiGioco::da_testi(&f)
}

#[test]
fn prezzi_non_positivi_rifiutati() {
    for nome in [
        "prezzo_sondaggio_privato",
        "prezzo_test_misura",
        "prezzo_centro_studi",
    ] {
        let e = con_manopola(nome, "0.0").unwrap_err();
        assert_eq!(
            e.to_string(),
            format!("manopola non valida: {nome} non positivo: 0")
        );
        assert!(con_manopola(nome, "1.0").is_ok(), "{nome}");
    }
}

#[test]
fn orizzonte_del_test_fuori_da_1_20_o_non_intero_rifiutato() {
    for valore in ["0.0", "21.0", "2.5"] {
        let e = con_manopola("orizzonte_test", valore).unwrap_err();
        let x: f64 = valore.parse().unwrap();
        assert_eq!(
            e.to_string(),
            format!("manopola non valida: orizzonte_test non intero fra 1 e 20: {x}")
        );
    }
    for valore in ["1.0", "20.0"] {
        assert!(con_manopola("orizzonte_test", valore).is_ok(), "{valore}");
    }
}

#[test]
fn soglie_della_reazione_rovesciate_rifiutate() {
    let e = con_manopola("soglia_reazione_poco", "0.1").unwrap_err();
    assert_eq!(
        e.to_string(),
        "manopola non valida: soglia_reazione_poco (0.1) oltre soglia_reazione_medio (0.061)"
    );
    assert!(con_manopola("soglia_reazione_poco", "0.061").is_ok());
}

#[test]
fn soglie_delle_fasce_rovesciate_rifiutate() {
    let f = imposta(
        file_base(),
        "manopole",
        "[soglia_fascia_poco]\nvalore = ",
        "0.1",
    );
    let e = DatiGioco::da_testi(&f).unwrap_err();
    let testo = e.to_string();
    assert!(
        testo.contains("soglia_fascia_poco") && testo.contains("soglia_fascia_medio"),
        "{e}"
    );
}

#[test]
fn indicatore_di_servizio_senza_spesa_di_funzionamento_rifiutato() {
    let f = sostituisci(
        file_base(),
        "dicasteri",
        "salari_pil = 2.0153",
        "salari_pil = 0.0",
    );
    let f = sostituisci(
        f,
        "dicasteri",
        "acquisti_pil = 4.1095",
        "acquisti_pil = 0.0",
    );
    let e = DatiGioco::da_testi(&f).unwrap_err();
    assert!(
        matches!(e, Errore::Grandezze(_)) && e.to_string().contains("sanita"),
        "{e}"
    );
}

#[test]
fn piramide_senza_eta_da_lavoro_rifiutata() {
    let mut f = file_base();
    let inizio = f.economia.find("piramide = [").unwrap();
    let fine = inizio + f.economia[inizio..].find(']').unwrap() + 1;
    let eta: Vec<&str> = (0..=100)
        .map(|a| if (15..=66).contains(&a) { "0" } else { "1" })
        .collect();
    f.economia
        .replace_range(inizio..fine, &format!("piramide = [{}]", eta.join(", ")));
    let e = DatiGioco::da_testi(&f).unwrap_err();
    assert!(
        matches!(e, Errore::StrutturaIncoerente(_)) && e.to_string().contains("15"),
        "{e}"
    );
}

#[test]
fn coalizione_di_un_partito_solo_rifiutata() {
    let f = sostituisci(
        file_base(),
        "scenario",
        "[\"sinistra\", \"centrosinistra\"]",
        "[\"sinistra\"]",
    );
    let e = DatiGioco::da_testi(&f).unwrap_err();
    assert!(e.to_string().contains("sinistra"), "{e}");
}

#[test]
fn evento_senza_default_rifiutato_col_nome() {
    let f = sostituisci(file_base(), "eventi", "default = true", "default = false");
    let e = DatiGioco::da_testi(&f).unwrap_err();
    assert!(
        matches!(e, Errore::Eventi(_)) && e.to_string().contains("Recessione"),
        "{e}"
    );
}

#[test]
fn evento_con_due_default_rifiutato_col_nome() {
    let f = sostituisci(file_base(), "eventi", "default = false", "default = true");
    let e = DatiGioco::da_testi(&f).unwrap_err();
    assert!(
        matches!(e, Errore::Eventi(_)) && e.to_string().contains("Recessione"),
        "{e}"
    );
}

#[test]
fn situazione_con_soglie_rovesciate_rifiutata_col_nome() {
    let f = sostituisci(
        file_base(),
        "eventi",
        "accensione = 85.0",
        "accensione = 95.0",
    );
    let e = DatiGioco::da_testi(&f).unwrap_err();
    assert!(
        matches!(e, Errore::Eventi(_)) && e.to_string().contains("Criminalità"),
        "{e}"
    );
}

#[test]
fn uno_scandalo_ammette_solo_le_dimissioni_del_ministro() {
    // Un effetto di spesa sull'opzione «Difenderlo» sarebbe ignorato in silenzio.
    let f = sostituisci(
        file_base(),
        "eventi",
        "id = \"difenderlo\"\nnome = \"Difenderlo\"\ndefault = true\neffetti = []",
        "id = \"difenderlo\"\nnome = \"Difenderlo\"\ndefault = true\n\n[[evento.opzioni.effetti]]\ntipo = \"valence\"\ndelta = 0.1",
    );
    let e = DatiGioco::da_testi(&f).unwrap_err();
    assert!(
        matches!(e, Errore::Eventi(_))
            && e.to_string()
                .contains("Scandalo su un ministro: lo scandalo ammette solo"),
        "{e}"
    );
    // Le dimissioni non hanno senso sul leader.
    let f = sostituisci(
        file_base(),
        "eventi",
        "id = \"prendere_atto\"\nnome = \"Prendere atto\"\ndefault = true\neffetti = []",
        "id = \"prendere_atto\"\nnome = \"Prendere atto\"\ndefault = true\n\n[[evento.opzioni.effetti]]\ntipo = \"dimissioni\"",
    );
    let e = DatiGioco::da_testi(&f).unwrap_err();
    assert!(
        matches!(e, Errore::Eventi(_))
            && e.to_string()
                .contains("Scandalo su un leader: lo scandalo ammette solo"),
        "{e}"
    );
}

#[test]
fn evento_richiesto_di_un_altra_famiglia_nomina_quella_attesa() {
    let f = sostituisci(
        file_base(),
        "eventi",
        "id = \"scandalo_leader\"\nfamiglia = \"scandalo\"",
        "id = \"scandalo_leader\"\nfamiglia = \"shock\"",
    );
    let e = DatiGioco::da_testi(&f).unwrap_err();
    assert!(e.to_string().contains("serve la famiglia scandalo"), "{e}");
}

#[test]
fn pericolosita_fuori_da_zero_uno_rifiutata_col_nome_della_regione() {
    let f = sostituisci(file_base(), "regioni", "sisma = 0.001033,", "sisma = 1.5,");
    let e = DatiGioco::da_testi(&f).unwrap_err();
    assert!(
        matches!(e, Errore::StrutturaIncoerente(_))
            && e.to_string().contains("pericolosita.sisma di piemonte"),
        "{e}"
    );
}

#[test]
fn pesi_idro_che_non_sommano_a_uno_rifiutati() {
    let f = sostituisci(file_base(), "regioni", "idro = 0.046,", "idro = 0.2,");
    let e = DatiGioco::da_testi(&f).unwrap_err();
    assert!(
        matches!(e, Errore::StrutturaIncoerente(_)) && e.to_string().contains("pericolosita.idro"),
        "{e}"
    );
}

#[test]
fn emergenza_richiesta_dal_motore_e_mancante_rifiutata_col_nome() {
    let f = sostituisci(
        file_base(),
        "eventi",
        "id = \"emergenza_siccita\"",
        "id = \"altra_siccita\"",
    );
    let e = DatiGioco::da_testi(&f).unwrap_err();
    assert!(
        matches!(e, Errore::Eventi(_)) && e.to_string().contains("emergenza_siccita"),
        "{e}"
    );
}

#[test]
fn opzione_che_spende_senza_dicasteri_rifiutata() {
    let f = sostituisci(
        file_base(),
        "eventi",
        "dicasteri = [\"infrastrutture_trasporti\", \"interno\"]",
        "dicasteri = []",
    );
    let e = DatiGioco::da_testi(&f).unwrap_err();
    assert!(
        matches!(e, Errore::Eventi(_)) && e.to_string().contains("Emergenza sismica"),
        "{e}"
    );
}

#[test]
fn norme_del_decreto_rifiutate_col_messaggio() {
    let sismica = "norme = [\"codice_appalti\", \"detrazione_edilizia\", \"accise\"]";
    for (nuove, messaggio) in [
        (
            "norme = [\"codice_appalti\", \"non_esiste\"]",
            "Emergenza sismica: norma sconosciuta per il decreto: non_esiste",
        ),
        (
            "norme = [\"numero_parlamentari\"]",
            "Emergenza sismica: la norma numero_parlamentari non è ordinaria, il decreto non la ammette",
        ),
        (
            "norme = [\"accise\", \"accise\"]",
            "Emergenza sismica: norma ripetuta: accise",
        ),
    ] {
        let e =
            DatiGioco::da_testi(&sostituisci(file_base(), "eventi", sismica, nuove)).unwrap_err();
        assert!(
            matches!(e, Errore::Eventi(_)) && e.to_string().contains(messaggio),
            "{e}"
        );
    }
    let f = sostituisci(
        file_base(),
        "eventi",
        "decreto = false\nnorme = []",
        "decreto = false\nnorme = [\"accise\"]",
    );
    let e = DatiGioco::da_testi(&f).unwrap_err();
    assert!(
        e.to_string()
            .contains("Emergenza incendi: norme per il decreto senza decreto"),
        "{e}"
    );
}

#[test]
fn norme_e_famiglia_del_decreto_rifiutate_col_messaggio() {
    let sismica = "norme = [\"codice_appalti\", \"detrazione_edilizia\", \"accise\"]";
    let f = sostituisci(file_base(), "eventi", sismica, "norme = []");
    let e = DatiGioco::da_testi(&f).unwrap_err();
    assert!(
        e.to_string()
            .contains("Emergenza sismica: un'emergenza con decreto deve elencare le sue norme"),
        "{e}"
    );
    let f = sostituisci(
        file_base(),
        "eventi",
        "dicasteri = [\"economia_finanze\", \"sviluppo_economico\"]\ndecreto = true\nnorme = []",
        "dicasteri = [\"economia_finanze\", \"sviluppo_economico\"]\ndecreto = true\nnorme = [\"accise\"]",
    );
    let e = DatiGioco::da_testi(&f).unwrap_err();
    assert!(
        e.to_string().contains(
            "Recessione: un evento di situazione apre il suo ministero, niente norme per il decreto"
        ),
        "{e}"
    );
}

#[test]
fn emivita_del_danno_a_zero_rifiutata() {
    let f = sostituisci(
        file_base(),
        "manopole",
        "[emivita_danno]\nvalore = 8.0\nmin = 1.0",
        "[emivita_danno]\nvalore = 0.0\nmin = 0.0",
    );
    let e = DatiGioco::da_testi(&f).unwrap_err();
    assert!(
        matches!(e, Errore::Eventi(_))
            && e.to_string()
                .contains("manopola emivita_danno non positiva: 0"),
        "{e}"
    );
}

#[test]
fn gravita_da_dieci_in_su_per_emergenze_e_scandali_rifiutata() {
    for (nome, da) in [
        ("Emergenza sismica", "gravita = 5"),
        ("Scandalo su un leader", "gravita = 6"),
    ] {
        let riga = format!("nome = \"{nome}\"\n{da}");
        let f = sostituisci(
            file_base(),
            "eventi",
            &riga,
            &riga.replace(da, "gravita = 10"),
        );
        let e = DatiGioco::da_testi(&f).unwrap_err();
        let atteso = format!("{nome}: la gravità di emergenze e scandali sta sotto 10, è 10");
        assert!(
            matches!(e, Errore::Eventi(_)) && e.to_string().contains(&atteso),
            "{e}"
        );
    }
}

#[test]
fn due_effetti_di_spesa_nella_stessa_opzione_rifiutati() {
    let f = sostituisci(
        file_base(),
        "eventi",
        "[[evento.opzioni.effetti]]\ntipo = \"fondo_riserva\"\n",
        "[[evento.opzioni.effetti]]\ntipo = \"fondo_riserva\"\n\n[[evento.opzioni.effetti]]\ntipo = \"debito\"\n",
    );
    let e = DatiGioco::da_testi(&f).unwrap_err();
    assert!(
        matches!(e, Errore::Eventi(_))
            && e.to_string()
                .contains("l'opzione fondo ha più di un effetto di spesa"),
        "{e}"
    );
}

#[test]
fn condizionalita_con_disposizione_o_gradino_sconosciuti_rifiutata_col_messaggio() {
    let da = "disposizione = \"eta_pensionabile\"\ngradino = \"70\"";
    for (nuova, messaggio) in [
        (
            "disposizione = \"inesistente\"\ngradino = \"70\"",
            "condizionalità: disposizione sconosciuta inesistente",
        ),
        (
            "disposizione = \"eta_pensionabile\"\ngradino = \"99\"",
            "condizionalità: gradino sconosciuto «99» per eta_pensionabile",
        ),
    ] {
        let f = sostituisci(file_base(), "eventi", da, nuova);
        let e = DatiGioco::da_testi(&f).unwrap_err();
        assert!(
            matches!(e, Errore::Eventi(_)) && e.to_string().contains(messaggio),
            "{e}"
        );
    }
}

#[test]
fn mes_ristrutturazione_e_clausola_fuori_dalla_loro_famiglia_rifiutati_col_messaggio() {
    let aiuti = "id = \"aiuti\"\nnome = \"Aiuti con un decreto a debito\"\ndefault = false\n\n\
                 [[evento.opzioni.effetti]]\ntipo = \"debito\"";
    for (da, a, messaggio) in [
        (
            aiuti,
            aiuti.replace("tipo = \"debito\"", "tipo = \"mes\""),
            "Crollo della domanda interna: il MES e la ristrutturazione stanno solo sugli eventi del debito",
        ),
        (
            aiuti,
            aiuti.replace("tipo = \"debito\"", "tipo = \"ristrutturazione\""),
            "Crollo della domanda interna: il MES e la ristrutturazione stanno solo sugli eventi del debito",
        ),
        (
            "tipo = \"ristrutturazione\"",
            "tipo = \"clausola_energia\"".to_string(),
            "Aste scoperte: la clausola per l'energia sta solo sugli shock",
        ),
    ] {
        let f = sostituisci(file_base(), "eventi", da, &a);
        let e = DatiGioco::da_testi(&f).unwrap_err();
        assert!(e.to_string().contains(messaggio), "{e}");
    }
}

#[test]
fn evento_del_debito_con_decreto_e_norme_rifiutato_col_messaggio() {
    let da = "nome = \"Aste scoperte\"\ngravita = 10\ndicasteri = []\ndecreto = true\nnorme = []";
    let f = sostituisci(
        file_base(),
        "eventi",
        da,
        &da.replace("norme = []", "norme = [\"eta_pensionabile\"]"),
    );
    let e = DatiGioco::da_testi(&f).unwrap_err();
    assert!(
        e.to_string().contains(
            "Aste scoperte: un evento del debito apre ogni riforma, niente norme per il decreto"
        ),
        "{e}"
    );
}

#[test]
fn evento_del_debito_o_shock_mancante_rifiutato_col_nome() {
    for id in ["aste_scoperte", "shock_bce"] {
        let f = sostituisci(
            file_base(),
            "eventi",
            &format!("id = \"{id}\""),
            "id = \"altro\"",
        );
        let e = DatiGioco::da_testi(&f).unwrap_err();
        assert!(
            e.to_string()
                .contains(&format!("evento richiesto dal motore mancante: {id}")),
            "{e}"
        );
    }
}
