import { describe, expect, it } from "vitest";
import type { Vista } from "$lib/tipi/Vista";
import bilancio from "../../../../.scratch/v1-piano-11-ui/viste/04-governo-bilancio.json";
import opposizione from "../../../../.scratch/v1-piano-11-ui/viste/28-opposizione.json";
import { schema, type Filo, type Nodo } from "./schema";
import { RIGA_MIN, TESTA, tavola } from "./tavola";

const ordinario = bilancio as unknown as Vista;
const fine = (opposizione as unknown as Record<string, Record<string, Vista>>).fine.vista;

const nodo = (id: string, colonna: 0 | 1 | 2 | 3, tipo: Nodo["tipo"]): Nodo =>
  ({ id, colonna, tipo, nome: id, sotto: "sotto", spento: false, tuo: false, alGoverno: false });
const filo = (id: string, da: string, a: string, etichetta = id, contrario = false): Filo =>
  ({ id, da, a, tipo: "intesa", etichetta, frase: id, contrario, ritardo: false, cantiere: false, sezione: "partiti", fonte: "" });
// i due capi del tratto: M x,y ... x,y
const capi = (d: string) => {
  const n = [...d.matchAll(/-?\d+(\.\d+)?/g)].map((m) => +m[0]);
  return { x0: n[0], y0: n[1], x1: n.at(-2)!, y1: n.at(-1)! };
};

describe("tavola, geometria", () => {
  const W = 1000, gap = W * 0.055, cw = (W - gap * 3) / 4;
  const nodi = [nodo("p:a", 0, "partito"), nodo("p:b", 0, "partito"), nodo("camera", 1, "camera"), nodo("pdc", 2, "presidente"),
    nodo("d:x", 2, "ministero"), nodo("d:y", 2, "ministero"), nodo("mer", 3, "mercati"), nodo("r:z", 3, "paese")];

  it("quattro colonne fisse, larghe uguali, una riga per soggetto", () => {
    const T = tavola(nodi, [], W, 600);
    expect(T.colonne.map(Math.round)).toEqual([0, 1, 2, 3].map((c) => Math.round(c * (cw + gap))));
    expect(T.righe.map((r) => r.nodo.id)).toEqual(nodi.map((n) => n.id));
    for (const r of T.righe) expect(r.x).toBeCloseTo(T.colonne[r.nodo.colonna]);
  });

  it("righe dentro la colonna senza sovrapporsi; sotto il minimo la tavola si allunga invece di schiacciarle", () => {
    const molti = Array.from({ length: 20 }, (_, i) => nodo(`r:${i}`, 3, "paese"));
    const T = tavola(molti, [], W, 400);
    for (let i = 1; i < T.righe.length; i++) expect(T.righe[i].y - T.righe[i - 1].y).toBeGreaterThanOrEqual(RIGA_MIN);
    expect(T.altezza).toBeGreaterThan(400);
    expect(T.righe.at(-1)!.y + T.righe.at(-1)!.h / 2).toBeLessThanOrEqual(T.altezza);
    expect(tavola(nodi, [], W, 600).altezza).toBe(600);
    expect(T.righe[0].y - T.righe[0].h / 2).toBeGreaterThan(TESTA - 1);
  });

  it("riga: l'etichetta del primo filo di chi muove, «+N» per gli altri; niente etichetta su chi non muove", () => {
    const T = tavola(nodi, [filo("f1", "d:x", "r:z", "▲ subito", true), filo("f2", "d:x", "camera"), filo("f3", "d:x", "mer")], W, 600);
    const r = (id: string) => T.righe.find((x) => x.nodo.id === id)!;
    expect(r("d:x").etichetta).toEqual({ testo: "▲ subito +2", contrario: true });
    expect(r("r:z").etichetta).toBeNull();
  });

  it("un nome che non sta nel riquadro si accorcia con «…» secondo la misura data; quello che sta resta intero", () => {
    const lungo = { ...nodo("p:a", 0, "partito"), nome: "Partito dal nome lunghissimo" };
    const unPx = (s: string) => s.length;
    expect(tavola([lungo], [], W, 600, unPx).righe[0].nome).toBe("Partito dal nome lunghissimo");
    const stretto = tavola([lungo], [], 400, 600, (s) => s.length * 10).righe[0];
    expect(stretto.nome.endsWith("…")).toBe(true);
    expect(stretto.nome.length * 10).toBeLessThanOrEqual(stretto.w - 18);
  });

  it("riquadri scuri: Presidente, Camera e Senato, Mercati", () => {
    const T = tavola(nodi, [], W, 600);
    expect(T.righe.filter((r) => r.scura).map((r) => r.nodo.id)).toEqual(["camera", "pdc", "mer"]);
  });

  it("pettine: il filo verso il Presidente arriva dritto sulla sbarra del suo lato, all'altezza di chi parte", () => {
    const T = tavola(nodi, [filo("prop", "d:y", "pdc"), filo("int", "p:b", "pdc"), filo("ev", "r:z", "pdc"), filo("dim", "pdc", "d:x")], W, 600);
    const y = (id: string) => T.righe.find((r) => r.nodo.id === id)!.y;
    const tratto = (id: string) => capi(T.tratti.find((x) => x.filo.id === id)!.d);
    const sx = T.colonne[2] - gap * 0.4, dx = T.colonne[2] + cw + gap * 0.4;
    expect(tratto("prop")).toMatchObject({ x0: expect.closeTo(T.colonne[2], 0), x1: expect.closeTo(sx, 0) });
    expect(tratto("prop").y0).toBeCloseTo(y("d:y"), 0);
    expect(tratto("prop").y1).toBeCloseTo(y("d:y"), 0);
    expect(tratto("int")).toMatchObject({ x0: expect.closeTo(cw, 0), x1: expect.closeTo(sx, 0), y1: expect.closeTo(y("p:b"), 0) });
    expect(tratto("ev")).toMatchObject({ x0: expect.closeTo(T.colonne[3], 0), x1: expect.closeTo(dx, 0) });
    // dal Presidente al ministro: parte dalla sbarra, la punta sul ministro
    expect(tratto("dim")).toMatchObject({ x0: expect.closeTo(sx, 0), x1: expect.closeTo(T.colonne[2], 0), y0: expect.closeTo(y("d:x"), 0) });
    // una sbarra per lato, con una punta sola che entra nel Presidente
    expect(T.sbarre).toHaveLength(2);
    expect(T.sbarre.every((s) => s.punta)).toBe(true);
    expect(capi(T.sbarre[0].d)).toMatchObject({ x1: expect.closeTo(T.colonne[2], 0), y1: expect.closeTo(y("pdc"), 0) });
    expect(capi(T.sbarre[1].d)).toMatchObject({ x1: expect.closeTo(T.colonne[2] + cw, 0), y1: expect.closeTo(y("pdc"), 0) });
  });

  it("senza fili verso il Presidente niente sbarre; la sbarra col solo filo uscente non ha punta", () => {
    expect(tavola(nodi, [filo("f", "p:a", "camera")], W, 600).sbarre).toEqual([]);
    expect(tavola(nodi, [filo("dim", "pdc", "d:x")], W, 600).sbarre.map((s) => s.punta)).toEqual([false]);
  });

  it("il tondo del filo sta a metà del tratto; la riga porta i tratti che muove", () => {
    const T = tavola(nodi, [filo("prop", "d:y", "pdc"), filo("a", "p:a", "camera"), filo("b", "p:a", "mer")], W, 600);
    const y = (id: string) => T.righe.find((r) => r.nodo.id === id)!.y;
    const tr = (id: string) => T.tratti.find((x) => x.filo.id === id)!;
    expect(tr("prop")).toMatchObject({ x: expect.closeTo(T.colonne[2] - gap * 0.2, 0), y: expect.closeTo(y("d:y"), 0) });
    expect(tr("a")).toMatchObject({ x: expect.closeTo((cw + T.colonne[1]) / 2, 0), y: expect.closeTo((y("p:a") + y("camera")) / 2, 0) });
    expect(T.righe.find((r) => r.nodo.id === "p:a")!.uscenti.map((x) => x.filo.id)).toEqual(["a", "b"]);
    expect(T.righe.find((r) => r.nodo.id === "camera")!.uscenti).toEqual([]);
  });

  it("gli altri fili da bordo a bordo: verso destra dal lato destro di chi muove, nella stessa colonna un'ansa a sinistra", () => {
    const T = tavola(nodi, [filo("a", "p:a", "camera"), filo("b", "mer", "d:x"), filo("c", "camera", "camera")], W, 600);
    const tratto = (id: string) => capi(T.tratti.find((x) => x.filo.id === id)!.d);
    expect(tratto("a")).toMatchObject({ x0: expect.closeTo(cw, 0), x1: expect.closeTo(T.colonne[1], 0) });
    expect(tratto("b")).toMatchObject({ x0: expect.closeTo(T.colonne[3], 0), x1: expect.closeTo(T.colonne[2] + cw, 0) });
    expect(tratto("c").x0).toBeLessThan(T.colonne[1]);
  });
});

describe("tavola sulle Viste campione", () => {
  for (const [nome, v] of [["04 al governo", ordinario], ["28 fine", fine]] as const) {
    it(`${nome}: un tratto per filo disegnato, una riga per nodo`, () => {
      const s = schema(v), T = tavola(s.nodi, s.fili, 1100, 640);
      expect(T.tratti.map((x) => x.filo.id)).toEqual(s.fili.map((f) => f.id));
      expect(T.righe).toHaveLength(s.nodi.length);
      expect(T.tratti.every((x) => !x.d.includes("NaN"))).toBe(true);
    });
  }
});
