mod comune;
use comune::{
    chiudi_col_bilancio, chiudi_senza_opposizione_ia, con_arrivo, ferma_tutti, imposta,
    senza_shock, togli_opposizione_ia,
};
use sim_core::Partita;
use sim_core::dati::{DatiGioco, FileDati};
use sim_core::disposizioni::Leve;
use sim_core::elettorato::{Condizione, Fascia, condizione_di, fascia_di};

fn gioca(f: FileDati, turni: usize) -> Partita {
    let mut p = Partita::nuova(DatiGioco::da_testi(&f).unwrap(), 1);
    for _ in 0..turni {
        ferma_tutti(&mut p);
        chiudi_senza_opposizione_ia(&mut p);
    }
    p
}

fn media(
    p: &Partita,
    c: Condizione,
    f: Fascia,
    x: impl Fn(&sim_core::elettorato::Reddito) -> f64,
) -> f64 {
    let g = &p.elettorato().gruppi;
    let v: Vec<f64> = (0..g.len())
        .filter(|&i| condizione_di(i) == c && fascia_di(i) == f)
        .map(|i| x(&g[i].reddito))
        .collect();
    v.iter().sum::<f64>() / v.len() as f64
}

#[test]
fn partenza_non_tocca_il_motore() {
    let d = DatiGioco::da_testi(&senza_shock()).unwrap();
    let mut p = Partita::nuova(d.clone(), 1);
    assert_eq!(p.misure().leve, Leve::partenza(&d));
    ferma_tutti(&mut p);
    chiudi_senza_opposizione_ia(&mut p);
    // Il credito ZES arriva a 1 miliardo all'apertura del turno 1: fino a lì il saldo è zero.
    assert_eq!(p.economia().conti.spese.disposizioni, 0);
    for _ in 0..3 {
        ferma_tutti(&mut p);
        chiudi_senza_opposizione_ia(&mut p);
        assert_eq!(p.misure().leve, Leve::partenza(&d));
    }
}

#[test]
fn eta_piu_alta_riduce_le_annate_nuove() {
    let alta = gioca(
        con_arrivo(senza_shock(), "eta_pensionabile", "70", 2027, 1),
        9,
    );
    let base = gioca(senza_shock(), 9);
    assert!(alta.economia().conti.spese.pensioni < base.economia().conti.spese.pensioni);
}

#[test]
fn regime_tocca_solo_le_annate_nuove() {
    let r = gioca(
        con_arrivo(senza_shock(), "regime_pensioni", "retributivo", 2027, 1),
        5,
    );
    let c = gioca(senza_shock(), 5);
    let annata = |p: &Partita, anno: u16| {
        p.economia()
            .pensioni
            .iter()
            .find(|a| a.anno == anno)
            .unwrap()
            .importo
    };
    assert_eq!(annata(&r, 2026), annata(&c, 2026));
    assert!(annata(&r, 2027) > annata(&c, 2027));
}

#[test]
fn sussidi_universali_costano_di_piu() {
    let u = gioca(
        con_arrivo(
            senza_shock(),
            "sussidi_disoccupazione",
            "universale",
            2027,
            1,
        ),
        2,
    );
    let a = gioca(senza_shock(), 2);
    assert!(u.economia().conti.spese.sussidi > a.economia().conti.spese.sussidi);
}

#[test]
fn salario_minimo_alza_il_reddito_della_fascia_bassa_dei_dipendenti_privati() {
    let s = gioca(con_arrivo(senza_shock(), "salario_minimo", "9", 2027, 1), 2);
    let b = gioca(senza_shock(), 2);
    let primario = |p: &Partita, f| media(p, Condizione::DipendentePrivato, f, |r| r.primario);
    let bassa = primario(&s, Fascia::Bassa) / primario(&b, Fascia::Bassa);
    let alta = primario(&s, Fascia::Alta) / primario(&b, Fascia::Alta);
    assert!(bassa > alta + 0.01, "{bassa} {alta}");
}

#[test]
fn cuneo_azzerato_alza_le_imposte_dei_dipendenti() {
    let z = gioca(con_arrivo(senza_shock(), "cuneo_fiscale", "0", 2027, 1), 2);
    let b = gioca(senza_shock(), 2);
    let imposte = |p: &Partita| {
        media(p, Condizione::DipendentePrivato, Fascia::Bassa, |r| {
            r.imposte
        })
    };
    assert!(imposte(&z) > imposte(&b));
}

#[test]
fn accise_alte_alzano_l_energia_e_portano_gettito() {
    let a = gioca(con_arrivo(senza_shock(), "accise", "alte", 2027, 1), 2);
    let b = gioca(senza_shock(), 2);
    assert!(a.misure().prezzo_energia > b.misure().prezzo_energia);
    assert!(a.economia().conti.spese.disposizioni < b.economia().conti.spese.disposizioni);
}

#[test]
fn salario_minimo_non_sposta_il_gettito_irpef() {
    // Senza mobilità fra fasce: qui conta il canale diretto, non le quote che si spostano.
    let fermi = || {
        imposta(
            senza_shock(),
            "manopole",
            "[mobilita_fasce]\nvalore = ",
            "0.0",
        )
    };
    let s = gioca(con_arrivo(fermi(), "salario_minimo", "11", 2027, 1), 3);
    let b = gioca(fermi(), 3);
    assert_eq!(s.misure().fattore_irpef, b.misure().fattore_irpef);
    assert_eq!(
        s.economia().conti.entrate.irpef,
        b.economia().conti.entrate.irpef
    );
}

#[test]
fn forfettario_assente_alza_le_imposte_degli_autonomi_e_non_quelle_dei_dipendenti() {
    let a = gioca(
        con_arrivo(senza_shock(), "forfettario", "assente", 2027, 1),
        2,
    );
    let b = gioca(senza_shock(), 2);
    let imposte = |p: &Partita, c| media(p, c, Fascia::Media, |r| r.imposte);
    let (au, bu) = (
        imposte(&a, Condizione::Autonomo),
        imposte(&b, Condizione::Autonomo),
    );
    assert!(au > bu, "{au} {bu}");
    let dip = Condizione::DipendentePrivato;
    assert_eq!(imposte(&a, dip), imposte(&b, dip));
}

#[test]
fn reddito_minimo_universale_alza_i_trasferimenti_dei_disoccupati_di_fascia_bassa() {
    let u = gioca(
        con_arrivo(senza_shock(), "reddito_minimo", "universale", 2027, 1),
        2,
    );
    let b = gioca(senza_shock(), 2);
    let trasf = |p: &Partita, f| media(p, Condizione::DisoccupatoInattivo, f, |r| r.trasferimenti);
    // Categoriale 1200 -> universale 2400: scarto di 1200 euro, solo nella fascia bassa.
    let scarto = trasf(&u, Fascia::Bassa) - trasf(&b, Fascia::Bassa);
    assert!((scarto - 1200.0).abs() < 1e-6, "{scarto}");
    assert_eq!(trasf(&u, Fascia::Media), trasf(&b, Fascia::Media));
}

fn annata(p: &Partita, anno: u16) -> f64 {
    let e = p.economia();
    e.pensioni.iter().find(|a| a.anno == anno).unwrap().importo as f64
}

#[test]
fn pensione_minima_alta_tocca_solo_l_annata_nuova() {
    // 3 volte la pensione minima, il massimo ammesso: sopra la sostituzione, la leva morde
    // sull'annata nuova.
    let f = imposta(
        senza_shock(),
        "disposizioni",
        "{ id = \"piu_50\", nome = \"più 50%\", posizione = 2.0, saldo = 0.0, leve = { pensione_minima = ",
        "3.0",
    );
    let m = gioca(con_arrivo(f, "pensione_minima", "piu_50", 2027, 1), 5);
    let b = gioca(senza_shock(), 5);
    assert_eq!(annata(&m, 2026), annata(&b, 2026));
    assert!(annata(&m, 2027) > annata(&b, 2027));
}

#[test]
fn indicizzazione_piena_alza_tutte_le_annate_in_essere() {
    let p = gioca(
        con_arrivo(senza_shock(), "indicizzazione_pensioni", "piena", 2027, 1),
        7,
    );
    let b = gioca(senza_shock(), 7);
    for anno in [2026, 2027] {
        assert!(annata(&p, anno) > annata(&b, anno), "{anno}");
    }
}

#[test]
fn credito_zes_porta_alle_spese_il_saldo_per_il_pil_potenziale() {
    let p = gioca(con_arrivo(senza_shock(), "credito_zes", "1", 2027, 1), 2);
    let saldo = p.misure().saldo_disposizioni;
    assert!((saldo + 0.06).abs() < 1e-12, "{saldo}");
    let e = p.economia();
    let atteso = saldo / 100.0 * e.pil_potenziale_reale * e.livello_prezzi * 1e9;
    let spese = e.conti.spese.disposizioni as f64;
    // Il conto usa il PIL potenziale di inizio turno, e` cresciuto di poco a fine turno.
    assert!(
        spese != 0.0 && (spese / atteso - 1.0).abs() < 0.02,
        "{spese} {atteso}"
    );
}

#[test]
fn effetto_di_livello_resta_finche_la_disposizione_non_cambia() {
    let f = con_arrivo(senza_shock(), "licenziamento", "solo_indennizzo", 2027, 1);
    let mut p = Partita::nuova(DatiGioco::da_testi(&f).unwrap(), 1);
    let mut serie = Vec::new();
    for _ in 0..20 {
        ferma_tutti(&mut p);
        // Col bilancio il giocatore non cade: nessun governo IA a cambiare la disposizione; senza
        // le proposte dell'opposizione IA nemmeno lei.
        chiudi_col_bilancio(&mut p);
        togli_opposizione_ia(&mut p);
        serie.push(p.grandezza("regole_lavoro").unwrap());
    }
    // Arrivo all'apertura del turno 1, ritardo 4: vale dalla chiusura del turno 5.
    assert_eq!(serie[3], 0.0);
    assert!(
        serie[5..].iter().all(|&x| (x - 0.3).abs() < 1e-12),
        "{serie:?}"
    );
}
