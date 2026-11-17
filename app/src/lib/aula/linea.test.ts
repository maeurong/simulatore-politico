import { describe, expect, it } from "vitest";
import type { Vista } from "$lib/tipi/Vista";
import opposizione from "../../../../.scratch/v1-piano-11-ui/viste/28-opposizione.json";
import { costoDellaLinea } from "./linea";

const v = (opposizione as unknown as { linea: { vista: Vista } }).linea.vista;
const misura = (nome: string) => v.calendario.find((m) => m.nome.startsWith(nome))!;

describe("costoDellaLinea", () => {
  it("il Cuneo fiscale sposta la percepita da 1,94: sì a 2,01, default a 2,06", () => {
    const c = misura("Cuneo fiscale").costo_linea!;
    expect(costoDellaLinea(v, c.si)).toContain("va da 1,94 a 2,01");
    expect(costoDellaLinea(v, c.default)).toContain("va da 1,94 a 2,06");
  });
  it("la Legge di bilancio non la sposta in nessun ramo", () => {
    const c = misura("Legge di bilancio").costo_linea!;
    for (const r of [c.si, c.no, c.default]) expect(costoDellaLinea(v, r)).toContain("resta a 1,94");
  });
  it("porta anche i franchi attesi", () => {
    const c = misura("Cuneo fiscale").costo_linea!;
    expect(costoDellaLinea(v, c.si)).toContain("franchi attesi: 2,4 alla Camera, 1,3 al Senato");
  });
  it("uno scarto sotto il centesimo che si legge uguale è «resta»", () => {
    const c = { percepita: 1.9363 + 0.003, franchi_camera: 0, franchi_senato: 0 };
    expect(costoDellaLinea(v, c)).toContain("resta a 1,94");
  });
});
