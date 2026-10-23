"""Stima offline di beta_distanza e fedelta con un logit condizionale su ITANES 2022 (post).

U_k = -beta * (x - z_k)^2 + lambda_k + phi * [k = voto 2018], k = 8 partiti, poi astensione (lambda = 0).
Stessa forma del gioco (`crates/sim-core/src/elettorato/voto.rs`) e stessa scala: x = autocollocazione (Q19)
tagliata sui 7 punti col TAGLIO di `elettorato.py`, z_k = media delle collocazioni tagliate del partito (Q18).
beta e' quindi gia' beta_distanza, senza conversione. Solo libreria standard.
I microdati restano fuori dal repo: qui escono solo i parametri.

    python3 -I strumenti/taratura/stima_voto.py [--itanes CARTELLA] [--solo-stima]
    python3 -I strumenti/taratura/stima_voto.py --selftest
"""
import argparse, csv, glob, math, os, random, re, sys, tempfile

REPO = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
MANOPOLE = os.path.join(REPO, "dati", "base", "manopole.toml")
CACHE = os.path.expanduser("~/.cache/simulatore-politico-taratura/itanes")
PARTITI = ["FdI", "Lega", "FI", "Azione-IV", "M5S", "PD", "+Europa", "AVS"]
ASTENSIONE = 8
TAGLIO = (0, 0, 1, 1, 2, 3, 4, 5, 5, 6, 6)  # uguale a elettorato.py: il selftest lo confronta
# voto 2022 (voto_post) -> alternativa; 8 Italexit, 10 Impegno Civico, 11 Noi Moderati, 12 altro, 94 bianca: fuori scelta
VOTO22 = {1: 0, 2: 1, 3: 2, 4: 3, 5: 4, 6: 5, 7: 6, 9: 7}
# D4: voto 2018 (Q14b) -> erede 2022 per famiglia politica; None = senza erede, nessun termine di fedelta'
EREDE18 = {1: 1, 2: 2, 3: 0, 4: 4, 5: 5, 6: 6, 7: 7, 8: None, 9: None, 10: None}


def verifica(cond, msg):
    if not cond:
        raise AssertionError(msg)


def risolvi(a, b):
    """Gauss con pivot parziale: a x = b, b matrice (colonne)."""
    n = len(a)
    m = [r[:] + c[:] for r, c in zip(a, b)]
    for i in range(n):
        p = max(range(i, n), key=lambda r: abs(m[r][i]))
        if abs(m[p][i]) < 1e-12:
            raise ValueError("matrice singolare: parametro non identificato")
        m[i], m[p] = m[p], m[i]
        for r in range(n):
            if r != i:
                f = m[r][i] / m[i][i]
                m[r] = [x - f * y for x, y in zip(m[r], m[i])]
    return [[x / m[i][i] for x in m[i][n:]] for i in range(n)]


def feature(o, k):
    """Coppie (indice, valore) dei regressori dell'alternativa k: [-d2, [k=m], lambda_k]; l'ultima alternativa e' il riferimento."""
    f = [(0, -o["d2"][k])]
    if o["m"] == k:
        f.append((1, 1.0))
    if k < len(o["d2"]) - 1:
        f.append((2 + k, 1.0))
    return f


def probs(th, o):
    fs = [feature(o, k) for k in range(len(o["d2"]))]
    u = [sum(th[i] * v for i, v in f) for f in fs]
    mx = max(u)
    e = [math.exp(x - mx) for x in u]
    s = sum(e)
    return fs, [x / s for x in e], u[o["y"]] - mx - math.log(s)


def logver(th, oss):
    return sum(probs(th, o)[2] for o in oss)


def derivate(th, oss, P):
    """Gradiente e matrice dell'informazione osservata."""
    g, I = [0.0] * P, [[0.0] * P for _ in range(P)]
    for o in oss:
        fs, p, _ = probs(th, o)
        mean = [0.0] * P
        for pk, f in zip(p, fs):
            for i, v in f:
                mean[i] += pk * v
        for i, v in fs[o["y"]]:
            g[i] += v
        for i in range(P):
            g[i] -= mean[i]
            for j in range(P):
                I[i][j] -= mean[i] * mean[j]
        for pk, f in zip(p, fs):
            for i, v in f:
                for j, w in f:
                    I[i][j] += pk * v * w
    return g, I


def stima(oss, iters=50):
    """MLE per Newton-Raphson. Ritorna (theta, errori standard, log-verosimiglianza); non converge = errore."""
    P = 1 + len(oss[0]["d2"])  # beta, phi, lambda delle alternative tranne il riferimento
    th = [0.0] * P
    for _ in range(iters):
        g, I = derivate(th, oss, P)
        d = [r[0] for r in risolvi(I, [[x] for x in g])]
        ll, t = logver(th, oss), 1.0
        while logver([a + t * b for a, b in zip(th, d)], oss) < ll - 1e-9:
            t /= 2  # dimezza il passo finche' la verosimiglianza non cala
            if t < 1e-6:
                raise RuntimeError("Newton non converge: il dimezzamento del passo si e' esaurito")
        th = [a + t * b for a, b in zip(th, d)]
        if max(abs(t * x) for x in d) < 1e-9:
            break
    else:
        raise RuntimeError(f"Newton non converge in {iters} giri")
    g, I = derivate(th, oss, P)
    inv = risolvi(I, [[float(i == j) for j in range(P)] for i in range(P)])
    return th, [math.sqrt(inv[i][i]) for i in range(P)], logver(th, oss)


def sintetici(n, theta, z, seed):
    """Utilita' scritta in chiaro (non passa da feature()): theta = [beta, phi, lambda_0..lambda_7], astensione = 0."""
    rng = random.Random(seed)
    beta, phi, lam = theta[0], theta[1], list(theta[2:]) + [0.0]
    oss = []
    for _ in range(n):
        x = rng.uniform(1, 7)
        m = rng.choice([None] + list(range(ASTENSIONE + 1)))
        d2 = [(x - zk) ** 2 for zk in z] + [0.0]
        u = [-beta * d2[k] + lam[k] + phi * (k == m) for k in range(ASTENSIONE + 1)]
        mx = max(u)
        y = rng.choices(range(ASTENSIONE + 1), [math.exp(a - mx) for a in u])[0]
        oss.append({"d2": d2, "m": m, "y": y})
    return oss


def da_righe(righe, fedele_nr=False, proprie=False, solo_votanti=False, centro_nc=False):
    """Osservazioni da righe del post ITANES 2022. Ritorna (oss, z, scarti); x e z sui 7 punti del gioco."""
    def num(r, c):
        try:
            return float(r[c])
        except (KeyError, ValueError, TypeError):  # TypeError: riga corta, DictReader dà None
            return None
    def pos(v):  # codici 1..11 = 0..10 -> 7 punti 1..7; 96-99 = non collocato
        return TAGLIO[int(v) - 1] + 1 if v is not None and 1 <= v <= 11 else None
    pp = [[pos(num(r, f"Q18{c}")) for c in "abcdefgh"] for r in righe]
    z = [sum(v) / len(v) for v in ([p[k] for p in pp if p[k] is not None] for k in range(8))]
    oss = []
    scarti = {"senza_autocollocazione": 0, "voto_22_non_risponde": 0, "voto_22_fuori_scelta": 0, "voto_18_ignoto": 0}
    for r, p in zip(righe, pp):
        x = pos(num(r, "Q19"))
        if x is None and centro_nc:
            x = TAGLIO[5] + 1  # non collocati al centro, come suggerito in dati-per-tarare-i-gruppi:91
        if x is None:
            scarti["senza_autocollocazione"] += 1
            continue
        v22 = int(num(r, "voto_post") or 0)
        if num(r, "Q10LHa") == 2:
            y = ASTENSIONE
        elif v22 in (0, 98, 99):
            scarti["voto_22_non_risponde"] += 1
            continue
        else:
            y = VOTO22.get(v22)
        if y is None or (solo_votanti and y == ASTENSIONE):
            scarti["voto_22_fuori_scelta"] += 1
            continue
        a, b = num(r, "Q14a"), int(num(r, "Q14b") or 0)
        m = ASTENSIONE if a == 2 else None if a == 3 else EREDE18.get(b) if a == 1 and b in EREDE18 else "ignoto"
        if m == "ignoto":
            if not fedele_nr:
                scarti["voto_18_ignoto"] += 1
                continue
            m = None
        if solo_votanti and m == ASTENSIONE:
            m = None
        zz = [pk if proprie and pk is not None else zk for pk, zk in zip(p, z)]
        d2 = [(x - a) ** 2 for a in zz]
        oss.append({"d2": d2 if solo_votanti else d2 + [0.0], "m": m, "y": y})
    return oss, z, scarti


def leggi(cartella, **kw):
    fs = glob.glob(os.path.join(cartella, "*POST*weighted.tab"))
    if len(fs) != 1:
        sys.exit(f"cerco un solo *POST*weighted.tab in {cartella}, trovati {len(fs)}: scarica il post ITANES 2022 in {CACHE} o passa --itanes CARTELLA (README, sezione Stima di beta e fedeltà)")
    with open(fs[0], encoding="utf-8", newline="") as f:
        return da_righe(list(csv.DictReader(f, delimiter="\t")), **kw)


def scrivi(path, b7, se_b7, phi, se_phi, phi_min, phi_max, n):
    """Scrive valore e commento di beta_distanza e fedelta; fuori da [min, max] esce senza scrivere."""
    t = open(path, encoding="utf-8").read()
    cit = "ITANES 2022 post (Vezzoni et al. 2023, release 01), N=%d" % n
    for sez, v, com in (
        ("beta_distanza", b7, f"{cit}: β {b7:.3f} (ES {se_b7:.3f}) sui 7 punti del gioco (TAGLIO di elettorato.py); min allargato da 0,3 a 0,1 perché la stima cade sotto 0,3; dati-per-tarare-i-gruppi:99-111; stima_voto.py"),
        ("fedelta", phi, f"{cit}: φ {phi:.2f} (ES {se_phi:.2f}; fra specificazioni {phi_min:.2f}-{phi_max:.2f}), tetto: il ricordo del 2018 si sposta verso il 2022; dati-per-tarare-i-gruppi:99-111; stima_voto.py"),
    ):
        lim = re.search(r"\[" + sez + r"\]\nvalore = [^\n]*\nmin = (-?[\d.]+)\nmax = (-?[\d.]+)", t)
        if lim is None:
            raise ValueError(f"[{sez}] non trovata in manopole.toml")
        if not float(lim[1]) <= v <= float(lim[2]):
            sys.exit(f"{sez} = {v:.3f} fuori da [{lim[1]}, {lim[2]}]: allarga a mano min/max, dichiaralo, rilancia. Niente scritto.")
        pat = re.compile(r"(?:^# [^\n]*\n)?(\[" + sez + r"\]\nvalore = )[^\n]*", re.M)
        t = pat.sub(lambda m: f"# {com}\n{m.group(1)}{v:.{3 if sez == 'beta_distanza' else 2}f}", t, count=1)
    open(path, "w", encoding="utf-8").write(t)


def selftest():
    z = [1.5, 2.5, 3.0, 4.0, 4.5, 3.5, 4.0, 2.0]
    vero = [0.35, 1.2] + [0.3, 0.1, 0.0, -0.4, 0.2, 0.5, -0.8, -0.2]
    th, se, _ = stima(sintetici(30000, vero, z, 20261023))
    for i, nome in ((0, "beta"), (1, "phi")):
        err = abs(th[i] - vero[i]) / vero[i]
        verifica(err < 0.05, f"{nome}: {th[i]:.4f} contro {vero[i]} (errore {err:.1%})")
        verifica(abs(th[i] - vero[i]) < 4 * se[i], f"{nome}: vero fuori da 4 errori standard")
    cop = [0, 0]  # copertura a 2 ES su 40 repliche piccole: gli ES non possono essere miseramente stretti
    for s in range(40):
        t2, s2, _ = stima(sintetici(1500, vero, z, s))
        for i in (0, 1):
            cop[i] += abs(t2[i] - vero[i]) < 2 * s2[i]
    verifica(min(cop) >= 32, f"copertura ES a 2 sigma: beta {cop[0]}/40, phi {cop[1]}/40")
    # il TAGLIO e' quello di elettorato.py
    src = open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "elettorato.py"), encoding="utf-8").read()
    verifica("TAGLIO = " + str(TAGLIO) in src, "TAGLIO diverso da elettorato.py")
    # righe .tab sintetiche, passate da da_righe: y, m e scarti attesi a mano
    def riga(q19, voto, q10a, q14a, q14b):
        r = {f"Q18{c}": "1" for c in "abcdefgh"}
        r.update(Q19=q19, voto_post=voto, Q10LHa=q10a, Q14a=q14a, Q14b=q14b)
        return r
    righe = [riga("6", "94", "2", "2", "94"),    # astensione 2022 e 2018: y=8, m=8; Q19=6 -> 5 -> TAGLIO 3 -> 4
             riga("11", "6", "1", "1", "8"),     # PD, 2018 Potere al popolo: senza erede, m=None; Q19=11 -> 10 -> 6 -> 7
             riga("1", "9", "1", "1", "7"),      # AVS, 2018 LeU: erede AVS, m=7
             riga("5", "1", "1", "3", "94"),     # FdI, troppo giovane nel 2018: m=None
             riga("5", "1", "1", "5", "94"),     # 2018 non ricordato: scartato
             riga("96", "1", "1", "1", "3"),     # non collocato: scartato
             riga("5", "12", "1", "1", "3"),     # voto 2022 fuori scelta
             riga("5", "98", "1", "1", "3")]     # voto 2022 non risponde
    oss, zz, sc = da_righe(righe)
    verifica([(o["y"], o["m"]) for o in oss] == [(8, 8), (5, None), (7, 7), (0, None)], f"y/m: {[(o['y'], o['m']) for o in oss]}")
    verifica(sc == {"senza_autocollocazione": 1, "voto_22_non_risponde": 1, "voto_22_fuori_scelta": 1, "voto_18_ignoto": 1}, f"scarti {sc}")
    verifica(zz == [1.0] * 8, f"z (Q18 = 1 -> punto 1): {zz}")
    # x = 4, 7, 1, 3 sui 7 punti (Q19 tagliato) e z = 1: d2 = 3^2, 6^2, 0, 2^2
    verifica([o["d2"][0] for o in oss] == [9.0, 36.0, 0.0, 4.0], f"d2: {[o['d2'][0] for o in oss]}")
    # allineamento: lettera Q18 k <-> PARTITI[k] <-> VOTO22; codici 1..8 danno z = punti 1, 1, 2, 2, 3, 4, 5, 6; una riga con Q18f = 96 non conta in z
    verifica(PARTITI[5] == "PD" and VOTO22[6] == 5, "VOTO22[6] (PD) deve puntare a PARTITI[5]")

    def distinta(q18f=None):
        r = riga("6", "6", "1", "1", "3")
        r.update({f"Q18{c}": str(i + 1) for i, c in enumerate("abcdefgh")})
        if q18f:
            r["Q18f"] = q18f
        return r
    oss, zz, sc = da_righe([distinta(), distinta(), distinta("96")])
    verifica(zz == [1.0, 1.0, 2.0, 2.0, 3.0, 4.0, 5.0, 6.0], f"z per lettera: {zz}")
    verifica([o["y"] for o in oss] == [5, 5, 5] and oss[0]["d2"][5] == 0.0 and oss[0]["d2"][0] == 9.0 and oss[0]["d2"][7] == 4.0, f"d2 per lettera: {oss[0]['d2']}")
    # le quattro varianti, una o due righe ciascuna, numeri fatti a mano
    astenuto = riga("5", "1", "1", "2", "94")  # FdI nel 2022, astensione nel 2018
    oss, _, sc = da_righe([riga("6", "94", "2", "2", "94"), astenuto], solo_votanti=True)
    verifica(len(oss) == 1 and oss[0]["m"] is None and len(oss[0]["d2"]) == 8 and sc["voto_22_fuori_scelta"] == 1, f"solo_votanti: {oss} {sc}")
    oss, _, _ = da_righe([riga("96", "1", "1", "1", "3")], centro_nc=True)
    verifica(len(oss) == 1 and oss[0]["d2"][0] == 9.0, f"centro_nc: x = 4, z = 1: {oss}")
    oss, _, _ = da_righe([riga("5", "1", "1", "5", "94")], fedele_nr=True)
    verifica(len(oss) == 1 and oss[0]["m"] is None, f"fedele_nr: {oss}")
    a, b = riga("1", "1", "1", "1", "3"), riga("1", "1", "1", "1", "3")
    a["Q18a"] = "11"  # punto 7 per questa riga, z[0] = (7 + 1) / 2 = 4
    verifica([o["d2"][0] for o in da_righe([a, b])[0]] == [9.0, 9.0] and [o["d2"][0] for o in da_righe([a, b], proprie=True)[0]] == [36.0, 0.0], "proprie")
    # solo_votanti toglie il riferimento: la stima a 8 alternative gira (P = 9)
    senza = [{"d2": o["d2"][:8], "m": None if o["m"] == 8 else o["m"], "y": o["y"]} for o in sintetici(3000, vero, z, 5) if o["y"] != 8]
    verifica(len(stima(senza)[0]) == 9, "stima a 8 alternative")
    # riga corta (DictReader mette None): scartata, non TypeError
    corta = riga("6", "1", "1", "1", "3")
    corta["Q19"] = None
    verifica(da_righe([corta])[0] == [], "riga corta")
    # scrivi: cambia solo le due voci, accetta un min negativo, fuori intervallo non scrive
    orig = ("# vecchio\n[beta_distanza]\nvalore = 1.0\nmin = 0.1\nmax = 3.0\nticket = \"14\"\n\n"
            "# vecchio\n[fedelta]\nvalore = 1.5\nmin = -1.0\nmax = 4.0\nticket = \"14\"\n")
    with tempfile.TemporaryDirectory() as d:
        p = os.path.join(d, "m.toml")
        open(p, "w").write(orig)
        scrivi(p, 0.2, 0.01, 1.1, 0.1, 1.0, 1.2, 100)
        nuovo = open(p).read()
        verifica("valore = 0.200\n" in nuovo and "valore = 1.10\n" in nuovo and "vecchio" not in nuovo, "scrivi")
        verifica(nuovo.count("min = ") == 2 and 'ticket = "14"' in nuovo and "min = -1.0" in nuovo, "scrivi: altro toccato")
        open(p, "w").write(orig)
        try:
            scrivi(p, 5.0, 0.01, 1.1, 0.1, 1.0, 1.2, 100)
            raise AssertionError("valore fuori intervallo accettato")
        except SystemExit:
            verifica(open(p).read() == orig, "scritto nonostante il valore fuori intervallo")
        try:
            scrivi(p, 0.2, 0.01, 9.0, 0.1, 1.0, 1.2, 100)
            raise AssertionError("phi fuori intervallo accettato")
        except SystemExit:
            verifica(open(p).read() == orig, "scritto beta nonostante phi fuori intervallo")
    # cartella senza dati: errore chiaro
    with tempfile.TemporaryDirectory() as d:
        try:
            leggi(d)
            raise AssertionError("cartella vuota accettata")
        except SystemExit as e:
            verifica("POST" in str(e), "messaggio")
    # un solo giro non basta: Newton che non converge deve dirlo
    try:
        stima(sintetici(500, vero, z, 1), iters=1)
        raise AssertionError("non convergenza ignorata")
    except RuntimeError:
        pass
    print("selftest ok: beta", round(th[0], 4), "phi", round(th[1], 4), f"copertura 2ES beta {cop[0]}/40 phi {cop[1]}/40")


VARIANTI = [("base", {}), ("posizioni proprie dei partiti", {"proprie": True}),
            ("voto 2018 non ricordato = senza termine", {"fedele_nr": True}),
            ("solo votanti", {"solo_votanti": True}), ("non collocati al centro", {"centro_nc": True})]


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--itanes", default=CACHE, help="cartella con il post ITANES 2022 (.tab), fuori dal repo")
    ap.add_argument("--manopole", default=MANOPOLE)
    ap.add_argument("--solo-stima", action="store_true", help="non scrivere manopole.toml")
    ap.add_argument("--selftest", action="store_true")
    a = ap.parse_args()
    if a.selftest:
        return selftest()
    ris = {}
    for nome, kw in VARIANTI:
        oss, z, sc = leggi(a.itanes, **kw)
        th, se, ll = stima(oss)
        ris[nome] = (th, se, len(oss))
        print(f"[{nome}] N={len(oss)} righe scartate={sc} log-verosimiglianza={ll:.1f}")
        print(f"  beta (scala a 7 punti) = {th[0]:.4f} (ES {se[0]:.4f})  phi = {th[1]:.3f} (ES {se[1]:.3f})")
        if nome == "base":
            print("  z partiti:", {p: round(v, 2) for p, v in zip(PARTITI, z)})
            print("  lambda:", {p: f"{v:.2f}±{s:.2f}" for p, v, s in zip(PARTITI, th[2:], se[2:])})
    th, se, n = ris["base"]
    fi = [v[0][1] for v in ris.values()]
    if not a.solo_stima:
        scrivi(a.manopole, th[0], se[0], th[1], se[1], min(fi), max(fi), n)
        print("scritto", a.manopole)


if __name__ == "__main__":
    main()
