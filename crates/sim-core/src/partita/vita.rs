//! La vita dei partiti IA: il partito di protesta (scelta 6 del piano 8b, vita-dei-partiti:111-116),
//! la scissione con la minaccia e la frangia di governo che decide se uscire (scelte 7-8,
//! vita-dei-partiti:46-67, :117-119) e lo scioglimento dopo due elezioni senza seggi (scelta 9,
//! vita-dei-partiti:120).

use super::Partita;
use crate::elettorato::{PUNTI, softmax, utilita};
use crate::legislatura::{Incarico, partito_nuovo, valore_al_governo};
use crate::partiti::{MASSIMO_PARTITI, Regola, Vocazione};

/// Elezioni di fila senza seggi dopo le quali un partito IA si scioglie.
pub(super) const ELEZIONI_SENZA_SEGGI: u8 = 2;

/// `x` con un decimale e la virgola.
fn virgola(x: f64) -> String {
    format!("{x:.1}").replace('.', ",")
}

/// L'indice (0 = punto 1) del punto dell'asse dove nasce il partito di protesta: fra i punti con la
/// quota di elettori almeno `soglia` (vita-dei-partiti:111), a `distanza` almeno da ogni dichiarata
/// attiva (:112) e con l'astensione sopra la `media` (:113), quello con la quota più alta; a parità
/// il più basso.
pub(crate) fn punto_di_protesta(
    quote: &[f64; PUNTI],
    astensione: &[f64; PUNTI],
    media: f64,
    dichiarate: &[f64],
    soglia: f64,
    distanza: f64,
) -> Option<usize> {
    let lontano = |k: usize| (dichiarate.iter()).all(|d| ((k + 1) as f64 - d).abs() >= distanza);
    (quote.iter().zip(astensione).enumerate())
        .filter(|&(k, (&q, &a))| q >= soglia && a > media && lontano(k))
        .min_by(|(_, (a, _)), (_, (b, _))| b.total_cmp(a))
        .map(|(k, _)| k)
}

impl Partita {
    /// Nell'apertura della campagna, dopo i movimenti: al più una a legislatura e con meno di
    /// `MASSIMO_PARTITI` attivi, nasce il partito di protesta sul `punto_di_protesta` dello stato
    /// vero. Quota del punto `k`: Σ elettori·h_k / Σ elettori; astensione: la media della
    /// probabilità d'astenersi al punto `k` (utilità senza fedeltà) pesata con elettori·h_k; media:
    /// Σ_k quota_k·astensione_k, sulla stessa base senza fedeltà (Ruling R11: la media delle
    /// intenzioni vere, con la fedeltà, sta sistematicamente più in alto). Hunter, identitario,
    /// come il partito fondato (valence con la luna di miele, cassa `dote_partito_nuovo`); nessun
    /// nome libero, nessuna nascita.
    pub(super) fn protesta(&mut self) {
        if self.legislatura.protesta || self.attivi() >= MASSIMO_PARTITI {
            return;
        }
        let d = self.dati.elettorato();
        let (mut elettori, mut quote, mut astenuti) = (0.0, [0.0; PUNTI], [0.0; PUNTI]);
        for (i, g) in self.elettorato.gruppi.iter().enumerate() {
            let w = self.elettorato.elettori(d, i);
            elettori += w;
            for (k, h) in g.posizione.iter().enumerate() {
                let x = (k + 1) as f64;
                let u = utilita(
                    &self.dati,
                    &self.elettorato,
                    &self.partiti,
                    &self.governo,
                    i,
                    x,
                );
                let astensione = softmax(&u).last().copied().unwrap_or(0.0);
                quote[k] += w * h;
                astenuti[k] += w * h * astensione;
            }
        }
        let astensione: [f64; PUNTI] = std::array::from_fn(|k| {
            if quote[k] > 0.0 {
                astenuti[k] / quote[k]
            } else {
                0.0
            }
        });
        quote.iter_mut().for_each(|q| *q /= elettori);
        let media: f64 = quote.iter().zip(&astensione).map(|(q, a)| q * a).sum();
        let dichiarate: Vec<f64> = (self.partiti.iter())
            .filter(|p| !p.sciolto)
            .map(|p| p.posizione_dichiarata)
            .collect();
        let (soglia, distanza) = (
            self.dati.manopola("soglia_protesta"),
            self.dati.manopola("distanza_protesta"),
        );
        let Some(k) = punto_di_protesta(&quote, &astensione, media, &dichiarate, soglia, distanza)
        else {
            return;
        };
        let gia: Vec<String> = self.partiti.iter().map(|p| p.nome.clone()).collect();
        let lista = self.dati.scenario().nomi_partiti.clone();
        let Some(nome) = self.nome_nuovo(&gia, &lista) else {
            return;
        };
        let mut p = partito_nuovo(&self.dati, &nome, (k + 1) as f64);
        p.leader = self.leader_nuovo();
        p.vocazione = Vocazione::Identitaria;
        p.regola = Regola::Hunter;
        let j = self.aggiungi_partito(p);
        self.legislatura.protesta = true;
        let testo = format!(
            "Nasce {nome}, partito di protesta in posizione {}: elettori lontani dai partiti e \
             astensione alta",
            k + 1
        );
        self.mossa(j, testo);
    }

    /// In chiusura, dopo l'aula (scelta 7 del piano 8b), per i partiti attivi in ordine d'indice,
    /// giocatore compreso, sullo stato vero: con la percepita a `soglia_scissione` dalla dichiarata
    /// e meno di `MASSIMO_PARTITI` attivi nasce la frangia (`scissione`); a `minaccia_scissione`,
    /// o al tetto, una riga di minaccia, una sola finché la distanza non torna sotto.
    pub(super) fn scissioni(&mut self) {
        let m = |nome: &str| self.dati.manopola(nome);
        let (minaccia, soglia) = (m("minaccia_scissione"), m("soglia_scissione"));
        for j in 0..self.partiti.len() {
            let Some(p) = self.partiti.get(j).filter(|p| !p.sciolto) else {
                continue;
            };
            let d = (p.posizione_percepita - p.posizione_dichiarata).abs();
            let gia = p.minaccia;
            if d >= soglia && self.attivi() < MASSIMO_PARTITI && self.scissione(j) {
                continue;
            }
            if let Some(p) = self.partiti.get_mut(j) {
                p.minaccia = d >= minaccia;
            }
            if d >= minaccia && !gia {
                let testo = format!(
                    "Minaccia di scissione: la posizione percepita dista {} dalla dichiarata",
                    virgola(d)
                );
                self.mossa(j, testo);
            }
        }
    }

    /// La frangia del partito `j`: nome e leader nuovi (`Flusso::PartitiIa`), sulla percepita di
    /// `j`, con vocazione, regola, `valence_base` e giudizio di `j` e la luna di miele del leader
    /// nuovo; prende `quota_scissione` dei seggi di Camera e Senato e della cassa (arrotondati) e
    /// del voto precedente di ogni gruppo, e l'intesa di `j`. Se `j` è al governo la frangia entra
    /// nella coalizione, senza dicasteri. `j` riparte con la percepita sulla dichiarata. Senza un
    /// nome libero non nasce: `false`.
    fn scissione(&mut self, j: usize) -> bool {
        let gia: Vec<String> = self.partiti.iter().map(|p| p.nome.clone()).collect();
        let lista = self.dati.scenario().nomi_partiti.clone();
        let Some(nome) = self.nome_nuovo(&gia, &lista) else {
            return false;
        };
        let leader = self.leader_nuovo();
        let (q, luna) = (
            self.dati.manopola("quota_scissione"),
            self.dati.manopola("luna_di_miele_leader"),
        );
        let Some(o) = self.partiti.get_mut(j) else {
            return false;
        };
        let x = o.posizione_percepita;
        let cassa = (o.cassa as f64 * q).round() as i64;
        o.cassa -= cassa;
        o.posizione_percepita = o.posizione_dichiarata;
        o.minaccia = false;
        let mut f = o.clone();
        f.nome.clone_from(&nome);
        f.leader = leader;
        f.posizione_dichiarata = x;
        f.posizione_percepita = x;
        f.valence = f.valence_base + luna;
        f.cassa = cassa;
        f.senza_seggi = 0;
        f.ultimo_passo = 0.0;
        let k = self.aggiungi_partito(f);
        let pl = &mut self.parlamento;
        let mut presi = [0; 2];
        for (v, preso) in [&mut pl.seggi_camera, &mut pl.seggi_senato]
            .into_iter()
            .zip(&mut presi)
        {
            let s = v.get(j).map_or(0, |&n| (f64::from(n) * q).round() as u32);
            if let Some(n) = v.get_mut(j) {
                *n -= s;
            }
            if let Some(n) = v.get_mut(k) {
                *n = s;
            }
            *preso = s;
        }
        for g in &mut self.elettorato.gruppi {
            let v = g.voto_precedente.get(j).map_or(0.0, |v| v * q);
            let giudizio = g.giudizio.get(j).copied().unwrap_or(0.0);
            if let Some(x) = g.voto_precedente.get_mut(j) {
                *x -= v;
            }
            if let Some(x) = g.voto_precedente.get_mut(k) {
                *x = v;
            }
            if let Some(x) = g.giudizio.get_mut(k) {
                *x = giudizio;
            }
        }
        if let Some(&intesa) = self.intesa.valori.get(j)
            && let Some(x) = self.intesa.valori.get_mut(k)
        {
            *x = intesa;
        }
        if self.governo.al_governo(j) {
            self.governo.coalizione.push(k);
        }
        self.legislatura.frange.push(k);
        let [c, s] = presi;
        let testo = format!(
            "Scissione: nasce {nome} con {c} deputati e {s} senatori, sulla posizione {}",
            virgola(x)
        );
        self.mossa(j, testo);
        true
    }

    /// In chiusura, nella finestra delle uscite degli alleati e fuori dalla crisi (scelta 8 del
    /// piano 8b): le frange della legislatura al governo, Presidente del Consiglio escluso, per cui
    /// il voto vale più che restare, col confronto di `voto_anticipato` (`Quadro::valore` sul
    /// sondaggio pubblico contro `valore_al_governo` coi seggi veri, senza la guida, più
    /// `margine_voto_anticipato`). Una riga per frangia; chi esce passa da `uscite`.
    pub(super) fn frange_in_uscita(&mut self) -> Vec<usize> {
        if self.legislatura.crisi.is_some() {
            return Vec::new();
        }
        let margine = self.dati.manopola("margine_voto_anticipato");
        let d = self.dichiarazione_corrente();
        let (q, veri) = (self.quadro(), self.quadro_veri());
        let g = &self.governo;
        let escono: Vec<usize> = (self.legislatura.frange.iter().copied())
            .filter(|&j| j != g.presidente && g.coalizione.contains(&j))
            .filter(|&j| {
                q.valore(&d, j) > valore_al_governo(&veri, j, &g.coalizione, false) + margine
            })
            .collect();
        for &j in &escono {
            let testo = "Esce dalla maggioranza: nei sondaggi il voto gli vale più che restare al \
                         governo";
            self.mossa(j, testo.into());
        }
        escono
    }

    /// In coda al voto: un partito IA attivo senza seggi in entrambe le Camere conta un'elezione in
    /// più, altrimenti il conto riparte; a `ELEZIONI_SENZA_SEGGI` si scioglie, con la sua riga. Lo
    /// sciolto esce dalle coalizioni dell'ultimo voto (quella che resta di un partito solo cade) e
    /// dall'incarico IA. Il giocatore no: senza seggi la partita finisce già.
    pub(super) fn scioglimenti(&mut self) {
        let (camera, senato) = (&self.parlamento.seggi_camera, &self.parlamento.seggi_senato);
        let vuoto = |v: &[u32], j: usize| v.get(j).is_none_or(|&x| x == 0);
        let senza: Vec<bool> = (0..self.partiti.len())
            .map(|j| vuoto(camera, j) && vuoto(senato, j))
            .collect();
        let mut sciolti = Vec::new();
        for (j, p) in self.partiti.iter_mut().enumerate() {
            if p.sciolto || j == self.giocatore {
                continue;
            }
            p.senza_seggi = if senza[j] {
                p.senza_seggi.saturating_add(1)
            } else {
                0
            };
            if p.senza_seggi >= ELEZIONI_SENZA_SEGGI {
                p.sciolto = true;
                sciolti.push(j);
            }
        }
        for &j in &sciolti {
            let testo = format!("Si scioglie dopo {ELEZIONI_SENZA_SEGGI} elezioni senza seggi");
            self.mossa(j, testo);
        }
        let l = &mut self.legislatura;
        // Una coalizione ridotta a un solo membro dallo scioglimento cade.
        l.coalizioni.retain_mut(|c| {
            let prima = c.len();
            c.retain(|j| !sciolti.contains(j));
            c.len() == prima || c.len() >= 2
        });
        if let Some(Incarico::Ia { coalizione }) = &mut l.incarico {
            coalizione.retain(|j| !sciolti.contains(j));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dati::dati_base;

    // Punti 1-7; partiti a 1 e 4. Scoperti, con l'astensione sopra 0,3 e la quota da 0,1: 2, 3,
    // 5, 6, 7; vince il 7 (quota 0,2).
    const QUOTE: [f64; 7] = [0.05, 0.12, 0.15, 0.2, 0.1, 0.18, 0.2];
    const ASTENSIONE: [f64; 7] = [0.5, 0.4, 0.4, 0.2, 0.4, 0.4, 0.4];
    const DICHIARATE: [f64; 2] = [1.0, 4.0];

    fn punto(q: &[f64; 7], a: &[f64; 7], d: &[f64]) -> Option<usize> {
        punto_di_protesta(q, a, 0.3, d, 0.1, 1.0)
    }

    #[test]
    fn punto_di_protesta_sceglie_lo_scoperto_con_la_quota_piu_alta() {
        assert_eq!(punto(&QUOTE, &ASTENSIONE, &DICHIARATE), Some(6));
    }

    #[test]
    fn punto_di_protesta_a_parita_il_piu_basso() {
        let mut q = QUOTE;
        q[5] = 0.2;
        assert_eq!(punto(&q, &ASTENSIONE, &DICHIARATE), Some(5));
    }

    #[test]
    fn punto_di_protesta_scarta_astensione_non_sopra_la_media() {
        let mut a = ASTENSIONE;
        a[6] = 0.3; // uguale alla media: non la supera
        assert_eq!(punto(&QUOTE, &a, &DICHIARATE), Some(5));
    }

    #[test]
    fn punto_di_protesta_scarta_un_punto_a_mezzo_punto_da_un_partito() {
        // Un partito a 6,5: il 6 e il 7 distano 0,5; resta il 3.
        assert_eq!(punto(&QUOTE, &ASTENSIONE, &[1.0, 4.0, 6.5]), Some(2));
    }

    #[test]
    fn punto_di_protesta_scarta_la_quota_sotto_soglia() {
        let mut q = QUOTE;
        q[6] = 0.09;
        assert_eq!(punto(&q, &ASTENSIONE, &DICHIARATE), Some(5));
    }

    #[test]
    fn punto_di_protesta_nessuno_senza_candidati() {
        assert_eq!(punto(&QUOTE, &[0.2; 7], &DICHIARATE), None);
        let tutti = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0];
        assert_eq!(punto(&QUOTE, &ASTENSIONE, &tutti), None);
    }

    /// La partita di prova con la destra a 4,5, l'estrema destra a 5 e la soglia all'1%: il punto 7
    /// è scoperto.
    fn scoperta() -> Partita {
        let mut dati = dati_base();
        dati.imposta_manopola("soglia_protesta", 0.01);
        let mut p = Partita::nuova(dati, 1);
        for (j, x) in [(5, 4.5), (6, 5.0)] {
            p.partiti[j].posizione_dichiarata = x;
            p.partiti[j].posizione_percepita = x;
        }
        p
    }

    /// Senza tetto la protesta nasce; con tre partiti aggiunti (10 attivi) no.
    #[test]
    fn al_tetto_nessuna_nascita() {
        let mut senza_tetto = scoperta();
        senza_tetto.protesta();
        assert_eq!(senza_tetto.partiti.len(), 8);
        let mut p = scoperta();
        for _ in 0..3 {
            p.aggiungi_partito(p.partiti[2].clone());
        }
        assert_eq!(p.attivi(), MASSIMO_PARTITI);
        p.protesta();
        assert_eq!(p.partiti.len(), 10);
        assert!(!p.legislatura.protesta);
    }

    /// Il centrodestra a un punto: la frangia prende il 12% di cassa e voto precedente, arrotondati
    /// i soli euro, e copia giudizio e intesa; le somme restano.
    #[test]
    fn la_scissione_divide_cassa_e_voto_precedente() {
        let mut p = Partita::nuova(dati_base(), 1);
        p.partiti[4].cassa = 1_000_001;
        p.partiti[4].posizione_percepita += 1.0;
        p.intesa.valori[4] = 55.0;
        let prima: Vec<(f64, f64)> = (p.elettorato.gruppi.iter())
            .map(|g| (g.voto_precedente[4], g.giudizio[4]))
            .collect();
        p.scissioni();
        assert_eq!(p.partiti.len(), 8);
        assert_eq!((p.partiti[4].cassa, p.partiti[7].cassa), (880_001, 120_000));
        assert_eq!(p.intesa.valori[7], 55.0);
        assert_eq!(p.intesa.ultimatum[7], None);
        for (g, &(v, giudizio)) in p.elettorato.gruppi.iter().zip(&prima) {
            assert_eq!(g.voto_precedente.len(), 9);
            assert!((g.voto_precedente[7] - 0.12 * v).abs() < 1e-12);
            assert!((g.voto_precedente[4] + g.voto_precedente[7] - v).abs() < 1e-12);
            assert_eq!(g.giudizio[7], giudizio);
        }
    }

    /// Con 10 attivi la scissione resta minaccia, con la sua riga.
    #[test]
    fn al_tetto_la_scissione_resta_minaccia() {
        let mut p = Partita::nuova(dati_base(), 1);
        for _ in 0..3 {
            p.aggiungi_partito(p.partiti[2].clone());
        }
        p.partiti[4].posizione_percepita += 1.0;
        p.scissioni();
        assert_eq!(p.partiti.len(), 10);
        assert!(p.partiti[4].minaccia);
        assert_eq!(p.mosse.len(), 1);
        assert!(p.mosse[0].testo.starts_with("Minaccia di scissione"));
    }
}
