import { describe, expect, it } from "vitest";
import type { Vista } from "$lib/tipi/Vista";
import type { AnteprimaSfiducia } from "$lib/tipi/AnteprimaSfiducia";
import opposizione from "../../../../.scratch/v1-piano-11-ui/viste/28-opposizione.json";
import { caricaAnteprime, conteggio, dicasteriConMinistro, quantiPassano } from "./sfiducia";

const stati = opposizione as unknown as Record<string, { prima?: Vista; vista?: Vista; anteprime?: { governo: { ok: AnteprimaSfiducia }; ministri: Record<string, { ok: AnteprimaSfiducia }> } }>;
const v = stati.sfiducia.prima!;
const a = (passa: boolean): AnteprimaSfiducia => ({ firme: true, linee: [], passa });

describe("dicasteriConMinistro", () => {
  it("salta i dicasteri in interim", () => {
    const interim = stati.fine.vista!;
    expect(dicasteriConMinistro(v)).toHaveLength(10);
    expect(dicasteriConMinistro(interim)).not.toContain("lavoro_welfare");
    expect(dicasteriConMinistro(interim)).toHaveLength(9);
  });
});

describe("caricaAnteprime", () => {
  it("chiede un dicastero alla volta, nell'ordine, e scrive ogni risposta", async () => {
    const chiamate: (string | null)[] = [];
    let inVolo = 0;
    const chiedi = async (d: string | null) => {
      expect(++inVolo).toBe(1);
      chiamate.push(d);
      await Promise.resolve();
      inVolo--;
      return a(d === "b");
    };
    const scritte: [string | null, AnteprimaSfiducia | null][] = [];
    await caricaAnteprime(v, ["a", "b", "c"], chiedi, () => v, (d, x) => scritte.push([d, x]));
    expect(chiamate).toEqual(["a", "b", "c"]);
    expect(scritte.map(([d, x]) => [d, x?.passa])).toEqual([["a", false], ["b", true], ["c", false]]);
  });
  it("la Vista cambia a metà: si ferma e le risposte vecchie non si scrivono", async () => {
    let corrente: Vista | null = v;
    const chiamate: (string | null)[] = [];
    const scritte: (string | null)[] = [];
    await caricaAnteprime(v, ["a", "b", "c"], async (d) => { chiamate.push(d); corrente = null; return a(true); }, () => corrente, (d) => scritte.push(d));
    expect(chiamate).toEqual(["a"]);
    expect(scritte).toEqual([]);
  });
  it("un guasto scrive null per quel dicastero e il giro continua", async () => {
    const scritte: [string | null, AnteprimaSfiducia | null][] = [];
    await caricaAnteprime(v, ["a", "b"], async (d) => { if (d === "a") throw new Error("guasto"); return a(true); }, () => v, (d, x) => scritte.push([d, x]));
    expect(scritte).toEqual([["a", null], ["b", a(true)]]);
  });
});

describe("quantiPassano", () => {
  it("conta i passa sul totale chiesto; null finché manca una risposta", () => {
    const m = new Map<string | null, AnteprimaSfiducia | null>([["a", a(true)], ["b", a(false)]]);
    expect(quantiPassano(m, ["a", "b"])).toBe(1);
    expect(quantiPassano(m, ["a", "b", "c"])).toBeNull();
    expect(quantiPassano(new Map([["a", null]]), ["a"])).toBeNull();
  });
});

describe("conteggio", () => {
  it("i sì e la soglia alla Camera e al Senato, dalle linee dell'anteprima del 28", () => {
    const g = stati.sfiducia.anteprime!.governo.ok;
    const camera = conteggio(v, g, "camera");
    const tot = v.seggi.reduce((s, x) => s + x.camera, 0);
    expect(camera.serve).toBe(Math.floor(tot / 2) + 1);
    const si = g.linee.filter((l) => l.si).reduce((s, l) => s + (v.seggi.find((x) => x.partito === l.partito)?.camera ?? 0), 0);
    expect(camera.si).toBe(si);
    expect(conteggio(v, g, "senato").serve).toBe(Math.floor(v.seggi.reduce((s, x) => s + x.senato, 0) / 2) + 1);
  });
});
