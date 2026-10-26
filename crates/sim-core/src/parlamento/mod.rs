//! Il parlamento in carica: seggi per partito, misure in calendario, esiti dei voti.

use crate::dati::DatiGioco;
use crate::disposizioni::{DatoDisposizione, Disposizioni, Norma, data_di};
use crate::errore::Errore;
use crate::ministri::{Causa, Ministri, penalizza};
use crate::partita::Trimestre;
use crate::partiti::{Governo, Partito};
use crate::scenario::Scenario;
use crate::vista::{Forbice, VoceMisura};
use serde::{Deserialize, Serialize};
#[cfg(test)]
use ts_rs::TS;

mod emendamenti;
mod intesa;
pub mod voto;

pub use emendamenti::{Attenuazione, RichiestaEmendamento, StatoEmendamento};
pub(crate) use emendamenti::{costo_fiducia, rinviata};
pub use intesa::{CausaIntesa, Intesa, VariazioneIntesa};

/// Posti del calendario per riforme ed elettorali.
pub const POSTI_ORDINARI: usize = 3;
pub const POSTI_COSTITUZIONALI: usize = 1;
/// Il posto dell'opposizione (iter-legislativo-italiano:151, un argomento su cinque): resta vuoto.
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
}

/// Una misura in calendario. `valori` = disposizione e gradino; `proponente` = dicastero;
/// `voto` = turno nella cui chiusura si vota; `deliberazione` 1 o 2 per le costituzionali,
/// 0 altrimenti.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Misura {
    pub id: u32,
    pub forma: FormaMisura,
    pub valori: Vec<(usize, usize)>,
    pub proponente: Option<usize>,
    pub presentata: u32,
    pub voto: u32,
    pub deliberazione: u8,
    pub fiducia: bool,
    pub emendamenti: Vec<RichiestaEmendamento>,
    pub emendamenti_aperti: bool,
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
}

/// Un valore di una legge presentata: id della disposizione e del gradino.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct ValoreProposto {
    pub disposizione: String,
    pub gradino: String,
}

/// Seggi per partito nell'ordine dello scenario. `respinte` = disposizione e turno da cui si
/// ripresenta; `prossima` = id della prossima misura.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Parlamento {
    pub seggi_camera: Vec<u32>,
    pub seggi_senato: Vec<u32>,
    pub misure: Vec<Misura>,
    pub respinte: Vec<(usize, u32)>,
    pub prossima: u32,
    pub esiti: Vec<EsitoVoto>,
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
/// `Partiti` col nome del ramo e del partito sconosciuto o coi seggi negativi, o con la somma
/// diversa dai seggi di partenza.
pub(crate) fn valida_uscente(
    scenario: &Scenario,
    disposizioni: &[DatoDisposizione],
) -> Result<(), Errore> {
    let ko = |m: String| Err(Errore::Partiti(format!("parlamento_uscente.{m}")));
    let Some((camera, senato)) = seggi_partenza(disposizioni) else {
        return ko("numero_parlamentari: manca coi seggi_camera e seggi_senato di partenza".into());
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

/// `gennaio-marzo 2027`.
fn mesi(t: Trimestre) -> String {
    let m = match t.trimestre {
        1 => "gennaio-marzo",
        2 => "aprile-giugno",
        3 => "luglio-settembre",
        _ => "ottobre-dicembre",
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
    pub partiti: &'a [Partito],
    pub governo: &'a Governo,
    pub intesa: &'a [f64],
}

/// Posizioni, linea e probabilità dei franchi tiratori di una misura, sullo stato attuale.
pub(crate) struct Previsione {
    pub posizioni: Option<(f64, f64)>,
    pub linea: Vec<bool>,
    pub franchi: Vec<f64>,
}

impl StatoAula<'_> {
    /// Sotto fiducia la linea a blocchi e nessun franco tiratore; altrimenti l'alleato con
    /// l'emendamento accolto vota sì con `ft_base`, quello col respinto ha i franchi tiratori
    /// moltiplicati per `1 + ft_emendamento_respinto`, entro `ft_max`.
    pub(crate) fn previsione(&self, m: &Misura) -> Previsione {
        let posizioni = posizioni(self.dati, self.disposizioni, &m.valori);
        let (dati, partiti, governo, intesa) = (self.dati, self.partiti, self.governo, self.intesa);
        let nuova = posizioni.map(|(_, n)| n);
        if m.fiducia {
            let linea = voto::linea_fiducia(dati, partiti, governo, intesa, nuova);
            return Previsione {
                posizioni,
                franchi: vec![0.0; linea.len()],
                linea,
            };
        }
        let mut linea = voto::linea(dati, partiti, governo, intesa, posizioni);
        let accolti = (m.emendamenti.iter()).filter(|e| e.stato == StatoEmendamento::Accolto);
        for e in accolti {
            if let Some(si) = linea.get_mut(e.partito) {
                *si = true;
            }
        }
        let mut franchi = voto::probabilita_franco(dati, partiti, governo, intesa, nuova, &linea);
        let (base, tetto) = (dati.manopola("ft_base"), dati.manopola("ft_max"));
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
        Previsione {
            posizioni,
            linea,
            franchi,
        }
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

    /// `PresentaLegge`: controlla, mette la misura in calendario e fa pagare al ministro
    /// proponente la firma contro la sua posizione. Un rifiuto non tocca nulla.
    ///
    /// # Errors
    /// `Comando`, nell'ordine: legge vuota, disposizione sconosciuta, ripetuta, gradino
    /// sconosciuto, norme di forma diversa, dicasteri diversi, gradino già in vigore, bloccato
    /// dal referendum, disposizione già in esame, respinta da poco, calendario pieno.
    pub(crate) fn presenta(
        &mut self,
        dati: &DatiGioco,
        disposizioni: &Disposizioni,
        ministri: &mut Ministri,
        turno: u32,
        valori: &[ValoreProposto],
    ) -> Result<(), Errore> {
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
                return ko(format!("disposizione ripetuta: {}", v.disposizione));
            }
            indici.push(d);
        }
        let mut scelti: Vec<(usize, usize)> = Vec::with_capacity(valori.len());
        for (v, &d) in valori.iter().zip(&indici) {
            let Some(g) = catalogo[d].gradino(&v.gradino) else {
                return ko(format!(
                    "gradino sconosciuto: {} {}",
                    v.disposizione, v.gradino
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
                "disposizioni di dicasteri diversi: {}",
                catalogo[d].id
            ));
        }
        for &(d, g) in &scelti {
            let (id, stato) = (&catalogo[d].id, &disposizioni.voci[d]);
            if stato.in_vigore == g {
                return ko(format!("{id} è già a {}", catalogo[d].gradini[g].nome));
            }
            if stato.bloccati.contains(&g) {
                let g = &catalogo[d].gradini[g].id;
                return ko(format!("gradino bloccato dal referendum: {id} {g}"));
            }
            if (self.misure.iter()).any(|m| m.valori.iter().any(|&(x, _)| x == d)) {
                return ko(format!("{id} è già in esame"));
            }
            if let Some(&(_, da)) = (self.respinte.iter()).find(|&&(x, da)| x == d && da > turno) {
                return ko(format!(
                    "{id} respinta: si ripresenta da {}",
                    mesi(data_di(da))
                ));
            }
        }
        let costituzionale = forma == FormaMisura::Costituzionale;
        let (posti, occupati) = if costituzionale {
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
            .filter(|m| occupati.contains(&m.forma))
            .count()
            >= posti
        {
            return ko("calendario pieno".into());
        }
        let proponente =
            (dicastero.as_ref()).and_then(|id| dati.dicasteri().iter().position(|x| &x.id == id));
        if let Some(st) = proponente.and_then(|i| ministri.dicasteri.get_mut(i)) {
            let m = st.ministro.as_ref().map(|m| m.posizione);
            if let (Some(m), Some((vigore, nuova))) = (m, posizioni(dati, disposizioni, &scelti)) {
                let lontananza = ((nuova - m).abs() - (vigore - m).abs()).max(0.0);
                let delta = -dati.manopola("costo_firma") * lontananza;
                penalizza(st, Causa::NormaControPosizione, delta, turno);
            }
        }
        self.misure.push(Misura {
            id: self.prossima,
            forma,
            valori: scelti,
            proponente,
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
        });
        self.prossima = self.prossima.saturating_add(1);
        Ok(())
    }

    /// Le misure in calendario per la `Vista`: linee e forbice, nessun franco tiratore estratto.
    pub(crate) fn voci_calendario(&self, aula: &StatoAula) -> Vec<VoceMisura> {
        let dati = aula.dati;
        let catalogo = dati.disposizioni();
        let maggioranza_camera = voto::maggioranza(self.seggi_camera.iter().sum());
        let maggioranza_senato = voto::maggioranza(self.seggi_senato.iter().sum());
        (self.misure.iter())
            .map(|m| {
                let p = aula.previsione(m);
                let nuova = p.posizioni.map(|(_, n)| n);
                // Senza fiducia, l'intesa dopo il suo costo; il resto sullo stato attuale.
                let a_blocchi = if m.fiducia {
                    p.linea.clone()
                } else {
                    let intesa: Vec<f64> = (aula.partiti.iter().zip(aula.intesa))
                        .map(|(x, &i)| {
                            let costo = costo_fiducia(dati, m.forma, x.posizione_dichiarata, nuova);
                            (i - costo).clamp(0.0, 100.0)
                        })
                        .collect();
                    voto::linea_fiducia(dati, aula.partiti, aula.governo, &intesa, nuova)
                };
                let camera = voto::forbice(&self.seggi_camera, &p.linea, &p.franchi);
                let senato = voto::forbice(&self.seggi_senato, &p.linea, &p.franchi);
                VoceMisura {
                    id: m.id,
                    nome: nome(dati, &m.valori),
                    forma: m.forma,
                    proponente: (m.proponente)
                        .and_then(|i| dati.dicasteri().get(i))
                        .map(|d| d.id.clone()),
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
                    forbice: Forbice {
                        camera,
                        senato,
                        maggioranza_camera,
                        maggioranza_senato,
                        in_bilico: camera[0] < maggioranza_camera || senato[0] < maggioranza_senato,
                    },
                    emendamenti: emendamenti::voci(dati, aula.partiti, &m.emendamenti),
                    alleati_contro_fiducia: (aula.partiti.iter().zip(a_blocchi).enumerate())
                        .filter(|&(j, (_, si))| !si && voto::alleato(aula.governo, j))
                        .map(|(_, (p, _))| p.id.clone())
                        .collect(),
                }
            })
            .collect()
    }

    /// Apertura del turno `turno`: restano gli esiti dei due turni prima.
    pub(crate) fn apri(&mut self, turno: u32) {
        let da = data_di(turno.saturating_sub(2));
        (self.esiti).retain(|e| (e.data.anno, e.data.trimestre) >= (da.anno, da.trimestre));
    }

    /// # Errors
    /// `SalvataggioIlleggibile` se i seggi non sono uno per partito o non sommano ai seggi dei
    /// rami, o una misura, un suo emendamento o una respinta esce dalle disposizioni, dai gradini,
    /// dai dicasteri o dai partiti.
    pub(crate) fn controlla(&self, dati: &DatiGioco, n_partiti: usize) -> Result<(), Errore> {
        let ko = |m: &str| Err(Errore::SalvataggioIlleggibile(format!("parlamento: {m}")));
        let (camera, senato) = seggi_partenza(dati.disposizioni()).unwrap_or_default();
        for (seggi, totale) in [(&self.seggi_camera, camera), (&self.seggi_senato, senato)] {
            if seggi.len() != n_partiti || f64::from(seggi.iter().sum::<u32>()) != totale {
                return ko("seggi non uno per partito o diversi dai seggi del ramo");
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
                })
        };
        let misure_ok = self.misure.iter().all(|m| {
            m.valori.iter().all(valore_ok)
                && m.proponente.is_none_or(|i| i < dati.dicasteri().len())
                && m.emendamenti.iter().all(emendamento_ok)
        });
        if !misure_ok || self.respinte.iter().any(|&(d, _)| d >= catalogo.len()) {
            return ko("una misura o una respinta fuori dalle disposizioni o dai dicasteri");
        }
        Ok(())
    }
}
