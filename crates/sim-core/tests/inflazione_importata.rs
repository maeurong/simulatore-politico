mod comune;
use comune::{file_base, imposta, senza_shock_vero};
use sim_core::economia::Economia;
use sim_core::misure::Misure;
use sim_core::{DatiGioco, FileDati, Trimestre};

/// L'economia dei dati `f` e la funzione che la porta avanti di un trimestre con le normali `z`.
fn economia(f: FileDati) -> (Economia, impl FnMut(&mut Economia, [f64; 5])) {
    let dati = DatiGioco::da_testi(&f).unwrap();
    let e = Economia::iniziale(&dati);
    let misure = Misure::neutre(&dati);
    let mut data = Trimestre::TURNO_ZERO;
    (e, move |e: &mut Economia, z| {
        e.passo_con(&dati, data, &misure, z);
        data = data.successivo();
    })
}

/// Lo scarto dell'energia dello scenario rientra da solo: i prezzi importati calano, ma
/// l'inflazione pubblicata non scende sotto zero.
#[test]
fn il_rientro_dell_energia_non_porta_deflazione() {
    let (mut e, mut passo) = economia(senza_shock_vero());
    for t in 1..=16 {
        passo(&mut e, [0.0; 5]);
        assert!(
            e.inflazione[0] >= 0.0,
            "trimestre {t}: inflazione {}",
            e.inflazione[0]
        );
    }
}

/// Due economie gemelle, una sola con un calo dell'energia di 2,5 sigma in un trimestre: dal
/// trimestre dopo, quando l'energia già risale, lo scarto dell'inflazione fra le due è sotto metà
/// di quello del trimestre dello shock.
#[test]
fn uno_shock_una_tantum_dell_energia_non_resta_nell_inflazione() {
    let (mut colpita, mut passo_colpita) = economia(file_base());
    let (mut gemella, mut passo_gemella) = economia(file_base());
    for _ in 0..4 {
        passo_colpita(&mut colpita, [0.0; 5]);
        passo_gemella(&mut gemella, [0.0; 5]);
    }
    passo_colpita(&mut colpita, [0.0, 0.0, -2.5, 0.0, 0.0]);
    passo_gemella(&mut gemella, [0.0; 5]);
    let impatto = colpita.inflazione[0] - gemella.inflazione[0];
    assert!(impatto < -1.0, "lo shock non muove l'inflazione: {impatto}");
    for dopo in 1..=4 {
        passo_colpita(&mut colpita, [0.0; 5]);
        passo_gemella(&mut gemella, [0.0; 5]);
        let scarto = colpita.inflazione[0] - gemella.inflazione[0];
        assert!(
            scarto.abs() < impatto.abs() / 2.0,
            "{dopo} trimestri dopo: scarto {scarto}, impatto {impatto}"
        );
    }
}

/// Nel trimestre di uno shock dell'energia e in quello dopo, l'inflazione pubblicata supera quella
/// di fondo della quota dei prezzi importati che non passa nei ritardi, e l'inflazione di fondo
/// scorre di un posto.
#[test]
fn pubblicata_meno_fondo_e_l_importata_fuori_dai_ritardi() {
    let s = DatiGioco::da_testi(&file_base())
        .unwrap()
        .manopola("secondo_giro_importata");
    let (mut e, mut passo) = economia(file_base());
    for _ in 0..4 {
        passo(&mut e, [0.0; 5]);
    }
    for z in [[0.0, 0.0, -2.5, 0.0, 0.0], [0.0; 5]] {
        let fondo_prima = e.inflazione_fondo[0];
        passo(&mut e, z);
        let fuori = (1.0 - s) * e.inflazione_importata;
        assert!(fuori.abs() > 0.01, "prezzi importati fermi: {fuori}");
        let scarto = e.inflazione[0] - e.inflazione_fondo[0];
        assert!(
            (scarto - fuori).abs() < 1e-9,
            "scarto {scarto}, atteso {fuori}"
        );
        assert_eq!(e.inflazione_fondo[1], fondo_prima);
    }
}

/// Con `secondo_giro_importata` a 1 i prezzi importati entrano interi nei ritardi: il percorso
/// senza shock è quello del motore di prima dell'inflazione di fondo (valori di allora, main
/// a02f6ad).
#[test]
fn col_secondo_giro_a_uno_il_percorso_e_quello_di_prima() {
    let f = imposta(
        senza_shock_vero(),
        "manopole",
        "[secondo_giro_importata]\nvalore = ",
        "1.0",
    );
    let (mut e, mut passo) = economia(f);
    let percorso: Vec<f64> = (0..5)
        .map(|_| {
            passo(&mut e, [0.0; 5]);
            e.inflazione[0]
        })
        .collect();
    for (t, attesa) in [(1, 2.397), (5, -1.488)] {
        assert!(
            (percorso[t - 1] - attesa).abs() < 0.001,
            "trimestre {t}: inflazione {}, attesa {attesa}",
            percorso[t - 1]
        );
    }
}
