mod comune;
use comune::{
    chiudi_col_bilancio, con_arrivo, dati_base, ferma_tutti, imposta, senza_shock,
    togli_opposizione_ia,
};
use sim_core::Partita;
use sim_core::dati::{DatiGioco, FileDati};

/// 6 condizioni × 3 fasce: i gruppi di una regione sono contigui.
const PER_REGIONE: usize = 18;

fn coppia(f: FileDati, g: FileDati, turni: usize, mut ogni: impl FnMut(&Partita, &Partita)) {
    let (mut a, mut b) = (
        Partita::nuova(DatiGioco::da_testi(&f).unwrap(), 3),
        Partita::nuova(DatiGioco::da_testi(&g).unwrap(), 3),
    );
    for _ in 0..turni {
        ferma_tutti(&mut a);
        ferma_tutti(&mut b);
        // Col bilancio il giocatore non cade: nessun governo IA a muovere le disposizioni; senza
        // le proposte dell'opposizione IA nemmeno lei.
        chiudi_col_bilancio(&mut a);
        chiudi_col_bilancio(&mut b);
        togli_opposizione_ia(&mut a);
        togli_opposizione_ia(&mut b);
        ogni(&a, &b);
    }
}

fn u(p: &Partita, id: &str) -> f64 {
    p.economia()
        .regioni
        .iter()
        .find(|r| r.id == id)
        .unwrap()
        .disoccupazione
}

fn r(id: &str) -> usize {
    dati_base()
        .regioni()
        .iter()
        .position(|x| x.id == id)
        .unwrap()
}

#[test]
fn decontribuzione_abbassa_la_disoccupazione_nel_mezzogiorno() {
    let (mut calabria, mut lombardia) = (0.0, 0.0);
    coppia(
        con_arrivo(senza_shock(), "decontribuzione", "30", 2027, 1),
        senza_shock(),
        8,
        |a, b| {
            calabria += u(a, "calabria") - u(b, "calabria");
            lombardia += u(a, "lombardia") - u(b, "lombardia");
        },
    );
    assert!(calabria < 0.0);
    assert!(calabria.abs() > 10.0 * lombardia.abs());
}

#[test]
fn merito_della_decontribuzione_al_partito_del_dicastero() {
    let f = imposta(
        senza_shock(),
        "scenario",
        "lavoro_welfare = ",
        "\"centrodestra\"",
    );
    let f = imposta(f, "manopole", "[quota_colpa_presidente]\nvalore = ", "0.0");
    let mut fine = (0.0, 0.0);
    coppia(
        con_arrivo(f.clone(), "decontribuzione", "30", 2027, 1),
        f,
        6,
        |a, b| {
            let j = a
                .partiti()
                .iter()
                .position(|x| x.id == "centrodestra")
                .unwrap();
            let media = |p: &Partita, reg: usize| {
                let g = &p.elettorato().gruppi[reg * PER_REGIONE..(reg + 1) * PER_REGIONE];
                g.iter().map(|x| x.giudizio[j]).sum::<f64>() / g.len() as f64
            };
            fine = (
                media(a, r("calabria")) - media(b, r("calabria")),
                media(a, r("lombardia")) - media(b, r("lombardia")),
            );
        },
    );
    assert!(fine.0 > 0.0 && fine.0 > 10.0 * fine.1.abs(), "{fine:?}");
}

#[test]
fn zes_pesa_meta_dove_il_perimetro_e_ridotto() {
    let mut ultimo = (0.0, 0.0, 0.0);
    let k = dati_base().grandezza("investimenti_privati").unwrap();
    coppia(
        senza_shock(),
        con_arrivo(senza_shock(), "credito_zes", "0", 2027, 1),
        30,
        |a, b| {
            let d = |reg: &str| a.grandezze().valori[k][r(reg)] - b.grandezze().valori[k][r(reg)];
            ultimo = (d("calabria"), d("abruzzo"), d("lombardia"));
        },
    );
    assert!(ultimo.0 > 0.0);
    assert!(
        (ultimo.1 - 0.5 * ultimo.0).abs() < 1e-9 && ultimo.2 == 0.0,
        "{ultimo:?}"
    );
}

#[test]
fn autonomia_sposta_domanda_verso_il_perimetro() {
    let f = con_arrivo(
        senza_shock(),
        "autonomia_differenziata",
        "intese_complete",
        2027,
        1,
    );
    let mut p = Partita::nuova(DatiGioco::da_testi(&f).unwrap(), 1);
    // L'arrivo è all'apertura del turno 1: lo leggono le misure della chiusura del turno 1.
    for _ in 0..2 {
        ferma_tutti(&mut p);
        p.chiudi_turno();
    }
    let m = &p.misure().domanda_regionale;
    assert!(m[r("lombardia")] > 0.0 && m[r("sicilia")] < 0.0);
    let el = dati_base();
    let pesata: f64 = el
        .elettorato()
        .regioni
        .iter()
        .zip(m)
        .map(|(x, d)| x.elettori as f64 * d)
        .sum();
    assert!(
        pesata.abs()
            < 1e-6
                * el.elettorato()
                    .regioni
                    .iter()
                    .map(|x| x.elettori as f64)
                    .sum::<f64>()
    );
}

#[test]
fn perimetro_nella_vista() {
    let v = Partita::nuova(dati_base(), 1).vista();
    let a = v
        .disposizioni
        .iter()
        .find(|d| d.id == "autonomia_differenziata")
        .unwrap();
    assert_eq!(a.perimetro, ["liguria", "lombardia", "piemonte", "veneto"]);
}

#[test]
fn autonomia_non_si_muove_col_salario_minimo() {
    // Senza mobilità fra fasce il salario minimo non sposta le quote dei gruppi: resta solo il
    // canale del reddito, che non deve arrivare all'autonomia.
    let f = imposta(
        senza_shock(),
        "manopole",
        "[mobilita_fasce]\nvalore = ",
        "0.0",
    );
    let f = con_arrivo(f, "autonomia_differenziata", "intese_complete", 2027, 1);
    let domanda = |f: &FileDati| {
        let mut p = Partita::nuova(DatiGioco::da_testi(f).unwrap(), 1);
        for _ in 0..4 {
            ferma_tutti(&mut p);
            p.chiudi_turno();
        }
        let m = p.misure().domanda_regionale.clone();
        ["liguria", "lombardia", "piemonte", "veneto"].map(|x| m[r(x)])
    };
    let con_salario = con_arrivo(f.clone(), "salario_minimo", "11", 2027, 1);
    assert_eq!(domanda(&f), domanda(&con_salario));
}
