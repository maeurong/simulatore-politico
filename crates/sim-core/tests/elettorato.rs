#![expect(
    clippy::unwrap_used,
    reason = "nei test un unwrap che fallisce e' il fallimento voluto"
)]

mod comune;
use comune::{dati_base, file_base, imposta, riscrivi, senza_shock, sostituisci};
use sim_core::caso::{Caso, Flusso};
use sim_core::elettorato::{Condizione, Fascia, Reddito, indice_gruppo, intenzioni};
use sim_core::{DatiGioco, Errore, FileDati, Partita, Trimestre};

/// Messaggio dell'errore, dopo aver verificato che la variante e' quella attesa.
fn rifiuto(f: FileDati, variante: fn(&Errore) -> bool) -> String {
    let e = DatiGioco::da_testi(&f).unwrap_err();
    assert!(variante(&e), "variante sbagliata: {e}");
    e.to_string()
}

fn elettorato(f: FileDati) -> String {
    rifiuto(f, |e| matches!(e, Errore::Elettorato(_)))
}

fn partiti(f: FileDati) -> String {
    rifiuto(f, |e| matches!(e, Errore::Partiti(_)))
}

#[test]
fn dati_base_hanno_360_gruppi_in_ordine() {
    let d = dati_base();
    let e = d.elettorato();
    assert_eq!(e.gruppi.len(), 360);
    for (r, reg) in d.regioni().iter().enumerate() {
        let somma: f64 = (0..18).map(|k| e.gruppi[r * 18 + k].quota).sum();
        assert!((somma - 1.0).abs() < 1e-6, "{}", reg.id);
        assert_eq!(e.gruppi[r * 18].regione, reg.id);
    }
    assert_eq!(
        e.gruppi[indice_gruppo(0, Condizione::Pensionato, Fascia::Alta)].fascia,
        Fascia::Alta
    );
}

#[test]
fn gruppo_di_regione_sconosciuta_rifiutato_col_nome() {
    let m = elettorato(imposta(
        file_base(),
        "elettorato",
        "regione = \"",
        "molisse\"",
    ));
    assert!(m.contains("molisse"), "{m}");
}

#[test]
fn istogramma_che_non_somma_a_uno_rifiutato_col_gruppo() {
    let m = elettorato(imposta(file_base(), "elettorato", "posizione = [", "0.9"));
    assert!(
        m.contains("dipendente_privato") && m.contains("bassa"),
        "{m}"
    );
}

#[test]
fn interessi_con_indicatore_sconosciuto_rifiutati_col_nome() {
    let f = sostituisci(
        file_base(),
        "elettorato",
        "qualita_trasporti = 0.10",
        "trasporto = 0.10",
    );
    let m = elettorato(f);
    assert!(m.contains("trasporto"), "{m}");
}

#[test]
fn pesi_degli_interessi_che_non_sommano_a_uno_rifiutati() {
    let f = sostituisci(
        file_base(),
        "elettorato",
        "disoccupazione = 0.25",
        "disoccupazione = 0.35",
    );
    let m = elettorato(f);
    assert!(m.contains("dipendente_privato") && m.contains("1.1"), "{m}");
}

#[test]
fn campione_sotto_cento_rifiutato() {
    let m = elettorato(riscrivi(
        file_base(),
        "elettorato",
        "campione = ",
        "campione = 99",
    ));
    assert!(m.contains("campione minimo 99"), "{m}");
}

#[test]
fn nomi_vuoti_rifiutati() {
    let m = elettorato(riscrivi(file_base(), "elettorato", "nomi = ", "nomi = []"));
    assert!(m.contains("nomi"), "{m}");
}

#[test]
fn regione_senza_elettori_rifiutata_col_nome() {
    let m = elettorato(imposta(file_base(), "elettorato", "elettori = ", "0"));
    assert!(m.contains("piemonte"), "{m}");
}

#[test]
fn partito_sconosciuto_nel_governo_rifiutato_col_nome() {
    let f = sostituisci(
        file_base(),
        "scenario",
        "presidente = \"destra\"",
        "presidente = \"destrra\"",
    );
    let m = partiti(f);
    assert!(m.contains("destrra"), "{m}");
}

#[test]
fn gruppi_fuori_ordine_canonico_rifiutati() {
    for (da, a) in [
        ("fascia = \"bassa\"", "fascia = \"media\""),
        (
            "condizione = \"dipendente_privato\"",
            "condizione = \"autonomo\"",
        ),
    ] {
        let m = elettorato(sostituisci(file_base(), "elettorato", da, a));
        assert!(m.contains("gruppo 0 fuori ordine"), "{a}: {m}");
    }
}

#[test]
fn partito_sconosciuto_in_un_dicastero_rifiutato_col_nome() {
    let f = sostituisci(
        file_base(),
        "scenario",
        "sanita = \"destra\"",
        "sanita = \"destrra\"",
    );
    let m = partiti(f);
    assert!(m.contains("destrra"), "{m}");
}

#[test]
fn trasferimenti_seguono_pensione_e_sussidio() {
    let d = dati_base();
    let p = Partita::nuova(d.clone(), 1);
    let mut el = p.elettorato().clone();
    el.base.pensione *= 0.5;
    el.base.sussidio *= 0.25;
    el.aggiorna_redditi(&d, p.economia());
    for (c, k) in [
        (Condizione::Pensionato, 2.0),
        (Condizione::DisoccupatoInattivo, 4.0),
        (Condizione::DipendentePrivato, 1.0),
    ] {
        let i = indice_gruppo(5, c, Fascia::Media);
        let atteso = k * d.elettorato().gruppi[i].trasferimenti;
        assert!(
            (el.gruppi[i].reddito.trasferimenti - atteso).abs() < 1e-6 * atteso.max(1.0),
            "{c}"
        );
    }
}

#[test]
fn dicastero_senza_partito_rifiutato_col_nome() {
    let f = sostituisci(file_base(), "scenario", "sanita = \"destra\", ", "");
    let m = partiti(f);
    assert!(m.contains("sanita"), "{m}");
}

#[test]
fn partito_in_due_coalizioni_rifiutato_col_nome() {
    let f = sostituisci(
        file_base(),
        "scenario",
        "[\"sinistra\", \"centrosinistra\"]",
        "[\"sinistra\", \"centrosinistra\", \"destra\"]",
    );
    let m = partiti(f);
    assert!(m.contains("destra"), "{m}");
}

#[test]
fn valence_fuori_intervallo_rifiutata_col_nome() {
    let f = imposta(
        file_base(),
        "scenario",
        "id = \"centro\"\nnome = \"Centro\"\nposizione = 4.0\n\n[partito.valence]\nvalore = ",
        "9.0",
    );
    let e = DatiGioco::da_testi(&f).unwrap_err();
    assert!(
        matches!(&e, Errore::ScenarioFuoriIntervallo { nome, .. } if nome == "valence di centro"),
        "{e}"
    );
}

#[test]
fn partita_nuova_ha_il_governo_uscente() {
    let p = Partita::nuova(dati_base(), 1);
    let pres = &p.partiti()[p.governo().presidente];
    assert_eq!(pres.id, "destra");
    assert_eq!(p.partiti().len(), 7);
    assert!(
        p.elettorato()
            .gruppi
            .iter()
            .all(|g| g.giudizio.iter().all(|&x| x == 0.0))
    );
}

#[test]
fn redditi_al_turno_zero_dai_dati() {
    let d = dati_base();
    let p = Partita::nuova(d.clone(), 1);
    let i = indice_gruppo(3, Condizione::Autonomo, Fascia::Alta);
    let g = &d.elettorato().gruppi[i];
    let lordo = g.primario + g.trasferimenti;
    let atteso = lordo - d.elettorato().aliquote[&Fascia::Alta] * lordo;
    assert!((p.elettorato().gruppi[i].reddito.reale() - atteso).abs() < 1e-6);
}

#[test]
fn bilancio_fermo_erode_il_reddito_dei_dipendenti_pubblici() {
    // 4 turni senza shock: salari pubblici fermi in nominale fino al bilancio di gennaio,
    // salari privati che seguono il PIL nominale.
    let mut p = Partita::nuova(DatiGioco::da_testi(&senza_shock()).unwrap(), 1);
    let pubblico = indice_gruppo(11, Condizione::DipendentePubblico, Fascia::Media);
    let privato = indice_gruppo(11, Condizione::DipendentePrivato, Fascia::Media);
    let reale = |p: &Partita, i: usize| p.elettorato().gruppi[i].reddito.reale();
    let (pu0, pr0) = (reale(&p, pubblico), reale(&p, privato));
    for _ in 0..4 {
        p.chiudi_turno();
    }
    assert!(reale(&p, pubblico) / pu0 < reale(&p, privato) / pr0);
}

#[test]
fn gini_nullo_con_redditi_uguali() {
    let f = riscrivi(
        riscrivi(
            riscrivi(
                file_base(),
                "elettorato",
                "primario = ",
                "primario = 1000.0",
            ),
            "elettorato",
            "trasferimenti = ",
            "trasferimenti = 0.0",
        ),
        "elettorato",
        "aliquote = ",
        "aliquote = { bassa = 0.2, media = 0.2, alta = 0.2 }",
    );
    let d = DatiGioco::da_testi(&f).unwrap();
    assert!(Partita::nuova(d.clone(), 1).elettorato().gini(&d).abs() < 1e-12);
    let base = dati_base();
    let g = Partita::nuova(base.clone(), 1).elettorato().gini(&base);
    assert!(g > 0.1 && g < 0.6, "{g}");
}

#[test]
fn pressione_estrema_e_base_nulla_lasciano_redditi_finiti_e_non_negativi() {
    let d = dati_base();
    let p = Partita::nuova(d.clone(), 1);
    let mut el = p.elettorato().clone();
    el.base.pressione *= 1e-6;
    el.base.salari_pubblici = 0.0;
    el.base.pensione = 0.0;
    el.aggiorna_redditi(&d, p.economia());
    for g in &el.gruppi {
        assert!(g.reddito.reale().is_finite() && g.reddito.netto() >= 0.0);
    }
}

#[test]
fn quote_di_regione_sommano_a_uno_dopo_40_turni() {
    let mut p = Partita::nuova(dati_base(), 7);
    for _ in 0..40 {
        p.chiudi_turno();
    }
    for r in 0..20 {
        let s: f64 = (0..18)
            .map(|k| p.elettorato().gruppi[r * 18 + k].quota)
            .sum();
        assert!((s - 1.0).abs() < 1e-9, "regione {r}: {s}");
    }
}

fn quota_disoccupati(p: &Partita, r: usize) -> f64 {
    Fascia::TUTTE
        .iter()
        .map(|&f| p.elettorato().gruppi[indice_gruppo(r, Condizione::DisoccupatoInattivo, f)].quota)
        .sum()
}

#[test]
fn la_disoccupazione_sposta_dipendenti_fra_i_disoccupati() {
    let mut p = Partita::nuova(dati_base(), 5);
    let u0: Vec<f64> = p
        .economia()
        .regioni
        .iter()
        .map(|r| r.disoccupazione)
        .collect();
    let q0: Vec<f64> = (0..20).map(|r| quota_disoccupati(&p, r)).collect();
    p.chiudi_turno();
    let mut mossi = 0;
    for r in 0..20 {
        let du = p.economia().regioni[r].disoccupazione - u0[r];
        if du.abs() > 1e-9 {
            mossi += 1;
            let dq = quota_disoccupati(&p, r) - q0[r];
            assert!(dq * du > 0.0, "regione {r}: du {du}, dq {dq}");
        }
    }
    assert!(mossi > 0, "nessuna regione ha cambiato disoccupazione");
}

#[test]
fn i_pensionati_cambiano_solo_a_gennaio() {
    let d = dati_base();
    let mut p = Partita::nuova(dati_base(), 3);
    let pensionati = |p: &Partita| -> f64 {
        (0..360)
            .filter(|i| i % 18 / 3 == Condizione::Pensionato as usize)
            .map(|i| {
                p.elettorato().gruppi[i].quota * d.elettorato().regioni[i / 18].elettori as f64
            })
            .sum()
    };
    let t0 = pensionati(&p);
    p.chiudi_turno();
    assert!((pensionati(&p) - t0).abs() < 1e-6 * t0);
    p.chiudi_turno();
    assert!((pensionati(&p) - t0).abs() > 1e-6 * t0);
}

#[test]
fn elettori_e_gini_seguono_la_quota_viva() {
    let d = dati_base();
    let mut el = Partita::nuova(dati_base(), 2).elettorato().clone();
    let (prima, g0) = (el.elettori(d.elettorato(), 0), el.gini(&d));
    let a = indice_gruppo(0, Condizione::Pensionato, Fascia::Alta);
    el.sposta(0, a, el.gruppi[0].quota / 2.0);
    assert!((el.elettori(d.elettorato(), 0) - prima / 2.0).abs() < 1e-6 * prima);
    assert!(
        (el.gini(&d) - g0).abs() > 1e-12,
        "gini fermo alle quote del turno zero"
    );
}

type Gr = sim_core::elettorato::Gruppo;

/// Elettorato del turno zero con `ritocca` sui gruppi, prima e dopo `composizione`.
fn compone(
    data: Trimestre,
    ritocca: impl Fn(usize, &mut Gr),
) -> (
    sim_core::elettorato::Elettorato,
    sim_core::elettorato::Elettorato,
) {
    let p = Partita::nuova(dati_base(), 4);
    let mut prima = p.elettorato().clone();
    for (i, g) in prima.gruppi.iter_mut().enumerate() {
        ritocca(i, g);
    }
    let mut dopo = prima.clone();
    dopo.composizione(&dati_base(), p.economia(), data);
    (prima, dopo)
}

#[test]
fn mobilita_sposta_verso_la_fascia_bassa_se_la_condizione_cresce_meno() {
    let d = dati_base();
    let aut = Condizione::Autonomo as usize;
    let (prima, dopo) = compone(
        Trimestre {
            anno: 2027,
            trimestre: 2,
        },
        |i, g| {
            if i % 18 / 3 == aut {
                g.reddito_prima.primario *= 1.1;
                g.reddito_prima.trasferimenti *= 1.1;
            }
        },
    );
    let (mut n, mut w_aut, mut g_aut, mut g_tot) = (0.0, 0.0, 0.0, 0.0);
    for (i, g) in prima.gruppi.iter().enumerate() {
        let w = prima.elettori(d.elettorato(), i);
        let gamma = g.reddito.reale() / g.reddito_prima.reale() - 1.0;
        n += w;
        g_tot += w * gamma;
        if i % 18 / 3 == aut {
            w_aut += w;
            g_aut += w * gamma;
        }
    }
    let scarto = g_aut / w_aut - g_tot / n;
    assert!(scarto < 0.0, "scarto {scarto}");
    let f = d.manopola("mobilita_fasce") * scarto.abs();
    for r in 0..20 {
        let [b, m, a] = Fascia::TUTTE.map(|f| indice_gruppo(r, Condizione::Autonomo, f));
        let guadagno_bassa = dopo.gruppi[b].quota - prima.gruppi[b].quota;
        let perdita_alta = prima.gruppi[a].quota - dopo.gruppi[a].quota;
        assert!(
            (guadagno_bassa - f * prima.gruppi[m].quota).abs() < 1e-12,
            "regione {r}"
        );
        assert!(
            (perdita_alta - f * prima.gruppi[a].quota).abs() < 1e-12,
            "regione {r}"
        );
    }
}

#[test]
fn il_pensionamento_svuota_anche_i_disoccupati() {
    // Uscita degli studenti a zero, disoccupazione e redditi fermi: a gennaio si muove solo
    // il pensionamento, `t · quota` da ciascuna delle quattro condizioni.
    let f = imposta(
        file_base(),
        "manopole",
        "[uscita_studenti]\nvalore = ",
        "0.0",
    );
    let d = DatiGioco::da_testi(&f).unwrap();
    let p = Partita::nuova(d.clone(), 4);
    let mut e = p.economia().clone();
    let mut annata = e.pensioni[0].clone();
    (annata.anno, annata.pensionati) = (2027, 1_000_000);
    e.pensioni.push(annata);
    let prima = p.elettorato().clone();
    let mut dopo = prima.clone();
    let data = Trimestre {
        anno: 2027,
        trimestre: 1,
    };
    dopo.composizione(&d, &e, data);
    use Condizione::{Autonomo, DipendentePrivato, DipendentePubblico, DisoccupatoInattivo};
    let quattro = [
        DipendentePrivato,
        DipendentePubblico,
        Autonomo,
        DisoccupatoInattivo,
    ];
    let attivi: f64 = (0..360)
        .filter(|i| quattro.contains(&Condizione::TUTTE[i % 18 / 3]))
        .map(|i| prima.elettori(d.elettorato(), i))
        .sum();
    let t = 1e6 / attivi;
    for r in 0..20 {
        for f in Fascia::TUTTE {
            for c in quattro {
                let i = indice_gruppo(r, c, f);
                let atteso = prima.gruppi[i].quota * (1.0 - t);
                assert!(
                    (dopo.gruppi[i].quota - atteso).abs() < 1e-12,
                    "regione {r} {c} {f}"
                );
            }
        }
    }
}

#[test]
fn senza_shock_le_condizioni_restano_vicine_al_turno_zero() {
    let d = DatiGioco::da_testi(&senza_shock()).unwrap();
    let per_condizione = |p: &Partita| -> [f64; 6] {
        let mut q = [0.0; 6];
        for i in 0..360 {
            q[i % 18 / 3] += p.elettorato().elettori(d.elettorato(), i);
        }
        let n: f64 = q.iter().sum();
        q.map(|x| x / n)
    };
    let mut p = Partita::nuova(d.clone(), 11);
    let t0 = per_condizione(&p);
    for _ in 0..20 {
        p.chiudi_turno();
    }
    let t20 = per_condizione(&p);
    for (c, (a, b)) in Condizione::TUTTE.iter().zip(t0.iter().zip(t20)) {
        assert!((b / a - 1.0).abs() <= 0.10, "{c}: {a} -> {b}");
    }
}

#[test]
fn studenti_escono_verso_lavoro_e_disoccupazione_solo_a_gennaio() {
    let d = dati_base();
    let (studenti, uscite) = (
        d.manopola("uscita_studenti"),
        d.manopola("uscite_pensionati"),
    );
    let p = Partita::nuova(dati_base(), 4);
    assert!(
        p.economia().pensioni.iter().all(|a| a.anno != 2027),
        "precondizione: nessun pensionamento nel 2027 al turno zero"
    );
    let (prima, gennaio) = compone(
        Trimestre {
            anno: 2027,
            trimestre: 1,
        },
        |_, _| {},
    );
    let (_, aprile) = compone(
        Trimestre {
            anno: 2027,
            trimestre: 2,
        },
        |_, _| {},
    );
    assert_eq!(
        aprile.gruppi, prima.gruppi,
        "fuori da gennaio nulla si muove"
    );
    for r in 0..20 {
        let u = p.economia().regioni[r].disoccupazione / 100.0;
        for f in Fascia::TUTTE {
            let q =
                |el: &sim_core::elettorato::Elettorato, c| el.gruppi[indice_gruppo(r, c, f)].quota;
            let massa = studenti
                * (q(&prima, Condizione::Studente) + uscite * q(&prima, Condizione::Pensionato));
            let delta = |c| q(&gennaio, c) - q(&prima, c);
            let occupati = [
                Condizione::DipendentePrivato,
                Condizione::DipendentePubblico,
                Condizione::Autonomo,
            ];
            let zero = |c| d.elettorato().gruppi[indice_gruppo(r, c, f)].quota;
            let tot: f64 = occupati.iter().map(|&c| zero(c)).sum();
            for c in occupati {
                let atteso = (1.0 - u) * massa * zero(c) / tot;
                assert!((delta(c) - atteso).abs() < 1e-12, "regione {r} {f} {c}");
            }
            let d_dis = delta(Condizione::DisoccupatoInattivo);
            assert!(d_dis > 0.0, "regione {r} {f}");
            assert!((d_dis - u * massa).abs() < 1e-12, "regione {r} {f}");
        }
    }
}

#[test]
fn solo_i_partiti_di_governo_hanno_un_giudizio() {
    let mut p = Partita::nuova(dati_base(), 5);
    for _ in 0..8 {
        p.chiudi_turno();
    }
    let gov = p.governo().clone();
    for j in (0..p.partiti().len()).filter(|j| !gov.al_governo(*j)) {
        assert!(p.elettorato().gruppi.iter().all(|g| g.giudizio[j] == 0.0));
    }
    assert!(
        p.elettorato()
            .gruppi
            .iter()
            .any(|g| g.giudizio[gov.presidente] != 0.0)
    );
}

/// Giudizi sul centrodestra dopo 6 turni con shock, con il lavoro affidato al centrodestra.
fn giudizi_del_centrodestra(quota_presidente: &str) -> Vec<f64> {
    let f = sostituisci(
        file_base(),
        "scenario",
        "lavoro_welfare = \"destra\"",
        "lavoro_welfare = \"centrodestra\"",
    );
    let f = imposta(
        f,
        "manopole",
        "[quota_colpa_presidente]\nvalore = ",
        quota_presidente,
    );
    let mut p = Partita::nuova(DatiGioco::da_testi(&f).unwrap(), 3);
    for _ in 0..6 {
        p.chiudi_turno();
    }
    let j = p
        .partiti()
        .iter()
        .position(|x| x.id == "centrodestra")
        .unwrap();
    p.elettorato()
        .gruppi
        .iter()
        .map(|g| g.giudizio[j])
        .collect()
}

#[test]
fn la_colpa_di_trasferimenti_e_disoccupazione_va_al_partito_del_lavoro() {
    assert!(giudizi_del_centrodestra("0.3").iter().any(|x| *x != 0.0));
    assert!(giudizi_del_centrodestra("1.0").iter().all(|x| *x == 0.0));
}

/// Giudizio di un solo turno sul gruppo `i` (turno zero, disoccupazione ferma) da `prima` a `ora`.
fn giudizio_a_mano(
    f: FileDati,
    i: usize,
    prima: Reddito,
    ora: Reddito,
) -> (DatiGioco, Partita, Vec<f64>) {
    let d = DatiGioco::da_testi(&f).unwrap();
    let p = Partita::nuova(d.clone(), 1);
    let mut el = p.elettorato().clone();
    el.gruppi[i].reddito_prima = prima;
    el.gruppi[i].reddito = ora;
    el.giudizio(&d, p.economia(), p.governo());
    let g = el.gruppi[i].giudizio.clone();
    (d, p, g)
}

const CENTO: Reddito = Reddito {
    primario: 100.0,
    trasferimenti: 0.0,
    imposte: 0.0,
    paniere: 1.0,
};

#[test]
fn lavoro_del_dipendente_pubblico_va_a_economia_e_finanze() {
    // EF al centrodestra, presidente destra. Netto 100, primario -2: una sola componente,
    // Δ_y = -2/σ_r, α = 2/(1+ρ); al centrodestra (1-q), alla destra q.
    let f = sostituisci(
        file_base(),
        "scenario",
        "economia_finanze = \"destra\"",
        "economia_finanze = \"centrodestra\"",
    );
    let i = indice_gruppo(0, Condizione::DipendentePubblico, Fascia::Media);
    let ora = Reddito {
        primario: 98.0,
        ..CENTO
    };
    let (d, p, g) = giudizio_a_mano(f, i, CENTO, ora);
    let (rho, q, sigma) = (
        d.manopola("asimmetria_giudizio"),
        d.manopola("quota_colpa_presidente"),
        d.manopola("scala_reddito"),
    );
    let w = d.elettorato().interessi[&Condizione::DipendentePubblico]["reddito"];
    let nuovo = w * (-2.0 / sigma) * 2.0 / (1.0 + rho);
    let j = |id: &str| p.partiti().iter().position(|x| x.id == id).unwrap();
    assert!((g[j("centrodestra")] - (1.0 - q) * nuovo).abs() < 1e-9);
    assert!((g[j("destra")] - q * nuovo).abs() < 1e-9);
    assert!(g[j("destra")] < 0.0);
}

#[test]
fn a_reddito_reale_fermo_il_giudizio_resta_zero() {
    // Primario e paniere +5%: il nominale sale, il reale no.
    let i = indice_gruppo(4, Condizione::DipendentePrivato, Fascia::Bassa);
    let ora = Reddito {
        primario: 105.0,
        paniere: 1.05,
        ..CENTO
    };
    let (_, _, g) = giudizio_a_mano(file_base(), i, CENTO, ora);
    assert!(g.iter().all(|x| x.abs() < 1e-9), "{g:?}");
}

#[test]
fn il_giudizio_distribuito_somma_all_asimmetrica_del_reddito() {
    // Componenti di segno opposto: lavoro +3, prezzi -1, trasferimenti +1, imposte -4 punti.
    // Δ_y = -1/σ_r; la somma sui partiti è w·asimmetrica(Δ_y), non la somma delle asimmetriche.
    let i = indice_gruppo(7, Condizione::Pensionato, Fascia::Media);
    let prima = Reddito {
        primario: 60.0,
        trasferimenti: 50.0,
        imposte: 10.0,
        paniere: 1.0,
    };
    let ora = Reddito {
        primario: 63.0,
        trasferimenti: 51.0,
        imposte: 14.0,
        paniere: 1.01,
    };
    let (d, _, g) = giudizio_a_mano(file_base(), i, prima, ora);
    let (rho, sigma) = (
        d.manopola("asimmetria_giudizio"),
        d.manopola("scala_reddito"),
    );
    let w = d.elettorato().interessi[&Condizione::Pensionato]["reddito"];
    let dy = (3.0 - 1.0 + 1.0 - 4.0) / sigma;
    let atteso = w * dy * 2.0 / (1.0 + rho);
    let somma: f64 = g.iter().sum();
    assert!((somma - atteso).abs() < 1e-9, "{somma} {atteso}");
}

#[test]
fn base_degenere_lascia_giudizio_e_crescite_finiti() {
    let d = dati_base();
    let p = Partita::nuova(dati_base(), 1);
    let mut el = p.elettorato().clone();
    el.gruppi[0].reddito_prima = Reddito::default();
    el.giudizio(&d, p.economia(), p.governo());
    assert_eq!(el.gruppi[0].crescite, vec![0.0]);
    assert!(el.gruppi[0].giudizio.iter().all(|x| x.is_finite()));
}

#[test]
fn le_aspettative_ricordano_al_massimo_la_finestra() {
    let mut p = Partita::nuova(dati_base(), 2);
    assert!(p.elettorato().gruppi.iter().all(|g| g.crescite.is_empty()));
    for _ in 0..20 {
        p.chiudi_turno();
    }
    assert!(p.elettorato().gruppi.iter().all(|g| g.crescite.len() == 8));
}

fn somma_uno(v: &[f64]) -> bool {
    (v.iter().sum::<f64>() - 1.0).abs() < 1e-9
}

#[test]
fn intenzioni_sommano_a_uno() {
    let p = Partita::nuova(dati_base(), 1);
    let i = p.intenzioni_vere();
    assert_eq!(i.per_gruppo.len(), 360);
    assert!(i.per_gruppo.iter().all(|v| v.len() == 8 && somma_uno(v)));
    assert!(somma_uno(&i.nazionali));
}

#[test]
fn al_turno_zero_l_intenzione_e_il_voto_precedente() {
    let p = Partita::nuova(dati_base(), 1);
    let i = p.intenzioni_vere();
    assert_eq!(i.per_gruppo.len(), 360);
    for (g, v) in p.elettorato().gruppi.iter().zip(&i.per_gruppo) {
        for (a, b) in g.voto_precedente.iter().zip(v) {
            assert!((a - b).abs() < 1e-9);
        }
    }
}

fn con_manopola(f: FileDati, nome: &str, valore: &str) -> FileDati {
    imposta(f, "manopole", &format!("[{nome}]\nvalore = "), valore)
}

/// Intenzioni del gruppo `i` dopo aver ritoccato l'elettorato del turno zero.
fn intenzione(f: &FileDati, i: usize, ritocca: impl Fn(&mut Gr)) -> Vec<f64> {
    let d = DatiGioco::da_testi(f).unwrap();
    let p = Partita::nuova(d.clone(), 1);
    let mut el = p.elettorato().clone();
    ritocca(&mut el.gruppi[i]);
    intenzioni(&d, &el, p.partiti(), p.governo()).swap_remove(i)
}

fn uno_in(n: usize, k: usize) -> Vec<f64> {
    (0..n).map(|j| f64::from(u8::from(j == k))).collect()
}

#[test]
fn fedelta_trattiene_il_voto_precedente() {
    let voto = |f: &FileDati, k: usize| intenzione(f, 0, |g| g.voto_precedente = uno_in(8, k))[0];
    let base = file_base();
    assert!(voto(&base, 0) > voto(&base, 1) + 1e-3);
    let senza = con_manopola(file_base(), "fedelta", "0.0");
    assert!((voto(&senza, 0) - voto(&senza, 1)).abs() < 1e-12);
}

#[test]
fn giudizio_negativo_toglie_voti_al_governo() {
    let p = Partita::nuova(dati_base(), 1);
    let pres = p.governo().presidente;
    let fuori = (0..p.partiti().len())
        .find(|j| !p.governo().al_governo(*j))
        .unwrap();
    let f = file_base();
    let zero = intenzione(&f, 0, |_| {});
    let punito = intenzione(&f, 0, |g| g.giudizio[pres] = -5.0);
    assert!(punito[pres] < zero[pres] - 1e-3);
    assert_eq!(intenzione(&f, 0, |g| g.giudizio[fuori] = -5.0), zero);
}

#[test]
fn la_distanza_avvicina_il_voto_al_partito_vicino() {
    let p = Partita::nuova(dati_base(), 1);
    let posizioni: Vec<f64> = p.partiti().iter().map(|x| x.posizione_percepita).collect();
    let estremo = |piu: bool| {
        let pos = posizioni.iter().copied().enumerate();
        let scegli = |a: &(usize, f64), b: &(usize, f64)| a.1.total_cmp(&b.1);
        if piu {
            pos.max_by(scegli)
        } else {
            pos.min_by(scegli)
        }
        .unwrap()
        .0
    };
    let (sinistra, destra) = (estremo(false), estremo(true));
    let a_sinistra = intenzione(&file_base(), 0, |g| {
        g.posizione = [1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        g.voto_precedente = vec![1.0 / 8.0; 8];
    });
    assert!(a_sinistra[sinistra] > a_sinistra[destra]);
    let piatta = con_manopola(file_base(), "beta_distanza", "0.3");
    let poco = intenzione(&piatta, 0, |g| {
        g.posizione = [1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        g.voto_precedente = vec![1.0 / 8.0; 8];
    });
    assert!(
        a_sinistra[sinistra] - a_sinistra[destra] > poco[sinistra] - poco[destra],
        "beta più alto, distanza che pesa di più"
    );
}

#[test]
fn l_astensione_segue_fascia_e_condizione() {
    // Studenti con un termine d'astensione proprio; posizioni e voto precedente uguali.
    let f = sostituisci(
        file_base(),
        "elettorato",
        "studente = 0.0 }",
        "studente = 1.0 }",
    );
    let d = DatiGioco::da_testi(&f).unwrap();
    let p = Partita::nuova(d.clone(), 1);
    let mut el = p.elettorato().clone();
    for g in &mut el.gruppi {
        g.posizione = [0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0];
        g.voto_precedente = vec![1.0 / 8.0; 8];
    }
    let i = intenzioni(&d, &el, p.partiti(), p.governo());
    let ast = |c, fa| i[indice_gruppo(0, c, fa)][7];
    use Condizione::{DipendentePrivato as Priv, Studente};
    assert!(ast(Priv, Fascia::Bassa) > ast(Priv, Fascia::Media));
    assert!(ast(Priv, Fascia::Media) > ast(Priv, Fascia::Alta));
    assert!(ast(Studente, Fascia::Media) > ast(Priv, Fascia::Media));
}

#[test]
fn piu_valence_piu_voti() {
    let nazionali = |valence: &str| {
        let f = imposta(
            file_base(),
            "scenario",
            "id = \"centro\"\nnome = \"Centro\"\nposizione = 4.0\n\n[partito.valence]\nvalore = ",
            valence,
        );
        Partita::nuova(DatiGioco::da_testi(&f).unwrap(), 1)
            .intenzioni_vere()
            .nazionali
    };
    let (prima, dopo) = (nazionali("0.0"), nazionali("1.0"));
    let j = Partita::nuova(dati_base(), 1)
        .partiti()
        .iter()
        .position(|x| x.id == "centro")
        .unwrap();
    assert!(dopo[j] > prima[j]);
    // Gli estremi salgono: con la fedeltà il centro attira chi si asteneva, e l'astensione
    // scende. Scendono i vicini del centro.
    for k in [j - 1, j + 1] {
        assert!(dopo[k] < prima[k], "voce {k}");
    }
    assert!(dopo[7] < prima[7]);
}

#[test]
fn piu_astensione_in_una_regione_meno_affluenza() {
    let affluenza = |p: &Partita, r: usize| {
        let i = p.intenzioni_vere();
        (r * 18..(r + 1) * 18)
            .map(|k| p.elettorato().gruppi[k].quota * (1.0 - i.per_gruppo[k][7]))
            .sum::<f64>()
    };
    let f = imposta(file_base(), "elettorato", "\nastensione = ", "5.0");
    let base = Partita::nuova(dati_base(), 1);
    let alta = Partita::nuova(DatiGioco::da_testi(&f).unwrap(), 1);
    assert!(affluenza(&alta, 0) < affluenza(&base, 0) - 1e-3);
    for r in 1..20 {
        assert!((affluenza(&alta, r) - affluenza(&base, r)).abs() < 1e-12);
    }
}

#[test]
fn affluenza_del_turno_zero_plausibile() {
    let n = Partita::nuova(dati_base(), 1).intenzioni_vere().nazionali;
    let affluenza = 1.0 - n[n.len() - 1];
    assert!((0.5..0.8).contains(&affluenza), "{affluenza}");
}

#[test]
fn il_rumore_delle_posizioni_non_tocca_l_economia() {
    // Oggi è vero per costruzione: l'economia non legge l'elettorato e ha il suo flusso. Il test
    // è una guardia di regressione, scatta se un giorno l'economia legge le posizioni o se il
    // rumore delle posizioni finisce sul flusso dell'economia.
    let gioca = |rumore: &str| {
        let f = imposta(
            file_base(),
            "manopole",
            "[rumore_posizioni]\nvalore = ",
            rumore,
        );
        let mut p = Partita::nuova(DatiGioco::da_testi(&f).unwrap(), 6);
        for _ in 0..5 {
            p.chiudi_turno();
        }
        p
    };
    let (fermo, mosso) = (gioca("0.0"), gioca("0.2"));
    assert_eq!(fermo.economia(), mosso.economia());
    assert_ne!(
        fermo.elettorato().gruppi[0].posizione,
        mosso.elettorato().gruppi[0].posizione
    );
}

#[test]
fn posizioni_estrae_sempre_sette_normali_per_gruppo() {
    let (zero, mosso) = (
        imposta(
            file_base(),
            "manopole",
            "[rumore_posizioni]\nvalore = ",
            "0.0",
        ),
        imposta(
            file_base(),
            "manopole",
            "[rumore_posizioni]\nvalore = ",
            "0.2",
        ),
    );
    let mut sentinella = Caso::nuovo(9);
    for _ in 0..360 * 7 {
        sentinella.normale(Flusso::Elettorato);
    }
    let atteso = sentinella.prossimo(Flusso::Elettorato);
    for f in [zero, mosso] {
        let d = DatiGioco::da_testi(&f).unwrap();
        let mut el = Partita::nuova(d.clone(), 1).elettorato().clone();
        let mut caso = Caso::nuovo(9);
        el.posizioni(&d, &mut caso);
        assert_eq!(caso.prossimo(Flusso::Elettorato), atteso);
    }
}

#[test]
fn la_posizione_torna_verso_l_ancora_senza_rumore() {
    let f = imposta(
        file_base(),
        "manopole",
        "[rumore_posizioni]\nvalore = ",
        "0.0",
    );
    let d = DatiGioco::da_testi(&f).unwrap();
    let mut el = Partita::nuova(d.clone(), 1).elettorato().clone();
    el.gruppi[0].posizione = [1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    el.gruppi[0].ancora = [0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    el.posizioni(&d, &mut Caso::nuovo(1));
    let a = d.manopola("ancora_posizioni");
    assert!((el.gruppi[0].posizione[0] - (1.0 - a)).abs() < 1e-12);
    assert!((el.gruppi[0].posizione[1] - a).abs() < 1e-12);
}

#[test]
fn elettorato_resta_valido_dopo_200_turni() {
    let mut p = Partita::nuova(dati_base(), 11);
    for _ in 0..200 {
        p.chiudi_turno();
    }
    let gruppi = &p.elettorato().gruppi;
    for r in 0..20 {
        let s: f64 = gruppi[r * 18..(r + 1) * 18].iter().map(|g| g.quota).sum();
        assert!((s - 1.0).abs() < 1e-9, "regione {r}: {s}");
    }
    for g in gruppi {
        assert!(g.quota.is_finite() && g.quota >= 0.0);
        assert!(g.posizione.iter().all(|x| x.is_finite() && *x >= 0.0));
        assert!(somma_uno(&g.posizione));
        assert!(g.reddito.reale().is_finite() && g.reddito.reale() > 0.0);
        assert!(g.giudizio.iter().chain(&g.crescite).all(|x| x.is_finite()));
    }
    let i = p.intenzioni_vere();
    assert_eq!(i.per_gruppo.len(), 360);
    assert!(i.per_gruppo.iter().all(|v| somma_uno(v)));
    assert!(somma_uno(&i.nazionali));
    let v = p.vista();
    let s = &v.sondaggio;
    let quote = s.istituti.iter().map(|r| &r.quote);
    let incroci = s.condizioni.iter().chain(&s.fasce).chain(&s.regioni);
    for q in quote.chain(incroci.clone().map(|i| &i.quote)) {
        assert!(somma_uno(q) && q.iter().all(|x| x.is_finite() && *x >= 0.0));
    }
    assert!(incroci.clone().all(|i| somma_uno(&i.posizione)));
    assert!(incroci.into_iter().all(|i| i.approvazione.is_finite()));
    assert!(v.proiezione.is_some());
    let salvato = p.salva().expect("il salvataggio riesce");
    assert!(Partita::carica(&salvato).unwrap() == p);
}
