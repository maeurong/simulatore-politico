mod comune;
use comune::{edita, file_base, imposta, indice, presenta, sostituisci};
use serde_json::json;
use sim_core::dati::{DatiGioco, FileDati};
use sim_core::ministri::Causa;
use sim_core::vista::VoceMinistero;
use sim_core::{Comando, Partita};

const LAVORO: &str = "lavoro_welfare";

fn ministero(p: &Partita, id: &str) -> VoceMinistero {
    let v = p.vista();
    v.ministeri.into_iter().find(|m| m.dicastero == id).unwrap()
}

/// Le variazioni del ministro `id` con la causa `c`; 0 in interim.
fn variazioni(p: &Partita, id: &str, c: Causa) -> usize {
    (ministero(p, id).ministro.iter())
        .flat_map(|m| &m.variazioni)
        .filter(|v| v.causa == c)
        .count()
}

/// Disposizione e gradino della proposta di riforma del ministro del Lavoro.
fn riforma(p: &Partita) -> Option<(String, String)> {
    let x = ministero(p, LAVORO).proposta?;
    Some((x.disposizione?, x.gradino?))
}

fn accogli(dicastero: &str) -> Comando {
    Comando::Accogli {
        dicastero: dicastero.into(),
    }
}

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
        let prima = p.salva().unwrap();
        let e = p
            .comanda(Comando::Accogli {
                dicastero: "lavoro_welfare".into(),
            })
            .unwrap_err();
        assert!(e.to_string().contains("calendario pieno"), "{e}");
        assert!(proposta_di_riforma(&p).is_some());
        assert_eq!(p.salva().unwrap(), prima, "il rifiuto non tocca la partita");
        trovata = true;
    }
    assert!(trovata);
}

#[test]
fn salvataggio_con_una_proposta_di_riforma_si_ricarica() {
    let p = Partita::nuova(DatiGioco::da_testi(&reintegro()).unwrap(), 1);
    assert!(riforma(&p).is_some());
    let s = p.salva().unwrap();
    assert_eq!(Partita::carica(&s).unwrap().salva().unwrap(), s);
}

/// Leggi sempre respinte: la riforma accolta torna indietro e la disposizione non si ripresenta
/// per due turni; il ministro non la ripropone.
#[test]
fn legge_respinta_da_poco_non_si_ripropone() {
    let ft = |f, nome: &str, x| imposta(f, "manopole", &format!("[{nome}]\nvalore = "), x);
    let f = ft(ft(reintegro(), "ft_base", "0.9"), "ft_max", "1.0");
    let mut trovata = false;
    for seme in 1..=20 {
        let mut p = Partita::nuova(DatiGioco::da_testi(&f).unwrap(), seme);
        let Some((d, _)) = riforma(&p) else { continue };
        p.comanda(accogli(LAVORO)).unwrap();
        p.chiudi_turno();
        // Una proposta nata nel turno del voto lascerebbe il ministro senza innesco al turno dopo.
        let _ = p.comanda(Comando::Respingi {
            dicastero: LAVORO.into(),
        });
        p.chiudi_turno();
        assert!(!p.vista().esiti[0].approvata);
        if ministero(&p, LAVORO).ministro.is_none() {
            continue;
        }
        assert_ne!(riforma(&p).map(|r| r.0), Some(d));
        // Controllo: il None non è un ministro muto, entro quattro turni propone altro.
        let altra = (0..4).find_map(|_| {
            p.chiudi_turno();
            riforma(&p)
        });
        assert!(altra.is_some(), "il ministro non propone più nulla");
        trovata = true;
    }
    assert!(trovata);
}

/// Il giocatore presenta lui la legge proposta, nel turno in cui la proposta scadrebbe: la
/// proposta decade senza costo.
#[test]
fn proposta_superata_dalla_legge_del_giocatore_decade_senza_costo() {
    let mut trovata = false;
    for seme in 1..=20 {
        let mut p = Partita::nuova(DatiGioco::da_testi(&reintegro()).unwrap(), seme);
        let Some(r) = riforma(&p) else { continue };
        p.chiudi_turno();
        if riforma(&p).as_ref() != Some(&r) {
            continue;
        }
        presenta(&mut p, &[(&r.0, &r.1)]).unwrap();
        p.chiudi_turno();
        if ministero(&p, LAVORO).ministro.is_none() {
            continue;
        }
        assert_eq!(variazioni(&p, LAVORO, Causa::PropostaRespinta), 0);
        trovata = true;
    }
    assert!(trovata);
}

/// Accogli di uno sblocco col ministro all'estremo opposto: la legge lo allontana dalla sua
/// posizione, ma la vuole lui e non paga la firma.
#[test]
fn accogliere_uno_sblocco_non_costa_la_firma() {
    let f = sostituisci(
        file_base(),
        "disposizioni",
        "partenza = \"attuale\"\nlegge = \"L. 199/2025, art. 1 c. 400\"",
        "partenza = \"ridotto\"\nlegge = \"L. 199/2025, art. 1 c. 400\"",
    );
    let s = indice("sanita");
    let mut trovata = false;
    for seme in 1..=20 {
        let p = Partita::nuova(DatiGioco::da_testi(&f).unwrap(), seme);
        let sblocco =
            (ministero(&p, "sanita").proposta).is_some_and(|x| x.motivo.starts_with("sblocca"));
        if !sblocco {
            continue;
        }
        let mut q = edita(&p, |j| {
            j["ministri"]["dicasteri"][s]["ministro"]["posizione"] = json!(1.0);
        })
        .unwrap();
        q.comanda(accogli("sanita")).unwrap();
        assert_eq!(
            q.vista().calendario[0].valori[0].disposizione,
            "privato_accreditato"
        );
        assert_eq!(variazioni(&q, "sanita", Causa::NormaControPosizione), 0);
        trovata = true;
    }
    assert!(trovata);
}

/// L'innesco di una riforma in corso si ricontrolla: il ministro ora vicino alla disposizione, o
/// il prerequisito ormai soddisfatto, e la proposta decade senza costo.
#[test]
fn riforma_senza_piu_innesco_decade_senza_costo() {
    let lw = indice(LAVORO);
    let p = Partita::nuova(DatiGioco::da_testi(&reintegro()).unwrap(), 1);
    let r = riforma(&p).unwrap();
    let mut q = edita(&p, |j| {
        j["ministri"]["dicasteri"][lw]["ministro"]["posizione"] = json!(2.0);
    })
    .unwrap();
    q.chiudi_turno();
    assert_ne!(riforma(&q), Some(r));
    assert_eq!(variazioni(&q, LAVORO, Causa::PropostaRespinta), 0);

    let f = sostituisci(
        file_base(),
        "disposizioni",
        "partenza = \"attuale\"\nlegge = \"L. 199/2025, art. 1 c. 400\"",
        "partenza = \"ridotto\"\nlegge = \"L. 199/2025, art. 1 c. 400\"",
    );
    let privato = comune::dati_base()
        .disposizione("privato_accreditato")
        .unwrap();
    let mut trovata = false;
    for seme in 1..=20 {
        let p = Partita::nuova(DatiGioco::da_testi(&f).unwrap(), seme);
        let sblocco =
            (ministero(&p, "sanita").proposta).is_some_and(|x| x.motivo.starts_with("sblocca"));
        if !sblocco {
            continue;
        }
        // «ampio» soddisfa il prerequisito senza essere il gradino proposto («attuale»).
        let mut q = edita(&p, |j| {
            j["disposizioni"]["voci"][privato]["in_vigore"] = json!(2);
        })
        .unwrap();
        q.chiudi_turno();
        let x = ministero(&q, "sanita").proposta;
        assert!(!x.is_some_and(|x| x.motivo.starts_with("sblocca")));
        assert_eq!(variazioni(&q, "sanita", Causa::PropostaRespinta), 0);
        trovata = true;
    }
    assert!(trovata);
}
