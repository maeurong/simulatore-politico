import { describe, expect, it } from "vitest";
import { REGIONI } from "./regioni";
import { accanto, classi, estremi, idDi, indiceDi } from "./mappa";
import type { Vista } from "$lib/tipi/Vista";
import zero from "../../../../.scratch/v1-piano-11-ui/viste/01-turno-zero.json";
import campagna from "../../../../.scratch/v1-piano-11-ui/viste/26-campagna.json";

const v = zero as unknown as Vista;

describe("regioni", () => {
  it("venti, id distinti, nell'ordine di sondaggio.regioni", () => {
    expect(REGIONI).toHaveLength(20);
    expect(new Set(REGIONI.map((r) => r.id)).size).toBe(20);
    expect(REGIONI.map((r) => r.id)).toEqual(v.sondaggio.regioni.map((r) => r.id));
  });
});

describe("indice dei dati e id", () => {
  it("la Campania è 14, come il campione 26-campagna (piano.ok[].bersaglio.regione)", () => {
    const piano = (campagna as unknown as { piano: { ok: { bersaglio: { regione?: number } }[] } }).piano.ok;
    expect(piano.find((x) => x.bersaglio.regione !== undefined)!.bersaglio.regione).toBe(14);
    expect(idDi(v, 14)).toBe("campania");
    expect(indiceDi(v, "campania")).toBe(14);
  });
  it("fuori dai venti: null", () => {
    expect(idDi(v, 20)).toBeNull();
    expect(idDi(v, -1)).toBeNull();
    expect(indiceDi(v, "atlantide")).toBeNull();
  });
});

describe("classi", () => {
  it("cinque passi uguali", () => expect(classi([0, 25, 50, 75, 100])).toEqual([0, 1, 2, 3, 4]));
  it("tutti uguali: una classe sola", () => expect(new Set(classi([10, 10, 10])).size).toBe(1));
  it("null resta null", () => expect(classi([null, 5])).toEqual([null, 0]));
  it("vuoto", () => expect(classi([])).toEqual([]));
  it("estremi e passo; senza dati null", () => {
    expect(estremi([null, 10, 20])).toEqual({ min: 10, max: 20, passo: 2 });
    expect(estremi([null])).toBeNull();
  });
});

describe("accanto", () => {
  const ordine = REGIONI.map((r) => r.id);
  it("gira agli estremi", () => {
    expect(ordine.at(-1)).toBe("sardegna");
    expect(accanto("sardegna", ordine, 1)).toBe(ordine[0]);
    expect(accanto(ordine[0], ordine, -1)).toBe("sardegna");
    expect(accanto("lombardia", ordine, 1)).toBe("trentino_alto_adige");
  });
});
