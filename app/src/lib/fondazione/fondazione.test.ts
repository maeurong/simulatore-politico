import { describe, expect, it } from "vitest";
import { conChiCorrere, cosaManca, fra, LUNGHEZZA_NOME, nomeValido } from "./fondazione";
import campione from "../../../../.scratch/v1-piano-11-ui/viste/15-fondazione.json";
import type { AnteprimaFondazione } from "$lib/tipi/AnteprimaFondazione";
import type { PartitoCatalogo } from "$lib/tipi/PartitoCatalogo";

const partiti = campione.partiti as PartitoCatalogo[];
const oggi = campione.anteprime["oggi, seme 4"] as AnteprimaFondazione;

describe("nomeValido", () => {
  it("solo spazi è vuoto", () => {
    expect(nomeValido("   ")).toBe(false);
    expect(nomeValido("")).toBe(false);
    expect(nomeValido("Patto")).toBe(true);
  });
  it("il limite è quello di fonda", () => expect(LUNGHEZZA_NOME).toBe(40));
});

describe("cosaManca", () => {
  it("con l'anteprima pronta: prima il nome, poi il posto", () => {
    expect(cosaManca("", 4.5, "pronta")).toBe("fondazione.manca_il_nome");
    expect(cosaManca("  ", null, "pronta")).toBe("fondazione.manca_il_nome");
    expect(cosaManca("Patto", null, "pronta")).toBe("fondazione.manca_il_posto");
    expect(cosaManca("Patto", 4.5, "pronta")).toBeNull();
  });
  it("senza l'anteprima dello scenario corrente non si fonda, anche con nome e posto in memoria", () => {
    expect(cosaManca("Patto", 4.5, "in_arrivo")).toBe("fondazione.anteprima_in_arrivo");
    expect(cosaManca("Patto", 4.5, "fallita")).toBe("fondazione.anteprima_fallita");
    // l'anteprima viene prima: il nome da solo non basterebbe
    expect(cosaManca("", null, "fallita")).toBe("fondazione.anteprima_fallita");
  });
});

describe("fra", () => {
  it("4,5 sta fra Centro e Centrodestra", () => {
    expect(fra(4.5, partiti).map((p) => p.nome)).toEqual(["Centro", "Centrodestra"]);
  });
  it("comunque arrivino i partiti", () => {
    expect(fra(1.5, [...partiti].reverse()).map((p) => p.id)).toEqual(["estrema_sinistra", "sinistra"]);
  });
});

describe("conChiCorrere", () => {
  const v = oggi.posizioni.find((p) => p.posizione === 4.5)!;
  const voci = conChiCorrere(v, partiti);
  it("i sette, prima i disponibili, a parità nell'ordine dei dati", () => {
    expect(voci).toHaveLength(7);
    expect(voci.map((x) => [x.partito.nome, x.disponibile])).toEqual([
      ["Centrodestra", true],
      ["Destra", true],
      ["Estrema sinistra", false],
      ["Sinistra", false],
      ["Centrosinistra", false],
      ["Centro", false],
      ["Estrema destra", false],
    ]);
  });
  it("un id che il catalogo non conosce resta, col nome grezzo", () => {
    const strano = { ...v, disponibilita: [...v.disponibilita, { partito: "ignoto", disponibile: true, causa: "c" }] };
    const r = conChiCorrere(strano, partiti);
    expect(r).toHaveLength(8);
    expect(r[2]).toMatchObject({ partito: { id: "ignoto", nome: "ignoto" }, disponibile: true, causa: "c" });
  });
  it("il Centro non è disponibile, con la causa del campione", () => {
    const centro = voci.find((x) => x.partito.id === "centro")!;
    expect(centro.disponibile).toBe(false);
    expect(centro.causa).toBe("posizione troppo lontana dalla tua");
  });
});
