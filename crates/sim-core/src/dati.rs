use crate::atti::Atto;
use crate::disposizioni::{self, DatoDisposizione, Norma};
use crate::economia::produzione_riferimento;
use crate::elettorato::DatiElettorato;
use crate::elezioni::{DatiElezioni, LeggeElettorale};
use crate::errore::Errore;
use crate::grandezze::{DatoGrandezza, Dinamica};
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
    pub grandezze: String,
    pub atti: String,
    pub disposizioni: String,
}

impl FileDati {
    pub const NOMI: [&str; 10] = [
        "dicasteri.toml",
        "manopole.toml",
        "economia.toml",
        "regioni.toml",
        "scenario.toml",
        "elezioni.toml",
        "elettorato.toml",
        "grandezze.toml",
        "atti.toml",
        "disposizioni.toml",
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
            grandezze: leggi(Self::NOMI[7])?,
            atti: leggi(Self::NOMI[8])?,
            disposizioni: leggi(Self::NOMI[9])?,
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
    "scala_servizio",
    "elasticita_servizio",
    "velocita_servizio",
    "velocita_turnover",
    "soddisfazione_base",
    "ritorno_soddisfazione",
    "soglia_dimissioni",
    "soglia_minaccia",
    "costo_calo_servizio",
    "premio_obiettivo",
    "passo_obiettivo",
    "riduzione_rodaggio",
    "scarto_ministri",
    "peso_obiettivo",
    "peso_posizione",
    "rumore_agenda",
    "costo_fermare",
    "peso_bisogno_bersaglio",
    "peso_voti_bersaglio",
    "costo_dirottare_regione",
    "costo_respingere",
    "distanza_evidenza",
    "elasticita_riscossione",
    "elasticita_imponibile_irpef",
    "elasticita_imponibile_iva",
    "elasticita_imponibile_ires",
    "elasticita_imponibile_contributi",
    "spread_capacita",
    "stock_disoccupazione",
    "crescita_infrastrutture",
    "crescita_capitale_umano",
    "crescita_giustizia",
    "crescita_energia",
    "crescita_demografia",
    "costo_taglio_budget",
    "fondo_riserva",
    "ore_salario_minimo",
    "copertura_salario_minimo",
    "elasticita_autonomia",
    "intesa_base",
    "ritorno_intesa",
    "soglia_ultimatum",
    "costo_firma",
    "turni_blocco_respinta",
    "peso_intesa_linea",
    "malus_opposizione",
    "ft_base",
    "ft_distanza",
    "ft_intesa",
    "ft_max",
    "costo_legge_respinta",
    "valence_legge_respinta",
    "costo_decreto_decaduto",
    "valence_decreto_decaduto",
    "distanza_alleato",
    "costo_norma_lontana",
    "scivolamento_percepita",
    "costo_emendamento_respinto",
    "ft_emendamento_respinto",
    "costo_fiducia",
    "costo_fiducia_distanza",
    "turni_rinvio_regime",
    "seggi_per_punto_linea",
    "costo_seggio_perso",
    "astensione_confermativo",
    "astensione_abrogativo",
    "mobilitazione_toccati",
    "peso_governo_confermativo",
    "valence_referendum_perso",
    "valence_referendum_fallito",
    "ritorno_valence",
    "invito_astensione",
    "errore_fascia",
    "soglia_fascia_poco",
    "soglia_fascia_medio",
    "luna_di_miele",
    "valence_partito_nuovo",
];

/// Grandezze che il motore legge per nome, con il `regionale` atteso.
const GRANDEZZE_RICHIESTE: &[(&str, bool)] = &[
    ("occupazione_pubblica", false),
    ("stock_infrastrutture", true),
    ("investimenti_privati", true),
    ("domanda_regionale", true),
    ("prezzo_energia", false),
    ("politiche_attive", false),
    ("capitale_umano", false),
    ("efficienza_riscossione", false),
    ("durata_processi", false),
    ("regole_lavoro", false),
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
    grandezze: Vec<DatoGrandezza>,
    atti: Vec<Atto>,
    disposizioni: Vec<DatoDisposizione>,
}

#[derive(Deserialize)]
struct FileDisposizioni {
    disposizione: Vec<DatoDisposizione>,
}

#[derive(Deserialize)]
struct FileAtti {
    atto: Vec<Atto>,
}

#[derive(Deserialize)]
struct FileDicasteri {
    dicastero: Vec<Dicastero>,
}

#[derive(Deserialize)]
struct FileGrandezze {
    grandezza: Vec<DatoGrandezza>,
}

/// I dati di base, per i test d'unità.
#[cfg(test)]
#[expect(
    clippy::unwrap_used,
    reason = "nei test un unwrap che fallisce e' il fallimento voluto"
)]
pub(crate) fn dati_base() -> DatiGioco {
    let file = FileDati {
        dicasteri: include_str!("../../../dati/base/dicasteri.toml").into(),
        manopole: include_str!("../../../dati/base/manopole.toml").into(),
        economia: include_str!("../../../dati/base/economia.toml").into(),
        regioni: include_str!("../../../dati/base/regioni.toml").into(),
        scenario: include_str!("../../../dati/base/scenario.toml").into(),
        elezioni: include_str!("../../../dati/base/elezioni.toml").into(),
        elettorato: include_str!("../../../dati/base/elettorato.toml").into(),
        grandezze: include_str!("../../../dati/base/grandezze.toml").into(),
        atti: include_str!("../../../dati/base/atti.toml").into(),
        disposizioni: include_str!("../../../dati/base/disposizioni.toml").into(),
    };
    DatiGioco::da_testi(&file).unwrap()
}

#[cfg(test)]
impl DatiGioco {
    /// Per i test d'unità: il valore della manopola `nome`, se c'è.
    pub(crate) fn imposta_manopola(&mut self, nome: &str, valore: f64) {
        if let Some(m) = self.manopole.get_mut(nome) {
            m.valore = valore;
        }
    }
}

fn errore_toml(file: &str, e: toml::de::Error) -> Errore {
    Errore::Toml {
        file: file.into(),
        messaggio: e.to_string(),
    }
}

/// Le leggi elettorali dello scenario, iniziale e precedente, entrambe nel menu.
fn leggi_elettorali<'a>(
    scenario: &'a Scenario,
    elezioni: &'a DatiElezioni,
) -> Result<(&'a LeggeElettorale, &'a LeggeElettorale), Errore> {
    let Some(iniziale) = elezioni.impostazioni.get(&scenario.legge_elettorale) else {
        return Err(Errore::LeggeElettorale(format!(
            "lo scenario sceglie un'impostazione inesistente: {}",
            scenario.legge_elettorale
        )));
    };
    let precedente = &scenario.legge_elettorale_precedente;
    precedente.valida(&elezioni.menu).map_err(|e| match e {
        Errore::LeggeElettorale(m) => {
            Errore::LeggeElettorale(format!("legge elettorale precedente: {m}"))
        }
        altro => altro,
    })?;
    Ok((iniziale, precedente))
}

impl DatiGioco {
    pub fn dicasteri(&self) -> &[Dicastero] {
        &self.dicasteri
    }

    pub fn grandezze(&self) -> &[DatoGrandezza] {
        &self.grandezze
    }

    pub fn atti(&self) -> &[Atto] {
        &self.atti
    }

    /// Le disposizioni del file nel loro ordine, poi le nove elettorali.
    pub fn disposizioni(&self) -> &[DatoDisposizione] {
        &self.disposizioni
    }

    /// Posizione di una disposizione per id.
    pub fn disposizione(&self, id: &str) -> Option<usize> {
        self.disposizioni.iter().position(|d| d.id == id)
    }

    /// Gli atti di un dicastero (per indice) con il loro indice nel catalogo, nell'ordine del file.
    pub fn atti_del_dicastero(&self, d: usize) -> impl Iterator<Item = (usize, &Atto)> {
        let id = self.dicasteri.get(d).map(|d| d.id.as_str());
        self.atti
            .iter()
            .enumerate()
            .filter(move |(_, a)| Some(a.dicastero.as_str()) == id)
    }

    /// Posizione di una grandezza per id.
    pub fn grandezza(&self, id: &str) -> Option<usize> {
        self.grandezze.iter().position(|g| g.id == id)
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
        let grandezze = toml::from_str::<FileGrandezze>(&file.grandezze)
            .map_err(|e| errore_toml("grandezze.toml", e))?
            .grandezza;
        let atti = toml::from_str::<FileAtti>(&file.atti)
            .map_err(|e| errore_toml("atti.toml", e))?
            .atto;
        let mut disposizioni = toml::from_str::<FileDisposizioni>(&file.disposizioni)
            .map_err(|e| errore_toml("disposizioni.toml", e))?
            .disposizione;
        if let Some(d) = disposizioni.iter().find(|d| d.norma == Norma::Elettorale) {
            return Err(Errore::Disposizioni(format!(
                "{}: norma elettorale solo dal menu",
                d.id
            )));
        }
        elezioni.valida(&regioni)?;
        let (iniziale, precedente) = leggi_elettorali(&scenario, &elezioni)?;
        disposizioni.extend(disposizioni::elettorali(
            &elezioni.menu,
            iniziale,
            precedente,
        ));

        let dati = DatiGioco {
            dicasteri,
            manopole,
            struttura,
            regioni,
            scenario,
            elezioni,
            elettorato,
            grandezze,
            atti,
            disposizioni,
        };
        dati.valida()?;
        Ok(dati)
    }

    fn valida_grandezze(&self) -> Result<(), Errore> {
        let ko = |m: String| Err(Errore::Grandezze(m));
        let grandezze = &self.grandezze;
        for (i, g) in grandezze.iter().enumerate() {
            if grandezze[..i].iter().any(|p| p.id == g.id) {
                return ko(format!("grandezza ripetuta: {}", g.id));
            }
            let finite = g.iniziale.is_finite() && g.min.is_finite() && g.max.is_finite();
            if !(finite && g.min <= g.iniziale && g.iniziale <= g.max) {
                return ko(format!("{}: iniziale fuori da min-max", g.id));
            }
            for id in &g.dicasteri {
                let Some(d) = self.dicasteri.iter().find(|d| &d.id == id) else {
                    return ko(format!("{}: dicastero sconosciuto {id}", g.id));
                };
                if g.regionale && !d.regione_bersaglio {
                    return ko(format!(
                        "{}: regionale, ma {id} non ha regione bersaglio",
                        g.id
                    ));
                }
            }
        }
        for d in &self.dicasteri {
            let ind = &d.indicatore_servizio;
            let giusta = grandezze
                .iter()
                .any(|g| &g.id == ind && g.dinamica == Dinamica::Servizio && !g.regionale);
            if !giusta {
                return ko(format!("indicatore di servizio senza grandezza: {ind}"));
            }
            // Valori negativi o non finiti li rifiuta la struttura, con il loro nome.
            if d.salari_pil == 0.0 && d.acquisti_pil == 0.0 {
                return ko(format!(
                    "{}: indicatore di servizio {ind} senza spesa di funzionamento",
                    d.id
                ));
            }
        }
        for (id, regionale) in GRANDEZZE_RICHIESTE {
            if !grandezze
                .iter()
                .any(|g| g.id == *id && g.regionale == *regionale)
            {
                return ko(format!(
                    "grandezza richiesta mancante o di forma sbagliata: {id}"
                ));
            }
        }
        Ok(())
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
    /// [`DatiElezioni`]), `LeggeElettorale` se lo scenario sceglie un'impostazione inesistente;
    /// `Grandezze` (id ripetuti, intervalli, dicasteri, indicatori di servizio, grandezze richieste);
    /// `Atti` (catalogo: id, caselle, posizioni, effetti contro le grandezze); `Disposizioni`
    /// (catalogo: id, gradini, leve, effetti, perimetro, arrivi); `LeggeElettorale` anche per la
    /// legge elettorale precedente dello scenario.
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
        self.valida_grandezze()?;
        self.elezioni.valida(&self.regioni)?;
        leggi_elettorali(&self.scenario, &self.elezioni)?;
        disposizioni::valida(
            &self.disposizioni,
            &self.dicasteri,
            &self.grandezze,
            &self.regioni,
        )?;
        crate::atti::valida(self)?;
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
        let (poco, medio) = (
            self.manopola("soglia_fascia_poco"),
            self.manopola("soglia_fascia_medio"),
        );
        if poco > medio {
            return Err(Errore::Grandezze(format!(
                "manopola soglia_fascia_poco ({poco}) oltre soglia_fascia_medio ({medio})"
            )));
        }
        let scala = self.manopola("scala_servizio");
        if scala <= 0.0 {
            return Err(Errore::Grandezze(format!(
                "manopola scala_servizio non positiva: {scala}"
            )));
        }
        self.struttura.valida(&self.regioni, &self.dicasteri)?;
        self.elettorato
            .valida(&self.regioni, &self.dicasteri, self.struttura.settori.len())?;
        self.scenario.valida()?;
        partiti::valida(&self.scenario, &self.dicasteri)?;
        crate::parlamento::valida_uscente(&self.scenario, &self.disposizioni)?;
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
