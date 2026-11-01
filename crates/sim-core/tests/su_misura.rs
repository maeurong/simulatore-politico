#![expect(
    clippy::unwrap_used,
    reason = "nei test un unwrap che fallisce e' il fallimento voluto"
)]

mod comune;
use sim_core::Partita;
use sim_core::disposizioni::Norma;
use sim_core::legislatura::Fondazione;
use sim_core::su_misura::{ETICHETTA_GOVERNO_USCENTE, SuMisura};
use sim_core::vista::VoceDisposizione;

#[test]
fn la_base_non_cambia_i_dati() {
    let base = comune::dati_base();
    let d = base.con_su_misura(&SuMisura::base(&base)).unwrap();
    assert_eq!(d.scenario(), base.scenario());
    assert_eq!(d.elettorato(), base.elettorato());
    assert_eq!(d.struttura(), base.struttura());
    assert_eq!(d.disposizioni(), base.disposizioni());
    assert_eq!(d.nome_scenario(), "Italia di oggi");
    assert_eq!(base.nome_scenario(), "Italia di oggi");
    assert!(base.su_misura().is_none());
    assert_eq!(d.su_misura(), Some(&SuMisura::base(&base)));
}

#[test]
fn andata_e_ritorno_del_file() {
    let base = comune::dati_base();
    let mut s = SuMisura::base(&base);
    s.nome = "Italia indebitata".into();
    s.cursori.debito_pil = 150.0;
    s.sostituzioni.insert("costo_medio_debito".into(), 3.2);
    let testo = s.in_toml(&base);
    assert!(!testo.contains("ricchezza"), "solo differenze: {testo}");
    assert!(!testo.contains("inflazione"), "solo differenze: {testo}");
    assert_eq!(SuMisura::da_toml(&testo, &base).unwrap(), s);
}

#[test]
fn andata_e_ritorno_con_nomi_da_virgolettare() {
    let base = comune::dati_base();
    let mut s = SuMisura::base(&base);
    s.nome = "L'Italia \"di domani\"".into();
    s.cursori.ricchezza = -10.0;
    s.cursori.legge_elettorale = "proporzionale_3".into();
    s.disposizioni.insert("cuneo_fiscale".into(), "0".into());
    s.sostituzioni.insert("valence di destra".into(), 0.5);
    let testo = s.in_toml(&base);
    assert_eq!(SuMisura::da_toml(&testo, &base).unwrap(), s);
}

#[test]
fn file_vuoto_e_la_base_col_suo_nome() {
    let base = comune::dati_base();
    let s = SuMisura::da_toml("nome = \"Altra Italia\"\n", &base).unwrap();
    assert_eq!(s.nome, "Altra Italia");
    assert_eq!(s.cursori, SuMisura::base(&base).cursori);
    assert!(s.disposizioni.is_empty() && s.sostituzioni.is_empty());
}

#[test]
fn sostituzione_e_cursore_assoluto_arrivano_allo_scenario() {
    let base = comune::dati_base();
    let mut s = SuMisura::base(&base);
    s.nome = "Italia indebitata".into();
    s.cursori.debito_pil = 150.0;
    s.sostituzioni.insert("costo_medio_debito".into(), 3.2);
    let d = base.con_su_misura(&s).unwrap();
    assert_eq!(d.scenario().debito_pil.valore, 150.0);
    assert_eq!(d.scenario().costo_medio_debito.valore, 3.2);
    assert_eq!(d.scenario().debito_pil.min, base.scenario().debito_pil.min);
    assert_eq!(d.nome_scenario(), "Italia indebitata");
}

#[test]
fn sostituzione_fuori_intervallo_rifiutata_col_nome() {
    let base = comune::dati_base();
    let mut s = SuMisura::base(&base);
    s.sostituzioni.insert("costo_medio_debito".into(), 99.0);
    let e = base.con_su_misura(&s).unwrap_err().to_string();
    assert!(e.contains("costo_medio_debito"), "{e}");
}

#[test]
fn su_misura_ostile_rifiutato_col_nome() {
    let base = comune::dati_base();
    let casi = [
        ("[cursori]\npil = 1.0", "`pil`"),
        ("[sostituzioni]\ndebito = 1.0", "debito"),
        ("[sostituzioni]\ndebito_pil = 150.0", "debito_pil"),
        ("[sostituzioni]\nricchezza = 0.0", "ricchezza"),
        ("[cursori]\ndebito_pil = 170.0", "debito_pil"),
        ("[cursori]\ndisoccupazione = 4.0", "disoccupazione"),
        ("[cursori]\ninflazione = -1.0", "inflazione"),
        ("[cursori]\nricchezza = 25.0", "ricchezza"),
        ("[cursori]\neta_mediana = -6.0", "eta_mediana"),
        ("[cursori]\norientamento = nan", "orientamento"),
        (
            "[cursori]\nlegge_elettorale = \"maggioritario\"",
            "maggioritario",
        ),
        (
            "[disposizioni]\nnumero_parlamentari = \"400\"",
            "numero_parlamentari",
        ),
        (
            "[disposizioni]\nnumero_parlamentari = \"400_200\"",
            "numero_parlamentari: le disposizioni costituzionali",
        ),
        ("[disposizioni]\ncuneo_fiscale = \"99\"", "cuneo_fiscale"),
        ("[disposizioni]\ninesistente = \"0\"", "inesistente"),
        ("pil = 1.0", "`pil`"),
    ];
    for (corpo, colpevole) in casi {
        let testo = format!("nome = \"Ostile\"\n{corpo}\n");
        let e = SuMisura::da_toml(&testo, &base).unwrap_err().to_string();
        assert!(e.contains(colpevole), "{corpo} -> {e}");
    }
    let costituzionale = &base.disposizioni()[base.disposizione("numero_parlamentari").unwrap()];
    assert!(
        costituzionale.gradino("400_200").is_some(),
        "il caso costituzionale deve usare un gradino vero"
    );
    let e = SuMisura::da_toml("[cursori]\n", &base)
        .unwrap_err()
        .to_string();
    assert!(e.contains("nome"), "{e}");
}

#[test]
fn errore_su_misura_si_stampa_col_prefisso() {
    let e = sim_core::Errore::SuMisura("x".into());
    assert_eq!(e.to_string(), "scenario su misura non valido: x");
}

#[test]
fn dati_gia_su_misura_rifiutati() {
    let base = comune::dati_base();
    let mut s = SuMisura::base(&base);
    s.cursori.debito_pil = 150.0;
    let derivati = base.con_su_misura(&s).unwrap();
    let e = derivati.con_su_misura(&s);
    assert!(e.is_err());
    let atteso = "scenario su misura non valido: dati già su misura";
    assert_eq!(e.unwrap_err().to_string(), atteso);
    let e = SuMisura::da_toml("nome = \"Altra\"\n", &derivati).unwrap_err();
    assert_eq!(e.to_string(), atteso);
}

#[test]
fn la_disoccupazione_sposta_naspi_e_gruppi() {
    use sim_core::Partita;
    use sim_core::elettorato::{Condizione, DatoGruppo, Fascia, indice_gruppo};
    let base = comune::dati_base();
    let mut s = SuMisura::base(&base);
    s.cursori.disoccupazione = 8.0;
    let d = base.con_su_misura(&s).unwrap();
    let attesa = base.scenario().spesa_sussidi.valore * 8.0 / 5.6;
    assert!((d.scenario().spesa_sussidi.valore - attesa).abs() < 1e-9);
    let sussidio = |d| Partita::nuova(d, 1).economia().sussidio_per_disoccupato;
    assert!((sussidio(d.clone()) - sussidio(base.clone())).abs() <= 1);
    let dis = |g: &[DatoGruppo], r| -> f64 {
        Fascia::TUTTE
            .iter()
            .map(|&f| g[indice_gruppo(r, Condizione::DisoccupatoInattivo, f)].quota)
            .sum()
    };
    for r in 0..base.regioni().len() {
        let (prima, dopo) = (
            dis(&base.elettorato().gruppi, r),
            dis(&d.elettorato().gruppi, r),
        );
        assert!(dopo > prima, "regione {r}: {dopo} <= {prima}");
    }
}

#[test]
fn la_disoccupazione_in_calo_sposta_verso_il_lavoro() {
    use sim_core::economia::Economia;
    use sim_core::elettorato::{Condizione, DatoGruppo, Fascia, indice_gruppo};
    let base = comune::dati_base();
    let mut s = SuMisura::base(&base);
    s.cursori.disoccupazione = 4.7;
    let d = base.con_su_misura(&s).unwrap();
    let dis = |g: &[DatoGruppo], r| -> f64 {
        Fascia::TUTTE
            .iter()
            .map(|&f| g[indice_gruppo(r, Condizione::DisoccupatoInattivo, f)].quota)
            .sum()
    };
    let somma = |g: &[DatoGruppo], r: usize| -> f64 {
        g[r * 18..(r + 1) * 18].iter().map(|g| g.quota).sum()
    };
    let (gb, gd) = (&base.elettorato().gruppi, &d.elettorato().gruppi);
    for r in 0..base.regioni().len() {
        assert!(dis(gd, r) < dis(gb, r), "regione {r}");
        assert!((somma(gd, r) - somma(gb, r)).abs() < 1e-9, "regione {r}");
    }
    for e in &Economia::iniziale(&d).regioni {
        assert!(e.disoccupazione >= 0.5, "{}: {}", e.id, e.disoccupazione);
    }
}

#[test]
fn l_inflazione_sposta_il_tasso_bce() {
    let base = comune::dati_base();
    let mut s = SuMisura::base(&base);
    s.cursori.inflazione = 6.0;
    let d = base.con_su_misura(&s).unwrap();
    let attesa =
        base.scenario().tasso_bce.valore + (1.0 + base.manopola("taylor_inflazione")) * 1.8;
    assert!((d.scenario().tasso_bce.valore - attesa).abs() < 1e-9);
}

#[test]
fn gli_estremi_dell_inflazione_sono_giocabili() {
    let base = comune::dati_base();
    let i = &base.scenario().tasso_bce;
    for (inflazione, tasso) in [(0.0, i.min), (8.0, i.max)] {
        let mut s = SuMisura::base(&base);
        s.cursori.inflazione = inflazione;
        let d = base.con_su_misura(&s).unwrap();
        assert_eq!(
            d.scenario().tasso_bce.valore,
            tasso,
            "inflazione {inflazione}"
        );
    }
}

#[test]
fn il_debito_muove_interessi_e_spread() {
    use sim_core::economia::Economia;
    let base = comune::dati_base();
    let mut s = SuMisura::base(&base);
    s.cursori.debito_pil = 150.0;
    let d = base.con_su_misura(&s).unwrap();
    let (e0, e1) = (Economia::iniziale(&base), Economia::iniziale(&d));
    assert!(e1.spread > e0.spread);
    assert!(e1.conti.spese.interessi > e0.conti.spese.interessi);
}

/// Con ricchezza `x`: spese in euro e redditi dei gruppi × (1 + x/100). Torna base e derivati.
fn con_ricchezza(x: f64) -> (sim_core::DatiGioco, sim_core::DatiGioco) {
    let base = comune::dati_base();
    let mut s = SuMisura::base(&base);
    s.cursori.ricchezza = x;
    let d = base.con_su_misura(&s).unwrap();
    let k = 1.0 + x / 100.0;
    let (b, n) = (base.scenario(), d.scenario());
    for (nome, prima, dopo) in [
        ("pil", b.pil_trimestrale.valore, n.pil_trimestrale.valore),
        ("pensioni", b.spesa_pensioni.valore, n.spesa_pensioni.valore),
        ("sussidi", b.spesa_sussidi.valore, n.spesa_sussidi.valore),
        (
            "altre",
            b.altre_prestazioni.valore,
            n.altre_prestazioni.valore,
        ),
    ] {
        assert!(
            (dopo - prima * k).abs() < 1e-9,
            "{nome}: {dopo} != {prima} x {k}"
        );
    }
    for (g0, g1) in base.elettorato().gruppi.iter().zip(&d.elettorato().gruppi) {
        assert!((g1.primario - g0.primario * k).abs() < 1e-9);
        assert!((g1.trasferimenti - g0.trasferimenti * k).abs() < 1e-9);
    }
    (base, d)
}

#[test]
fn la_ricchezza_scala_le_spese_in_euro_e_fa_mordere_gli_scaglioni() {
    use sim_core::elettorato::Fascia;
    let (base, d) = con_ricchezza(10.0);
    let (b, n) = (base.scenario(), d.scenario());
    assert_eq!(n.aliquote, b.aliquote);
    assert_eq!(n.pensione_minima, b.pensione_minima);
    assert_eq!(d.manopole(), base.manopole());
    assert_eq!(d.disposizioni(), base.disposizioni());
    assert!(n.irpef_pil.valore > b.irpef_pil.valore + 0.1);
    // Bassa e media restano nel primo scaglione anche scalate: morde solo l'alta.
    let (a0, a1) = (&base.elettorato().aliquote, &d.elettorato().aliquote);
    for f in [Fascia::Bassa, Fascia::Media] {
        assert!((a1[&f] - a0[&f]).abs() < 1e-12, "fascia {f}");
    }
    assert!(a1[&Fascia::Alta] > a0[&Fascia::Alta] + 0.005);
}

#[test]
fn la_ricchezza_al_minimo_abbassa_l_irpef() {
    let (base, d) = con_ricchezza(-20.0);
    assert!(d.scenario().irpef_pil.valore < base.scenario().irpef_pil.valore);
}

#[test]
fn l_orientamento_sposta_la_media_e_conserva_la_massa() {
    for (x, atteso) in [(0.5, 0.473), (-0.5, -0.473)] {
        orientamento(x, atteso);
    }
}

fn orientamento(x: f64, atteso: f64) {
    let base = comune::dati_base();
    let mut s = SuMisura::base(&base);
    s.cursori.orientamento = x;
    let d = base.con_su_misura(&s).unwrap();
    let media = |d: &sim_core::DatiGioco| -> f64 {
        let el = d.elettorato();
        let (mut somma, mut peso) = (0.0, 0.0);
        for (i, g) in el.gruppi.iter().enumerate() {
            let w = el.regioni[i / 18].elettori as f64 * g.quota;
            for (k, p) in g.posizione.iter().enumerate() {
                somma += w * p * (k + 1) as f64;
            }
            peso += w;
        }
        somma / peso
    };
    for g in &d.elettorato().gruppi {
        assert!((g.posizione.iter().sum::<f64>() - 1.0).abs() < 1e-12, "{x}");
    }
    let spostamento = media(&d) - media(&base);
    assert!((spostamento - atteso).abs() <= 0.005, "{x}: {spostamento}");
}

/// Età mediana interpolata nell'anno: `a + (metà − cumulata prima di a) / c(a)`.
fn mediana(p: &[u32]) -> f64 {
    let meta = p.iter().map(|&x| f64::from(x)).sum::<f64>() / 2.0;
    let mut cumulata = 0.0;
    for (a, &x) in p.iter().enumerate() {
        let x = f64::from(x);
        if cumulata + x >= meta {
            return a as f64 + (meta - cumulata) / x;
        }
        cumulata += x;
    }
    p.len() as f64
}

fn con_eta(eta: f64, ricchezza: f64) -> sim_core::DatiGioco {
    let base = comune::dati_base();
    let mut s = SuMisura::base(&base);
    s.cursori.eta_mediana = eta;
    s.cursori.ricchezza = ricchezza;
    base.con_su_misura(&s).unwrap()
}

#[test]
fn l_eta_centra_la_mediana_e_conserva_il_totale() {
    use sim_core::elettorato::{Condizione, DatoGruppo, Fascia, indice_gruppo};
    let base = comune::dati_base();
    let d = con_eta(2.0, 0.0);
    let (p0, p1) = (&base.struttura().piramide, &d.struttura().piramide);
    let totale = |p: &[u32]| p.iter().map(|&x| i64::from(x)).sum::<i64>();
    assert!((totale(p1) - totale(p0)).abs() <= 1);
    let m = mediana(p1);
    assert!((m - 51.34).abs() <= 0.01, "mediana {m}");
    let oltre_67 = |p: &[u32]| totale(&p[67..]) as f64 / totale(p) as f64;
    assert!(
        (oltre_67(p1) - 0.246).abs() <= 0.002,
        "67+ {}",
        oltre_67(p1)
    );
    let r = oltre_67(p1) / oltre_67(p0);
    let (b, n) = (base.scenario(), d.scenario());
    assert!((n.pensionati.valore / b.pensionati.valore - r).abs() < 1e-9);
    assert!((n.spesa_pensioni.valore / b.spesa_pensioni.valore - r).abs() < 1e-9);
    let pens = |g: &[DatoGruppo], reg| -> f64 {
        Fascia::TUTTE
            .iter()
            .map(|&f| g[indice_gruppo(reg, Condizione::Pensionato, f)].quota)
            .sum()
    };
    let somma = |g: &[DatoGruppo], reg: usize| -> f64 {
        g[reg * 18..(reg + 1) * 18].iter().map(|g| g.quota).sum()
    };
    let (gb, gd) = (&base.elettorato().gruppi, &d.elettorato().gruppi);
    for reg in 0..base.regioni().len() {
        let atteso = pens(gb, reg) * r;
        assert!((pens(gd, reg) - atteso).abs() < 1e-9, "regione {reg}");
        assert!(
            (somma(gd, reg) - somma(gb, reg)).abs() < 1e-9,
            "regione {reg}"
        );
    }
}

#[test]
fn l_eta_piu_giovane_toglie_pensionati_e_conserva_le_quote() {
    use sim_core::elettorato::{Condizione, DatoGruppo, Fascia, indice_gruppo};
    let base = comune::dati_base();
    let d = con_eta(-5.0, 0.0);
    let m = mediana(&d.struttura().piramide);
    assert!((m - 44.34).abs() <= 0.01, "mediana {m}");
    let r = d.scenario().pensionati.valore / base.scenario().pensionati.valore;
    assert!(r < 1.0);
    let pens = |g: &[DatoGruppo], reg| -> f64 {
        Fascia::TUTTE
            .iter()
            .map(|&f| g[indice_gruppo(reg, Condizione::Pensionato, f)].quota)
            .sum()
    };
    let (gb, gd) = (&base.elettorato().gruppi, &d.elettorato().gruppi);
    for reg in 0..base.regioni().len() {
        assert!(
            (pens(gd, reg) - pens(gb, reg) * r).abs() < 1e-9,
            "regione {reg}"
        );
        let somma =
            |g: &[DatoGruppo]| -> f64 { g[reg * 18..(reg + 1) * 18].iter().map(|g| g.quota).sum() };
        assert!((somma(gd) - somma(gb)).abs() < 1e-9, "regione {reg}");
    }
}

#[test]
fn eta_e_ricchezza_commutano() {
    // Commuta solo `spesa_pensioni`: `irpef_pil` dipende dall'ordine, e l'ordine è fisso.
    let base = comune::dati_base().scenario().spesa_pensioni.valore;
    let spesa = |eta, ricchezza| con_eta(eta, ricchezza).scenario().spesa_pensioni.valore;
    let insieme = spesa(2.0, 10.0);
    assert!(insieme > spesa(0.0, 10.0), "l'età deve contare");
    let in_fila = base * (spesa(2.0, 0.0) / base) * (spesa(0.0, 10.0) / base);
    assert!((insieme - in_fila).abs() < 1e-6, "{insieme} != {in_fila}");
}

fn voce(p: &Partita, id: &str) -> VoceDisposizione {
    p.vista()
        .disposizioni
        .into_iter()
        .find(|d| d.id == id)
        .unwrap()
}

#[test]
fn il_proporzionale_dell_editor_ha_la_180_come_precedente() {
    let base = comune::dati_base();
    let mut s = SuMisura::base(&base);
    s.cursori.legge_elettorale = "proporzionale_3".into();
    let d = base.con_su_misura(&s).unwrap();
    let impostazione = |n: &str| base.elezioni().impostazioni[n].clone();
    assert_eq!(
        d.scenario().legge_elettorale_precedente,
        impostazione("legge_180")
    );
    let p = Partita::nuova(d, 1);
    assert_eq!(p.legge_elettorale(), impostazione("proporzionale_3"));
    // Premio: nessuno nel proporzionale, alla coalizione nella 180, nessuno nel Rosatellum.
    let premio = voce(&p, "premio");
    assert_eq!(premio.in_vigore, "nessuno");
    assert_eq!(premio.precedente, "coalizione");
    let voci = p.vista().disposizioni;
    for v in voci.iter().filter(|v| v.norma == Norma::Elettorale) {
        assert_eq!(v.legge, ETICHETTA_GOVERNO_USCENTE, "{}", v.id);
    }
}

#[test]
fn il_gradino_elettorale_dell_editor_e_la_partenza() {
    let base = comune::dati_base();
    let mut s = SuMisura::base(&base);
    s.disposizioni.insert("soglia_lista".into(), "300".into());
    let uguale = base.con_su_misura(&s).unwrap();
    assert_eq!(
        uguale.disposizioni(),
        base.disposizioni(),
        "uguale alla base"
    );
    s.disposizioni.insert("soglia_lista".into(), "500".into());
    let p = Partita::nuova(base.con_su_misura(&s).unwrap(), 1);
    assert_eq!(p.legge_elettorale().soglia_lista_pb, 500);
    let v = voce(&p, "soglia_lista");
    assert_eq!(
        (v.in_vigore.as_str(), v.precedente.as_str()),
        ("500", "300")
    );
    assert_eq!(v.legge, ETICHETTA_GOVERNO_USCENTE);
    // Un'altra voce, non toccata: la partenza è quella della 180, e la 180 è anche il precedente
    // (la base ha invece il Rosatellum come precedente: qui la legge di base fa da precedente).
    let premio = voce(&p, "premio");
    assert_eq!(
        (premio.in_vigore.as_str(), premio.precedente.as_str()),
        ("coalizione", "coalizione")
    );
}

#[test]
fn gradini_elettorali_incoerenti_rifiutati() {
    let base = comune::dati_base();
    let mut s = SuMisura::base(&base);
    // La 180 dà il premio alla coalizione: senza coalizioni non c'è a chi darlo.
    s.disposizioni.insert("coalizioni".into(), "no".into());
    let e = base.con_su_misura(&s).unwrap_err().to_string();
    assert_eq!(
        e,
        "scenario su misura non valido: legge elettorale: premio alla coalizione senza coalizioni"
    );
    s.disposizioni.insert("premio".into(), "nessuno".into());
    assert!(base.con_su_misura(&s).is_ok());
}

#[test]
fn legge_nuova_con_i_gradini_della_base_torna_al_nome_di_base() {
    let base = comune::dati_base();
    let mut s = SuMisura::base(&base);
    s.cursori.legge_elettorale = "proporzionale_3".into();
    for x in (base.disposizioni().iter()).filter(|x| x.norma == Norma::Elettorale) {
        s.disposizioni.insert(x.id.clone(), x.partenza.clone());
    }
    let d = base.con_su_misura(&s).unwrap();
    assert_eq!(
        d.scenario().legge_elettorale,
        base.scenario().legge_elettorale
    );
    assert_eq!(d.disposizioni(), base.disposizioni());
    assert_eq!(
        d.legge_elettorale_iniziale(),
        base.legge_elettorale_iniziale()
    );
}

#[test]
fn la_disposizione_dell_editor_ha_effetto() {
    let base = comune::dati_base();
    let mut s = SuMisura::base(&base);
    s.disposizioni.insert("cuneo_fiscale".into(), "0".into());
    let d = base.con_su_misura(&s).unwrap();
    let f = Fondazione {
        nome: "Partito nuovo".into(),
        posizione: 4.5,
    };
    let p = Partita::fonda(d.clone(), 7, &f).unwrap();
    let v = voce(&p, "cuneo_fiscale");
    assert_eq!((v.in_vigore.as_str(), v.precedente.as_str()), ("0", "7"));
    assert_eq!(v.legge, ETICHETTA_GOVERNO_USCENTE);
    // Gradino «0»: saldo −0,6 contro lo 0 della partenza «7».
    assert!((p.disposizioni().saldo(&d) + 0.6).abs() < 1e-12);
    assert_eq!(p.disposizioni().leve(&d).cuneo, 0.0);
    let nella_base = Partita::fonda(base, 7, &f).unwrap();
    assert_eq!(voce(&nella_base, "cuneo_fiscale").in_vigore, "7");
}

/// `SuMisura` con ogni cursore numerico all'estremo `scelto` del suo intervallo di base.
fn agli_estremi(base: &sim_core::DatiGioco, massimo: bool) -> SuMisura {
    let scelto = |i: &sim_core::scenario::Ingresso| if massimo { i.max } else { i.min };
    let (s, mut m) = (base.scenario(), SuMisura::base(base));
    m.cursori.debito_pil = scelto(&s.debito_pil);
    m.cursori.disoccupazione = scelto(&s.disoccupazione);
    m.cursori.inflazione = scelto(&s.inflazione);
    m.cursori.ricchezza = scelto(&s.cursori.ricchezza);
    m.cursori.eta_mediana = scelto(&s.cursori.eta_mediana);
    m.cursori.orientamento = scelto(&s.cursori.orientamento);
    m
}

#[test]
fn tutti_i_cursori_al_minimo_e_al_massimo_insieme_sono_giocabili() {
    let base = comune::dati_base();
    for massimo in [false, true] {
        let nome = if massimo { "massimi" } else { "minimi" };
        let s = agli_estremi(&base, massimo);
        let mut p = Partita::nuova(base.con_su_misura(&s).unwrap(), 1);
        let somma = |p: &Partita| p.elettorato().gruppi.iter().map(|g| g.quota).sum::<f64>();
        let prima = somma(&p);
        for _ in 0..8 {
            p.chiudi_turno();
        }
        let dopo = somma(&p);
        assert!(
            dopo.is_finite() && (dopo - prima).abs() < 1e-6,
            "{nome}: {prima} -> {dopo}"
        );
    }
}

#[test]
fn andata_e_ritorno_con_ogni_campo_diverso_dalla_base() {
    let base = comune::dati_base();
    let mut s = agli_estremi(&base, false);
    assert_ne!(
        s.cursori,
        SuMisura::base(&base).cursori,
        "il minimo deve differire"
    );
    s.nome = "Tutto cambiato".into();
    s.cursori.legge_elettorale = "proporzionale_3".into();
    s.disposizioni.insert("soglia_lista".into(), "500".into());
    s.sostituzioni.insert("costo_medio_debito".into(), 3.2);
    let testo = s.in_toml(&base);
    assert_eq!(SuMisura::da_toml(&testo, &base).unwrap(), s, "{testo}");
}
