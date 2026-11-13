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
  // fonda taglia gli spazi ai bordi e conta i caratteri Unicode (chars().count()); la UI limita con maxlength
  // (unità UTF-16) e nomeValido guarda solo se il nome è vuoto: la lunghezza non la decide lei
  it("40 caratteri e 41: nomeValido non conta, il tetto è del campo e del core", () => {
    expect(nomeValido("a".repeat(40))).toBe(true);
    expect(nomeValido("a".repeat(41))).toBe(true);
  });
  it("un nome di soli caratteri fuori dal piano di base non è vuoto", () => {
    expect(nomeValido("😀")).toBe(true);
    expect([..."😀".repeat(40)]).toHaveLength(40); // per il core sono 40 caratteri
    expect("😀".repeat(40).length).toBe(80); // per maxlength sono 80 unità: il campo ne accetta 20
  });
  it("spazi ai bordi non contano, come nel core", () => expect(nomeValido("  x ")).toBe(true));
  it("anche gli spazi Unicode (a capo, NBSP) sono vuoto", () => expect(nomeValido("\n\u00a0\t")).toBe(false));
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
  it("1,5 e 6,5: i due estremi dell'asse", () => {
    expect(fra(1.5, partiti).map((p) => p.id)).toEqual(["estrema_sinistra", "sinistra"]);
    expect(fra(6.5, partiti).map((p) => p.id)).toEqual(["destra", "estrema_destra"]);
  });
  it("ognuno dei sei posti sta fra i due partiti interi che lo chiudono", () => {
    for (const k of [1, 2, 3, 4, 5, 6]) expect(fra(k + 0.5, partiti).map((p) => p.posizione)).toEqual([k, k + 1]);
  });
  // oracolo: il tipo promette due partiti; un posto fuori dall'asse dà undefined e chi legge .nome si rompe
  it.fails("un posto fuori dall'asse dà comunque due partiti", () => {
    const [a, b] = fra(7.5, partiti);
    expect(a && b).toBeTruthy();
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
  it("ogni posto di ogni anteprima: sette voci, i disponibili prima, la causa alla lettera", () => {
    for (const [nomeAnteprima, a] of Object.entries(campione.anteprime) as [string, AnteprimaFondazione][]) {
      expect(a.posizioni.map((x) => x.posizione), nomeAnteprima).toEqual([1.5, 2.5, 3.5, 4.5, 5.5, 6.5]);
      for (const pos of a.posizioni) {
        const r = conChiCorrere(pos, partiti);
        const dove = `${nomeAnteprima} @ ${pos.posizione}`;
        expect(r, dove).toHaveLength(7);
        // prima tutti i disponibili: nessun disponibile dopo un non disponibile
        expect(r.map((x) => x.disponibile), dove).toEqual([...r.map((x) => x.disponibile)].sort((x, y) => Number(y) - Number(x)));
        // ogni voce porta la causa del core, intatta, e il partito del suo id
        for (const d of pos.disponibilita) {
          const voce = r.find((x) => x.partito.id === d.partito)!;
          expect(voce.causa, `${dove} ${d.partito}`).toBe(d.causa);
          expect(voce.disponibile, `${dove} ${d.partito}`).toBe(d.disponibile);
        }
      }
    }
  });
  it("con tutti disponibili o nessuno, l'ordine resta quello dei dati", () => {
    const tutti = { ...v, disponibilita: v.disponibilita.map((d) => ({ ...d, disponibile: true })) };
    expect(conChiCorrere(tutti, partiti).map((x) => x.partito.id)).toEqual(v.disponibilita.map((d) => d.partito));
    const nessuno = { ...v, disponibilita: v.disponibilita.map((d) => ({ ...d, disponibile: false })) };
    expect(conChiCorrere(nessuno, partiti).map((x) => x.partito.id)).toEqual(v.disponibilita.map((d) => d.partito));
  });
  it("non muta le voci che riceve", () => {
    const prima = structuredClone(v);
    conChiCorrere(v, partiti);
    expect(v).toEqual(prima);
  });
  it("il Centro non è disponibile, con la causa del campione", () => {
    const centro = voci.find((x) => x.partito.id === "centro")!;
    expect(centro.disponibile).toBe(false);
    expect(centro.causa).toBe("posizione troppo lontana dalla tua");
  });
});
