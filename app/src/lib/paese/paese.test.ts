import { describe, expect, it, vi } from "vitest";
import type { Vista } from "$lib/tipi/Vista";
import { LIVELLI, affluenzaPerRegione, consensoPerRegione, disposizioniDi, classi, lista, livelloManca, margineIstituti, margineTesto, media, segniDi, segniPerRegione, sessioneElettori, sondaggioDi, sondaggioLetto, territorioDi, valoriDi, votoPerRegione, type EsitoElettori } from "./paese";
import type { ElettoreCampione } from "$lib/tipi/ElettoreCampione";
import { coalizioneDi } from "$lib/partiti";
import { REGIONI } from "$lib/mappa/regioni";
import turnoZeroJson from "../../../../.scratch/v1-piano-11-ui/viste/01-turno-zero.json";
import paeseJson from "../../../../.scratch/v1-piano-11-ui/viste/23-paese.json";
import cruscottoJson from "../../../../.scratch/v1-piano-11-ui/viste/17-cruscotto.json";
import ereditaJson from "../../../../.scratch/v1-piano-11-ui/viste/30-eredita.json";

const zero = turnoZeroJson as unknown as Vista;
const campione = paeseJson as unknown as Record<string, { vista: Vista }>;
const governo = campione.governo.vista;
const grave = campione.grave.vista;
const dieci = (cruscottoJson as unknown as Record<string, { vista: Vista }>).crisi_provvisorio.vista;
const eredita = (ereditaJson as unknown as { vista: Vista }).vista;

/** La quota sui votanti di un partito, dal JSON grezzo: media degli istituti di quote[j] / (1 − astensione). */
const quotaGrezza = (v: Vista, j: number) => v.sondaggio.istituti.reduce((a, r) => a + r.quote[j] / (1 - r.quote.at(-1)!), 0) / v.sondaggio.istituti.length;
const somma = (l: { quota: number }[]) => l.reduce((a, x) => a + x.quota, 0);

describe("livelloManca", () => {
  it("senza elezioni il voto manca, con le elezioni no, gli altri livelli hanno i dati", () => {
    expect(livelloManca("voto", zero)).toBe("paese.voto_nessuno");
    expect(livelloManca("voto", governo)).toBeNull();
    for (const l of LIVELLI.filter((l) => l !== "voto")) expect(livelloManca(l, zero)).toBeNull();
  });
});

describe("livelli del territorio", () => {
  const lombardia = governo.regioni.find((r) => r.id === "lombardia")!;
  it("valoriDi: venti regioni, i valori della Vista", () => {
    const d = valoriDi(governo, "disoccupazione");
    expect(Object.keys(d)).toHaveLength(20);
    expect(d.lombardia).toBe(lombardia.disoccupazione);
    expect(valoriDi(governo, "reddito").lombardia).toBe(lombardia.reddito);
    expect(valoriDi(governo, "pericolosita", "sisma").lombardia).toBe(lombardia.pericolosita.sisma);
  });
  it("classi: pericolosità a passi di 0,2; altrimenti uguali fra min e max; valori uguali una classe", () => {
    const p = classi([0.1, 0.5, 0.9], "pericolosita");
    expect([p.classe(0.5), p.classe(1), p.classe(0)]).toEqual([2, 4, 0]);
    const d = classi([0, 10], "reddito");
    expect([d.classe(0), d.classe(5), d.classe(10)]).toEqual([0, 2, 4]);
    const u = classi([5, 5, 5], "disoccupazione");
    expect(u.soglie).toEqual([]);
    expect(u.classe(5)).toBe(0);
  });
  it("classi: sulle soglie 0,2 0,4 0,6 0,8 il valore passa alla classe sopra", () => {
    const p = classi([0, 1], "pericolosita");
    expect([0.1999, 0.2, 0.4, 0.6, 0.8, 0.9999].map(p.classe)).toEqual([0, 1, 2, 3, 4, 4]);
  });
  it("classi: pericolosità con valori tutti uguali tiene i passi fissi, non una classe sola", () => {
    const p = classi([0.5, 0.5], "pericolosita");
    expect(p.soglie).toEqual([0.2, 0.4, 0.6, 0.8]);
    expect(p.classe(0.5)).toBe(2);
  });
  it("classi: senza valori non ci sono soglie e ogni valore è nella classe 0", () => {
    const u = classi([], "reddito");
    expect([u.soglie, u.classe(7)]).toEqual([[], 0]);
  });
  it("valoriDi: la pericolosità senza tipo è il totale", () => {
    expect(valoriDi(governo, "pericolosita").lombardia).toBe(lombardia.pericolosita.totale);
  });
  it("votoPerRegione: una regione assente dall'ultima elezione non ha voce", () => {
    const e = governo.elezioni.at(-1)!;
    const v = { ...governo, elezioni: [{ ...e, regioni: e.regioni.filter((r) => r.regione !== "lombardia") }] } as Vista;
    expect(votoPerRegione(v)!.lombardia).toBeUndefined();
    expect(votoPerRegione(v)!.toscana).toBeDefined();
  });
  it("votoPerRegione: l'ultima elezione, null senza", () => {
    expect(votoPerRegione(zero)).toBeNull();
    expect(votoPerRegione(governo)!.lombardia.primo).toBe(governo.elezioni.at(-1)!.regioni.find((r) => r.regione === "lombardia")!.primo);
  });
  it("territorioDi: terzi e regione ignota", () => {
    const x = territorioDi(governo, "lombardia")!;
    expect(x.macroarea).toBe(lombardia.macroarea);
    expect(x.pericolosita.sisma).toBe(lombardia.pericolosita.sisma < 1 / 3 ? "bassa" : lombardia.pericolosita.sisma < 2 / 3 ? "media" : "alta");
    expect(territorioDi(governo, "atlantide")).toBeNull();
  });
  it("le opere finite sono segni, col trimestre del turno", () => {
    const v = governo;
    const o = v.opere[0];
    const s = segniDi(v, o.regione).find((x) => x.segno === "opera")!;
    expect(s.testo).toContain(o.atto);
    expect(s.testo).toMatch(/\d{4}$/);
  });
  it("il trimestre dell'opera: un turno è un trimestre, anche a cavallo d'anno", () => {
    const o = governo.opere[0];
    const v = { ...governo, turno: 10, data: { anno: 2030, trimestre: 1 }, opere: [10, 9, 6, 5].map((turno) => ({ ...o, turno })) };
    expect(segniDi(v, o.regione).map((x) => x.testo.split(" · ")[1])).toEqual(["gennaio–marzo 2030", "ottobre–dicembre 2029", "gennaio–marzo 2029", "ottobre–dicembre 2028"]);
  });
});

it("margineTesto: in punti, con la virgola", () => {
  expect(margineTesto(0.0447)).toBe("± 4,5 punti");
});

it("margineIstituti: intervallo se diversi, un valore se uguali", () => {
  const con = (...m: number[]) => ({ istituti: m.map((margine) => ({ margine })) }) as never;
  expect(margineIstituti(con(0.03, 0.05))).toBe("margine degli istituti da ±3,0 a ±5,0 punti");
  expect(margineIstituti(con(0.044, 0.044))).toBe("± 4,4 punti");
  expect(margineIstituti(con(0.044))).toBe("± 4,4 punti");
  expect(margineIstituti(con())).toBe("");
});

describe("consensoPerRegione", () => {
  const per = consensoPerRegione(governo, coalizioneDi(governo));
  const valori = Object.entries(per) as [string, number][];
  const ordinati = [...valori].sort((a, b) => a[1] - b[1]);
  it("la coalizione di governo va dal 37,6% della Basilicata al 69,2% della Toscana", () => {
    expect(ordinati[0][0]).toBe("basilicata");
    expect(+(ordinati[0][1] * 100).toFixed(1)).toBe(37.6);
    expect(ordinati.at(-1)![0]).toBe("toscana");
    expect(+(ordinati.at(-1)![1] * 100).toFixed(1)).toBe(69.2);
  });
  it("un partito fuori da Vista.partiti non azzera gli altri; da solo non dà dato", () => {
    const solo = consensoPerRegione(governo, ["giocatore"]);
    expect(consensoPerRegione(governo, ["giocatore", "nato_in_partita"])).toEqual(solo);
    expect(Object.values(consensoPerRegione(governo, ["nato_in_partita"])).every((x) => x === null)).toBe(true);
  });
});

describe("un partito nato dopo il sondaggio", () => {
  // `zero` ha nove quote: otto partiti e l'astensione. Il nono partito della Vista non ha riga e non deve prendere quella dell'astensione.
  const nato = { ...zero.partiti[0], id: "nuovo_8", nome: "Nuovo" };
  const v = { ...zero, partiti: [...zero.partiti, nato] } as Vista;
  it("non è nella media né nel sondaggio della regione, e gli altri sommano ancora a uno", () => {
    const m = media(v, v.sondaggio);
    expect(m.partiti).toHaveLength(8);
    expect(m.partiti.some((p) => p.id === "nuovo_8")).toBe(false);
    expect(somma(m.partiti)).toBeCloseTo(1, 9);
    const r = sondaggioDi(v, "toscana");
    expect(r).toHaveLength(8);
    expect(somma(r)).toBeCloseTo(1, 9);
  });
  it("sulla mappa non cambia il consenso degli altri", () => {
    expect(consensoPerRegione(v, ["giocatore", "nuovo_8"])).toEqual(consensoPerRegione(v, ["giocatore"]));
  });
});

describe("con dieci partiti, due nati", () => {
  it("la media ha tutti e dieci i partiti, ognuno alla quota della sua posizione, e somma a uno", () => {
    const m = media(dieci, dieci.sondaggio);
    expect(m.partiti).toHaveLength(10);
    expect(somma(m.partiti)).toBeCloseTo(1, 9);
    for (const id of ["nuovo_8", "nuovo_9"]) {
      expect(m.partiti.find((p) => p.id === id)!.quota).toBeCloseTo(quotaGrezza(dieci, dieci.partiti.findIndex((p) => p.id === id)), 12);
    }
  });
  it("il sondaggio della regione e la mappa li contano", () => {
    expect(sondaggioDi(dieci, "toscana")).toHaveLength(10);
    const per = consensoPerRegione(dieci, ["nuovo_8", "nuovo_9"]);
    expect(Object.values(per).every((x) => x !== null && x > 0)).toBe(true);
  });
});

describe("un partito sciolto non compare", () => {
  const sciolto = eredita.partiti.find((p) => p.sciolto)!;
  it("sulla 30 l'Estrema sinistra, sciolta, non è nella media: gli altri nove sommano a uno", () => {
    expect(sciolto.id).toBe("estrema_sinistra");
    const m = media(eredita, eredita.sondaggio);
    expect(m.partiti).toHaveLength(9);
    expect(m.partiti.some((p) => p.id === sciolto.id)).toBe(false);
    expect(somma(m.partiti)).toBeLessThan(1 + 1e-9);
    expect(somma(m.partiti)).toBeCloseTo(1 - quotaGrezza(eredita, 0), 9);
  });
  it("e nemmeno nel sondaggio della regione", () => {
    const r = sondaggioDi(eredita, "toscana");
    expect(r).toHaveLength(9);
    expect(r.some((p) => p.id === sciolto.id)).toBe(false);
  });
});

describe("sondaggioLetto", () => {
  it("il privato comprato prende il posto del pubblico", () => {
    const l = sondaggioLetto(governo);
    expect(l.privato).toBe(true);
    expect(l.sondaggio).toBe(governo.acquisti.sondaggio_privato);
  });
  it("senza acquisto resta il pubblico", () => {
    const l = sondaggioLetto(zero);
    expect(l.privato).toBe(false);
    expect(l.sondaggio).toBe(zero.sondaggio);
  });
});

describe("affluenzaPerRegione", () => {
  it("uno meno l'astensione della regione: la Toscana, dal JSON grezzo", () => {
    const toscana = zero.sondaggio.regioni.find((r) => r.id === "toscana")!;
    const per = affluenzaPerRegione(zero);
    expect(Object.keys(per)).toHaveLength(20);
    expect(per.toscana).toBeCloseTo(1 - toscana.quote.at(-1)!, 12);
  });
  it("una regione senza quote non ha dato", () => {
    const v = { ...zero, sondaggio: { ...zero.sondaggio, regioni: zero.sondaggio.regioni.map((r) => (r.id === "toscana" ? { ...r, quote: [] } : r)) } } as Vista;
    expect(affluenzaPerRegione(v).toscana).toBeNull();
  });
});

describe("segniPerRegione", () => {
  it("l'emergenza aperta va sull'id della regione che porta per nome", () => {
    expect(grave.eventi[0].regione).toBe("Lazio");
    expect(segniPerRegione(grave).lazio).toContain("emergenza");
  });
  it("solo la famiglia emergenza dà il segno: shock, scandalo e gli altri con regione no", () => {
    const altri = (["situazione", "scandalo", "shock", "debito"] as const).map((famiglia) => ({ ...grave.eventi[0], famiglia, regione: "Toscana" }));
    const v = { ...grave, eventi: [...grave.eventi, ...altri] };
    const segni = segniPerRegione(v);
    expect(segni.toscana).not.toContain("emergenza");
    expect(segni.lazio).toEqual(["emergenza"]);
  });
  it("sulla 23 governo: un atto in Sicilia, un cantiere in Umbria, un'emergenza nelle Marche", () => {
    expect(segniPerRegione(governo)).toEqual({ sicilia: ["atto"], toscana: ["opera", "opera"], umbria: ["cantiere"], marche: ["emergenza"] });
  });
  it("ogni regione che il core nomina in un'emergenza trova il suo id, con l'apostrofo dritto o curvo", () => {
    for (const r of REGIONI) {
      for (const nome of [r.nome, r.nome.replace("’", "'"), r.nome.replace("'", "’")]) {
        const v = { ...zero, eventi: [{ ...grave.eventi[0], famiglia: "emergenza", regione: nome }] } as Vista;
        expect(segniPerRegione(v)[r.id], nome).toEqual(["emergenza"]);
      }
    }
  });
  it("senza atti, cantieri ed emergenze è vuoto", () => {
    expect(segniPerRegione(zero)).toEqual({});
  });
});

describe("lista", () => {
  const valori = consensoPerRegione(governo, ["giocatore"]);
  it("per valore è decrescente e i null stanno in fondo", () => {
    const l = lista({ ...valori, lazio: null, umbria: null }, "valore");
    expect(l).toHaveLength(20);
    expect(l.slice(-2).map((r) => r.valore)).toEqual([null, null]);
    const dati = l.slice(0, -2).map((r) => r.valore!);
    expect(dati).toEqual([...dati].sort((a, b) => b - a));
  });
  it("per nome è in ordine alfabetico italiano, col nome della regione", () => {
    const l = lista(valori, "nome");
    expect(l[0].nome).toBe("Abruzzo");
    expect(l.map((r) => r.nome)).toEqual([...l.map((r) => r.nome)].sort((a, b) => a.localeCompare(b, "it")));
    expect(l.find((r) => r.id === "valle_d_aosta")?.nome).toBe("Valle d’Aosta");
  });
});

describe("media", () => {
  it("le quote dei partiti sui votanti e l'astensione sommano a uno", () => {
    for (const v of [zero, governo]) {
      const m = media(v, sondaggioLetto(v).sondaggio);
      const votanti = m.partiti.reduce((s, p) => s + p.quota, 0);
      expect(votanti).toBeCloseTo(1, 9);
      expect(votanti * (1 - m.astensione) + m.astensione).toBeCloseTo(1, 9);
    }
  });
  it("senza istituti non c'è niente da mostrare", () => {
    expect(media(zero, { ...zero.sondaggio, istituti: [] })).toEqual({ partiti: [], astensione: 0 });
  });
});

describe("disposizioniDi", () => {
  it("in Campania valgono quelle col perimetro che la contiene, mai quelle di tutto il Paese", () => {
    const d = disposizioniDi(governo, "campania");
    expect(d.map((x) => x.id)).toEqual(["decontribuzione", "credito_zes"]);
    expect(governo.disposizioni.some((x) => x.perimetro.length === 0)).toBe(true);
    expect(d.every((x) => x.perimetro.includes("campania"))).toBe(true);
  });
  it("una regione fuori da ogni perimetro non ne ha", () => {
    expect(disposizioniDi(governo, "toscana")).toEqual([]);
  });
});

describe("sondaggioDi", () => {
  it("una voce per partito attivo, dal più al meno votato, e sommano a uno", () => {
    for (const v of [zero, governo]) {
      const s = sondaggioDi(v, "toscana");
      expect(s).toHaveLength(v.partiti.filter((p) => !p.sciolto).length);
      expect(s.reduce((a, p) => a + p.quota, 0)).toBeCloseTo(1, 9);
      expect(s.map((p) => p.quota)).toEqual([...s.map((p) => p.quota)].sort((a, b) => b - a));
    }
  });
  it("col privato comprato legge il privato, non il pubblico", () => {
    const privato = sondaggioDi(governo, "toscana");
    const pubblico = sondaggioDi({ ...governo, acquisti: { ...governo.acquisti, sondaggio_privato: null } }, "toscana");
    expect(privato.map((p) => p.quota)).not.toEqual(pubblico.map((p) => p.quota));
  });
  it("una regione sconosciuta non dà voci", () => {
    expect(sondaggioDi(zero, "atlantide")).toEqual([]);
  });
});

describe("segniDi", () => {
  it("il cantiere dell'Umbria porta il nome dell'atto, per esteso", () => {
    const s = segniDi(governo, "umbria");
    expect(s).toHaveLength(1);
    expect(s[0].segno).toBe("cantiere");
    expect(s[0].testo).toMatch(/^cantiere in corso: .+/);
  });
  it("l'emergenza delle Marche porta nome e causa", () => {
    const s = segniDi(governo, "marche");
    expect(s.map((x) => x.segno)).toEqual(["emergenza"]);
    expect(s[0].testo).toContain("Emergenza idrogeologica");
  });
  it("una regione senza segni non ne dà", () => {
    expect(segniDi(zero, "umbria")).toEqual([]);
  });
});

describe("sessioneElettori", () => {
  const e = (nome: string): ElettoreCampione => ({ nome, eta: 30, posizione: 4, voto: null, motivo: "protesta" });
  function aMano() {
    const attese: { risolvi: (r: ElettoreCampione[]) => void; rifiuta: (x: unknown) => void }[] = [];
    const leggi = vi.fn((_r: string, _c: string, _f: string) => new Promise<ElettoreCampione[]>((risolvi, rifiuta) => attese.push({ risolvi, rifiuta })));
    const esiti: EsitoElettori[] = [];
    const sessione = sessioneElettori(leggi as never, (x) => esiti.push(x));
    return { leggi, attese, esiti, sessione };
  }
  const svuota = () => new Promise((r) => setTimeout(r, 0));

  it("cambiare casella o regione in volo mostra solo l'ultima scelta", async () => {
    const { leggi, attese, esiti, sessione } = aMano();
    sessione.chiedi("abruzzo", "autonomo", "alta", 3);
    sessione.chiedi("abruzzo", "autonomo", "bassa", 3);
    sessione.chiedi("molise", "autonomo", "bassa", 3);
    attese[0].risolvi([e("A")]);
    await svuota();
    expect(esiti).toEqual([]);
    expect(leggi.mock.calls.map((c) => c.join("|"))).toEqual(["abruzzo|autonomo|alta", "molise|autonomo|bassa"]);
    attese[1].risolvi([e("C")]);
    await svuota();
    expect(esiti).toEqual([{ elettori: [e("C")] }]);
  });
  it("la stessa casella a un turno nuovo si rilegge: la risposta del turno vecchio non si mostra", async () => {
    const { leggi, attese, esiti, sessione } = aMano();
    sessione.chiedi("abruzzo", "autonomo", "alta", 3);
    sessione.chiedi("abruzzo", "autonomo", "alta", 4);
    attese[0].risolvi([e("vecchio")]);
    await svuota();
    expect(esiti).toEqual([]);
    attese[1].risolvi([e("nuovo")]);
    await svuota();
    expect(leggi).toHaveBeenCalledTimes(2);
    expect(esiti).toEqual([{ elettori: [e("nuovo")] }]);
  });
  it("una risposta arrivata dopo l'annulla non si mostra", async () => {
    const { attese, esiti, sessione } = aMano();
    sessione.chiedi("abruzzo", "autonomo", "alta", 3);
    sessione.annulla();
    attese[0].risolvi([e("A")]);
    await svuota();
    expect(esiti).toEqual([]);
  });
  it("il rifiuto del core è un rifiuto senza prefisso, il guasto un guasto", async () => {
    const { attese, esiti, sessione } = aMano();
    sessione.chiedi("atlantide", "autonomo", "alta", 3);
    attese[0].rifiuta({ specie: "rifiuto", testo: "comando non ammesso: regione sconosciuta" });
    await svuota();
    sessione.chiedi("abruzzo", "autonomo", "alta", 3);
    attese[1].rifiuta({ specie: "guasto", testo: "core non risponde" });
    await svuota();
    expect(esiti).toEqual([{ rifiuto: "regione sconosciuta" }, { guasto: "core non risponde" }]);
  });
});
