import { describe, expect, it, vi } from "vitest";
import type { Vista } from "$lib/tipi/Vista";
import ordinario from "../../../../.scratch/v1-piano-11-ui/viste/03-governo-ordinario.json";
import aula from "../../../../.scratch/v1-piano-11-ui/viste/19-aula.json";
import { cosaCambia, perimetro, quando, stimatore, type Misurabile } from "./anteprima";
import { ampiezzaPer, oscilla } from "./oscilla";

const v03 = ordinario as unknown as Vista;
const piena = (aula as unknown as Record<string, { vista: Vista }>).piena.vista;

describe("quando", () => {
  it("subito, turni, anni", () => {
    expect(quando(0)).toBe("subito");
    expect(quando(1)).toBe("dopo 1 turno");
    expect(quando(4)).toBe("dopo 4 turni");
    expect(quando(40)).toBe("dopo 40 turni · 10 anni");
  });
});

describe("cosaCambia", () => {
  it("salario minimo a 11 €/h: una riga, dal gradino in vigore a 11 €/h", () => {
    expect(cosaCambia(v03, [{ disposizione: "salario_minimo", gradino: "11" }])).toEqual([{ disposizione: "Salario minimo legale", ora: "nessuno", con: "11 €/h" }]);
  });
  it("una disposizione che la Vista non conosce non fa riga", () => {
    expect(cosaCambia(v03, [{ disposizione: "inesistente", gradino: "x" }])).toEqual([]);
  });
});

describe("stimatore", () => {
  it("il ministro del dicastero proponente, con la sua capacità", () => {
    expect(stimatore(v03, "lavoro_welfare")).toMatchObject({ nome: "Simone Ricci", dicastero: "Lavoro e Welfare", capacita: 2 });
  });
  it("senza proponente la stima è neutra", () => {
    expect(stimatore(v03, null)).toBeNull();
  });
});

describe("perimetro", () => {
  it("la ZES: le regioni della disposizione; il salario minimo: tutto il Paese (vuoto)", () => {
    const zes = v03.disposizioni.find((d) => d.id === "credito_zes")!;
    expect(zes.perimetro.length).toBeGreaterThan(0);
    expect(perimetro(v03, [{ disposizione: "credito_zes", gradino: "2_3" }])).toEqual(zes.perimetro);
    expect(perimetro(v03, [{ disposizione: "salario_minimo", gradino: "11" }])).toEqual([]);
  });
});

describe("Misurabile", () => {
  it("la voce del salario minimo in calendario porta costo ed effetti come un'anteprima", () => {
    const m: Misurabile = piena.calendario.find((x) => x.id === 3)!;
    expect(typeof m.costo_annuo).toBe("number");
    expect(m.effetti.length).toBeGreaterThan(0);
  });
});

describe("oscilla", () => {
  const el = () => ({ animate: vi.fn(() => ({}) as Animation) }) as unknown as HTMLElement & { animate: ReturnType<typeof vi.fn> };
  it("parte con una molla smorzata e finisce ferma; la durata segue k", () => {
    const e = el();
    expect(oscilla(e, 1)).not.toBeNull();
    const [passi, opzioni] = e.animate.mock.calls[0] as [{ transform: string }[], { duration: number }];
    expect(opzioni.duration).toBe(1200);
    expect(passi.at(-1)!.transform).toBe("rotate(0deg)");
    const gradi = passi.map((p) => Math.abs(parseFloat(p.transform.slice(7))));
    expect(Math.max(...gradi.slice(0, 8))).toBeGreaterThan(Math.max(...gradi.slice(-6)) * 4);
    const e2 = el();
    oscilla(e2, 0.5);
    expect((e2.animate.mock.calls[0] as unknown as [unknown, { duration: number }])[1].duration).toBe(600);
  });
  it("su un foglio alto il fondo non si sposta oltre il margine della colonna (32 px, spillo a 20 px)", () => {
    const e = Object.assign(el(), { offsetHeight: 1200 });
    oscilla(e, 1);
    const passi = e.animate.mock.calls[0][0] as { transform: string }[];
    const gradi = Math.max(...passi.map((p) => Math.abs(parseFloat(p.transform.slice(7)))));
    expect(1220 * Math.sin((gradi * Math.PI) / 180)).toBeLessThanOrEqual(32.2);
  });
  it("un foglio di 500-700 px oscilla in modo che si veda: picco di almeno 2,5° e dentro il margine", () => {
    for (const h of [500, 600, 700]) {
      const e = Object.assign(el(), { offsetHeight: h });
      oscilla(e, 1);
      const passi = e.animate.mock.calls[0][0] as { transform: string }[];
      const picco = Math.max(...passi.map((p) => Math.abs(parseFloat(p.transform.slice(7)))));
      expect(picco, `${h} px`).toBeGreaterThanOrEqual(2.4);
      expect((h + 20) * Math.sin((picco * Math.PI) / 180), `${h} px`).toBeLessThanOrEqual(32.2);
      expect(ampiezzaPer(h)).toBeGreaterThanOrEqual(2.5);
    }
  });
  it("col movimento spento (k = 0) non parte", () => {
    const e = el();
    expect(oscilla(e, 0)).toBeNull();
    expect(e.animate).not.toHaveBeenCalled();
  });
  it("nel tono grave non parte", () => {
    vi.stubGlobal("document", { documentElement: { classList: { contains: (c: string) => c === "grave" } } });
    const e = el();
    expect(oscilla(e, 1)).toBeNull();
    expect(e.animate).not.toHaveBeenCalled();
    vi.unstubAllGlobals();
  });
});
