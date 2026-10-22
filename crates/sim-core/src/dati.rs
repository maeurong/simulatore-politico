use crate::economia::produzione_riferimento;
use crate::elettorato::DatiElettorato;
use crate::elezioni::{DatiElezioni, LeggeElettorale};
use crate::errore::Errore;
use crate::partiti;
use crate::scenario::Scenario;
use crate::struttura::{FileRegioni, Regione, Struttura};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fmt;

/// Testi dei file di dati, letti da chi fa I/O.
#[derive(Clone, Debug)]
pub struct FileDati {
    pub dicasteri: String,
    pub manopole: String,
    pub economia: String,
    pub regioni: String,
    pub scenario: String,
    pub elezioni: String,
    pub elettorato: String,
}

impl FileDati {
    pub const NOMI: [&str; 7] = [
        "dicasteri.toml",
        "manopole.toml",
        "economia.toml",
        "regioni.toml",
        "scenario.toml",
        "elezioni.toml",
        "elettorato.toml",
    ];

    /// Legge i file per nome con la lettura di chi fa I/O.
    ///
    /// # Errors
    /// Il primo errore della lettura.
    pub fn leggi<E>(mut leggi: impl FnMut(&str) -> Result<String, E>) -> Result<FileDati, E> {
        Ok(FileDati {
            dicasteri: leggi(Self::NOMI[0])?,
            manopole: leggi(Self::NOMI[1])?,
            economia: leggi(Self::NOMI[2])?,
            regioni: leggi(Self::NOMI[3])?,
            scenario: leggi(Self::NOMI[4])?,
            elezioni: leggi(Self::NOMI[5])?,
            elettorato: leggi(Self::NOMI[6])?,
        })
    }
}

/// Manopole che il motore legge: se mancano, il caricamento fallisce.
const MANOPOLE_RICHIESTE: &[&str] = &[
    "is_gap_1",
    "is_gap_2",
    "is_tasso",
    "phillips_1",
    "phillips_2_4",
    "phillips_5_8",
    "phillips_gap",
    "ancoraggio_aspettative",
    "tasso_naturale_reale",
    "crescita_potenziale",
    "sigma_domanda",
    "sigma_produttivita",
    "sigma_energia",
    "persistenza_energia",
    "sigma_domanda_estera",
    "persistenza_domanda_estera",
    "sigma_bce",
    "quota_italia_area_euro",
    "taylor_inerzia",
    "taylor_inflazione",
    "taylor_gap",
    "persistenza_resto_area",
    "okun_nazionale",
    "isteresi_disoccupazione",
    "moltiplicatore_fiscale",
    "elasticita_irpef",
    "elasticita_ires",
    "elasticita_iva",
    "elasticita_contributi",
    "quota_margine",
    "quota_redditi_lavoro",
    "quota_profitti_distribuiti",
    "elasticita_export_prezzi",
    "sostituzione_retributivo",
    "sostituzione_contributivo",
    "quota_base_universale",
    "uscite_pensionati",
    "copertura_pensionamento",
    "sussidio_universale",
    "spread_s0",
    "spread_beta_1",
    "spread_beta_2",
    "spread_minimo",
    "spread_massimo",
    "spread_disavanzo",
    "emivita_premio_sfiducia",
    "premio_termine_bund",
    "deviazione_annua_rilevante",
    "deviazione_cumulata_rilevante",
    "ammenda_pde",
    "ammenda_semestrale",
    "sospensione_coesione",
    "beta_distanza",
    "fedelta",
    "peso_giudizio",
    "quota_colpa_presidente",
    "asimmetria_giudizio",
    "emivita_giudizio",
    "finestra_aspettative",
    "scala_reddito",
    "scala_disoccupazione",
    "ancora_posizioni",
    "rumore_posizioni",
    "mobilita_fasce",
    "uscita_studenti",
    "sigma_bias_comune",
    "sigma_istituto",
    "effetto_disegno",
    "errore_approvazione",
    "errore_stima_pil",
    "errore_stima_disoccupazione",
    "errore_stima_inflazione",
    "errore_stima_disavanzo",
];

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Ruolo {
    ColpaDisoccupazioneTrasferimenti,
    ColpaImposteImpiegoPubblico,
    BilancioECredibilita,
}

impl Ruolo {
    pub const TUTTI: [Ruolo; 3] = [
        Ruolo::ColpaDisoccupazioneTrasferimenti,
        Ruolo::ColpaImposteImpiegoPubblico,
        Ruolo::BilancioECredibilita,
    ];
}

impl fmt::Display for Ruolo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Ruolo::ColpaDisoccupazioneTrasferimenti => "colpa_disoccupazione_trasferimenti",
            Ruolo::ColpaImposteImpiegoPubblico => "colpa_imposte_impiego_pubblico",
            Ruolo::BilancioECredibilita => "bilancio_e_credibilita",
        })
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Dicastero {
    pub id: String,
    pub nome: String,
    pub indicatore_servizio: String,
    pub peso_spartizione: u8,
    pub regione_bersaglio: bool,
    pub ruoli: Vec<Ruolo>,
    /// % del PIL.
    pub spesa_pil: f64,
    /// % del PIL.
    pub salari_pil: f64,
    /// % del PIL.
    pub acquisti_pil: f64,
    /// Quota della spesa per settore; assente = si usa il vettore nazionale spesa_pubblica.
    #[serde(default)]
    pub settori: Option<BTreeMap<String, f64>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Manopola {
    pub valore: f64,
    pub min: f64,
    pub max: f64,
    pub ticket: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DatiGioco {
    dicasteri: Vec<Dicastero>,
    manopole: BTreeMap<String, Manopola>,
    struttura: Struttura,
    regioni: Vec<Regione>,
    scenario: Scenario,
    elezioni: DatiElezioni,
    elettorato: DatiElettorato,
}

#[derive(Deserialize)]
struct FileDicasteri {
    dicastero: Vec<Dicastero>,
}

fn errore_toml(file: &str, e: toml::de::Error) -> Errore {
    Errore::Toml {
        file: file.into(),
        messaggio: e.to_string(),
    }
}

impl DatiGioco {
    pub fn dicasteri(&self) -> &[Dicastero] {
        &self.dicasteri
    }

    pub fn manopole(&self) -> &BTreeMap<String, Manopola> {
        &self.manopole
    }

    pub fn struttura(&self) -> &Struttura {
        &self.struttura
    }

    pub fn regioni(&self) -> &[Regione] {
        &self.regioni
    }

    pub fn scenario(&self) -> &Scenario {
        &self.scenario
    }

    pub fn elezioni(&self) -> &DatiElezioni {
        &self.elezioni
    }

    pub fn elettorato(&self) -> &DatiElettorato {
        &self.elettorato
    }

    /// Impostazione elettorale scelta dallo scenario.
    ///
    /// # Panics
    /// Se l'impostazione non esiste: la presenza e' un invariante dei dati validati.
    #[expect(
        clippy::expect_used,
        reason = "la presenza e' garantita da valida (LeggeElettorale)"
    )]
    pub fn legge_elettorale_iniziale(&self) -> &LeggeElettorale {
        self.elezioni
            .impostazioni
            .get(&self.scenario.legge_elettorale)
            .expect("impostazione validata al caricamento")
    }

    /// Valore di una manopola.
    ///
    /// # Panics
    /// Se la manopola non esiste: la presenza e' un invariante dei dati validati.
    #[expect(
        clippy::expect_used,
        reason = "la presenza e' garantita da valida (ManopolaMancante)"
    )]
    pub fn manopola(&self, nome: &str) -> f64 {
        self.manopole
            .get(nome)
            .expect("manopola validata al caricamento")
            .valore
    }

    /// Parsa e valida i testi dei file di dati.
    ///
    /// # Errors
    /// `Toml` se un file è malformato, poi tutti gli errori di [`DatiGioco::valida`].
    pub fn da_testi(file: &FileDati) -> Result<DatiGioco, Errore> {
        let dicasteri = toml::from_str::<FileDicasteri>(&file.dicasteri)
            .map_err(|e| errore_toml("dicasteri.toml", e))?
            .dicastero;
        let manopole = toml::from_str::<BTreeMap<String, Manopola>>(&file.manopole)
            .map_err(|e| errore_toml("manopole.toml", e))?;

        let struttura = toml::from_str::<Struttura>(&file.economia)
            .map_err(|e| errore_toml("economia.toml", e))?;
        let regioni = toml::from_str::<FileRegioni>(&file.regioni)
            .map_err(|e| errore_toml("regioni.toml", e))?
            .regione;
        let scenario = toml::from_str::<Scenario>(&file.scenario)
            .map_err(|e| errore_toml("scenario.toml", e))?;
        let elezioni = toml::from_str::<DatiElezioni>(&file.elezioni)
            .map_err(|e| errore_toml("elezioni.toml", e))?;
        let elettorato = toml::from_str::<DatiElettorato>(&file.elettorato)
            .map_err(|e| errore_toml("elettorato.toml", e))?;

        let dati = DatiGioco {
            dicasteri,
            manopole,
            struttura,
            regioni,
            scenario,
            elezioni,
            elettorato,
        };
        dati.valida()?;
        Ok(dati)
    }

    /// Coerenza dei dati, anche per quelli letti da un salvataggio.
    ///
    /// # Errors
    /// `DicasteroDoppio`, `IndicatoreDoppio`, `RuoloOrfano`, `RuoloDoppio`; `FuoriIntervallo`
    /// (manopola non finita o fuori intervallo); `ManopolaMancante`; `ScenarioFuoriIntervallo`;
    /// `StrutturaIncoerente` (forma, numeri non finiti o di segno sbagliato, produzione di
    /// riferimento non positiva, piramide); `SettoreNonProduttivo`; `QuoteNonUnitarie`;
    /// `SettoreSconosciuto`; `Elettorato` e `Partiti` (vedi [`DatiElettorato`] e
    /// [`partiti::valida`]); `LeggeElettorale` e `DatiElettorali` (vedi
    /// [`DatiElezioni`]), `LeggeElettorale` se lo scenario sceglie un'impostazione inesistente.
    pub fn valida(&self) -> Result<(), Errore> {
        let dicasteri = &self.dicasteri;
        for (i, d) in dicasteri.iter().enumerate() {
            if dicasteri[..i].iter().any(|p| p.id == d.id) {
                return Err(Errore::DicasteroDoppio { id: d.id.clone() });
            }
            if let Some(p) = dicasteri[..i]
                .iter()
                .find(|p| p.indicatore_servizio == d.indicatore_servizio)
            {
                return Err(Errore::IndicatoreDoppio {
                    indicatore: d.indicatore_servizio.clone(),
                    primo: p.id.clone(),
                    secondo: d.id.clone(),
                });
            }
        }
        for ruolo in Ruolo::TUTTI {
            let mut titolari = dicasteri.iter().filter(|d| d.ruoli.contains(&ruolo));
            match (titolari.next(), titolari.next()) {
                (None, _) => return Err(Errore::RuoloOrfano { ruolo }),
                (Some(a), Some(b)) => {
                    return Err(Errore::RuoloDoppio {
                        ruolo,
                        primo: a.id.clone(),
                        secondo: b.id.clone(),
                    });
                }
                _ => {}
            }
        }
        for (nome, m) in &self.manopole {
            let finite = m.valore.is_finite() && m.min.is_finite() && m.max.is_finite();
            if !(finite && m.min <= m.valore && m.valore <= m.max) {
                return Err(Errore::FuoriIntervallo {
                    nome: nome.clone(),
                    valore: m.valore,
                    min: m.min,
                    max: m.max,
                });
            }
        }
        for nome in MANOPOLE_RICHIESTE {
            if !self.manopole.contains_key(*nome) {
                return Err(Errore::ManopolaMancante {
                    nome: (*nome).into(),
                });
            }
        }
        self.struttura.valida(&self.regioni, &self.dicasteri)?;
        self.elettorato
            .valida(&self.regioni, &self.dicasteri, self.struttura.settori.len())?;
        self.scenario.valida()?;
        partiti::valida(&self.scenario, &self.dicasteri)?;
        self.elezioni.valida(&self.regioni)?;
        if !self
            .elezioni
            .impostazioni
            .contains_key(&self.scenario.legge_elettorale)
        {
            return Err(Errore::LeggeElettorale(format!(
                "lo scenario sceglie un'impostazione inesistente: {}",
                self.scenario.legge_elettorale
            )));
        }
        self.struttura.valida_piramide()?;
        let x0 = produzione_riferimento(self);
        match x0.iter().position(|&x| x.is_nan() || x <= 0.0) {
            Some(i) => Err(Errore::StrutturaIncoerente(format!(
                "il settore {} non ha produzione di riferimento positiva",
                self.struttura.settori[i].id
            ))),
            None => Ok(()),
        }
    }
}
