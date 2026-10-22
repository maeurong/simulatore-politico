"""Dati provvisori dell'elettorato: 20 regioni x 6 condizioni x 3 fasce di reddito.

Uso: python3 -I strumenti/taratura/elettorato.py [--selftest]

Legge solo i TOML già nel repo (regioni, elezioni, scenario, economia) e costanti citate,
scrive dati/base/elettorato.toml. Le funzioni sono lo scheletro che la PR B riempie con
le fonti vere (ISTAT, MEF, ESS): cambiano i corpi, non le firme.
Regole provvisorie e assunzioni: strumenti/taratura/README.md.
"""

import datetime
import json
import math
import pathlib
import sys
import tomllib

BASE = pathlib.Path(__file__).resolve().parents[2] / "dati" / "base"
COMANDO = "python3 -I strumenti/taratura/elettorato.py"
OGGI = datetime.date.today().isoformat()

ELETTORI_ITALIA = 46_350_000  # assunzione: 46.253.014 della Camera 2022 (risultati-elettorali-italia.md:13) più la Valle d'Aosta
CONDIZIONI = ["dipendente_privato", "dipendente_pubblico", "autonomo", "pensionato", "disoccupato_inattivo", "studente"]
FASCE = ["bassa", "media", "alta"]
DIPENDENTI, AUTONOMI = 18.93, 5.18  # milioni, dati-per-tarare-i-gruppi.md:45
SHARE_PUBBLICO = 0.17  # assunzione: RGS non aperto
SHARE_STUDENTI = 0.06  # assunzione
# (bassa, media, alta) per condizione: assunzione dai quinti di dati-per-tarare-i-gruppi.md:44 e dalle fasce 3-4-3 di :66
FASCIA = {
    "dipendente_privato": (0.25, 0.45, 0.30),
    "dipendente_pubblico": (0.15, 0.45, 0.40),
    "autonomo": (0.30, 0.35, 0.35),
    "pensionato": (0.30, 0.42, 0.28),
    "disoccupato_inattivo": (0.60, 0.32, 0.08),
    "studente": (0.40, 0.40, 0.20),
}
BASSA_PER_MACROAREA = {"nord": 0.85, "centro": 1.0, "mezzogiorno": 1.3}
ISTOGRAMMA = [0.05, 0.12, 0.17, 0.25, 0.16, 0.17, 0.08]  # segnaposto: la PR B lo ricava da ESS
PRIMARIO = {"dipendente_privato": 24000.0, "dipendente_pubblico": 30000.0, "autonomo": 28000.0,
            "pensionato": 0.0, "disoccupato_inattivo": 4000.0, "studente": 3000.0}
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


def elettori(regioni, elezioni):
    pop = dict.fromkeys(regioni, 0)
    for c in elezioni["camera"]["circoscrizione"]:
        pop[c["regione"]] += c["popolazione"]
    tot = sum(pop.values())
    e = [round(ELETTORI_ITALIA * pop[r] / tot) for r in regioni[:-1]]
    return dict(zip(regioni, e + [ELETTORI_ITALIA - sum(e)]))


def persone(regioni, el, scenario):
    """Persone per regione e condizione, il residuo ai disoccupati o inattivi; e i tagli
    (regione -> persone di residuo negativo tagliato a 0)."""
    out, tagli = {}, {}
    for r in regioni:
        fl, u = r["forze_lavoro"] * 1000, r["disoccupazione_2025"]
        occ = fl * (1 - u / 100)
        dip = occ * DIPENDENTI / (DIPENDENTI + AUTONOMI)
        p = {
            "dipendente_pubblico": dip * SHARE_PUBBLICO,
            "dipendente_privato": dip * (1 - SHARE_PUBBLICO),
            "autonomo": occ * AUTONOMI / (DIPENDENTI + AUTONOMI),
            "pensionato": el[r["id"]] * scenario["pensionati"]["valore"] / ELETTORI_ITALIA,
            "studente": SHARE_STUDENTI * el[r["id"]],
        }
        residuo = el[r["id"]] - sum(p.values())
        if residuo < 0:
            tagli[r["id"]] = round(-residuo)
        # assunzione del piano 4: le forze di lavoro contano anche chi non vota (stranieri, minorenni occupati);
        # lo scarto si spalma in proporzione su tutte le condizioni (quota = persone / somma persone). La PR B lo sostituisce col proporzionamento iterativo (IPF).
        p["disoccupato_inattivo"] = fl * u / 100 + max(0.0, residuo)
        out[r["id"]] = p
    return out, tagli


def quote_gruppi(regioni, el, pers):
    """Per regione le 18 quote (condizione x fascia), in ordine canonico."""
    out = {}
    for r in regioni:
        v = []
        for c in CONDIZIONI:
            b, m, a = FASCIA[c]
            w = [b * BASSA_PER_MACROAREA[r["macroarea"]], m, a]
            v += [pers[r["id"]][c] * x / sum(w) for x in w]
        out[r["id"]] = quote(v)
    return out


def istogrammi():
    return {(c, f): list(ISTOGRAMMA) for c in CONDIZIONI for f in FASCE}


def redditi(regioni, pers, scenario):
    nazionale = {c: sum(pers[r["id"]][c] for r in regioni) for c in CONDIZIONI}
    trasf = {c: 0.0 for c in CONDIZIONI}
    trasf["pensionato"] = scenario["spesa_pensioni"]["valore"] * 1e9 / scenario["pensionati"]["valore"]
    trasf["disoccupato_inattivo"] = scenario["spesa_sussidi"]["valore"] * 1e9 / nazionale["disoccupato_inattivo"]
    return {
        (r["id"], c, f): (round(PRIMARIO[c] * MOLT_FASCIA[i] * MOLT_MACROAREA[r["macroarea"]], 2), round(trasf[c] * MOLT_FASCIA[i], 2))
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
    el = elettori(ids, leggi("elezioni.toml"))
    pers, tagli = persone(regioni, el, scenario)
    return {
        "regioni": regioni, "elettori": el, "quote": quote_gruppi(regioni, el, pers), "istogrammi": istogrammi(),
        "redditi": redditi(regioni, pers, scenario), "panieri": panieri(economia), "astensione": astensione(ids),
        "tagli": tagli,
    }


def tabella(v):
    """Tabella TOML in linea: `{ k = x, ... }`."""
    return "{ " + ", ".join(f"{k} = {x}" for k, x in v.items()) + " }"


def scrivi(d):
    s = json.dumps
    t = f"# Generato da `{COMANDO}` il {OGGI}. Non modificare a mano.\n"
    t += "# Valori provvisori del piano 4: vedi strumenti/taratura/README.md.\n\n"
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
                  f"posizione = [{', '.join(f'{x:.2f}' for x in d['istogrammi'][c, f])}]\nprimario = {primario}\ntrasferimenti = {trasf}\n")
    return t


def controlla(d):
    assert len(d["regioni"]) == 20 and all(d["elettori"][r["id"]] > 0 for r in d["regioni"]), "attese 20 regioni con elettori > 0"
    tot = sum(d["elettori"].values())
    # Affluenza del solo termine regionale (senza partiti né fedeltà): stesso intervallo del test Rust
    # affluenza_del_turno_zero_plausibile, coglie un segno o una costante sbagliati.
    affluenza = sum(d["elettori"][r] / (1 + math.exp(a)) for r, a in d["astensione"].items()) / tot
    assert 0.5 < affluenza < 0.8, f"affluenza implicita {affluenza:.3f}, attesa fra 0,5 e 0,8"
    for i, q in d["quote"].items():
        assert len(q) == 18 and min(q) >= 0 and abs(sum(q) - 1) < 1e-9, f"{i}: quote somma {sum(q):.12f}, minimo {min(q)}"
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
    d = costruisci()
    controlla(d)
    if "--selftest" in sys.argv:
        # I test Rust leggono il file committato: deve essere l'uscita dello script (intestazione
        # con la data esclusa). Dopo `sim-cli tara` (PR B) il file cambia: il confronto va ristretto lì.
        committato = (BASE / "elettorato.toml").read_text(encoding="utf-8").splitlines()[1:]
        assert scrivi(d).splitlines()[1:] == committato, "elettorato.toml non è l'uscita dello script: rilancialo"
        print("selftest ok: 20 regioni, 360 gruppi")
        return
    destinazione = BASE / "elettorato.toml"
    (BASE / "elettorato.toml.tmp").write_text(scrivi(d), encoding="utf-8")
    (BASE / "elettorato.toml.tmp").replace(destinazione)
    print("Scritto elettorato.toml in", BASE)
    print("Residuo tagliato a 0 (persone):", d["tagli"])


if __name__ == "__main__":
    main()
