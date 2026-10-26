#![expect(
    clippy::unwrap_used,
    reason = "nei test un unwrap che fallisce e' il fallimento voluto"
)]

mod comune;
use comune::{edita, file_base, indice, sostituisci};
use serde_json::{Value, json};
use sim_core::dati::{DatiGioco, FileDati};
use sim_core::ministri::Causa;
use sim_core::vista::{Vista, VoceMinistero};
use sim_core::{Comando, Errore, Partita};

fn ministero<'a>(v: &'a Vista, id: &str) -> &'a VoceMinistero {
    v.ministeri.iter().find(|m| m.dicastero == id).unwrap()
}

/// La Sanità con i quattro atti a 9 quote: nessuno è pagabile, il ministro chiede fondi.
fn sanita_cara() -> FileDati {
    sanita_a("9.0")
}

/// La Sanità con i quattro atti al costo `nuovo`, in quote del margine annuo.
fn sanita_a(nuovo: &str) -> FileDati {
    let effetto = "effetti = [\n  { grandezza = \"qualita_servizio_sanitario\", intensita = ";
    let mut f = file_base();
    for coda in [
        "8.0, ritardo = 1, forma = \"impulso\", durata = 2.0",
        "8.0, ritardo = 1, forma = \"impulso\", durata = 1.5",
        "4.0",
        "2.0",
    ] {
        let costo = if coda == "2.0" {
            "costo = 0.5\n"
        } else {
            "costo = 1.0\n"
        };
        let da = format!("{costo}{effetto}{coda}");
        f = sostituisci(
            f,
            "atti",
            &da,
            &da.replacen(costo, &format!("costo = {nuovo}\n"), 1),
        );
    }
    f
}

fn con_proposta(seme: u64) -> Partita {
    Partita::nuova(DatiGioco::da_testi(&sanita_cara()).unwrap(), seme)
}

#[test]
fn fondi_che_non_bastano_portano_una_proposta() {
    let v = con_proposta(1).vista();
    let m = ministero(&v, "sanita");
    assert!(m.agenda.is_none());
    let p = m.proposta.as_ref().unwrap();
    assert!(p.importo > 0 && p.motivo.starts_with("fondi che non bastano"));
}

/// Accogli è un decreto una tantum: l'importo si spende nel trimestre che si chiude, il bilancio
/// annuo resta quello della gemella che non accoglie.
#[test]
fn accogliere_spende_l_importo_nel_trimestre_e_mette_l_atto_in_agenda() {
    let (mut p, mut gemella) = (con_proposta(1), con_proposta(1));
    let v = p.vista();
    let m = ministero(&v, "sanita");
    let (importo, atto) = (
        m.proposta.as_ref().unwrap().importo,
        m.proposta.as_ref().unwrap().atto.clone(),
    );
    p.comanda(Comando::Accogli {
        dicastero: "sanita".into(),
    })
    .unwrap();
    let v = p.vista();
    let m = ministero(&v, "sanita");
    assert_eq!(m.agenda.as_ref().unwrap().atto, atto);
    assert_eq!(
        m.agenda.as_ref().unwrap().motivo,
        "fondi accolti dal Presidente del Consiglio"
    );
    assert!(m.proposta.is_none());
    assert_eq!(p.economia().decreti, importo);
    assert_eq!(p.economia().bilancio, gemella.economia().bilancio);
    p.chiudi_turno();
    gemella.chiudi_turno();
    let (e, g) = (p.economia(), gemella.economia());
    assert_eq!(e.decreti, 0);
    assert_eq!(e.conti.spese.dicasteri, g.conti.spese.dicasteri + importo);
    assert_eq!(e.bilancio, g.bilancio);
}

#[test]
fn rinviare_una_volta_senza_costo() {
    let mut p = con_proposta(1);
    let s = |p: &Partita| {
        ministero(&p.vista(), "sanita")
            .ministro
            .as_ref()
            .unwrap()
            .soddisfazione
    };
    let prima = s(&p);
    p.comanda(Comando::Rinvia {
        dicastero: "sanita".into(),
    })
    .unwrap();
    assert_eq!(s(&p), prima);
    assert!(
        p.comanda(Comando::Rinvia {
            dicastero: "sanita".into()
        })
        .is_err()
    );
}

#[test]
fn proposta_ignorata_vale_respinta_dopo_un_turno() {
    let mut p = con_proposta(1);
    p.chiudi_turno();
    assert!(ministero(&p.vista(), "sanita").proposta.is_some());
    p.chiudi_turno();
    let v = p.vista();
    let x = ministero(&v, "sanita").ministro.as_ref().unwrap();
    assert!(
        x.variazioni
            .iter()
            .any(|v| v.causa == Causa::PropostaRespinta)
    );
}

#[test]
fn dimissioni_annullano_agenda_e_proposta() {
    const IT: &str = "infrastrutture_trasporti";
    let mut p = con_proposta(1);
    p.chiudi_turno();
    // Turno 1: margine di un anno intero, Infrastrutture e Trasporti avvia un cantiere.
    let v = p.vista();
    let it = ministero(&v, IT);
    let cantiere = it
        .atti
        .iter()
        .filter(|a| a.turni > 1 && a.pagabile)
        .min_by_key(|a| a.costo)
        .unwrap()
        .id
        .clone();
    if !it.cantieri.is_empty() {
        // Un cantiere c'è già dal turno 0: fermo l'agenda, così il margine resta per il turno 2.
        p.comanda(Comando::Ferma {
            dicastero: IT.into(),
        })
        .unwrap();
    } else if it.agenda.as_ref().unwrap().atto != cantiere {
        p.comanda(Comando::Dirotta {
            dicastero: IT.into(),
            atto: Some(cantiere),
            regione: None,
        })
        .unwrap();
    }
    p.chiudi_turno();
    // Turno 2: cantiere in corso, agenda annunciata, la Sanità ha di nuovo una proposta.
    let v = p.vista();
    assert!(ministero(&v, "sanita").proposta.is_some() && ministero(&v, IT).agenda.is_some());
    let (residuo, rate) = (
        ministero(&v, IT).margine_residuo,
        ministero(&v, IT)
            .cantieri
            .iter()
            .map(|c| c.rata)
            .sum::<i64>(),
    );
    p.comanda(Comando::ChiediDimissioni {
        dicastero: "sanita".into(),
    })
    .unwrap();
    p.comanda(Comando::ChiediDimissioni {
        dicastero: IT.into(),
    })
    .unwrap();
    p.chiudi_turno();
    // Turno 3, interim: pagata solo la rata del cantiere, non l'agenda; la proposta è decaduta.
    let v = p.vista();
    assert!(
        ministero(&v, "sanita").ministro.is_none() && ministero(&v, "sanita").proposta.is_none()
    );
    assert_eq!(ministero(&v, IT).margine_residuo, residuo - rate);
    let mancanti = |v: &Vista| -> Vec<u32> {
        (ministero(v, IT).cantieri.iter())
            .map(|c| c.turni_mancanti)
            .collect()
    };
    let prima = mancanti(&v);
    p.chiudi_turno();
    // Turno 4: nell'interim i cantieri sono rimasti fermi, senza perdere quanto speso.
    let v = p.vista();
    assert_eq!(mancanti(&v), prima);
    assert_eq!(ministero(&v, IT).margine_residuo, residuo - rate);
}

#[test]
fn comandi_delle_proposte_non_ammessi_rifiutati_col_nome() {
    let mut p = Partita::nuova(DatiGioco::da_testi(&file_base()).unwrap(), 1);
    let e = p
        .comanda(Comando::Accogli {
            dicastero: "giustizia".into(),
        })
        .unwrap_err();
    assert!(
        matches!(e, Errore::Comando(_)) && e.to_string().contains("giustizia"),
        "{e}"
    );
}

#[test]
fn accogliere_fa_crescere_il_margine_residuo_dell_importo() {
    let mut p = con_proposta(1);
    let v = p.vista();
    let (importo, residuo) = (
        ministero(&v, "sanita").proposta.as_ref().unwrap().importo,
        ministero(&v, "sanita").margine_residuo,
    );
    p.comanda(Comando::Accogli {
        dicastero: "sanita".into(),
    })
    .unwrap();
    let v = p.vista();
    assert_eq!(ministero(&v, "sanita").margine_residuo, residuo + importo);
}

#[test]
fn respingere_toglie_la_proposta_e_costa_soddisfazione() {
    let costo = DatiGioco::da_testi(&sanita_cara())
        .unwrap()
        .manopola("costo_respingere");
    let mut p = con_proposta(1);
    let s = |p: &Partita| {
        let v = p.vista();
        ministero(&v, "sanita")
            .ministro
            .as_ref()
            .unwrap()
            .soddisfazione
    };
    let prima = s(&p);
    p.comanda(Comando::Respingi {
        dicastero: "sanita".into(),
    })
    .unwrap();
    assert!(ministero(&p.vista(), "sanita").proposta.is_none());
    assert!((prima - s(&p) - costo).abs() < 1e-9, "{prima} -> {}", s(&p));
}

/// Rinvia al turno della nascita: la proposta sopravvive a due chiusure e vale respinta alla
/// terza; al ricalcolo dell'importo `turni_in_sospeso` e `rinviata` restano.
#[test]
fn rinvia_tiene_la_proposta_un_turno_poi_vale_respinta() {
    let mut p = con_proposta(1);
    let proposta = |p: &Partita| p.ministri().dicasteri[3].proposta;
    let nata = proposta(&p).unwrap();
    p.comanda(Comando::Rinvia {
        dicastero: "sanita".into(),
    })
    .unwrap();
    p.chiudi_turno();
    let dopo = proposta(&p).unwrap();
    assert_ne!(
        dopo.importo, nata.importo,
        "il margine annuo cambia l'importo"
    );
    assert_eq!((dopo.turni_in_sospeso, dopo.rinviata), (1, true));
    let respinte = |p: &Partita| {
        let v = p.vista();
        let x = ministero(&v, "sanita").ministro.clone().unwrap();
        (x.variazioni.iter())
            .filter(|v| v.causa == Causa::PropostaRespinta)
            .count()
    };
    p.chiudi_turno();
    assert_eq!(proposta(&p).unwrap().turni_in_sospeso, 2);
    assert_eq!(respinte(&p), 0);
    p.chiudi_turno();
    assert_eq!(respinte(&p), 1);
    // L'atto resta impagabile: all'apertura nasce una proposta nuova.
    let nuova = proposta(&p).unwrap();
    assert_eq!((nuova.turni_in_sospeso, nuova.rinviata), (0, false));
}

/// Costo 2 quote: impagabile al turno 0 (un trimestre di margine), pagabile al turno 1
/// (margine annuo), quindi la proposta del turno 0 decade all'apertura del turno 1.
#[test]
fn la_proposta_decade_quando_l_atto_diventa_pagabile() {
    let mut p = Partita::nuova(DatiGioco::da_testi(&sanita_a("2.0")).unwrap(), 1);
    assert!(ministero(&p.vista(), "sanita").proposta.is_some());
    p.chiudi_turno();
    let v = p.vista();
    let m = ministero(&v, "sanita");
    assert!(m.proposta.is_none() && m.agenda.is_some());
    let x = m.ministro.as_ref().unwrap();
    assert!(
        x.variazioni
            .iter()
            .all(|v| v.causa != Causa::PropostaRespinta)
    );
}

type Modifica = Box<dyn Fn(&mut Value)>;

#[test]
fn salvataggi_con_proposte_corrotte_rifiutati() {
    let p = con_proposta(1);
    let s = indice("sanita");
    let atto = p.ministri().dicasteri[s].proposta.unwrap().atto;
    assert!(edita(&p, |_| ()).is_ok());
    let righe: Vec<(&str, Modifica)> = vec![
        (
            "importo nullo",
            Box::new(move |j| {
                j["ministri"]["dicasteri"][s]["proposta"]["importo"] = json!(0);
            }),
        ),
        (
            "importo negativo",
            Box::new(move |j| {
                j["ministri"]["dicasteri"][s]["proposta"]["importo"] = json!(-5);
            }),
        ),
        (
            "importo oltre il tetto",
            Box::new(move |j| {
                j["ministri"]["dicasteri"][s]["proposta"]["importo"] =
                    json!(1_000_000_000_000_001_i64);
            }),
        ),
        (
            "proposta in interim",
            Box::new(move |j| {
                j["ministri"]["dicasteri"][s]["ministro"] = Value::Null;
            }),
        ),
        (
            "atto di un altro dicastero",
            Box::new(move |j| {
                j["ministri"]["dicasteri"][s]["proposta"]["atto"] = json!(0);
            }),
        ),
        (
            "proposta insieme a un'agenda",
            Box::new(move |j| {
                j["ministri"]["dicasteri"][s]["agenda"] = json!({"atto": atto, "regione": null,
                "stato": "annunciata", "motivo": "x", "attaccamenti": [0.0, 0.0, 0.0, 0.0],
                "bersaglio": []});
            }),
        ),
    ];
    for (caso, modifica) in righe {
        let e = edita(&p, |j| modifica(j)).err();
        assert!(
            matches!(e, Some(Errore::SalvataggioIlleggibile(_))),
            "{caso}: {e:?}"
        );
    }
}

/// Il comando è rifiutato con `Errore::Comando` che contiene `nome`, e la partita resta identica.
fn rifiuta(p: &mut Partita, c: Comando, nome: &str) {
    let prima = p.salva().unwrap();
    let e = p.comanda(c.clone()).unwrap_err();
    assert!(
        matches!(e, Errore::Comando(_)) && e.to_string().contains(nome),
        "{c:?}: {e}"
    );
    assert_eq!(p.salva().unwrap(), prima, "{c:?}");
}

fn accogli() -> Comando {
    Comando::Accogli {
        dicastero: "sanita".into(),
    }
}

/// Accogli ricalcola quanto manca: se il margine ormai basta rifiuta, altrimenti spende il
/// valore ricalcolato e non quello memorizzato.
#[test]
fn accogli_ricalcola_quanto_manca() {
    let p = con_proposta(1);
    let s = indice("sanita");
    let vera = p.ministri().dicasteri[s].proposta.unwrap();
    let id = ministero(&p.vista(), "sanita")
        .proposta
        .as_ref()
        .unwrap()
        .atto
        .clone();
    let mut ricco = edita(&p, |j| {
        j["ministri"]["dicasteri"][s]["margine_residuo"] = json!(1_000_000_000_000_000_i64);
    })
    .unwrap();
    rifiuta(
        &mut ricco,
        accogli(),
        &format!("fondi già sufficienti per {id}"),
    );
    let mut falsa = edita(&p, |j| {
        j["ministri"]["dicasteri"][s]["proposta"]["importo"] = json!(1);
    })
    .unwrap();
    falsa.comanda(accogli()).unwrap();
    assert_eq!(falsa.economia().decreti, vera.importo);
}

#[test]
fn accogli_rifiuta_un_atto_gia_in_cantiere() {
    let p = con_proposta(1);
    let s = indice("sanita");
    let atto = p.ministri().dicasteri[s].proposta.unwrap().atto;
    let id = ministero(&p.vista(), "sanita")
        .proposta
        .as_ref()
        .unwrap()
        .atto
        .clone();
    let mut q = edita(&p, |j| {
        let c = json!({"atto": atto, "regione": null, "turni_mancanti": 2, "rata": 0,
            "speso": 0});
        j["ministri"]["dicasteri"][s]["cantieri"] = json!([c]);
    })
    .unwrap();
    rifiuta(&mut q, accogli(), &format!("cantiere già in corso: {id}"));
}
