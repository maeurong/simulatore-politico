//! Il parlamento in carica: seggi per partito, misure in calendario, esiti dei voti.

use crate::dati::DatiGioco;
use crate::disposizioni::{DatoDisposizione, Disposizioni, Norma, data_di};
use crate::elettorato::Affluenza;
use crate::errore::Errore;
use crate::informazione::Sondaggio;
use crate::ministri::{Causa, Ministri, penalizza};
use crate::partita::Trimestre;
use crate::partiti::{Governo, Partito};
use crate::scenario::Scenario;
use crate::vista::{Forbice, VariazioneSeggi, VoceMisura};
use serde::{Deserialize, Serialize};
#[cfg(test)]
use ts_rs::TS;

mod anteprima;
mod decreti;
mod emendamenti;
mod intesa;
mod regole;
pub mod voto;

pub(crate) use anteprima::anteprima;
pub(crate) use decreti::ammetti;
pub use decreti::{Decreto, FondiDecreto};
pub use emendamenti::{Attenuazione, RichiestaEmendamento, StatoEmendamento};
pub(crate) use emendamenti::{costo_fiducia, richiesta_bilancio, rinviata};
pub use intesa::{CausaIntesa, Intesa, VariazioneIntesa};
pub use regole::{EsitoReferendum, Referendum, TipoReferendum};
pub(crate) use regole::{due_terzi, legge_non_valida, proietta, quinto_contro, riparto_con};

/// Posti del calendario per riforme ed elettorali.
pub const POSTI_ORDINARI: usize = 3;
pub const POSTI_COSTITUZIONALI: usize = 1;
/// Il posto riservato all'opposizione (iter-legislativo-italiano:119, un argomento su cinque), a
/// rotazione fra i partiti fuori dal governo (scelta 15 del piano 7b): un posto per turno, del
/// giocatore o di un partito IA (scelta 16 del piano 8a). Conta solo le proposte presentate nel
/// turno: quella del turno prima, al voto ora, non lo occupa.
pub const POSTI_OPPOSIZIONE: usize = 1;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(test, derive(TS), ts(export))]
#[serde(rename_all = "snake_case")]
pub enum FormaMisura {
    Riforma,
    Elettorale,
    Costituzionale,
    Conversione,
    Bilancio,
    /// La mozione di sfiducia, al governo o a un ministro: votata a blocchi, mai in calendario.
    Sfiducia,
}

/// Una misura in calendario. `valori` = disposizione e gradino; `proponente` = dicastero;
/// `promotore` = il partito d'opposizione che l'ha portata nel posto riservato; `voto` = turno
/// nella cui chiusura si vota; `deliberazione` 1 o 2 per le costituzionali, 0 altrimenti. `fondi`
/// solo sulle conversioni. `linea_giocatore` = la linea scelta dal giocatore all'opposizione, o
/// al governo sulle proposte dell'opposizione; `None` = quella di default.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Misura {
    pub id: u32,
    pub forma: FormaMisura,
    pub valori: Vec<(usize, usize)>,
    pub proponente: Option<usize>,
    pub promotore: Option<usize>,
    pub linea_giocatore: Option<bool>,
    pub presentata: u32,
    pub voto: u32,
    pub deliberazione: u8,
    pub fiducia: bool,
    pub emendamenti: Vec<RichiestaEmendamento>,
    pub emendamenti_aperti: bool,
    pub fondi: Option<FondiDecreto>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct Conteggio {
    pub si: u32,
    pub no: u32,
}

/// `partito` è un id.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct VoceLinea {
    pub partito: String,
    pub si: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct EsitoVoto {
    pub misura: u32,
    pub nome: String,
    pub forma: FormaMisura,
    pub data: Trimestre,
    pub deliberazione: u8,
    pub camera: Conteggio,
    pub senato: Conteggio,
    pub approvata: bool,
    pub fiducia: bool,
    pub fiducia_respinta: bool,
    pub linee: Vec<VoceLinea>,
    /// La misura decaduta senza voto: il motivo leggibile, per il giocatore.
    pub decaduta: Option<String>,
}

/// Un valore di una legge presentata: id della disposizione e del gradino.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct ValoreProposto {
    pub disposizione: String,
    pub gradino: String,
}

/// Seggi per partito nell'ordine dello scenario. `respinte` = disposizione e turno da cui si
/// ripresenta; `prossima` = id della prossima misura; `referendum` = l'abrogativo in corso,
/// uno alla volta; `confermativi` = i referendum confermativi in attesa del voto; `decaduti` =
/// le firme dei decreti decaduti, che non si ripetono; `sfiducia` = la mozione del giocatore o
/// dell'opposizione IA del turno (dicastero, `None` = al governo; promotore), votata in chiusura.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Parlamento {
    pub seggi_camera: Vec<u32>,
    pub seggi_senato: Vec<u32>,
    pub misure: Vec<Misura>,
    pub respinte: Vec<(usize, u32)>,
    pub prossima: u32,
    pub esiti: Vec<EsitoVoto>,
    pub referendum: Option<Referendum>,
    pub confermativi: Vec<Referendum>,
    pub esiti_referendum: Vec<EsitoReferendum>,
    pub decaduti: Vec<FondiDecreto>,
    pub sfiducia: Option<(Option<usize>, usize)>,
}

/// Seggi di Camera e Senato al gradino di partenza di `numero_parlamentari`.
fn seggi_partenza(disposizioni: &[DatoDisposizione]) -> Option<(f64, f64)> {
    let d = disposizioni
        .iter()
        .find(|d| d.id == "numero_parlamentari")?;
    let leve = &d.gradini.get(d.gradino(&d.partenza)?)?.leve;
    Some((*leve.get("seggi_camera")?, *leve.get("seggi_senato")?))
}

/// Il parlamento uscente dello scenario contro partiti e `numero_parlamentari`.
///
/// # Errors
/// `Partiti` col nome del ramo e del partito sconosciuto, coi seggi negativi o oltre i seggi di
/// partenza del ramo, o con la somma diversa dai seggi di partenza.
pub(crate) fn valida_uscente(
    scenario: &Scenario,
    disposizioni: &[DatoDisposizione],
) -> Result<(), Errore> {
    let ko = |m: String| Err(Errore::Partiti(format!("parlamento_uscente.{m}")));
    let Some((camera, senato)) = seggi_partenza(disposizioni) else {
        return ko("numero_parlamentari: mancano seggi_camera e seggi_senato di partenza".into());
    };
    let u = &scenario.parlamento_uscente;
    for (ramo, seggi, attesi) in [("camera", &u.camera, camera), ("senato", &u.senato, senato)] {
        for (id, &n) in seggi {
            if !scenario.partiti.iter().any(|p| &p.id == id) {
                return ko(format!("{ramo}: partito sconosciuto: {id}"));
            }
            if n < 0 {
                return ko(format!("{ramo}: seggi negativi a {id}"));
            }
            if n as f64 > attesi {
                return ko(format!(
                    "{ramo}: {n} seggi a {id}, più dei {attesi} del ramo"
                ));
            }
        }
        let somma: i64 = seggi.values().sum();
        if somma as f64 != attesi {
            return ko(format!(
                "{ramo}: {somma} seggi, numero_parlamentari ne dà {attesi}"
            ));
        }
    }
    Ok(())
}

fn ko<T>(m: String) -> Result<T, Errore> {
    Err(Errore::Comando(m))
}

/// `gennaio 2027`: il primo mese del trimestre.
fn mese(t: Trimestre) -> String {
    let m = match t.trimestre {
        1 => "gennaio",
        2 => "aprile",
        3 => "luglio",
        _ => "ottobre",
    };
    format!("{m} {}", t.anno)
}

/// `"{nome disposizione}: {nome gradino}"` uniti da `"; "`.
pub(crate) fn nome(dati: &DatiGioco, valori: &[(usize, usize)]) -> String {
    let d = dati.disposizioni();
    (valori.iter())
        .filter_map(|&(k, g)| Some(format!("{}: {}", d.get(k)?.nome, d[k].gradini.get(g)?.nome)))
        .collect::<Vec<_>>()
        .join("; ")
}

/// Il nome della misura: dei fondi per una conversione, dei valori altrimenti.
pub(crate) fn nome_misura(dati: &DatiGioco, m: &Misura) -> String {
    if m.forma == FormaMisura::Bilancio {
        return "Legge di bilancio".into();
    }
    m.fondi
        .map_or_else(|| nome(dati, &m.valori), |f| f.nome(dati))
}

/// Il voto dell'abrogativo chiesto nel `turno`, ad aprile-giugno dell'anno dopo; `None` dopo
/// luglio-settembre, fuori finestra.
fn voto_abrogativo(turno: u32) -> Option<u32> {
    let trimestre = u32::from(data_di(turno).trimestre);
    (trimestre <= 3).then(|| turno.saturating_add(6 - trimestre))
}

/// La legge di bilancio col voto nel `turno`.
pub(crate) fn bilancio_del_turno(m: &Misura, turno: u32) -> bool {
    m.forma == FormaMisura::Bilancio && m.voto == turno
}

/// La legge col voto nel turno, che nel turno del bilancio slitta al turno dopo.
fn slitta(m: &Misura, turno: u32) -> bool {
    let legge = matches!(
        m.forma,
        FormaMisura::Riforma | FormaMisura::Elettorale | FormaMisura::Costituzionale
    );
    legge && m.voto == turno
}

/// Posizione media in vigore e media nuova di `valori`, sulle sole disposizioni in cui hanno
/// posizione sia il gradino in vigore sia il nuovo; `None` se nessuna.
pub(crate) fn posizioni(
    dati: &DatiGioco,
    disposizioni: &Disposizioni,
    valori: &[(usize, usize)],
) -> Option<(f64, f64)> {
    let coppie: Vec<(f64, f64)> = (valori.iter())
        .filter_map(|&(d, g)| {
            let x = dati.disposizioni().get(d)?;
            Some((
                x.posizione(disposizioni.voci.get(d)?.in_vigore)?,
                x.posizione(g)?,
            ))
        })
        .collect();
    let n = coppie.len() as f64;
    let media = |f: fn(&(f64, f64)) -> f64| coppie.iter().map(f).sum::<f64>() / n;
    (!coppie.is_empty()).then(|| (media(|c| c.0), media(|c| c.1)))
}

/// Ciò che le linee e i franchi tiratori leggono della partita.
pub(crate) struct StatoAula<'a> {
    pub dati: &'a DatiGioco,
    pub disposizioni: &'a Disposizioni,
    pub sondaggio: &'a Sondaggio,
    pub partiti: &'a [Partito],
    pub governo: &'a Governo,
    pub intesa: &'a [f64],
    pub ultimatum: &'a [Option<CausaIntesa>],
    /// Le coalizioni dell'ultimo voto, o nel turno del voto quelle al voto, per i seggi attesi.
    pub coalizioni: Vec<Vec<usize>>,
    pub giocatore: usize,
    /// `Legislatura::ultimo_anno` del turno, per il malus dei partiti IA (scelta 19 del piano 8a).
    pub ultimo_anno: bool,
}

/// Posizioni, linea e probabilità dei franchi tiratori di una misura, sullo stato attuale.
/// `franchi_camera` = `franchi` tranne sull'elettorale, dove il voto finale alla Camera è
/// segreto; sotto fiducia sono quelli del voto finale. `seggi` = seggi attesi di Camera e Senato
/// per partito, solo sull'elettorale.
pub(crate) struct Previsione {
    pub posizioni: Option<(f64, f64)>,
    pub linea: Vec<bool>,
    pub franchi: Vec<f64>,
    pub franchi_camera: Vec<f64>,
    pub seggi: Vec<(i32, i32)>,
}

impl StatoAula<'_> {
    /// Sotto fiducia la linea a blocchi e nessun franco tiratore; altrimenti l'alleato con
    /// l'emendamento accolto vota sì con `ft_base`, quello col respinto ha i franchi tiratori
    /// moltiplicati per `1 + ft_emendamento_respinto`, entro `ft_max`. Sull'elettorale il
    /// guadagno della linea è `Δseggi / seggi_per_punto_linea` e alla Camera i franchi tiratori
    /// raddoppiano, entro `2 · ft_max`, anche nel voto finale dopo la fiducia. Sulla conversione
    /// e sul bilancio la maggioranza vota sì e l'opposizione no; senza posizione, i franchi
    /// tiratori vengono dalla sola intesa. Il promotore della proposta d'opposizione vota sì, e su
    /// di essa il Presidente del Consiglio segue il guadagno, o la linea scelta se è il giocatore,
    /// e gli alleati gli sono fedeli in quella direzione (scelta 15 del piano 8a).
    /// La linea scelta dal giocatore (scelta 14 del piano 7b) vale per il suo partito, anche
    /// sotto fiducia; fuori dalla fiducia coi franchi tiratori
    /// `ft_base + ft_distanza · |x − sostenuta| / 6` entro `ft_max`: `sostenuta` è la posizione
    /// nuova col sì, la vigente col no; senza posizione `ft_base`.
    pub(crate) fn previsione(&self, m: &Misura) -> Previsione {
        let posizioni = posizioni(self.dati, self.disposizioni, &m.valori);
        let (dati, partiti, governo, intesa) = (self.dati, self.partiti, self.governo, self.intesa);
        let nuova = posizioni.map(|(_, n)| n);
        let elettorale = m.forma == FormaMisura::Elettorale;
        let seggi = if elettorale {
            self.seggi_attesi(&m.valori)
        } else {
            Vec::new()
        };
        let tetto = dati.manopola("ft_max");
        let segreto = |q: &[f64]| -> Vec<f64> {
            if elettorale {
                q.iter().map(|q| (2.0 * q).min(2.0 * tetto)).collect()
            } else {
                q.to_vec()
            }
        };
        let g = self.giocatore;
        let scelta = |linea: &mut [bool]| {
            if let (Some(scelta), Some(si)) = (m.linea_giocatore, linea.get_mut(g)) {
                *si = scelta;
            }
        };
        if m.fiducia {
            let mut linea = voto::linea_fiducia(dati, partiti, governo, self.ultimatum, nuova);
            scelta(&mut linea);
            let franchi = vec![0.0; linea.len()];
            let franchi_camera = if elettorale {
                segreto(&voto::probabilita_franco(
                    dati, partiti, governo, intesa, nuova, &linea,
                ))
            } else {
                franchi.clone()
            };
            return Previsione {
                posizioni,
                linea,
                franchi,
                franchi_camera,
                seggi,
            };
        }
        let guadagno = if elettorale {
            let per_punto = dati.manopola("seggi_per_punto_linea");
            (seggi.iter())
                .map(|&(c, s)| f64::from(c + s) / per_punto)
                .collect()
        } else {
            voto::guadagni(partiti, posizioni)
        };
        // Conversione e bilancio (scelta 4): maggioranza sì, opposizione no.
        let mut linea = if matches!(m.forma, FormaMisura::Conversione | FormaMisura::Bilancio) {
            (0..partiti.len()).map(|j| governo.al_governo(j)).collect()
        } else {
            // Sulla proposta dell'opposizione il Presidente del Consiglio è libero: la linea scelta
            // dal giocatore se presiede lui, altrimenti il guadagno.
            let pc = governo.presidente;
            let linea_pdc = m.promotore.map(|_| {
                (m.linea_giocatore.filter(|_| pc == g))
                    .unwrap_or_else(|| guadagno.get(pc).is_some_and(|&x| x > 0.0))
            });
            voto::linea(dati, governo, intesa, &guadagno, &self.malus(), linea_pdc)
        };
        let accolti = (m.emendamenti.iter()).filter(|e| e.stato == StatoEmendamento::Accolto);
        for j in accolti.map(|e| e.partito).chain(m.promotore) {
            if let Some(si) = linea.get_mut(j) {
                *si = true;
            }
        }
        scelta(&mut linea);
        let mut franchi = voto::probabilita_franco(dati, partiti, governo, intesa, nuova, &linea);
        let base = dati.manopola("ft_base");
        let respinto = 1.0 + dati.manopola("ft_emendamento_respinto");
        for e in &m.emendamenti {
            if let Some(q) = franchi.get_mut(e.partito) {
                *q = match e.stato {
                    StatoEmendamento::Aperto => *q,
                    StatoEmendamento::Accolto => base.min(tetto),
                    StatoEmendamento::Respinto => (*q * respinto).min(tetto),
                };
            }
        }
        if let (Some(si), Some(q), Some(x)) =
            (m.linea_giocatore, franchi.get_mut(g), partiti.get(g))
        {
            let sostenuta = posizioni.map(|(vigore, nuova)| if si { nuova } else { vigore });
            let d = sostenuta.map_or(0.0, |s| (x.posizione_dichiarata - s).abs() / 6.0);
            *q = (base + dati.manopola("ft_distanza") * d).min(tetto);
        }
        Previsione {
            posizioni,
            linea,
            franchi_camera: segreto(&franchi),
            franchi,
            seggi,
        }
    }

    /// Il malus della linea per partito (scelta 19 del piano 8a): zero al governo,
    /// `malus_opposizione` fuori, per i partiti IA × `(1 + crescita_malus_ultimo_anno)` nell'ultimo
    /// anno. Il giocatore ha la sua linea.
    pub(crate) fn malus(&self) -> Vec<f64> {
        let base = self.dati.manopola("malus_opposizione");
        let crescita = 1.0 + self.dati.manopola("crescita_malus_ultimo_anno");
        (0..self.partiti.len())
            .map(|j| {
                if self.governo.al_governo(j) {
                    0.0
                } else if self.ultimo_anno && j != self.giocatore {
                    base * crescita
                } else {
                    base
                }
            })
            .collect()
    }

    /// Una voce per partito, nell'ordine dei partiti.
    pub(crate) fn voci_linea(&self, linea: &[bool]) -> Vec<VoceLinea> {
        (self.partiti.iter().zip(linea))
            .map(|(p, &si)| VoceLinea {
                partito: p.id.clone(),
                si,
            })
            .collect()
    }

    /// Una voce per partito dei seggi attesi della previsione; vuota fuori dall'elettorale.
    pub(crate) fn voci_seggi(&self, seggi: &[(i32, i32)]) -> Vec<VariazioneSeggi> {
        (self.partiti.iter().zip(seggi))
            .map(|(x, &(camera, senato))| VariazioneSeggi {
                partito: x.id.clone(),
                camera,
                senato,
            })
            .collect()
    }
}

impl Parlamento {
    /// La legislatura uscente dello scenario; un partito assente non ha seggi.
    pub(crate) fn uscente(dati: &DatiGioco, partiti: &[Partito]) -> Parlamento {
        let u = &dati.scenario().parlamento_uscente;
        let seggi = |ramo: &std::collections::BTreeMap<String, i64>| {
            (partiti.iter())
                .map(|p| {
                    ramo.get(&p.id)
                        .map_or(0, |&n| u32::try_from(n).unwrap_or(0))
                })
                .collect()
        };
        Parlamento {
            seggi_camera: seggi(&u.camera),
            seggi_senato: seggi(&u.senato),
            ..Parlamento::default()
        }
    }

    /// La disposizione `d` è in una misura in calendario o in un referendum in corso, abrogativo
    /// o confermativo.
    pub(crate) fn in_esame(&self, d: usize) -> bool {
        let in_misura = (self.misure.iter()).any(|m| m.valori.iter().any(|&(x, _)| x == d));
        let mut referendum = self.referendum.iter().chain(&self.confermativi);
        in_misura || referendum.any(|r| r.valori.iter().any(|v| v.0 == d))
    }

    /// Il turno da cui la disposizione `d`, in una legge respinta, si ripresenta, se è dopo
    /// `turno`.
    pub(crate) fn respinta_da_poco(&self, d: usize, turno: u32) -> Option<u32> {
        (self.respinte.iter()).find_map(|&(x, da)| (x == d && da > turno).then_some(da))
    }

    /// Il decreto accolto dal dicastero `i` per `atto` è decaduto: la firma non si ripete
    /// (iter-legislativo-italiano:20-23, 67).
    pub(crate) fn decaduto_accolto(&self, i: usize, atto: usize) -> bool {
        (self.decaduti.iter()).any(|f| {
            matches!(
                *f,
                FondiDecreto::Accolti { dicastero, atto: k, .. } if dicastero == i && k == atto
            )
        })
    }

    /// Il referendum abrogativo che `promotore` chiede su `disposizione` al turno `turno`
    /// (iter-legislativo-italiano:131-137): firme in chiusura del turno, voto ad aprile-giugno
    /// dell'anno dopo; `valori` = la disposizione e il suo valore dopo l'abrogazione. Non tocca
    /// nulla.
    ///
    /// # Errors
    /// `Comando`, nell'ordine: promotore sconosciuto o al governo, disposizione sconosciuta, non
    /// referendabile, niente da abrogare, valore dopo l'abrogazione già abrogato da un
    /// referendum, legge elettorale non valida dopo l'abrogazione, disposizione in una misura in
    /// calendario, un referendum già in corso, fuori finestra.
    pub(crate) fn abrogativo(
        &self,
        aula: &StatoAula,
        turno: u32,
        promotore: &str,
        disposizione: &str,
    ) -> Result<Referendum, Errore> {
        let (dati, partiti) = (aula.dati, aula.partiti);
        let Some(j) = partiti.iter().position(|p| p.id == promotore) else {
            return ko(format!("partito sconosciuto: {promotore}"));
        };
        if aula.governo.al_governo(j) {
            return ko(format!("il promotore è al governo: {}", partiti[j].nome));
        }
        let Some(d) = dati.disposizione(disposizione) else {
            return ko(format!("disposizione sconosciuta: {disposizione}"));
        };
        let (x, s) = (&dati.disposizioni()[d], &aula.disposizioni.voci[d]);
        if !x.referendabile {
            return ko(format!("{} non è referendabile", x.nome));
        }
        let dopo = (x.dopo_abrogazione.as_deref())
            .and_then(|g| x.gradino(g))
            .unwrap_or(s.precedente);
        // Un gradino abrogato non torna: niente reviviscenza (disposizioni-al-turno-zero:64).
        let nulla = dopo == s.in_vigore && s.in_arrivo.is_empty();
        if s.legge.is_empty() || nulla {
            return ko(format!("niente da abrogare: {}", x.nome));
        }
        if s.bloccati.contains(&dopo) {
            return ko(format!(
                "{}: il gradino che tornerebbe in vigore è già stato abrogato da un referendum",
                x.nome
            ));
        }
        if let Some(m) = legge_non_valida(dati, aula.disposizioni, &[(d, dopo)]) {
            return ko(format!("quesito inammissibile su {}: {m}", x.nome));
        }
        if (self.misure.iter()).any(|m| m.valori.iter().any(|&(y, _)| y == d)) {
            return ko(format!("{} è già in esame", x.nome));
        }
        if self.referendum.is_some() {
            return ko("un referendum è già in corso".into());
        }
        let Some(voto) = voto_abrogativo(turno) else {
            return ko("fuori finestra: si chiede entro luglio-settembre".into());
        };
        Ok(Referendum {
            tipo: TipoReferendum::Abrogativo,
            misura: None,
            valori: vec![(d, dopo)],
            promotore: Some(j),
            del_giocatore: false,
            voto,
            firme: false,
            invito_astensione: false,
            richiesto: true,
        })
    }

    /// `InvitaAstensione`: il governo invita a disertare l'abrogativo indetto, una volta.
    ///
    /// # Errors
    /// `Comando` senza un abrogativo con le firme raccolte, o con l'invito già fatto.
    pub(crate) fn invita_astensione(&mut self) -> Result<(), Errore> {
        let Some(r) = self.referendum.as_mut().filter(|r| r.firme) else {
            return ko("nessun referendum da disertare".into());
        };
        if r.invito_astensione {
            return ko("astensione già chiesta".into());
        }
        r.invito_astensione = true;
        Ok(())
    }

    /// `PresentaLegge`: controlla, mette la misura in calendario e fa pagare al ministro
    /// proponente la firma contro la sua posizione, anche sulla riforma che ha proposto lui; con
    /// `ministri = None` nessuna firma. Un rifiuto non tocca nulla.
    ///
    /// # Errors
    /// Quelli di `bozza`, poi `Comando` col calendario pieno.
    pub(crate) fn presenta(
        &mut self,
        dati: &DatiGioco,
        disposizioni: &Disposizioni,
        ministri: Option<&mut Ministri>,
        turno: u32,
        valori: &[ValoreProposto],
    ) -> Result<(), Errore> {
        let m = self.bozza(dati, disposizioni, turno, valori)?;
        let (posti, occupati) = if m.forma == FormaMisura::Costituzionale {
            (POSTI_COSTITUZIONALI, &[FormaMisura::Costituzionale][..])
        } else {
            (
                POSTI_ORDINARI,
                &[FormaMisura::Riforma, FormaMisura::Elettorale][..],
            )
        };
        if self
            .misure
            .iter()
            .filter(|m| occupati.contains(&m.forma) && m.promotore.is_none())
            .count()
            >= posti
        {
            return ko("calendario pieno".into());
        }
        let firma = (m.proponente)
            .zip(ministri)
            .and_then(|(i, x)| x.dicasteri.get_mut(i));
        if let Some(st) = firma {
            let x = st.ministro.as_ref().map(|x| x.posizione);
            if let (Some(x), Some((vigore, nuova))) = (x, posizioni(dati, disposizioni, &m.valori))
            {
                let lontananza = ((nuova - x).abs() - (vigore - x).abs()).max(0.0);
                let delta = -dati.manopola("costo_firma") * lontananza;
                penalizza(st, Causa::NormaControPosizione, delta, turno);
            }
        }
        self.misure.push(m);
        self.prossima = self.prossima.saturating_add(1);
        Ok(())
    }

    /// `PresentaProposta` del partito `promotore` nel posto riservato (scelta 15 del piano 7b),
    /// fuori dai posti ordinari: le regole di `bozza` senza dicastero proponente, voto al turno
    /// dopo. Un rifiuto non tocca nulla.
    ///
    /// # Errors
    /// `Comando` col posto riservato già occupato, poi quelli di `bozza`, poi con una norma
    /// costituzionale.
    pub(crate) fn presenta_proposta(
        &mut self,
        dati: &DatiGioco,
        disposizioni: &Disposizioni,
        turno: u32,
        promotore: usize,
        valori: &[ValoreProposto],
    ) -> Result<(), Errore> {
        if !self.posto_riservato_libero(turno) {
            return ko("il posto riservato è già occupato".into());
        }
        let m = self.bozza_proposta(dati, disposizioni, turno, promotore, valori)?;
        self.misure.push(m);
        self.prossima = self.prossima.saturating_add(1);
        Ok(())
    }

    /// La proposta del partito `promotore` che `presenta_proposta` metterebbe nel posto riservato,
    /// senza guardare se il posto è libero; non tocca nulla.
    ///
    /// # Errors
    /// `Comando`: quelli di `bozza`, poi con una norma costituzionale.
    pub(crate) fn bozza_proposta(
        &self,
        dati: &DatiGioco,
        disposizioni: &Disposizioni,
        turno: u32,
        promotore: usize,
        valori: &[ValoreProposto],
    ) -> Result<Misura, Errore> {
        let m = self.bozza(dati, disposizioni, turno, valori)?;
        if m.forma == FormaMisura::Costituzionale {
            return ko("una norma costituzionale non entra nel posto riservato".into());
        }
        Ok(Misura {
            proponente: None,
            promotore: Some(promotore),
            ..m
        })
    }

    /// Nessuna proposta dell'opposizione presentata nel `turno` occupa il posto riservato.
    pub(crate) fn posto_riservato_libero(&self, turno: u32) -> bool {
        (self.misure.iter())
            .filter(|m| m.promotore.is_some() && m.presentata == turno)
            .count()
            < POSTI_OPPOSIZIONE
    }

    /// `Linea`: la linea del giocatore sulla misura, `None` = quella di default.
    ///
    /// # Errors
    /// `Comando` con la misura sconosciuta.
    pub(crate) fn linea(&mut self, misura: u32, si: Option<bool>) -> Result<(), Errore> {
        let Some(m) = self.misure.iter_mut().find(|m| m.id == misura) else {
            return ko(format!("misura sconosciuta: {misura}"));
        };
        m.linea_giocatore = si;
        Ok(())
    }

    /// La misura che `PresentaLegge` metterebbe in calendario, con tutte le sue regole tranne il
    /// posto in calendario; non tocca nulla.
    ///
    /// # Errors
    /// `Comando`, nell'ordine: legge vuota, disposizione sconosciuta, ripetuta, gradino
    /// sconosciuto, norme di forma diversa, dicasteri diversi, gradino già in vigore, bloccato
    /// dal referendum, disposizione già in esame (anche in un referendum in corso), respinta
    /// da poco, legge elettorale non valida.
    pub(crate) fn bozza(
        &self,
        dati: &DatiGioco,
        disposizioni: &Disposizioni,
        turno: u32,
        valori: &[ValoreProposto],
    ) -> Result<Misura, Errore> {
        let catalogo = dati.disposizioni();
        if valori.is_empty() {
            return ko("legge vuota".into());
        }
        let mut indici: Vec<usize> = Vec::with_capacity(valori.len());
        for v in valori {
            let Some(d) = dati.disposizione(&v.disposizione) else {
                return ko(format!("disposizione sconosciuta: {}", v.disposizione));
            };
            if indici.contains(&d) {
                return ko(format!("disposizione ripetuta: {}", catalogo[d].nome));
            }
            indici.push(d);
        }
        let mut scelti: Vec<(usize, usize)> = Vec::with_capacity(valori.len());
        for (v, &d) in valori.iter().zip(&indici) {
            let Some(g) = catalogo[d].gradino(&v.gradino) else {
                return ko(format!(
                    "{}: gradino sconosciuto «{}»",
                    catalogo[d].nome, v.gradino
                ));
            };
            scelti.push((d, g));
        }
        let norma = catalogo[indici[0]].norma;
        if indici.iter().any(|&d| catalogo[d].norma != norma) {
            return ko("norme di forma diversa nella stessa legge".into());
        }
        let forma = match norma {
            Norma::Ordinaria => FormaMisura::Riforma,
            Norma::Elettorale => FormaMisura::Elettorale,
            Norma::Costituzionale => FormaMisura::Costituzionale,
        };
        let dicastero = &catalogo[indici[0]].dicastero;
        if let Some(&d) = indici
            .iter()
            .find(|&&d| &catalogo[d].dicastero != dicastero)
        {
            return ko(format!(
                "le disposizioni di una legge devono stare nello stesso dicastero: «{}» sta altrove",
                catalogo[d].nome
            ));
        }
        for &(d, g) in &scelti {
            let (nome, stato) = (&catalogo[d].nome, &disposizioni.voci[d]);
            let gradino = &catalogo[d].gradini[g].nome;
            if stato.in_vigore == g {
                return ko(format!("{nome} è già a {gradino}"));
            }
            if stato.bloccati.contains(&g) {
                return ko(format!(
                    "{nome}: il gradino «{gradino}» è bloccato dal referendum"
                ));
            }
            if self.in_esame(d) {
                return ko(format!("{nome} è già in esame"));
            }
            if let Some(da) = self.respinta_da_poco(d, turno) {
                return ko(format!(
                    "{nome}: legge respinta, si ripresenta da {}",
                    mese(data_di(da))
                ));
            }
        }
        if forma == FormaMisura::Elettorale {
            // Dopo il voto e dopo gli arrivi elettorali in corso.
            let con_arrivi = [disposizioni.arrivi_elettorali(), scelti.clone()].concat();
            for v in [&scelti, &con_arrivi] {
                if let Some(m) = legge_non_valida(dati, disposizioni, v) {
                    return ko(format!("legge elettorale non applicabile: {m}"));
                }
            }
        }
        let costituzionale = forma == FormaMisura::Costituzionale;
        let proponente =
            (dicastero.as_ref()).and_then(|id| dati.dicasteri().iter().position(|x| &x.id == id));
        Ok(Misura {
            id: self.prossima,
            forma,
            valori: scelti,
            proponente,
            promotore: None,
            linea_giocatore: None,
            presentata: turno,
            voto: if costituzionale {
                turno
            } else {
                turno.saturating_add(1)
            },
            deliberazione: u8::from(costituzionale),
            fiducia: false,
            emendamenti: Vec::new(),
            emendamenti_aperti: false,
            fondi: None,
        })
    }

    /// La conversione del decreto `fondi`, votata a fine turno.
    pub(crate) fn apri_conversione(&mut self, turno: u32, fondi: FondiDecreto) {
        let m = self.conversione(turno, fondi);
        self.misure.push(m);
        self.prossima = self.prossima.saturating_add(1);
    }

    /// La misura di conversione di `fondi` come entrerebbe in calendario.
    pub(crate) fn conversione(&self, turno: u32, fondi: FondiDecreto) -> Misura {
        Misura {
            id: self.prossima,
            forma: FormaMisura::Conversione,
            valori: Vec::new(),
            proponente: Some(fondi.proponente()),
            promotore: None,
            linea_giocatore: None,
            presentata: turno,
            voto: turno,
            deliberazione: 0,
            fiducia: false,
            emendamenti: Vec::new(),
            emendamenti_aperti: false,
            fondi: Some(fondi),
        }
    }

    /// La legge di bilancio presentata nel turno, votata a fine turno, fuori dal calendario
    /// ordinario; ripresentata, resta la stessa misura.
    pub(crate) fn apri_bilancio(&mut self, turno: u32) {
        if self.turno_del_bilancio(turno) {
            return;
        }
        self.misure.push(Misura {
            id: self.prossima,
            forma: FormaMisura::Bilancio,
            valori: Vec::new(),
            proponente: None,
            promotore: None,
            linea_giocatore: None,
            presentata: turno,
            voto: turno,
            deliberazione: 0,
            fiducia: false,
            emendamenti: Vec::new(),
            emendamenti_aperti: false,
            fondi: None,
        });
        self.prossima = self.prossima.saturating_add(1);
    }

    /// In calendario c'è la legge di bilancio col voto nel turno.
    pub(crate) fn turno_del_bilancio(&self, turno: u32) -> bool {
        (self.misure.iter()).any(|m| bilancio_del_turno(m, turno))
    }

    /// Nel turno del bilancio o di crisi le leggi col voto nel turno passano al turno dopo e
    /// perdono la questione di fiducia; i loro emendamenti aperti restano aperti, e quelli caduti
    /// con la fiducia si riaprono. Rende le slittate che avevano la questione di fiducia, il cui
    /// costo si rimborsa.
    pub(crate) fn slitta_leggi(&mut self, turno: u32) -> Vec<Misura> {
        let mut con_fiducia = Vec::new();
        for m in self.misure.iter_mut().filter(|m| slitta(m, turno)) {
            if m.fiducia {
                con_fiducia.push(m.clone());
                m.emendamenti_aperti = false;
            }
            m.voto = turno.saturating_add(1);
            m.fiducia = false;
        }
        con_fiducia
    }

    /// La conversione del decreto dei fondi accolti per `dicastero`, se c'è, esce dal calendario.
    pub(crate) fn ritira(&mut self, dicastero: usize) -> Option<Misura> {
        let k = (self.misure.iter()).position(|m| {
            matches!(m.fondi, Some(FondiDecreto::Accolti { dicastero: d, .. }) if d == dicastero)
        })?;
        Some(self.misure.remove(k))
    }

    /// I fondi della conversione accolta per `dicastero` in calendario, se c'è.
    pub(crate) fn in_conversione(&self, dicastero: usize) -> Option<FondiDecreto> {
        (self.misure.iter()).find_map(|m| {
            m.fondi.filter(
                |f| matches!(f, FondiDecreto::Accolti { dicastero: d, .. } if *d == dicastero),
            )
        })
    }

    /// La conversione accolta per `dicastero` passa all'atto `atto`.
    pub(crate) fn sposta_conversione(&mut self, dicastero: usize, atto: usize) {
        for m in &mut self.misure {
            if let Some(FondiDecreto::Accolti {
                dicastero: d,
                atto: a,
                ..
            }) = &mut m.fondi
                && *d == dicastero
            {
                *a = atto;
            }
        }
    }

    /// Le misure in calendario per la `Vista`: linee e forbice, nessun franco tiratore estratto.
    pub(crate) fn voci_calendario(&self, aula: &StatoAula) -> Vec<VoceMisura> {
        let dati = aula.dati;
        let catalogo = dati.disposizioni();
        (self.misure.iter())
            .map(|m| {
                let p = aula.previsione(m);
                let nuova = p.posizioni.map(|(_, n)| n);
                // L'ultimatum è lo stato della chiusura precedente: il costo della fiducia non lo
                // cambia prima del voto.
                let a_blocchi =
                    voto::linea_fiducia(dati, aula.partiti, aula.governo, aula.ultimatum, nuova);
                VoceMisura {
                    id: m.id,
                    nome: nome_misura(dati, m),
                    forma: m.forma,
                    proponente: (m.proponente)
                        .and_then(|i| dati.dicasteri().get(i))
                        .map(|d| d.id.clone()),
                    promotore: (m.promotore)
                        .and_then(|j| aula.partiti.get(j))
                        .map(|p| p.id.clone()),
                    linea_giocatore: m.linea_giocatore,
                    valori: (m.valori.iter())
                        .map(|&(d, g)| ValoreProposto {
                            disposizione: catalogo[d].id.clone(),
                            gradino: catalogo[d].gradini[g].id.clone(),
                        })
                        .collect(),
                    voto: data_di(m.voto),
                    deliberazione: m.deliberazione,
                    fiducia: m.fiducia,
                    linee: aula.voci_linea(&p.linea),
                    forbice: self.forbice(&p),
                    emendamenti: emendamenti::voci(dati, aula.partiti, &m.emendamenti),
                    seggi: aula.voci_seggi(&p.seggi),
                    alleati_contro_fiducia: (aula.partiti.iter().zip(a_blocchi).enumerate())
                        .filter(|&(j, (_, si))| {
                            m.forma != FormaMisura::Costituzionale
                                && !si
                                && voto::alleato(aula.governo, j)
                        })
                        .map(|(_, (p, _))| p.id.clone())
                        .collect(),
                }
            })
            .collect()
    }

    /// I sì attesi ± 2σ per ramo con la linea e i franchi tiratori della previsione.
    pub(crate) fn forbice(&self, p: &Previsione) -> Forbice {
        let camera = voto::forbice(&self.seggi_camera, &p.linea, &p.franchi_camera);
        let senato = voto::forbice(&self.seggi_senato, &p.linea, &p.franchi);
        let maggioranza_camera = voto::maggioranza(self.seggi_camera.iter().sum());
        let maggioranza_senato = voto::maggioranza(self.seggi_senato.iter().sum());
        Forbice {
            camera,
            senato,
            maggioranza_camera,
            maggioranza_senato,
            in_bilico: camera[0] < maggioranza_camera || senato[0] < maggioranza_senato,
        }
    }

    /// Ogni misura in calendario per cui vale `decade` decade senza voto, senza costi e senza
    /// blocco, con l'esito `decaduta`; le altre restano.
    pub(crate) fn decadi_calendario(
        &mut self,
        dati: &DatiGioco,
        data: Trimestre,
        decaduta: &str,
        decade: impl Fn(&Misura) -> bool,
    ) {
        let (via, restano): (Vec<Misura>, Vec<Misura>) = std::mem::take(&mut self.misure)
            .into_iter()
            .partition(decade);
        self.misure = restano;
        for m in via {
            self.esiti.insert(
                0,
                EsitoVoto {
                    misura: m.id,
                    nome: nome_misura(dati, &m),
                    forma: m.forma,
                    data,
                    deliberazione: m.deliberazione,
                    camera: Conteggio { si: 0, no: 0 },
                    senato: Conteggio { si: 0, no: 0 },
                    approvata: false,
                    fiducia: m.fiducia,
                    fiducia_respinta: false,
                    linee: Vec::new(),
                    decaduta: Some(decaduta.into()),
                },
            );
        }
    }

    /// Fine della legislatura, al voto prima del riparto (iter-legislativo-italiano:80, 135): ogni
    /// misura in calendario decade (`decadi_calendario`). Al voto in calendario possono esserci
    /// solo riforme, leggi elettorali e costituzionali (decadono anche quelle), mai fiducie né
    /// conversioni: la fiducia posta (`poni_fiducia`) e la conversione aperta hanno `voto = turno`
    /// e si votano prima. Le respinte si dimenticano; l'abrogativo indetto e non ancora votato è
    /// sospeso, senza valence. I confermativi restano.
    pub(crate) fn decadi_tutto(
        &mut self,
        dati: &DatiGioco,
        disposizioni: &Disposizioni,
        data: Trimestre,
    ) {
        self.decadi_calendario(dati, data, "Decaduta per la fine della legislatura", |_| {
            true
        });
        self.respinte.clear();
        if let Some(r) = self.referendum.take() {
            let vigente = (r.valori.iter())
                .filter_map(|&(d, _)| Some((d, disposizioni.voci.get(d)?.in_vigore)))
                .collect::<Vec<_>>();
            let nessuno = Affluenza {
                si: 0.0,
                no: 0.0,
                astenuti: 0.0,
            };
            self.esiti_referendum.insert(
                0,
                EsitoReferendum {
                    decaduto: Some("Sospeso per lo scioglimento delle Camere".into()),
                    ..EsitoReferendum::da(r.tipo, nome(dati, &vigente), data, &nessuno)
                },
            );
        }
    }

    /// Apertura del turno `turno`: restano gli esiti dei due turni prima.
    pub(crate) fn apri(&mut self, turno: u32) {
        let da = data_di(turno.saturating_sub(2));
        let recente = |d: Trimestre| (d.anno, d.trimestre) >= (da.anno, da.trimestre);
        (self.esiti).retain(|e| recente(e.data));
        (self.esiti_referendum).retain(|e| recente(e.data));
    }

    /// # Errors
    /// `SalvataggioIlleggibile` se: i seggi non sono uno per partito o non sommano ai `totali` dei
    /// rami del parlamento eletto (zero a Camere sciolte), o li sommano oltre il gradino più alto
    /// di `seggi_camera` o `seggi_senato` nel catalogo; una misura, un suo emendamento o una
    /// respinta esce dalle disposizioni, dai gradini, dai dicasteri o dai partiti, come la
    /// sfiducia; un referendum
    /// esce dalle disposizioni, dai gradini o dai partiti, o ha il voto oltre `turno + 6`;
    /// l'abrogativo non è richiesto, o senza firme ha un voto diverso da quello chiesto nel
    /// `turno`.
    pub(crate) fn controlla(
        &self,
        dati: &DatiGioco,
        n_partiti: usize,
        turno: u32,
        totali: (u32, u32),
    ) -> Result<(), Errore> {
        let ko = |m: &str| Err(Errore::SalvataggioIlleggibile(format!("parlamento: {m}")));
        // Il tetto di un ramo: il gradino più alto del catalogo, da cui escono i totali eletti.
        let tetto = |leva: &str| {
            (dati.disposizioni().iter())
                .flat_map(|d| &d.gradini)
                .filter_map(|g| g.leve.get(leva))
                .fold(0.0, |m: f64, &x| m.max(x))
                .round() as u64
        };
        for (seggi, totale, leva) in [
            (&self.seggi_camera, totali.0, "seggi_camera"),
            (&self.seggi_senato, totali.1, "seggi_senato"),
        ] {
            // Somma in u64 uguale a un totale u32 entro il tetto: i `sum::<u32>()` a valle non
            // traboccano, e il voto seggio per seggio resta breve.
            let somma: u64 = seggi.iter().map(|&n| u64::from(n)).sum();
            if seggi.len() != n_partiti || somma != u64::from(totale) || somma > tetto(leva) {
                return ko(
                    "seggi non uno per partito, diversi dai seggi del ramo eletto o oltre i seggi \
                     del catalogo",
                );
            }
        }
        let catalogo = dati.disposizioni();
        let valore_ok =
            |&(d, g): &(usize, usize)| catalogo.get(d).is_some_and(|x| g < x.gradini.len());
        let emendamento_ok = |e: &RichiestaEmendamento| {
            e.partito < n_partiti
                && e.proposta.iter().all(|&(d, x)| match x {
                    Attenuazione::Gradino(g) => valore_ok(&(d, g)),
                    Attenuazione::Rinvio => d < catalogo.len(),
                    Attenuazione::Indicizza => d < dati.dicasteri().len(),
                })
        };
        let misure_ok = self.misure.iter().all(|m| {
            m.valori.iter().all(valore_ok)
                && m.proponente.is_none_or(|i| i < dati.dicasteri().len())
                && m.promotore.is_none_or(|j| j < n_partiti)
                && m.emendamenti.iter().all(emendamento_ok)
                && m.fondi.is_none_or(|f| f.valido(dati))
        });
        let sfiducia_ok = self
            .sfiducia
            .is_none_or(|(d, j)| d.is_none_or(|i| i < dati.dicasteri().len()) && j < n_partiti);
        if !self.decaduti.iter().all(|f| f.valido(dati)) {
            return ko("un decreto decaduto fuori dai dicasteri, dagli atti o da 0-1e15");
        }
        // L'abrogativo si vota sempre; senza firme è chiesto nel turno.
        let firme_ok = (self.referendum.iter())
            .all(|r| r.richiesto && (r.firme || Some(r.voto) == voto_abrogativo(turno)));
        let referendum_ok = (self.referendum.iter().chain(&self.confermativi)).all(|r| {
            r.valori.iter().all(valore_ok)
                && r.promotore.is_none_or(|j| j < n_partiti)
                && r.voto <= turno.saturating_add(6)
        });
        if !misure_ok || self.respinte.iter().any(|&(d, _)| d >= catalogo.len()) {
            return ko("una misura o una respinta fuori dalle disposizioni o dai dicasteri");
        }
        if !sfiducia_ok {
            return ko("la sfiducia fuori dai dicasteri o dai partiti");
        }
        if !referendum_ok {
            return ko("un referendum fuori dalle disposizioni, dai gradini o dai partiti");
        }
        if !firme_ok {
            return ko("l'abrogativo non richiesto, o senza firme e col voto fuori dal turno");
        }
        Ok(())
    }
}
