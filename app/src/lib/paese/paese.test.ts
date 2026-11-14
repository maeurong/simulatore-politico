import { describe, expect, it } from "vitest";
import type { Vista } from "$lib/tipi/Vista";
import { LIVELLI, affluenzaPerRegione, consensoPerRegione, lista, livelloManca, media, segniPerRegione, sondaggioLetto } from "./paese";
import { coalizioneDi } from "$lib/partiti";
import turnoZeroJson from "../../../../.scratch/v1-piano-11-ui/viste/01-turno-zero.json";
import paeseJson from "../../../../.scratch/v1-piano-11-ui/viste/23-paese.json";

const zero = turnoZeroJson as unknown as Vista;
const campione = paeseJson as unknown as Record<string, { vista: Vista }>;
const governo = campione.governo.vista;
const grave = campione.grave.vista;

describe("livelloManca", () => {
  it("consenso e affluenza hanno i dati", () => {
    expect(livelloManca("consenso")).toBeNull();
    expect(livelloManca("affluenza")).toBeNull();
  });
  it("gli altri quattro arrivano con la 11g", () => {
    const altri = LIVELLI.filter((l) => l !== "consenso" && l !== "affluenza");
    expect(altri).toHaveLength(4);
    for (const l of altri) expect(livelloManca(l)).toBe("11g");
  });
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
  it("venti valori fra 0 e 1", () => {
    const v = Object.values(affluenzaPerRegione(zero));
    expect(v).toHaveLength(20);
    for (const x of v) expect(x!).toBeGreaterThan(0), expect(x!).toBeLessThan(1);
  });
});

describe("segniPerRegione", () => {
  it("l'emergenza aperta va sull'id della regione che porta per nome", () => {
    expect(grave.eventi[0].regione).toBe("Lazio");
    expect(segniPerRegione(grave).lazio).toContain("emergenza");
  });
  it("cantieri e atti in agenda con regione danno i loro segni", () => {
    const tutti = Object.values(segniPerRegione(governo)).flat();
    const attesi = governo.ministeri.flatMap((m) => [...(m.agenda?.regione ? ["atto"] : []), ...m.cantieri.filter((c) => c.regione).map(() => "cantiere")]);
    expect(tutti.filter((s) => s !== "emergenza").sort()).toEqual(attesi.sort());
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
