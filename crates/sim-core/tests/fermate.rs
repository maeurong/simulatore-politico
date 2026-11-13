mod comune;

use comune::{al_governo, all_opposizione, dati_base, edita, fondata, togli_opposizione_ia, vinta};
use serde_json::json;
use sim_core::Partita;
use sim_core::elettorato::Condizione;
use sim_core::vista::Fermata;

const ID: u32 = 9999;

/// Una disposizione nazionale che tocca alcune condizioni e non tutte: (indice, una toccata, una no).
fn nazionale_parziale() -> (usize, Condizione, Condizione) {
    let dati = dati_base();
    (dati.disposizioni().iter().enumerate())
        .find_map(|(d, x)| {
            let toccata = *x.gruppi.first()?;
            let fuori = Condizione::TUTTE
                .into_iter()
                .find(|c| !x.gruppi.contains(c))?;
            x.perimetro.is_empty().then_some((d, toccata, fuori))
        })
        .unwrap()
}

/// `p` con la riforma `ID` sulle disposizioni `d` al voto ora, e nelle condizioni del sondaggio pubblico gli elettori del giocatore solo nei `campioni`,
/// con quota 1; le altre condizioni hanno campione e quota zero.
fn con_misura(p: &Partita, d: &[usize], campioni: &[(Condizione, u64)]) -> Partita {
    let g = p.giocatore();
    edita(p, |j| {
        let turno = j["turno"].clone();
        (j["parlamento"]["misure"].as_array_mut().unwrap()).push(json!({
            "id": ID, "forma": "riforma", "valori": d.iter().map(|&d| json!([d, 0])).collect::<Vec<_>>(), "proponente": null,
            "promotore": null, "linea_giocatore": null, "presentata": turno, "voto": turno,
            "deliberazione": 0, "fiducia": false, "emendamenti": [], "emendamenti_aperti": false,
            "fondi": null,
        }));
        for inc in j["informazione"]["sondaggio"]["condizioni"]
            .as_array_mut()
            .unwrap()
        {
            let n = (campioni.iter())
                .find(|(c, _)| inc["id"] == c.to_string())
                .map_or(0, |&(_, n)| n);
            inc["campione"] = json!(n);
            inc["quote"][g] = json!(if n > 0 { 1.0 } else { 0.0 });
        }
    })
    .unwrap()
}

fn misura(p: &Partita) -> sim_core::parlamento::Misura {
    (p.parlamento().misure.iter())
        .find(|m| m.id == ID)
        .cloned()
        .unwrap()
}

#[test]
fn al_governo_solo_governo() {
    assert_eq!(al_governo().vista().fermate, [Fermata::Governo]);
}

#[test]
fn a_partita_finita_solo_fine() {
    let mut p = all_opposizione();
    p.abbandona().unwrap();
    assert_eq!(p.vista().fermate, [Fermata::Fine]);
}

#[test]
fn turno_quieto_lista_vuota() {
    let mut p = all_opposizione();
    togli_opposizione_ia(&mut p);
    // Le misure al voto ora slittano al turno dopo: toglierle lascerebbe orfano il decreto in
    // agenda della loro conversione.
    let p = edita(&p, |j| {
        let turno = j["turno"].as_u64().unwrap();
        for m in j["parlamento"]["misure"].as_array_mut().unwrap() {
            if m["voto"] == turno {
                m["voto"] = json!(turno + 1);
            }
        }
    })
    .unwrap();
    let v = p.vista();
    assert!(v.fermate.is_empty(), "{:?} {:?}", v.fermate, v.data);
}

#[test]
fn il_trenta_per_cento_ferma() {
    let (d, toccata, fuori) = nazionale_parziale();
    let p = all_opposizione();
    let trenta = con_misura(&p, &[d], &[(toccata, 30), (fuori, 70)]);
    assert_eq!(trenta.quota_toccata(&misura(&trenta)), 0.3);
    assert!(
        trenta
            .vista()
            .fermate
            .contains(&Fermata::Voto { misura: ID }),
        "{:?}",
        trenta.vista().fermate
    );
    let ventinove = con_misura(&p, &[d], &[(toccata, 29), (fuori, 71)]);
    assert!(ventinove.quota_toccata(&misura(&ventinove)) < 0.3);
    assert!(
        !(ventinove.vista().fermate).contains(&Fermata::Voto { misura: ID }),
        "{:?}",
        ventinove.vista().fermate
    );
}

#[test]
fn una_misura_che_vota_un_altro_turno_non_ferma() {
    let (d, toccata, fuori) = nazionale_parziale();
    let p = con_misura(&all_opposizione(), &[d], &[(toccata, 30), (fuori, 70)]);
    let dopo = edita(&p, |j| {
        let turno = j["turno"].as_u64().unwrap();
        for m in j["parlamento"]["misure"].as_array_mut().unwrap() {
            if m["id"] == ID {
                m["voto"] = json!(turno + 1);
            }
        }
    })
    .unwrap();
    assert_eq!(dopo.quota_toccata(&misura(&dopo)), 0.3);
    assert!(
        !(dopo.vista().fermate).contains(&Fermata::Voto { misura: ID }),
        "{:?}",
        dopo.vista().fermate
    );
}

/// `p` con gli elettori del giocatore nelle sole regioni dei `campioni` (id, n), quota 1.
fn con_regioni(p: &Partita, campioni: &[(&str, u64)]) -> Partita {
    let g = p.giocatore();
    edita(p, |j| {
        for inc in j["informazione"]["sondaggio"]["regioni"]
            .as_array_mut()
            .unwrap()
        {
            let n = (campioni.iter())
                .find(|(r, _)| inc["id"] == *r)
                .map_or(0, |&(_, n)| n);
            inc["campione"] = json!(n);
            inc["quote"][g] = json!(if n > 0 { 1.0 } else { 0.0 });
        }
    })
    .unwrap()
}

#[test]
fn il_perimetro_pesa_la_quota_della_disposizione() {
    let dati = dati_base();
    // Perimetro: abruzzo a peso 0,5, campania a peso 1, la Lombardia fuori.
    let d = dati.disposizione("credito_zes").unwrap();
    let gruppo = dati.disposizioni()[d].gruppi[0];
    let p = con_misura(&all_opposizione(), &[d], &[(gruppo, 100)]);
    let quota = |campioni: &[(&str, u64)]| {
        let q = con_regioni(&p, campioni);
        let ferma = (q.vista().fermate).contains(&Fermata::Voto { misura: ID });
        (q.quota_toccata(&misura(&q)), ferma)
    };
    let (q, ferma) = quota(&[("campania", 29), ("lombardia", 71)]);
    assert!(q < 0.3 && !ferma, "{q} {ferma}");
    assert_eq!(quota(&[("campania", 30), ("lombardia", 70)]), (0.3, true));
    assert_eq!(quota(&[("abruzzo", 100)]), (0.5, true));
}

#[test]
fn la_quota_della_misura_e_la_piu_alta_delle_sue_disposizioni() {
    let dati = dati_base();
    let nazionali: Vec<_> = (dati.disposizioni().iter().enumerate())
        .filter(|(_, x)| x.perimetro.is_empty() && !x.gruppi.is_empty())
        .collect();
    // Due disposizioni nazionali coi gruppi disgiunti.
    let ((a, xa), (b, xb)) = nazionali
        .iter()
        .flat_map(|&u| nazionali.iter().map(move |&v| (u, v)))
        .find(|&((_, x), (_, y))| x.gruppi.iter().all(|c| !y.gruppi.contains(c)))
        .unwrap();
    let campioni = [(xa.gruppi[0], 40), (xb.gruppi[0], 60)];
    let p = all_opposizione();
    for ordine in [[a, b], [b, a]] {
        let q = con_misura(&p, &ordine, &campioni);
        assert_eq!(q.quota_toccata(&misura(&q)), 0.6, "{ordine:?}");
    }
}

#[test]
fn senza_elettori_nessuna_fermata_sul_voto() {
    let (d, toccata, _) = nazionale_parziale();
    let p = all_opposizione();
    let g = p.giocatore();
    // Quota zero in ogni incrocio, campione pieno.
    let quota_zero = edita(&con_misura(&p, &[d], &[]), |j| {
        let s = &mut j["informazione"]["sondaggio"];
        for chiave in ["condizioni", "regioni"] {
            for inc in s[chiave].as_array_mut().unwrap() {
                inc["campione"] = json!(500);
                inc["quote"][g] = json!(0.0);
            }
        }
    })
    .unwrap();
    // Campione zero in ogni incrocio, quota piena.
    let campione_zero = edita(&con_misura(&p, &[d], &[(toccata, 0)]), |j| {
        let s = &mut j["informazione"]["sondaggio"];
        for chiave in ["condizioni", "regioni"] {
            for inc in s[chiave].as_array_mut().unwrap() {
                inc["campione"] = json!(0);
                inc["quote"][g] = json!(1.0);
            }
        }
    })
    .unwrap();
    for p in [quota_zero, campione_zero] {
        for m in &p.parlamento().misure {
            assert_eq!(p.quota_toccata(m), 0.0, "misura {}", m.id);
        }
        let fermate = p.vista().fermate;
        assert!(
            !fermate.iter().any(|f| matches!(f, Fermata::Voto { .. })),
            "{fermate:?}"
        );
    }
}

#[test]
fn l_ultimatum_ferma_una_volta() {
    let p = all_opposizione();
    let gov = p.governo().clone();
    let alleato = (0..p.partiti().len())
        .find(|&j| j != gov.presidente && gov.al_governo(j))
        .unwrap();
    let id = p.partiti()[alleato].id.clone();
    let mut p = edita(&p, |j| {
        j["intesa"]["valori"][alleato] = json!(35.0);
        j["intesa"]["ultimatum"][alleato] = json!(null);
    })
    .unwrap();
    p.chiudi_turno();
    let ultimatum = Fermata::Ultimatum { partito: id };
    assert!(
        p.vista().fermate.contains(&ultimatum),
        "{:?} {:?}",
        p.vista().fermate,
        p.vista().esito
    );
    p.chiudi_turno();
    assert!(!p.vista().fermate.contains(&ultimatum));
}

#[test]
fn nel_turno_del_voto_campagna() {
    let v = fondata(4.5).vista();
    assert_eq!(v.turni_al_voto, 0);
    assert!(v.fermate.contains(&Fermata::Campagna), "{:?}", v.fermate);
}

#[test]
fn incarico_al_giocatore_ferma() {
    let v = vinta().vista();
    assert!(v.fermate.contains(&Fermata::Incarico), "{:?}", v.fermate);
    assert!(!v.fermate.contains(&Fermata::Governo));
}

#[test]
fn posto_riservato_ferma() {
    let mut p = all_opposizione();
    for _ in 0..12 {
        if p.vista().posto_riservato {
            break;
        }
        p.chiudi_turno();
    }
    let v = p.vista();
    assert!(v.posto_riservato);
    assert!(
        v.fermate.contains(&Fermata::PostoRiservato),
        "{:?}",
        v.fermate
    );
}

#[test]
fn luglio_settembre_referendum() {
    let mut p = all_opposizione();
    // Fuori dai sei mesi dopo i comizi, e a luglio-settembre.
    for _ in 0..12 {
        let v = p.vista();
        if v.data.trimestre == 3 && v.turno > p.legislatura().inizio {
            break;
        }
        p.chiudi_turno();
    }
    let senza = edita(&p, |j| j["parlamento"]["referendum"] = json!(null)).unwrap();
    let v = senza.vista();
    assert_eq!(v.data.trimestre, 3);
    assert!(v.fermate.contains(&Fermata::Referendum), "{:?}", v.fermate);
    // Con un referendum in corso, a luglio-settembre non si ferma.
    let k = dati_base().disposizione("licenziamento").unwrap();
    let voto = senza.vista().turno + 1;
    let con = edita(&senza, |j| {
        j["parlamento"]["referendum"] = json!({
            "tipo": "abrogativo", "misura": null, "valori": [[k, 0]], "promotore": 0,
            "del_giocatore": false, "voto": voto, "firme": true, "invito_astensione": false,
            "richiesto": true,
        });
    })
    .unwrap();
    assert!(!con.vista().fermate.contains(&Fermata::Referendum));
}
