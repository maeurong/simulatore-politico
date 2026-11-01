//! Scenario su misura: le differenze dallo scenario di base, controllate sui suoi intervalli.

use crate::dati::DatiGioco;
use crate::disposizioni::Norma;
use crate::economia::Economia;
use crate::elettorato::{Elettorato, Fascia, PUNTI, fascia_di, spostamenti_lavoro};
use crate::errore::Errore;
use crate::scenario::Ingresso;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
#[cfg(test)]
use ts_rs::TS;

/// Il nome dello scenario di base.
pub const NOME_BASE: &str = "Italia di oggi";

/// Gli ingressi che i tre cursori assoluti scrivono: le sostituzioni non li nominano (scelta 9).
pub(crate) const NOMI_CURSORI_ASSOLUTI: [&str; 3] = ["debito_pil", "disoccupazione", "inflazione"];

/// Gli ingressi dei tre cursori sintetici, in `[cursori]` di `scenario.toml`: nemmeno questi.
pub(crate) const NOMI_CURSORI_SINTETICI: [&str; 3] = ["ricchezza", "eta_mediana", "orientamento"];

/// I cursori dell'editor. I tre assoluti sono il valore dell'ingresso; i tre sintetici uno scarto
/// dalla base (0 nella base).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct Cursori {
    /// % del PIL.
    pub debito_pil: f64,
    /// % nazionale.
    pub disoccupazione: f64,
    /// % annua.
    pub inflazione: f64,
    /// ± % sul PIL pro capite di base.
    pub ricchezza: f64,
    /// ± anni sull'età mediana di base.
    pub eta_mediana: f64,
    /// ± punti sull'asse 1-7.
    pub orientamento: f64,
    /// Nome di un'impostazione di `elezioni.toml`.
    pub legge_elettorale: String,
}

/// Uno scenario su misura, con tutti i campi pieni: è quello che va nel salvataggio.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct SuMisura {
    pub nome: String,
    pub cursori: Cursori,
    /// Id della disposizione -> id del gradino.
    pub disposizioni: BTreeMap<String, String>,
    /// Nome dell'ingresso come in `Scenario::ingressi` -> valore.
    pub sostituzioni: BTreeMap<String, f64>,
}

/// Il file di differenze: un campo assente è una differenza non scritta.
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct FileSuMisura {
    nome: String,
    #[serde(default, skip_serializing_if = "FileCursori::vuoti")]
    cursori: FileCursori,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    disposizioni: BTreeMap<String, String>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    sostituzioni: BTreeMap<String, f64>,
}

#[derive(Default, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct FileCursori {
    #[serde(skip_serializing_if = "Option::is_none")]
    debito_pil: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disoccupazione: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inflazione: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ricchezza: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    eta_mediana: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    orientamento: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    legge_elettorale: Option<String>,
}

impl FileCursori {
    fn vuoti(&self) -> bool {
        *self == FileCursori::default()
    }
}

/// `Some(x)` se `x` differisce dalla base.
fn diverso<T: PartialEq + Clone>(x: &T, base: &T) -> Option<T> {
    (x != base).then(|| x.clone())
}

fn nell_intervallo(nome: &str, valore: f64, i: &Ingresso) -> Result<(), Errore> {
    if valore.is_finite() && i.min <= valore && valore <= i.max {
        Ok(())
    } else {
        Err(Errore::ScenarioFuoriIntervallo {
            nome: nome.into(),
            valore,
            min: i.min,
            max: i.max,
        })
    }
}

impl SuMisura {
    /// Lo scenario di base, coi valori dei dati: nessuna differenza. Solo dati di base: da dati
    /// già su misura darebbe «Italia di oggi» coi valori derivati.
    pub fn base(dati: &DatiGioco) -> SuMisura {
        debug_assert!(dati.su_misura().is_none(), "solo dati di base");
        let s = dati.scenario();
        let c = &s.cursori;
        SuMisura {
            nome: NOME_BASE.into(),
            cursori: Cursori {
                debito_pil: s.debito_pil.valore,
                disoccupazione: s.disoccupazione.valore,
                inflazione: s.inflazione.valore,
                ricchezza: c.ricchezza.valore,
                eta_mediana: c.eta_mediana.valore,
                orientamento: c.orientamento.valore,
                legge_elettorale: s.legge_elettorale.clone(),
            },
            disposizioni: BTreeMap::new(),
            sostituzioni: BTreeMap::new(),
        }
    }

    /// Legge un file di differenze, riempie dalla base (solo dati di base) ciò che manca e lo
    /// valida come
    /// [`DatiGioco::con_su_misura`].
    ///
    /// # Errors
    /// `SuMisura` per un TOML malformato o un nome sconosciuto, poi gli errori di
    /// [`DatiGioco::con_su_misura`].
    pub fn da_toml(testo: &str, base: &DatiGioco) -> Result<SuMisura, Errore> {
        let f: FileSuMisura = toml::from_str(testo).map_err(|e| Errore::SuMisura(e.to_string()))?;
        let b = SuMisura::base(base).cursori;
        let c = f.cursori;
        let s = SuMisura {
            nome: f.nome,
            cursori: Cursori {
                debito_pil: c.debito_pil.unwrap_or(b.debito_pil),
                disoccupazione: c.disoccupazione.unwrap_or(b.disoccupazione),
                inflazione: c.inflazione.unwrap_or(b.inflazione),
                ricchezza: c.ricchezza.unwrap_or(b.ricchezza),
                eta_mediana: c.eta_mediana.unwrap_or(b.eta_mediana),
                orientamento: c.orientamento.unwrap_or(b.orientamento),
                legge_elettorale: c.legge_elettorale.unwrap_or(b.legge_elettorale),
            },
            disposizioni: f.disposizioni,
            sostituzioni: f.sostituzioni,
        };
        base.con_su_misura(&s)?;
        Ok(s)
    }

    /// Il file di differenze: solo i cursori diversi dalla base (solo dati di base), e `da_toml`
    /// lo rilegge uguale.
    #[expect(
        clippy::expect_used,
        reason = "il file ha solo stringhe, f64 e mappe a chiave stringa, che toml scrive sempre"
    )]
    pub fn in_toml(&self, base: &DatiGioco) -> String {
        let b = SuMisura::base(base).cursori;
        let c = &self.cursori;
        let f = FileSuMisura {
            nome: self.nome.clone(),
            cursori: FileCursori {
                debito_pil: diverso(&c.debito_pil, &b.debito_pil),
                disoccupazione: diverso(&c.disoccupazione, &b.disoccupazione),
                inflazione: diverso(&c.inflazione, &b.inflazione),
                ricchezza: diverso(&c.ricchezza, &b.ricchezza),
                eta_mediana: diverso(&c.eta_mediana, &b.eta_mediana),
                orientamento: diverso(&c.orientamento, &b.orientamento),
                legge_elettorale: diverso(&c.legge_elettorale, &b.legge_elettorale),
            },
            disposizioni: self.disposizioni.clone(),
            sostituzioni: self.sostituzioni.clone(),
        };
        toml::to_string(&f).expect("tipi che toml sa scrivere")
    }

    /// Cursori, legge, disposizioni e nomi delle sostituzioni contro i dati di base; i valori
    /// delle sostituzioni li controlla `DatiGioco::valida` dopo averle applicate.
    fn valida(&self, base: &DatiGioco) -> Result<(), Errore> {
        let s = base.scenario();
        let c = &self.cursori;
        for (nome, valore, i) in [
            ("debito_pil", c.debito_pil, &s.debito_pil),
            ("disoccupazione", c.disoccupazione, &s.disoccupazione),
            ("inflazione", c.inflazione, &s.inflazione),
            ("ricchezza", c.ricchezza, &s.cursori.ricchezza),
            ("eta_mediana", c.eta_mediana, &s.cursori.eta_mediana),
            ("orientamento", c.orientamento, &s.cursori.orientamento),
        ] {
            nell_intervallo(nome, valore, i)?;
        }
        if !base
            .elezioni()
            .impostazioni
            .contains_key(&c.legge_elettorale)
        {
            return Err(Errore::SuMisura(format!(
                "legge elettorale inesistente: {}",
                c.legge_elettorale
            )));
        }
        for (id, gradino) in &self.disposizioni {
            let Some(d) = base
                .disposizione(id)
                .and_then(|i| base.disposizioni().get(i))
            else {
                return Err(Errore::SuMisura(format!("disposizione inesistente: {id}")));
            };
            if d.norma == Norma::Costituzionale {
                return Err(Errore::SuMisura(format!(
                    "{id}: le disposizioni costituzionali non si cambiano nello scenario"
                )));
            }
            if d.gradino(gradino).is_none() {
                return Err(Errore::SuMisura(format!(
                    "{id}: gradino inesistente {gradino}"
                )));
            }
        }
        let ingressi = s.ingressi();
        for nome in self.sostituzioni.keys() {
            if NOMI_CURSORI_ASSOLUTI
                .iter()
                .chain(&NOMI_CURSORI_SINTETICI)
                .any(|n| n == nome)
            {
                return Err(Errore::SuMisura(format!(
                    "{nome}: è un cursore, si cambia in [cursori]"
                )));
            }
            if !ingressi.iter().any(|(n, _)| n == nome) {
                return Err(Errore::SuMisura(format!(
                    "sostituzione sconosciuta: {nome}"
                )));
            }
        }
        Ok(())
    }
}

impl DatiGioco {
    /// Una copia di questi dati con lo scenario su misura applicato, poi validata.
    ///
    /// Applica i tre cursori assoluti coi loro derivati (disoccupazione: spesa NASpI e quote dei
    /// gruppi; inflazione: tasso BCE), ricchezza e orientamento, il nome della legge elettorale
    /// e le sostituzioni, per ultime. L'età e le disposizioni si validano soltanto.
    ///
    /// # Errors
    /// `ScenarioFuoriIntervallo` per un cursore fuori dall'intervallo della base, `SuMisura`
    /// per una legge, una disposizione, un gradino o una sostituzione che la base non ha, poi
    /// gli errori di [`DatiGioco::valida`].
    pub fn con_su_misura(&self, s: &SuMisura) -> Result<DatiGioco, Errore> {
        debug_assert!(self.su_misura().is_none(), "solo dati di base");
        s.valida(self)?;
        let mut d = self.clone();
        let c = &s.cursori;
        let sc = &mut d.scenario;
        let (u0, pi0) = (sc.disoccupazione.valore, sc.inflazione.valore);
        sc.debito_pil.valore = c.debito_pil;
        sc.disoccupazione.valore = c.disoccupazione;
        sc.inflazione.valore = c.inflazione;
        // Sussidio a testa fermo: la NASpI segue i disoccupati.
        sc.spesa_sussidi.valore *= c.disoccupazione / u0;
        // Spostamento dell'obiettivo di Taylor, senza la quota italiana: anche `inflazione_resto`
        // segue il cursore, quindi l'inflazione d'area si sposta di Δπ intero.
        // −0,5 è il pavimento storico della BCE; dal primo turno la Taylor muove comunque il tasso.
        let t = &mut sc.tasso_bce;
        t.valore = (t.valore + (1.0 + self.manopola("taylor_inflazione")) * (c.inflazione - pi0))
            .clamp(t.min, t.max);
        let el = &mut d.elettorato;
        let quote: Vec<f64> = el.gruppi.iter().map(|g| g.quota).collect();
        for r in 0..el.regioni.len() {
            for (da, a, m) in spostamenti_lavoro(&quote, el, r, c.disoccupazione - u0) {
                el.sposta(da, a, m);
            }
        }
        // Sintetici come scarti: `Scenario::valida` (scenario.rs:200) vuole 0 nella base.
        ricchezza(&mut d, 1.0 + c.ricchezza / 100.0);
        // Qui l'età (Task 3).
        orientamento(&mut d, c.orientamento);
        // Poi legge, gradini elettorali e sostituzioni, che vincono.
        let sc = &mut d.scenario;
        sc.legge_elettorale.clone_from(&c.legge_elettorale);
        for (nome, valore) in &s.sostituzioni {
            if let Some((_, i)) = sc.ingressi_mut().into_iter().find(|(n, _)| n == nome) {
                i.valore = *valore;
            }
        }
        d.su_misura = Some(s.clone());
        d.valida()?;
        Ok(d)
    }
}

/// Spese in euro e redditi dei gruppi per `k`; soglie in euro ferme, quindi `irpef_pil` e le
/// aliquote di fascia seguono l'aliquota media IRPEF sui redditi scalati.
fn ricchezza(d: &mut DatiGioco, k: f64) {
    let aliquote = |d: &DatiGioco| {
        let el = Elettorato::iniziale(d, 0, &Economia::iniziale(d));
        let s = &d.scenario().aliquote.irpef;
        let fasce = Fascia::TUTTE.map(|f| el.aliquota_media_irpef_se(d, s, |i| fascia_di(i) == f));
        (el.aliquota_media_irpef(d, s), fasce)
    };
    let (t1, f1) = aliquote(d);
    let sc = &mut d.scenario;
    for i in [
        &mut sc.pil_trimestrale,
        &mut sc.spesa_pensioni,
        &mut sc.spesa_sussidi,
        &mut sc.altre_prestazioni,
    ] {
        i.valore *= k;
    }
    for g in &mut d.elettorato.gruppi {
        g.primario *= k;
        g.trasferimenti *= k;
    }
    let (tk, fk) = aliquote(d);
    debug_assert!(t1 > 0.0, "aliquota media IRPEF nulla");
    d.scenario.irpef_pil.valore *= tk / t1;
    for ((f, a), b) in Fascia::TUTTE.iter().zip(fk).zip(f1) {
        debug_assert!(b > 0.0, "aliquota media IRPEF nulla nella fascia {f}");
        if let Some(x) = d.elettorato.aliquote.get_mut(f) {
            *x *= a / b;
        }
    }
}

/// Ogni istogramma passa `|x|` della massa di ogni punto al vicino nel verso di `x`; al bordo
/// resta dov'è.
fn orientamento(d: &mut DatiGioco, x: f64) {
    for g in &mut d.elettorato.gruppi {
        let p = g.posizione;
        g.posizione = [0.0; PUNTI];
        for (k, m) in p.iter().enumerate() {
            let vicino = if x >= 0.0 {
                (k + 1).min(PUNTI - 1)
            } else {
                k.saturating_sub(1)
            };
            g.posizione[k] += (1.0 - x.abs()) * m;
            g.posizione[vicino] += x.abs() * m;
        }
    }
}
