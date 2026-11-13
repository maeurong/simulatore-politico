import libRs from "../../../src-tauri/src/lib.rs?raw";
import { afterEach, beforeEach, describe, expect, it } from "vitest";
import { clearMocks, mockIPC } from "@tauri-apps/api/mocks";
import { comeErrore, guscio } from "./chiamate";

// mockIPC scrive su `window`: in Node basta farlo coincidere con globalThis.
beforeEach(() => {
  (globalThis as { window?: unknown }).window = globalThis;
});
afterEach(() => clearMocks());

describe("comeErrore", () => {
  it("lascia com'è la forma giusta", () => {
    const e = { specie: "rifiuto", testo: "x" } as const;
    expect(comeErrore(e)).toEqual(e);
  });
  it("trasforma una stringa in guasto", () => {
    expect(comeErrore("disco pieno")).toEqual({ specie: "guasto", testo: "disco pieno" });
  });
  it("trasforma un Error in guasto", () => {
    expect(comeErrore(new Error("y"))).toEqual({ specie: "guasto", testo: "Error: y" });
  });
});

describe("guscio", () => {
  it("passa comando e argomenti a invoke", async () => {
    const chiamate: [string, unknown][] = [];
    mockIPC((cmd, args) => {
      chiamate.push([cmd, args]);
    });
    await guscio.riprendi("abc");
    expect(chiamate).toEqual([["riprendi", { id: "abc" }]]);
  });
  it("usa il nome Rust in snake_case", async () => {
    const chiamate: string[] = [];
    mockIPC((cmd) => {
      chiamate.push(cmd);
    });
    await guscio.chiudiTurno();
    expect(chiamate).toEqual(["chiudi_turno"]);
  });
  it("rilancia un rifiuto come ErroreGuscio", async () => {
    mockIPC(() => {
      throw { specie: "rifiuto", testo: "no" };
    });
    await expect(guscio.riprendi("abc")).rejects.toEqual({ specie: "rifiuto", testo: "no" });
  });
});

// L'oracolo dei nomi è l'elenco `generate_handler!` di Rust, non il modulo sotto test.
const nomiRust = [...libRs
  .match(/generate_handler!\[([^\]]*)\]/)![1]
  .matchAll(/\w+/g)].map((m) => m[0]);

const scenario = { nome: "x" } as never;
const tabella: [string, () => Promise<unknown>, Record<string, unknown>][] = [
  ["chiudi_turno", () => guscio.chiudiTurno(), {}],
  ["partite", () => guscio.partite(), {}],
  ["fonda_partita", () => guscio.fondaPartita(scenario, { nome: "f" } as never), { scenario, fondazione: { nome: "f" } }],
  ["riprendi", () => guscio.riprendi("abc"), { id: "abc" }],
  ["comanda", () => guscio.comanda({ tipo: "c" } as never), { comando: { tipo: "c" } }],
  ["riavvolgi", () => guscio.riavvolgi(3), { turno: 3 }],
  ["registro", () => guscio.registro(2), { turno: 2 }],
  ["abbandona", () => guscio.abbandona(), {}],
  ["conferma_eredita", () => guscio.confermaEredita(), {}],
  ["albo", () => guscio.albo(), {}],
  ["scenari", () => guscio.scenari(), {}],
  ["salva_scenario", () => guscio.salvaScenario(scenario), { scenario }],
  ["carica_scenario", () => guscio.caricaScenario("n"), { nome: "n" }],
  ["riepilogo_scenario", () => guscio.riepilogoScenario(null), { scenario: null }],
  ["anteprima", () => guscio.anteprima([]), { valori: [] }],
  ["anteprima_decreto", () => guscio.anteprimaDecreto({ id: "d" } as never), { decreto: { id: "d" } }],
  ["anteprima_accogli", () => guscio.anteprimaAccogli("tesoro"), { dicastero: "tesoro" }],
  ["serie", () => guscio.serie(), {}],
  ["anteprima_fondazione", () => guscio.anteprimaFondazione(null), { scenario: null }],
  ["catalogo_scenario", () => guscio.catalogoScenario(), {}],
];

describe("guscio: un comando Rust per funzione", () => {
  it("la tabella copre tutti i comandi registrati in lib.rs", () => {
    expect(nomiRust).toHaveLength(20);
    expect(tabella.map(([nome]) => nome).sort()).toEqual([...nomiRust].sort());
  });
  it.each(tabella)("%s: nome e argomenti", async (nome, chiama, args) => {
    const viste: [string, unknown][] = [];
    mockIPC((cmd, a) => {
      viste.push([cmd, a ?? {}]);
    });
    await chiama();
    expect(viste).toEqual([[nome, args]]);
  });
});
