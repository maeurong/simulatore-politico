"""Elettorato: 20 regioni x 6 condizioni x 3 fasce di reddito.

Uso: python3 -I strumenti/taratura/elettorato.py [--senza-ibf] [--cache DIR] [--ess FILE]
     python3 -I strumenti/taratura/elettorato.py --selftest

Le quote dei gruppi vengono da un IPF sui margini reali: elettori della Camera 2022, ISTAT (occupati,
disoccupati, inattivi, quinti di reddito), MEF (pensionati, classi di reddito), seme IBF 2022
(--senza-ibf: seme d'indipendenza). Scarica nella cache (fuori dal repo) e scrive dati/base/elettorato.toml.
Le risposte e gli archivi scaricati sono dati: si leggono, non si eseguono. I microdati IBF restano nella cache.
Le posizioni vengono da un IPF a 4 dimensioni sui margini ESS round 11 (--ess FILE, CSV del Data Portal; senza, provvisorie),
i redditi da lavoro dai margini MEF. Il resto (panieri, interessi) è ancora provvisorio: strumenti/taratura/README.md.
"""

import collections
import csv
import datetime
import hashlib
import importlib.util
import io
import json
import math
import pathlib
import random
import sys
import time
import tomllib
import zipfile

BASE = pathlib.Path(__file__).resolve().parents[2] / "dati" / "base"
COMANDO = "python3 -I strumenti/taratura/elettorato.py"
OGGI = datetime.date.today().isoformat()
if "--cache" in sys.argv and sys.argv.index("--cache") + 1 == len(sys.argv):
    sys.exit("--cache vuole una cartella: --cache DIR")
if "--ess" in sys.argv and sys.argv.index("--ess") + 1 == len(sys.argv):
    sys.exit("--ess vuole un file: --ess FILE")
ESS = pathlib.Path(sys.argv[sys.argv.index("--ess") + 1]) if "--ess" in sys.argv else None
CACHE = pathlib.Path(sys.argv[sys.argv.index("--cache") + 1] if "--cache" in sys.argv else pathlib.Path.home() / ".cache" / "simulatore-politico-taratura")

URL_CAMERA = "https://documenti.camera.it/leg19/risultatiElettorali2022/UGE/VotiListe/ItaliaVotiListeCIRC.csv"
URL_IBF = "https://www.bancaditalia.it/statistiche/tematiche/indagini-famiglie-imprese/bilanci-famiglie/distribuzione-microdati/documenti/ind22_ascii.zip"
URL_MEF = "https://www1.finanze.gov.it/finanze/analisi_stat/public/v_4_0_0/contenuti/REG_tipo_reddito_2025.csv"
# sha256 dei file senza versione (docs/ricerca/dati/risultati-elettorali-italia.md:13, banca-italia-ibf.md:14, mef-statistiche-fiscali.md); ISTAT cambia a ogni edizione
SHA256 = {
    "camera/ItaliaVotiListeCIRC.csv": "bbda02a9c9b692c23aceaad38857052df1ad989fe182b231b679a86fb8f77f3f",
    "ibf/ind22_ascii.zip": "fd31bb4263363b9098de6f0d0640a90484dc30d0d4fd0c5323706d61dc9fb675",
    "mef/REG_tipo_reddito_2025.csv": "7a31be66b19c0932d26675e8e948cc413ff8ed807e214b80c03fb7c406d50643",
}
SDMX = "application/vnd.sdmx.data+csv;version=1.0.0"
ANNO_LAVORO, ANNO_QUINTI = 2025, 2024
# assunzione: l'ordine di IREG (codebook dei file annuali non trovato) è quello ISTAT; il gradiente del reddito equivalente per IREG lo conferma
IBF_IREG = ["piemonte", "valle_d_aosta", "lombardia", "trentino_alto_adige", "veneto", "friuli_venezia_giulia", "liguria", "emilia_romagna",
            "toscana", "umbria", "marche", "lazio", "abruzzo", "molise", "campania", "puglia", "basilicata", "calabria", "sicilia", "sardegna"]
TIPI_MEF = {"dipendente": ["Reddito da lavoro dipendente e assimilati"],
            "autonomo": ["Reddito da lavoro autonomo", "Reddito di spettanza dell'imprenditore in contabilita' ordinaria",
                         "Reddito di spettanza dell'imprenditore in contabilita' semplificata"]}
SOGLIE_FASCE = (0.3, 0.7)  # decili 1-3 bassa, 4-7 media, 8-10 alta (dati-per-tarare-i-gruppi.md:66)
VUOTE_IBF = 1e-3  # celle senza osservazioni: questa frazione del prodotto dei margini
CONDIZIONI = ["dipendente_privato", "dipendente_pubblico", "autonomo", "pensionato", "disoccupato_inattivo", "studente"]
FASCE = ["bassa", "media", "alta"]
SHARE_PUBBLICO = 0.17  # assunzione: RGS non aperto
SHARE_STUDENTI = 0.06  # assunzione
ISTOGRAMMA = [0.05, 0.12, 0.17, 0.25, 0.16, 0.17, 0.08]  # segnaposto senza --ess
TAGLIO = (0, 0, 1, 1, 2, 3, 4, 5, 5, 6, 6)  # autocollocazione 0-10 -> 7 punti (dati-per-tarare-i-gruppi.md:80-95)
MIN_CASI_REGIONE = 100  # sotto, la regione prende la distribuzione della macroarea
LISCIA = 5.0  # assunzione: pseudo-casi con la distribuzione nazionale, perché nessuna cella ESS resti vuota
PRIMARIO = {"disoccupato_inattivo": 4000.0, "studente": 3000.0}  # provvisorio: nessun tipo MEF; pensionato = 0, dipendenti e autonomi dal MEF
MOLT_FASCIA = (0.55, 1.0, 1.9)
MOLT_MACROAREA = {"nord": 1.1, "centro": 1.0, "mezzogiorno": 0.8}
# affluenza 2022, risultati-elettorali-italia.md:16-27; Valle d'Aosta = nazionale 63,6%
AFFLUENZA = {
    "calabria": 0.506, "sardegna": 0.530, "campania": 0.532, "molise": 0.564, "puglia": 0.565,
    "sicilia": 0.569, "basilicata": 0.586, "lazio": 0.634, "abruzzo": 0.637, "liguria": 0.639,
    "trentino_alto_adige": 0.657, "friuli_venezia_giulia": 0.660, "piemonte": 0.661, "marche": 0.682,
    "umbria": 0.685, "toscana": 0.694, "lombardia": 0.698, "veneto": 0.700, "emilia_romagna": 0.716,
    "valle_d_aosta": 0.636,
}
# assunzione del piano 4: tarata a mano sull'affluenza 2022 coi parametri di default, fedeltà inclusa; la PR B (tara) la sostituisce
# (0,57 = ln sum_j e^{-(4-j)^2} per un elettore al centro con beta = 1, senza fedeltà; Δ = -0,492)
COSTANTE_ASTENSIONE = 0.081
# ticket 17, 17-tabella-degli-interessi.md:28-35
INTERESSI = {
    "dipendente_privato": {"reddito": 0.45, "disoccupazione": 0.25, "qualita_trasporti": 0.10, "qualita_servizio_sanitario": 0.10, "sicurezza": 0.10},
    "dipendente_pubblico": {"reddito": 0.50, "occupazione_pubblica": 0.20, "qualita_servizio_sanitario": 0.15, "qualita_scuola": 0.15},
    "autonomo": {"reddito": 0.55, "durata_processi": 0.15, "sostegno_imprese": 0.15, "sicurezza": 0.15},
    "pensionato": {"reddito": 0.50, "qualita_servizio_sanitario": 0.35, "sicurezza": 0.15},
    "disoccupato_inattivo": {"reddito": 0.35, "disoccupazione": 0.40, "copertura_welfare": 0.25},
    "studente": {"reddito": 0.30, "qualita_scuola": 0.35, "disoccupazione": 0.35},
}
ALIQUOTE = {"bassa": 0.15, "media": 0.25, "alta": 0.33}  # assunzione: aliquote medie per fascia
ASTENSIONE_FASCIA = {"bassa": 0.3, "media": 0.0, "alta": -0.2}  # assunzione: la fascia bassa vota meno
ISTITUTI = (("Istituto A", 1000), ("Istituto B", 800), ("Istituto C", 1200), ("Istituto D", 1500))  # segnaposto
NOMI = ["Giulia", "Marco", "Francesca", "Luca", "Chiara", "Andrea", "Sara", "Matteo", "Elena", "Davide", "Anna", "Paolo"]
COGNOMI = ["Rossi", "Esposito", "Bianchi", "Romano", "Colombo", "Ricci", "Marino", "Greco", "Bruno", "Gallo", "Conti", "De Luca"]


def leggi(nome):
    return tomllib.loads((BASE / nome).read_text(encoding="utf-8"))


def quote(valori):
    """Quote che sommano a 1: arrotondate a 6 decimali, l'ultima corretta."""
    tot = sum(valori)
    assert tot > 0 and all(v >= 0 for v in valori), f"quote: somma {tot}, minimo {min(valori)} (attesi somma > 0, minimo >= 0)"
    q = [round(v / tot, 6) for v in valori[:-1]]
    return q + [round(1 - sum(q), 6)]


def prova_ipf():
    """Caso sintetico 2 x 2 x 2: i margini di una tabella nota tornano entro 1e-9."""
    assi3 = ((0, 1), (0, 2), (1, 2))

    def torna(t, margini, dove):
        for assi, m in margini.items():
            assert all(abs(x - m[k]) < 1e-9 for k, x in proiezione(t, assi).items()), f"{dove}: margine {assi} non ritrovato"

    nota = {(0, 0, 0): 3.0, (0, 0, 1): 1.0, (0, 1, 0): 2.0, (0, 1, 1): 4.0, (1, 0, 0): 5.0, (1, 0, 1): 2.0, (1, 1, 0): 1.0, (1, 1, 1): 2.0}
    margini = {assi: proiezione(nota, assi) for assi in assi3}
    torna(ipf(dict.fromkeys(nota, 1.0), margini), margini, "seme uniforme")
    nota[0, 0, 0] = 0.0
    margini = {assi: proiezione(nota, assi) for assi in assi3}
    seme = dict.fromkeys(nota, 1.0)
    seme[0, 0, 0] = 0.0
    t = ipf(seme, margini)
    assert t[0, 0, 0] == 0.0, "cella zero consentita dal margine non rimasta zero"
    torna(t, margini, "cella zero")
    vuoto = dict.fromkeys(nota, 1.0)
    vuoto[0, 0, 0] = vuoto[0, 0, 1] = 0.0
    try:
        ipf(vuoto, margini)
    except AssertionError as e:
        assert "vuole massa" in str(e), f"seme vuoto: messaggio {e}"
        return
    raise AssertionError("seme vuoto dove il margine vuole massa: attesa AssertionError")


def prova_posizioni():
    """Taglio 0-10 -> 7 su tutti gli 11 valori; IPF a 4 dimensioni su ESS sintetico: tutti i margini ritrovati entro 1e-9."""
    assert [TAGLIO[v] for v in range(11)] == [0, 0, 1, 1, 2, 3, 4, 5, 5, 6, 6], "taglio 0-10 -> 7 punti"
    rng = random.Random(1)
    macro = {"a": "nord", "b": "nord", "c": "sud", "d": "sud"}
    N = {(r, c, f): rng.uniform(1, 100) for r in macro for c in CONDIZIONI for f in FASCE}
    # la regione d ha 20 casi (accorpata al sud), le altre 150; alcuni senza fascia
    righe = [(rng.choice(CONDIZIONI), rng.choice(FASCE + [None]), rng.randrange(7), r, 1.0) for r in macro for _ in range(20 if r == "d" else 150)]
    t, margini = posizioni(N, macro, righe)
    assert set(margini) == {(0, 1, 2), (1, 2, 3), (0, 3)}, f"posizioni: margini {sorted(margini)}"
    assert [len(m) for m in margini.values()] == [72, 126, 28], "posizioni: chiavi dei margini attese 72, 126, 28"
    for assi, m in margini.items():
        assert all(abs(x - m[k]) <= 1e-9 * max(1, m[k]) for k, x in proiezione(t, assi).items()), f"posizioni: margine {assi} non ritrovato"


def economia():
    spec = importlib.util.spec_from_file_location("economia", pathlib.Path(__file__).with_name("economia.py"))
    modulo = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(modulo)
    return modulo


def preleva(fonte, nome, url, accept=None):
    """Byte di una fonte: ogni fonte ha la sua cartella nella cache, un file già scaricato non si riscarica
    e quelli con sha256 noto sono verificati."""
    f = CACHE / fonte / nome
    if not f.exists():
        f.parent.mkdir(parents=True, exist_ok=True)
        parziale = f.with_name(nome + ".part")
        parziale.write_bytes(economia().scarica(url, accept))
        parziale.replace(f)
        if accept:
            time.sleep(15)  # il servizio ISTAT limita le interrogazioni al minuto
    dati = f.read_bytes()
    atteso = SHA256.get(f"{fonte}/{nome}")
    assert atteso is None or hashlib.sha256(dati).hexdigest() == atteso, f"{f}: sha256 {hashlib.sha256(dati).hexdigest()}, atteso {atteso}: file cambiato o corrotto, cancellalo"
    return dati


def istat(nome, flusso, chiave, anno):
    url = f"{economia().ISTAT}IT1,{flusso},1.0/{chiave}?startPeriod={anno}&endPeriod={anno}"
    testo = preleva("istat", nome, url, SDMX).decode("utf-8-sig")
    return [r for r in csv.DictReader(io.StringIO(testo)) if r["TIME_PERIOD"] == str(anno)]


def proiezione(t, assi):
    """Somma della tabella {(r, c, f): v} sugli assi non nominati."""
    out = collections.defaultdict(float)
    for k, v in t.items():
        out[tuple(k[a] for a in assi)] += v
    return out


def ipf(seme, margini, tol=1e-12, max_giri=10_000):
    """Proporzionamento iterativo: scala a turno la tabella perché ogni margine ({assi: {chiave: bersaglio}})
    torni entro tol (relativo). Le celle a zero restano zero; un margine che vuole massa dove il seme è vuoto
    non è raggiungibile: AssertionError."""
    t = dict(seme)
    for _ in range(max_giri):
        scarto = 0.0
        for assi, bersaglio in margini.items():
            somme = proiezione(t, assi)
            scarto = max([scarto] + [abs(somme[k] - b) / (b or 1) for k, b in bersaglio.items()])
        if scarto <= tol:
            return t
        for assi, bersaglio in margini.items():
            somme = proiezione(t, assi)
            assert all(somme[k] > 0 or b == 0 for k, b in bersaglio.items()), f"ipf: margine {assi} vuole massa dove il seme è vuoto"
            fattore = {k: b / somme[k] if somme[k] else 0.0 for k, b in bersaglio.items()}
            t = {k: v * fattore[tuple(k[a] for a in assi)] for k, v in t.items()}
    raise AssertionError(f"ipf: scarto {scarto:.2e} dopo {max_giri} giri, atteso <= {tol}")


def camera():
    """Elettori della Camera 2022 per regione (19, senza Valle d'Aosta) e totale del file."""
    testo = preleva("camera", "ItaliaVotiListeCIRC.csv", URL_CAMERA).decode("utf-8-sig")
    righe = list(csv.reader(io.StringIO(testo), delimiter=";"))
    assert righe[0][6].split() == ["ELETTORI", "TOTALI"], f"camera: colonna 6 {righe[0][6]!r}, attesa ELETTORI TOTALI"
    out, totale = collections.Counter(), None
    for r in righe[1:]:
        if r[0] == "TOTALE":
            totale = int(r[6])
        elif r[0].isdigit():
            out[r[2].split("/")[0].rstrip("0123456789 ").lower().replace("-", "_").replace(" ", "_")] += int(r[6])
    assert totale == sum(out.values()), f"camera: somma {sum(out.values())}, totale del file {totale}"
    return dict(out), totale


def elettori(ids, elezioni, dati_camera):
    """Elettori per regione dal CSV della Camera; la Valle d'Aosta (collegio a parte, non nel file) dalla popolazione,
    in proporzione agli elettori delle altre regioni."""
    pop = dict.fromkeys(ids, 0)
    for c in elezioni["camera"]["circoscrizione"]:
        pop[c["regione"]] += c["popolazione"]
    e, totale = dati_camera
    assert set(e) == set(ids) - {"valle_d_aosta"}, f"camera: regioni {sorted(e)}, attese le 19 senza valle_d_aosta"
    e["valle_d_aosta"] = round(totale * pop["valle_d_aosta"] / (sum(pop.values()) - pop["valle_d_aosta"]))
    return {r: e[r] for r in ids}


def lavoro(ids):
    """Per regione (migliaia, 15-74 anni, 2025): dipendenti, indipendenti, disoccupati, inattivi. ISTAT RCFL."""
    cod = {r[0]: r[4] for r in economia().REGIONI}
    occ = {(r["REF_AREA"], r["POSIZ_PROF"]): float(r["OBS_VALUE"]) for r in istat("occ.csv", "150_938_DF_DCCV_OCCUPATIT1_23", "A" + "." * 13, ANNO_LAVORO)}
    dis = {r["REF_AREA"]: float(r["OBS_VALUE"]) for r in istat("dis.csv", "151_929_DF_DCCV_DISOCCUPT1_6", "", ANNO_LAVORO) if r["SEX"] == "9" and r["LABPROF_STATUS_A"] == "99"}
    ina = {r["REF_AREA"]: float(r["OBS_VALUE"]) for r in istat("ina.csv", "152_928_DF_DCCV_INATTIV1_9", "", ANNO_LAVORO)
           if r["SEX"] == "9" and r["AGE"] == "Y15-74" and r["LABPROF_STATUS_A"] == "IN"}
    return {r: (occ[cod[r], "1"], occ[cod[r], "2"], dis[cod[r]], ina[cod[r]]) for r in ids}


def fascia_percentile(prima, peso, totale):
    """Indice di fascia di chi sta, nell'ordine per reddito, da `prima` per `peso` su `totale`: dal percentile del punto medio."""
    centro = (prima + peso / 2) / totale
    return 0 if centro < SOGLIE_FASCE[0] else 1 if centro < SOGLIE_FASCE[1] else 2


def mef(ids):
    """Pensionati per regione (persone con reddito da pensione) e quota di contribuenti per fascia, MEF anno d'imposta 2024.
    Le classi di reddito individuale passano a fascia dal percentile nazionale del loro punto medio."""
    testo = preleva("mef", "REG_tipo_reddito_2025.csv", URL_MEF).decode("utf-8-sig")
    num = lambda s: int(s.replace(".", "") or 0)
    pens, classi, ordine = collections.Counter(), collections.defaultdict(int), []
    lav = collections.defaultdict(lambda: [0, 0])  # (regione, classe, tipo) -> [frequenza, ammontare]
    for r in csv.DictReader(io.StringIO(testo), delimiter=";"):
        reg = r["Regione"].split("(")[0].strip().lower().replace(" ", "_").replace("'", "_")
        if reg not in ids:
            continue  # "Mancante/errata"
        k = r["Classi di reddito complessivo in euro"]
        if k not in ordine:
            ordine.append(k)
        pens[reg] += num(r["Reddito da pensione - Frequenza"])
        classi[reg, k] += num(r["Numero contribuenti"])
        for tipo, voci in TIPI_MEF.items():
            for v in voci:
                lav[reg, k, tipo][0] += num(r[v + " - Frequenza"])
                lav[reg, k, tipo][1] += num(r[v + " - Ammontare in euro"])
    assert set(pens) == set(ids), f"mef: regioni mancanti {sorted(set(ids) - set(pens))}"
    nazionale = {k: sum(classi[r, k] for r in ids) for k in ordine}
    totale, prima, fascia = sum(nazionale.values()), 0, {}
    for k in ordine:
        fascia[k] = fascia_percentile(prima, nazionale[k], totale)
        prima += nazionale[k]
    quote_f = {}
    for r in ids:
        v = [sum(classi[r, k] for k in ordine if fascia[k] == f) for f in range(3)]
        quote_f[r] = [x / sum(v) for x in v]
    def medio(regioni_, tipo, f):
        freq, amm = (sum(lav[r, k, tipo][i] for r in regioni_ for k in ordine if fascia[k] == f) for i in (0, 1))
        return amm / freq
    # reddito medio per percettore: nella regione, nella fascia; dove nessuno percepisce quel tipo, la media nazionale della fascia
    redd = {(r, t, f): medio([r], t, f) if sum(lav[r, k, t][0] for k in ordine if fascia[k] == f) else medio(ids, t, f) for r in ids for t in TIPI_MEF for f in range(3)}
    return dict(pens), quote_f, redd


def quinti(peso_disoccupati):
    """Per condizione la quota di famiglie per fascia, nazionale, ISTAT EU-SILC 2024: bassa = quinto 1 + metà del 2,
    media = metà del 2 + quinto 3 + metà del 4, alta = metà del 4 + quinto 5 (decili 3-4-3).
    La condizione è quella del principale percettore; disoccupato_inattivo mescola 'in cerca' e 'altra non professionale'."""
    q = collections.defaultdict(dict)
    for r in istat("quin.csv", "32_38_DF_DCCV_QUINTOREDD_5", "", ANNO_QUINTI):
        if r["EDU_LEV_MAIN_EARN"] == "99":
            q[r["LABPROF_STATUS_C_MAIN_EARNER"]][r["FIFTH_EQUIV_HOUSE_INC"]] = float(r["OBS_VALUE"]) / 100
    fasce = {k: [x / sum(w) for x in w] for k, v in q.items() if k != "99"  # le percentuali ISTAT sommano 99,9 o 100,1
             for w in [[v["1"] + v["2"] / 2, v["2"] / 2 + v["3"] + v["4"] / 2, v["4"] / 2 + v["5"]]]}
    mix = [peso_disoccupati * a + (1 - peso_disoccupati) * b for a, b in zip(fasce["2B"], fasce["2C"])]
    return {"dipendente_privato": fasce["1B"], "dipendente_pubblico": fasce["1B"], "autonomo": fasce["1A"],
            "pensionato": fasce["2A"], "disoccupato_inattivo": mix, "studente": fasce["2C"]}


def ibf():
    """Seme IBF 2022: persone adulte per (regione, condizione, fascia di reddito familiare equivalente), pesate con pesofit2.
    Reddito equivalente = reddito familiare netto / radice del numero di componenti; fasce sul percentile nazionale pesato.
    Condizione da QUAL: 1-3 dipendente (pubblico per SHARE_PUBBLICO), 4-5 autonomo, 6 pensionato, 7 studente se sotto i 30 anni."""
    z = zipfile.ZipFile(io.BytesIO(preleva("ibf", "ind22_ascii.zip", URL_IBF)))
    leggi = lambda nome: csv.DictReader(io.TextIOWrapper(z.open(f"CSV/{nome}.csv"), encoding="utf-8-sig"))
    persone_, y = list(leggi("carcom22")), {r["NQUEST"]: float(r["Y"]) for r in leggi("rfam22")}
    n = collections.Counter(r["NQUEST"] for r in persone_)
    rec = []
    for r in persone_:
        eta, q, reg, w = int(r["ETA"]), int(r["QUAL"]), IBF_IREG[int(r["IREG"]) - 1], float(r["pesofit2"])
        red = y[r["NQUEST"]] / math.sqrt(n[r["NQUEST"]])
        if eta < 18:
            continue
        if q <= 3:
            rec += [(red, reg, "dipendente_pubblico", w * SHARE_PUBBLICO), (red, reg, "dipendente_privato", w * (1 - SHARE_PUBBLICO))]
        else:
            rec.append((red, reg, {4: "autonomo", 5: "autonomo", 6: "pensionato"}.get(q) or ("studente" if eta < 30 else "disoccupato_inattivo"), w))
    rec.sort()
    totale, prima, seme = sum(x[3] for x in rec), 0, collections.defaultdict(float)
    for red, reg, c, w in rec:
        seme[reg, c, FASCE[fascia_percentile(prima, w, totale)]] += w
        prima += w
    return seme


def persone(el, lav, pensionati):
    """Margine (b): persone per regione e condizione. Dipendenti (quota pubblica assunta), indipendenti, disoccupati e inattivi
    da ISTAT; pensionati da MEF; studenti assunti; i disoccupati o inattivi sono gli inattivi meno pensionati e studenti
    più i disoccupati (almeno i disoccupati). Ogni regione è scalata ai suoi elettori."""
    out, minimo, oltre = {}, [], 0.0
    for r, (dip, aut, dis, ina) in lav.items():
        p = {"dipendente_pubblico": dip * SHARE_PUBBLICO, "dipendente_privato": dip * (1 - SHARE_PUBBLICO), "autonomo": aut,
             "pensionato": pensionati[r] / 1000, "studente": SHARE_STUDENTI * el[r] / 1000}
        p["disoccupato_inattivo"] = max(dis + ina - p["pensionato"] - p["studente"], dis)
        tot = sum(p.values())
        if dis + ina - p["pensionato"] - p["studente"] < dis:
            minimo.append(f"{r} {tot * 1000 - el[r]:+,.0f}")
        oltre += max(0.0, tot * 1000 - el[r])
        out[r] = {c: el[r] * x / tot for c, x in p.items()}
    print(f"persone: residuo disoccupati/inattivi al minimo in {len(minimo)} regioni, con scarto somma delle condizioni - elettori ({'; '.join(minimo)}); "
          f"somma degli scarti positivi in tutte le regioni {oltre:,.0f} persone (poi scalata agli elettori)", file=sys.stderr)
    return out


def quote_gruppi(ids, el, pers, fasce_mef, fasce_istat, seme_ibf):
    """Per regione le 18 quote (condizione x fascia), in ordine canonico: IPF 20 x 6 x 3 sui margini
    (b) regione x condizione, (c) regione x fascia, (d) condizione x fascia nazionale.
    (c) e (d) non hanno la stessa somma per fascia: (c) si ricalibra prima su righe = elettori e colonne = totali di (d)."""
    rc = {(r, c): pers[r][c] for r in ids for c in CONDIZIONI}
    cf = {(c, f): sum(pers[r][c] for r in ids) * fasce_istat[c][i] for c in CONDIZIONI for i, f in enumerate(FASCE)}
    rf = ipf({(r, f): el[r] * fasce_mef[r][i] for r in ids for i, f in enumerate(FASCE)},
             {(0,): {(r,): el[r] for r in ids}, (1,): {(f,): sum(cf[c, f] for c in CONDIZIONI) for f in FASCE}})
    indipendenza = {(r, c, f): rc[r, c] * rf[r, f] / el[r] for r in ids for c in CONDIZIONI for f in FASCE}
    if seme_ibf is None:
        seme = indipendenza
    else:
        scala = sum(el.values()) / sum(seme_ibf.values())
        seme = {k: seme_ibf[k] * scala if seme_ibf.get(k) else VUOTE_IBF * v for k, v in indipendenza.items()}
    t = ipf(seme, {(0, 1): rc, (0, 2): rf, (1, 2): cf})
    return {r: quote([t[r, c, f] for c in CONDIZIONI for f in FASCE]) for r in ids}


def ess(percorso):
    """Righe (condizione, fascia o None, posizione 0-6, regione o None, peso) degli italiani con autocollocazione 0-10.
    CSV del Data Portal ESS round 11: cntry, lrscale, mainact, emplrel, tporgwk, hinctnta (10 categorie), region (NUTS 2), anweight.
    Chi non si colloca (77, 88, 99) è escluso da tutto; chi non ha condizione o reddito, dai margini che li usano."""
    nuts = {g: r[0] for r in economia().REGIONI for g in r[3]}
    out = []
    with open(percorso, encoding="utf-8-sig", newline="") as f:
        righe = csv.DictReader(f)
        assert righe.fieldnames and {"cntry", "lrscale", "mainact"} <= set(righe.fieldnames), f"{percorso}: colonne {righe.fieldnames}, attese almeno cntry, lrscale, mainact"
        for r in righe:
            lr, att, rel = r["lrscale"], r["mainact"], r.get("emplrel", "")
            if r["cntry"] != "IT" or lr not in map(str, range(11)):
                continue
            c = ("studente" if att == "2" else "pensionato" if att == "6" else "disoccupato_inattivo" if att in ("3", "4", "5", "7", "8", "9")
                 else None if att != "1" else "autonomo" if rel in ("2", "3") else None if rel != "1"
                 else "dipendente_pubblico" if r.get("tporgwk") in ("1", "2") else "dipendente_privato")
            d = r.get("hinctnta", "")
            fascia = None if d not in map(str, range(1, 11)) else FASCE[0 if int(d) <= 3 else 1 if int(d) <= 7 else 2]
            if c:
                out.append((c, fascia, TAGLIO[int(lr)], nuts.get(r.get("region", "")), float(r.get("anweight") or 1)))
    assert len(out) >= 500, f"{percorso}: {len(out)} italiani utilizzabili, attesi almeno 500: colonne o codici diversi dal Data Portal?"
    return out


def posizioni(N, macro, righe_ess):
    """IPF a 4 dimensioni (regione, condizione, fascia, posizione) su N={(r, c, f): elettori}. Margini: (r, c, f) = N;
    (c, f, p) = elettori di (c, f) x distribuzione ESS per posizione di (c, f); (r, p) = distribuzione ESS della regione
    (accorpata alla macroarea sotto MIN_CASI_REGIONE casi), ricalibrata sui totali per posizione di (c, f, p).
    Le distribuzioni sono lisciate verso la nazionale con LISCIA pseudo-casi. Restituisce tabella e margini."""
    ids = sorted({k[0] for k in N})
    peso = lambda righe: [sum(w for p, w in righe if p == k) for k in range(7)]
    globale = quote(peso([(p, w) for _, _, p, _, w in righe_ess]))
    distribuzione = lambda righe: quote([x + LISCIA * b for x, b in zip(peso(righe), globale)])
    tot_cf = {(c, f): sum(N[r, c, f] for r in ids) for c in CONDIZIONI for f in FASCE}
    cfp = {(c, f, p): tot_cf[c, f] * q for c, f in tot_cf
           for p, q in enumerate(distribuzione([(p, w) for c2, f2, p, _, w in righe_ess if (c2, f2) == (c, f)]))}
    el = {r: sum(N[r, c, f] for c in CONDIZIONI for f in FASCE) for r in ids}
    rp = {}
    for r in ids:
        gruppo = {r} if sum(1 for x in righe_ess if x[3] == r) >= MIN_CASI_REGIONE else {x for x in ids if macro[x] == macro[r]}
        rp.update({(r, p): el[r] * q for p, q in enumerate(distribuzione([(x[2], x[4]) for x in righe_ess if x[3] in gruppo]))})
    tot_p = {(p,): sum(cfp[c, f, p] for c, f in tot_cf) for p in range(7)}
    rp = ipf(rp, {(0,): {(r,): el[r] for r in ids}, (1,): tot_p})
    seme = {(r, c, f, p): N[r, c, f] * cfp[c, f, p] / tot_cf[c, f] for r in ids for c, f in tot_cf for p in range(7)}
    margini = {(0, 1, 2): N, (1, 2, 3): cfp, (0, 3): rp}
    return ipf(seme, margini), margini


def istogrammi(ids, macro, el, quote_, righe_ess):
    """Istogramma a 7 punti per gruppo (r, c, f). Senza ESS: il segnaposto."""
    if righe_ess is None:
        return {(r, c, f): list(ISTOGRAMMA) for r in ids for c in CONDIZIONI for f in FASCE}
    N = {(r, c, f): el[r] * quote_[r][3 * i + j] for r in ids for i, c in enumerate(CONDIZIONI) for j, f in enumerate(FASCE)}
    t, _ = posizioni(N, macro, righe_ess)
    return {(r, c, f): quote([t[r, c, f, p] for p in range(7)]) for r in ids for c, f in ((c, f) for c in CONDIZIONI for f in FASCE)}


def redditi(regioni, pers, scenario, mef_redditi):
    """Primario di dipendenti (pubblici e privati insieme: il MEF non li distingue) e autonomi = reddito medio MEF per percettore
    della regione e della fascia; disoccupati e studenti provvisori; pensionati solo trasferimenti."""
    def primario(r, c, i):
        if c == "pensionato":
            return 0.0
        if c in PRIMARIO:
            return PRIMARIO[c] * MOLT_FASCIA[i] * MOLT_MACROAREA[r["macroarea"]]
        return mef_redditi[r["id"], "autonomo" if c == "autonomo" else "dipendente", i]

    nazionale = {c: sum(pers[r["id"]][c] for r in regioni) for c in CONDIZIONI}
    trasf = {c: 0.0 for c in CONDIZIONI}
    trasf["pensionato"] = scenario["spesa_pensioni"]["valore"] * 1e9 / scenario["pensionati"]["valore"]
    trasf["disoccupato_inattivo"] = scenario["spesa_sussidi"]["valore"] * 1e9 / nazionale["disoccupato_inattivo"]
    return {
        (r["id"], c, f): (round(primario(r, c, i), 2), round(trasf[c] * MOLT_FASCIA[i], 2))
        for r in regioni for c in CONDIZIONI for i, f in enumerate(FASCE)
    }


def panieri(economia):
    return {f: quote(economia["consumi"]) for f in FASCE}  # segnaposto: la PR B li differenzia


def astensione(regioni):
    return {r: round(math.log((1 - AFFLUENZA[r]) / AFFLUENZA[r]) + COSTANTE_ASTENSIONE, 6) for r in regioni}


def costruisci():
    regioni = leggi("regioni.toml")["regione"]
    scenario, economia = leggi("scenario.toml"), leggi("economia.toml")
    ids = [r["id"] for r in regioni]
    dati_camera = camera()
    el = elettori(ids, leggi("elezioni.toml"), dati_camera)
    lav, (pens, fasce_mef, redd_mef) = lavoro(ids), mef(ids)
    peso_dis = sum(l[2] for l in lav.values()) / sum(l[2] + l[3] for l in lav.values())
    pers, fasce_istat = persone(el, lav, pens), quinti(peso_dis)
    seme = None if "--senza-ibf" in sys.argv else ibf()
    quote_ = quote_gruppi(ids, el, pers, fasce_mef, fasce_istat, seme)
    return {
        "regioni": regioni, "elettori": el, "quote": quote_, "istogrammi": istogrammi(ids, {r["id"]: r["macroarea"] for r in regioni}, el, quote_, ess(ESS) if ESS else None),
        "pers": pers, "fasce_istat": fasce_istat, "con_ess": ESS is not None,
        "redditi": redditi(regioni, pers, scenario, redd_mef), "panieri": panieri(economia), "astensione": astensione(ids),
        "attesi": dati_camera[1] + el["valle_d_aosta"],
    }


def tabella(v):
    """Tabella TOML in linea: `{ k = x, ... }`."""
    return "{ " + ", ".join(f"{k} = {x}" for k, x in v.items()) + " }"


def scrivi(d):
    s = json.dumps
    t = f"# Generato da `{COMANDO}` il {OGGI}. Non modificare a mano.\n"
    t += "# Quote dei gruppi da IPF sui margini reali, redditi da lavoro dal MEF, il resto provvisorio: vedi strumenti/taratura/README.md.\n"
    t += ("# Posizioni: elaborazione da European Social Survey round 11 (ESS ERIC, https://www.europeansocialsurvey.org), CC BY-NC-SA 4.0, adattato.\n\n"
          if d["con_ess"] else "# Posizioni: segnaposto, uguali per tutti i gruppi (senza --ess).\n\n")
    t += f"aliquote = {tabella(ALIQUOTE)}\n"
    t += f"astensione_condizione = {tabella(dict.fromkeys(CONDIZIONI, 0.0))}\n"
    t += f"astensione_fascia = {tabella(ASTENSIONE_FASCIA)}\n"
    t += f"nomi = {s(NOMI, ensure_ascii=False)}\ncognomi = {s(COGNOMI, ensure_ascii=False)}\n\n[paniere]\n"
    t += "".join(f"{f} = [{', '.join(f'{x:.6f}' for x in d['panieri'][f])}]\n" for f in FASCE)
    for c, pesi in INTERESSI.items():
        t += f"\n[interessi.{c}]\n" + "".join(f"{k} = {w:.2f}\n" for k, w in pesi.items())
    for nome, campione in ISTITUTI:
        t += f"\n[[istituto]]\nnome = {s(nome)}\ncampione = {campione}\n"
    for r in d["regioni"]:
        i = r["id"]
        t += f"\n[[regione]]\nid = {s(i)}\nelettori = {d['elettori'][i]}\nastensione = {d['astensione'][i]}\n"
    for r in d["regioni"]:
        i = r["id"]
        for k, (c, f) in enumerate((c, f) for c in CONDIZIONI for f in FASCE):
            primario, trasf = d["redditi"][i, c, f]
            t += (f"\n[[gruppo]]\nregione = {s(i)}\ncondizione = {s(c)}\nfascia = {s(f)}\nquota = {d['quote'][i][k]:.6f}\n"
                  f"posizione = [{', '.join(f'{x:.6f}' for x in d['istogrammi'][i, c, f])}]\nprimario = {primario}\ntrasferimenti = {trasf}\n")
    return t


def controlla(d):
    assert len(d["regioni"]) == 20 and all(d["elettori"][r["id"]] > 0 for r in d["regioni"]), "attese 20 regioni con elettori > 0"
    tot = sum(d["elettori"].values())
    assert tot == d["attesi"], f"elettori {tot}, attesi {d['attesi']} (elettori del CSV + Valle d'Aosta)"
    # Affluenza del solo termine regionale (senza partiti né fedeltà): stesso intervallo del test Rust
    # affluenza_del_turno_zero_plausibile, coglie un segno o una costante sbagliati.
    affluenza = sum(d["elettori"][r] / (1 + math.exp(a)) for r, a in d["astensione"].items()) / tot
    assert 0.5 < affluenza < 0.8, f"affluenza implicita {affluenza:.3f}, attesa fra 0,5 e 0,8"
    for i, q in d["quote"].items():
        assert len(q) == 18 and min(q) >= 0 and abs(sum(q) - 1) < 1e-9, f"{i}: quote somma {sum(q):.12f}, minimo {min(q)}"
    for r, q in d["quote"].items():
        for i, c in enumerate(CONDIZIONI):
            persone_c = d["elettori"][r] * sum(q[3 * i:3 * i + 3])
            assert abs(persone_c - d["pers"][r][c]) <= 1e-3 * d["pers"][r][c], f"{r} {c}: quote danno {persone_c:.0f} persone, margine (b) {d['pers'][r][c]:.0f}"
    for i, c in enumerate(CONDIZIONI):
        for j, f in enumerate(FASCE):
            dalle_quote = sum(d["elettori"][r] * d["quote"][r][3 * i + j] for r in d["quote"])
            bersaglio = sum(d["pers"][r][c] for r in d["quote"]) * d["fasce_istat"][c][j]
            assert abs(dalle_quote - bersaglio) <= 1e-3 * bersaglio, f"{c} {f}: quote danno {dalle_quote:.0f} persone, margine (d) {bersaglio:.0f}"
    assert len(d["istogrammi"]) == 360, f"{len(d['istogrammi'])} istogrammi, attesi 360"
    for k, h in d["istogrammi"].items():
        assert len(h) == 7 and min(h) >= 0 and abs(sum(h) - 1) < 1e-9, f"{k}: istogramma somma {sum(h):.12f}"
    for k, (p, t) in d["redditi"].items():
        assert p + t > 0, f"{k}: primario + trasferimenti = {p + t}, atteso > 0"
    for c, pesi in INTERESSI.items():
        assert abs(sum(pesi.values()) - 1) < 1e-9, f"{c}: interessi somma {sum(pesi.values())}"
    for f, p in d["panieri"].items():
        assert len(p) == 10 and abs(sum(p) - 1) < 1e-9, f"paniere {f}: {len(p)} quote, somma {sum(p):.12f}"
    letto = tomllib.loads(scrivi(d))
    assert len(letto["regione"]) == 20 and len(letto["gruppo"]) == 360, f"{len(letto['regione'])} regioni, {len(letto['gruppo'])} gruppi, attesi 20 e 360"
    atteso = [(r["id"], c, f) for r in d["regioni"] for c in CONDIZIONI for f in FASCE]
    assert [(g["regione"], g["condizione"], g["fascia"]) for g in letto["gruppo"]] == atteso, "gruppi fuori ordine canonico"
    assert "qualita_trasporti = 0.10" in scrivi(d), "peso trasporti non scritto con due decimali"


def main():
    if "--selftest" in sys.argv:
        prova_ipf()
        prova_posizioni()
    d = costruisci()
    controlla(d)
    if "--selftest" in sys.argv:
        # I test Rust leggono il file committato: deve essere l'uscita dello script (intestazione
        # con la data esclusa). Dopo `sim-cli tara` (Task 12) il file cambia: il confronto va ristretto lì.
        # Le righe `astensione = ` di [[regione]] sono di `sim-cli tara` (Task 12): fuori dal confronto.
        senza = lambda righe: [r for r in righe[1:] if not r.startswith("astensione = ")]
        committato = (BASE / "elettorato.toml").read_text(encoding="utf-8").splitlines()
        assert senza(scrivi(d).splitlines()) == senza(committato), "elettorato.toml non è l'uscita dello script: rilancialo"
        print("selftest ok: 20 regioni, 360 gruppi")
        return
    destinazione = BASE / "elettorato.toml"
    (BASE / "elettorato.toml.tmp").write_text(scrivi(d), encoding="utf-8")
    (BASE / "elettorato.toml.tmp").replace(destinazione)
    print("Scritto elettorato.toml in", BASE)


if __name__ == "__main__":
    main()
