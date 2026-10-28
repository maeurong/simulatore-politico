use sim_core::dati::{DatiGioco, FileDati};
use sim_core::ministri::StatoAgenda;
use sim_core::parlamento::ValoreProposto;
use sim_core::{Comando, Errore, Partita};

#[allow(
    dead_code,
    reason = "ogni crate di test usa solo una parte degli helper"
)]
pub fn file_base() -> FileDati {
    FileDati {
        dicasteri: include_str!("../../../../dati/base/dicasteri.toml").into(),
        manopole: include_str!("../../../../dati/base/manopole.toml").into(),
        economia: include_str!("../../../../dati/base/economia.toml").into(),
        regioni: include_str!("../../../../dati/base/regioni.toml").into(),
        scenario: include_str!("../../../../dati/base/scenario.toml").into(),
        elezioni: include_str!("../../../../dati/base/elezioni.toml").into(),
        elettorato: include_str!("../../../../dati/base/elettorato.toml").into(),
        grandezze: include_str!("../../../../dati/base/grandezze.toml").into(),
        atti: include_str!("../../../../dati/base/atti.toml").into(),
        disposizioni: include_str!("../../../../dati/base/disposizioni.toml").into(),
    }
}

#[allow(
    dead_code,
    reason = "ogni crate di test usa solo una parte degli helper"
)]
pub fn dati_base() -> DatiGioco {
    DatiGioco::da_testi(&file_base()).expect("i dati di base devono essere validi")
}

/// Come `file_base()`, con `valore = 0.0` in ogni sezione `sigma_*` delle manopole.
#[allow(
    dead_code,
    reason = "ogni crate di test usa solo una parte degli helper"
)]
pub fn senza_shock() -> FileDati {
    let mut f = file_base();
    let mut dentro = false;
    let mut azzerate = 0;
    f.manopole = f
        .manopole
        .lines()
        .map(|riga| {
            if riga.starts_with('[') {
                dentro = riga.starts_with("[sigma_");
            }
            if dentro && riga.starts_with("valore = ") {
                azzerate += 1;
                "valore = 0.0".to_string()
            } else {
                riga.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
        + "\n";
    assert!(azzerate > 0, "nessuna manopola sigma_ azzerata");
    f
}

/// Sostituisce la riga `valore = ...` subito sotto `[nome]` nello scenario.
#[allow(
    dead_code,
    reason = "ogni crate di test usa solo una parte degli helper"
)]
pub fn con_ingresso(mut f: FileDati, nome: &str, valore: f64) -> FileDati {
    let testa = format!("[{nome}]\n");
    let inizio = f
        .scenario
        .find(&testa)
        .unwrap_or_else(|| panic!("sezione {nome} assente nello scenario"))
        + testa.len();
    let fine = inizio + f.scenario[inizio..].find('\n').expect("riga valore");
    f.scenario
        .replace_range(inizio..fine, &format!("valore = {valore:?}"));
    f
}

/// Nella sezione `[[partito]]` di id `id` dello scenario, la riga `valore = ...` sotto
/// `[partito.valence]` diventa `v`.
#[allow(
    dead_code,
    reason = "ogni crate di test usa solo una parte degli helper"
)]
pub fn sostituisci_valence(mut f: FileDati, id: &str, v: f64) -> FileDati {
    let testa = format!("id = \"{id}\"\n");
    let partito = f
        .scenario
        .find(&testa)
        .unwrap_or_else(|| panic!("partito {id} assente nello scenario"));
    // La sezione del partito finisce al `[[partito]]` successivo.
    let sezione = &f.scenario[partito..];
    let sezione = &sezione[..sezione.find("[[partito]]").unwrap_or(sezione.len())];
    let valence = partito
        + sezione
            .find("[partito.valence]\nvalore = ")
            .unwrap_or_else(|| panic!("{id}: [partito.valence] assente nella sua sezione"))
        + "[partito.valence]\n".len();
    let fine = valence + f.scenario[valence..].find('\n').expect("riga valore");
    f.scenario
        .replace_range(valence..fine, &format!("valore = {v:?}"));
    f
}

/// `replacen(da, a, 1)` nel file di nome `file`; panico se `da` non c'e'.
#[allow(
    dead_code,
    reason = "ogni crate di test usa solo una parte degli helper"
)]
pub fn sostituisci(mut f: FileDati, file: &str, da: &str, a: &str) -> FileDati {
    let testo = testo_di(&mut f, file);
    assert!(testo.contains(da), "{da:?} assente in {file}");
    *testo = testo.replacen(da, a, 1);
    f
}

fn testo_di<'a>(f: &'a mut FileDati, file: &str) -> &'a mut String {
    match file {
        "dicasteri" => &mut f.dicasteri,
        "manopole" => &mut f.manopole,
        "economia" => &mut f.economia,
        "regioni" => &mut f.regioni,
        "scenario" => &mut f.scenario,
        "elezioni" => &mut f.elezioni,
        "elettorato" => &mut f.elettorato,
        "grandezze" => &mut f.grandezze,
        "atti" => &mut f.atti,
        "disposizioni" => &mut f.disposizioni,
        _ => panic!("file di dati sconosciuto: {file}"),
    }
}

/// Nella disposizione `id`, la riga `in_arrivo = ...` diventa un solo arrivo `gradino` dal
/// trimestre dato: cambia una disposizione a un turno noto senza passare dall'aula.
#[allow(
    dead_code,
    reason = "ogni crate di test usa solo una parte degli helper"
)]
pub fn con_arrivo(mut f: FileDati, id: &str, gradino: &str, anno: u16, trimestre: u8) -> FileDati {
    let testa = format!("id = \"{id}\"\n");
    let inizio = f
        .disposizioni
        .find(&testa)
        .unwrap_or_else(|| panic!("{id} assente"))
        + testa.len();
    let riga = inizio
        + f.disposizioni[inizio..]
            .find("in_arrivo = ")
            .unwrap_or_else(|| panic!("{id}: riga in_arrivo assente"));
    let fine = riga
        + f.disposizioni[riga..]
            .find('\n')
            .unwrap_or_else(|| panic!("{id}: fine della riga in_arrivo assente"));
    f.disposizioni.replace_range(
        riga..fine,
        &format!(
            "in_arrivo = [{{ gradino = \"{gradino}\", anno = {anno}, trimestre = {trimestre} }}]"
        ),
    );
    f
}

/// Nel file `file`, il primo valore dopo `prefisso` (fino a `,` `]` `}` spazio o fine riga)
/// diventa `nuovo`. Non dipende dai numeri dei dati, solo dalla forma del file.
#[allow(
    dead_code,
    reason = "ogni crate di test usa solo una parte degli helper"
)]
pub fn imposta(mut f: FileDati, file: &str, prefisso: &str, nuovo: &str) -> FileDati {
    let testo = testo_di(&mut f, file);
    let inizio = testo
        .find(prefisso)
        .unwrap_or_else(|| panic!("{prefisso:?} assente in {file}"))
        + prefisso.len();
    let fine = inizio
        + testo[inizio..]
            .find([',', ']', '}', ' ', '\n'])
            .expect("fine del valore");
    testo.replace_range(inizio..fine, nuovo);
    f
}

/// Ogni riga di `file` che inizia con `prefisso` diventa `nuova`.
#[allow(
    dead_code,
    reason = "ogni crate di test usa solo una parte degli helper"
)]
pub fn riscrivi(mut f: FileDati, file: &str, prefisso: &str, nuova: &str) -> FileDati {
    let testo = testo_di(&mut f, file);
    *testo = testo
        .lines()
        .map(|r| if r.starts_with(prefisso) { nuova } else { r })
        .collect::<Vec<_>>()
        .join("\n")
        + "\n";
    f
}

/// Ferma le agende ancora annunciate, tranne quella di `tranne` e di chi ha le dimissioni chieste
/// (non si esegue comunque), così due partite divergono solo per ciò che il test fa. Senza il
/// governo del giocatore (trattativa, governo IA dopo il voto alla scadenza) nessun comando è
/// ammesso: le agende si fermano nel salvataggio, senza il costo di `Ferma`.
#[allow(
    dead_code,
    reason = "ogni crate di test usa solo una parte degli helper"
)]
pub fn ferma_tranne(p: &mut Partita, tranne: &str) {
    if p.finita() {
        return;
    }
    if !p.governa() {
        let ids: Vec<String> = p.vista().dicasteri.into_iter().map(|d| d.id).collect();
        *p = edita(p, |j| {
            let dicasteri =
                (j["ministri"]["dicasteri"].as_array_mut()).expect("ministri.dicasteri è un array");
            for (st, id) in dicasteri.iter_mut().zip(&ids) {
                if id != tranne
                    && st["agenda"]["stato"] == "annunciata"
                    && st["dimissioni_chieste"] == false
                {
                    st["agenda"]["stato"] = "fermata".into();
                }
            }
        })
        .unwrap();
        return;
    }
    for m in p.vista().ministeri {
        let annunciata = m
            .agenda
            .as_ref()
            .is_some_and(|a| a.stato == StatoAgenda::Annunciata);
        if annunciata && !m.dimissioni_chieste && m.dicastero != tranne {
            p.comanda(Comando::Ferma {
                dicastero: m.dicastero,
            })
            .unwrap();
        }
    }
}

#[allow(
    dead_code,
    reason = "ogni crate di test usa solo una parte degli helper"
)]
pub fn ferma_tutti(p: &mut Partita) {
    ferma_tranne(p, "");
}

/// Salva, modifica il JSON con `f`, ricarica.
#[allow(
    dead_code,
    reason = "ogni crate di test usa solo una parte degli helper"
)]
pub fn edita(p: &Partita, f: impl FnOnce(&mut serde_json::Value)) -> Result<Partita, Errore> {
    let mut json: serde_json::Value = serde_json::from_str(&p.salva().unwrap()).unwrap();
    f(&mut json);
    Partita::carica(&json.to_string())
}

/// Indice del dicastero `id` nei dati di base.
#[allow(
    dead_code,
    reason = "ogni crate di test usa solo una parte degli helper"
)]
pub fn indice(id: &str) -> usize {
    (dati_base().dicasteri().iter())
        .position(|d| d.id == id)
        .unwrap()
}

#[allow(
    dead_code,
    reason = "ogni crate di test usa solo una parte degli helper"
)]
pub fn presenta(p: &mut Partita, valori: &[(&str, &str)]) -> Result<(), Errore> {
    let valori = valori
        .iter()
        .map(|(d, g)| ValoreProposto {
            disposizione: (*d).into(),
            gradino: (*g).into(),
        })
        .collect();
    p.comanda(Comando::PresentaLegge { valori })
}

/// `f` con i quattro atti della Sanità a 9 quote, come in tests/proposte.rs: nessuno è pagabile,
/// il ministro chiede fondi.
#[allow(
    dead_code,
    reason = "ogni crate di test usa solo una parte degli helper"
)]
pub fn sanita_cara(mut f: FileDati) -> FileDati {
    let effetto = "effetti = [\n  { grandezza = \"qualita_servizio_sanitario\", intensita = ";
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
        f = sostituisci(f, "atti", &da, &da.replacen(costo, "costo = 9.0\n", 1));
    }
    f
}

/// La partita vera col seme 7, fondata alla posizione `posizione`.
#[allow(
    dead_code,
    reason = "ogni crate di test usa solo una parte degli helper"
)]
pub fn fondata(posizione: f64) -> Partita {
    let f = sim_core::legislatura::Fondazione {
        nome: "Partito nuovo".into(),
        posizione,
    };
    Partita::fonda(dati_base(), 7, &f).unwrap()
}

/// Seggi scritti a mano nell'ordine dei partiti; le somme devono restare i `totali`.
#[allow(
    dead_code,
    reason = "ogni crate di test usa solo una parte degli helper"
)]
pub fn con_seggi(p: &Partita, camera: &[u32], senato: &[u32]) -> Partita {
    edita(p, |j| {
        let somma = |v: &[u32]| v.iter().sum::<u32>();
        let totali = serde_json::json!([somma(camera), somma(senato)]);
        assert_eq!(j["legislatura"]["totali"], totali);
        j["parlamento"]["seggi_camera"] = serde_json::json!(camera);
        j["parlamento"]["seggi_senato"] = serde_json::json!(senato);
    })
    .unwrap()
}

/// La partita vera (posizione 4,5) dopo il voto, in trattativa con l'incarico al giocatore,
/// con `coalizioni` (JSON di indici) e seggi scritti a mano.
#[allow(
    dead_code,
    reason = "ogni crate di test usa solo una parte degli helper"
)]
pub fn in_trattativa(coalizioni: serde_json::Value, camera: &[u32], senato: &[u32]) -> Partita {
    let mut p = fondata(4.5);
    p.chiudi_turno();
    let p = edita(&p, |j| {
        j["legislatura"]["incarico"] = serde_json::json!("Giocatore");
        j["legislatura"]["coalizioni"] = coalizioni;
    })
    .unwrap();
    con_seggi(&p, camera, senato)
}

/// Indice del partito `id` nello scenario: lo stesso nella partita di prova e in quella vera.
#[allow(
    dead_code,
    reason = "ogni crate di test usa solo una parte degli helper"
)]
pub fn partito(id: &str) -> usize {
    (dati_base().scenario().partiti.iter())
        .position(|p| p.id == id)
        .unwrap()
}

/// Alla posizione 4,5 con centrodestra e destra la coalizione prende il premio col seme 7 (sonda del
/// Task 9: al 6,5 nessun partito è disponibile, Task 7).
#[allow(
    dead_code,
    reason = "ogni crate di test usa solo una parte degli helper"
)]
pub fn vinta() -> Partita {
    let mut p = fondata(4.5);
    let alleati: Vec<String> = (p.disponibilita().into_iter())
        .filter(|v| {
            v.disponibile
                && ["destra", "centrodestra", "estrema_destra"].contains(&v.partito.as_str())
        })
        .map(|v| v.partito)
        .collect();
    p.comanda(Comando::DichiaraCoalizione {
        partiti: alleati,
        cessioni: vec![],
    })
    .unwrap();
    p.chiudi_turno();
    assert_eq!(
        p.legislatura().incarico,
        Some(sim_core::legislatura::Incarico::Giocatore),
        "{:?}",
        p.vista().elezioni[0]
    );
    p
}

/// `vinta()` con la trattativa chiusa: il giocatore governa.
#[allow(
    dead_code,
    reason = "ogni crate di test usa solo una parte degli helper"
)]
pub fn al_governo() -> Partita {
    let mut p = vinta();
    p.chiudi_turno();
    p
}

/// Quanto la valence di `j` in `p` è scesa oltre il ritorno di un turno, dai `prima` (i partiti
/// prima della chiusura).
#[allow(
    dead_code,
    reason = "ogni crate di test usa solo una parte degli helper"
)]
pub fn calo_oltre_il_ritorno(prima: &[sim_core::partiti::Partito], p: &Partita, j: usize) -> f64 {
    let r = dati_base().manopola("ritorno_valence");
    let x = &prima[j];
    x.valence + r * (x.valence_base - x.valence) - p.partiti()[j].valence
}
