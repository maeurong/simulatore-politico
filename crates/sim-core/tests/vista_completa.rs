mod comune;

use comune::{al_governo, all_opposizione, dati_base, edita, file_base, sostituisci};
use sim_core::{DatiGioco, Errore, Partita, VERSIONE_SCHEMA};

/// Media pesata sulle forze di lavoro, nell'ordine dei dati.
fn media_forze_lavoro(valori: impl Iterator<Item = f64>) -> f64 {
    let dati = dati_base();
    let (somma, pesi) = (dati.regioni().iter())
        .zip(valori)
        .fold((0.0, 0.0), |(s, p), (r, u)| {
            (s + r.forze_lavoro * u, p + r.forze_lavoro)
        });
    somma / pesi
}

#[test]
fn venti_regioni_nell_ordine_dei_dati() {
    let dati = dati_base();
    let v = Partita::nuova(dati_base(), 7).vista();
    assert_eq!(v.regioni.len(), 20);
    for (voce, r) in v.regioni.iter().zip(dati.regioni()) {
        assert_eq!(
            (&voce.id, &voce.nome, voce.macroarea),
            (&r.id, &r.nome, r.macroarea)
        );
        assert!(
            voce.reddito.is_finite() && voce.reddito > 0.0,
            "{}: {}",
            voce.id,
            voce.reddito
        );
    }
}

/// L'esposizione non anticipa nulla: in media le disoccupazioni regionali della Vista sono la
/// riveduta già pubblica, non la disoccupazione vera di oggi.
#[test]
fn la_disoccupazione_regionale_e_in_ritardo() {
    let dati = dati_base();
    let mut p = Partita::nuova(dati_base(), 7);
    // Al turno zero la copia è uguale alla corrente.
    let correnti = p.economia().regioni.iter().map(|r| r.disoccupazione);
    assert!(
        (p.vista().regioni.iter().map(|r| r.disoccupazione)).eq(correnti),
        "turno zero"
    );
    p.chiudi_turno();
    p.chiudi_turno();
    let v = p.vista();
    let media = media_forze_lavoro(v.regioni.iter().map(|r| r.disoccupazione));
    let riveduta = v.statistica.revisione.expect("revisione").riveduta;
    assert!(
        (media - riveduta.disoccupazione).abs() < 1e-9,
        "media {media}, riveduta {}",
        riveduta.disoccupazione
    );
    let corrente = p.economia().disoccupazione_nazionale(&dati);
    assert!(
        (media - corrente).abs() > 1e-9,
        "media {media} uguale alla corrente {corrente}"
    );
}

#[test]
fn le_quote_dei_gruppi_sommano_a_uno() {
    let mut p = Partita::nuova(dati_base(), 7);
    for turno in 0..3 {
        for r in p.vista().regioni {
            assert_eq!(r.gruppi.len(), 18, "turno {turno}, {}", r.id);
            let somma: f64 = r.gruppi.iter().sum();
            assert!(
                (somma - 1.0).abs() < 1e-9,
                "turno {turno}, {}: {somma}",
                r.id
            );
        }
        p.chiudi_turno();
    }
}

#[test]
fn gli_indici_di_pericolosita() {
    let regioni = Partita::nuova(dati_base(), 7).vista().regioni;
    let indici = |f: fn(&sim_core::vista::IndiciPericolosita) -> f64| {
        regioni
            .iter()
            .map(|r| f(&r.pericolosita))
            .collect::<Vec<_>>()
    };
    for (nome, valori) in [
        ("sisma", indici(|i| i.sisma)),
        ("idro", indici(|i| i.idro)),
        ("incendi", indici(|i| i.incendi)),
        ("totale", indici(|i| i.totale)),
    ] {
        assert!(
            valori.iter().all(|x| (0.0..=1.0).contains(x)),
            "{nome}: {valori:?}"
        );
        assert!(
            valori.iter().any(|x| (x - 1.0).abs() < 1e-12),
            "{nome}: nessuna regione a 1, {valori:?}"
        );
    }
}

#[test]
fn schema_18_rifiutato() {
    let p = Partita::nuova(dati_base(), 7);
    let mut json: serde_json::Value = serde_json::from_str(&p.salva().unwrap()).unwrap();
    json["versione_schema"] = 18.into();
    assert_eq!(
        Partita::carica(&json.to_string()),
        Err(Errore::SchemaDiverso {
            trovata: 18,
            attesa: VERSIONE_SCHEMA
        })
    );
    assert_eq!(VERSIONE_SCHEMA, 19);
}

#[test]
fn disoccupazione_regionale_manomessa_rifiutata() {
    let p = Partita::nuova(dati_base(), 7);
    let errore = edita(&p, |j| {
        j["economia"]["regioni"][0]["disoccupazione_prima"] = (-1.0).into();
    })
    .unwrap_err();
    assert!(
        errore.to_string().contains("disoccupazione_prima"),
        "{errore}"
    );
    let errore = edita(&p, |j| {
        let r = j["economia"]["regioni"][0].as_object_mut().unwrap();
        r.remove("disoccupazione_prima");
    })
    .unwrap_err();
    assert!(
        errore.to_string().contains("disoccupazione_prima"),
        "{errore}"
    );
}

#[test]
fn la_minaccia_sta_nel_partito() {
    let p = Partita::nuova(dati_base(), 7);
    assert!(p.vista().partiti.iter().all(|x| !x.minaccia));
    let p = edita(&p, |j| j["partiti"][1]["minaccia"] = true.into()).unwrap();
    let minacce: Vec<bool> = p.vista().partiti.iter().map(|x| x.minaccia).collect();
    assert!(minacce[1], "{minacce:?}");
    assert_eq!(minacce.iter().filter(|&&m| m).count(), 1, "{minacce:?}");
}

#[test]
fn vista_completa_al_turno_zero() {
    let nuova = Partita::nuova(dati_base(), 7);
    let v = nuova.vista();
    assert!(
        v.elezioni.iter().all(|e| e.regioni.is_empty()),
        "nessun voto"
    );
    let r = &v.regole_ue;
    assert_eq!(r.procedura, nuova.economia().regole_ue.pde);
    assert!(!r.coesione_sospesa);
    assert_eq!(r.margine_energia, None);
    // Con la clausola nazionale finita nel 2025 il margine per l'energia del 2026 conta.
    let f = sostituisci(
        file_base(),
        "scenario",
        "clausola_nazionale_fino_a = 2028",
        "clausola_nazionale_fino_a = 2025",
    );
    let corta = Partita::nuova(DatiGioco::da_testi(&f).unwrap(), 7);
    assert_eq!(corta.vista().regole_ue.margine_energia, None);
    let margine = edita(&corta, |j| {
        j["economia"]["regole_ue"]["clausola_energia_fino_a"] = 2026.into();
    })
    .unwrap();
    assert_eq!(margine.vista().regole_ue.margine_energia, Some(2026));
    for (nome, p) in [
        ("al governo", al_governo()),
        ("all'opposizione", all_opposizione()),
    ] {
        let v = p.vista();
        assert!(!v.elezioni.is_empty(), "{nome}");
        for e in &v.elezioni {
            assert_eq!(e.regioni.len(), v.regioni.len(), "{nome}: {:?}", e.data);
        }
        assert_eq!(
            (v.debito.mes, v.debito.ristrutturazione),
            (None, None),
            "{nome}"
        );
        let anno = v.regole_ue.proiezione.expect("proiezione").anno;
        let senza = edita(&p, |j| {
            let conto = j["economia"]["regole_ue"]["spesa_netta"]
                .as_object_mut()
                .unwrap();
            assert!(conto.remove(&anno.to_string()).is_some(), "{nome}: {anno}");
        })
        .unwrap();
        assert_eq!(senza.vista().regole_ue.proiezione, None, "{nome}");
    }
}
