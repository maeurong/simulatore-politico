import { describe, expect, it } from "vitest";
import type { Vista } from "$lib/tipi/Vista";
import { barreSeggi, maggioranza } from "./seggi";
import governoJson from "../../../../.scratch/v1-piano-11-ui/viste/03-governo-ordinario.json";

const governo = governoJson as unknown as Vista;

describe("maggioranza", () => {
  it("la metà più uno: 200 su 400 e 100 su 200 non bastano", () => {
    expect(maggioranza(400)).toBe(201);
    expect(maggioranza(200)).toBe(101);
    expect(maggioranza(315)).toBe(158);
  });
});

describe("barreSeggi", () => {
  it("una barra per ramo, il totale è la somma delle voci, coi nomi dei partiti", () => {
    const b = barreSeggi(governo, governo.seggi);
    expect(b.map((x) => x.ramo)).toEqual(["camera", "senato"]);
    for (const x of b) {
      expect(x.totale).toBe(x.righe.reduce((a, r) => a + r.seggi, 0));
      expect(x.righe.find((r) => r.id === "giocatore")!.nome).toBe(governo.partiti.find((p) => p.id === "giocatore")!.nome);
    }
    expect(b[0].totale).toBe(governo.seggi.reduce((a, s) => a + s.camera, 0));
  });
  it("senza elenco, o a zero seggi, la barra non c'è", () => {
    expect(barreSeggi(governo, null)).toEqual([]);
    expect(barreSeggi(governo, governo.seggi.map((s) => ({ ...s, camera: 0, senato: 0 })))).toEqual([]);
  });
});
