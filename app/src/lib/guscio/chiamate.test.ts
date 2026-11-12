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
