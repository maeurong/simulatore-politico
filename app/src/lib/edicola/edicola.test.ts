import { describe, expect, it } from "vitest";
import { anniAlGoverno, azioneDi, inOrdine, maleSe, tracce, voceNuova } from "./edicola";
import edicola from "../../../../.scratch/v1-piano-11-ui/viste/13-edicola.json";
import type { VoceAlbo } from "$lib/tipi/VoceAlbo";
import type { VocePartita } from "$lib/tipi/VocePartita";

const partite = edicola.partite as VocePartita[];
const albo = edicola.albo as unknown as VoceAlbo[];
const nome = (p: VocePartita) => p.indice.testata.nome;

describe("inOrdine", () => {
  it("per ultimo_salvataggio decrescente, comunque arrivino", () => {
    const ordinate = inOrdine([...partite].reverse());
    expect(nome(ordinate[0])).toBe("Patto Civico");
    expect(ordinate[0].indice.ultimo_salvataggio).toBe(1794000000);
    expect(nome(ordinate.at(-1)!)).toBe("Unione dei Pendolari");
  });
  it("non tocca l'elenco che riceve", () => {
    const copia = [...partite].reverse();
    inOrdine(copia);
    expect(nome(copia[0])).toBe("Unione dei Pendolari");
  });
});

describe("azioneDi", () => {
  it.each([
    ["Lista del Faro", "eredita"],
    ["Unione dei Pendolari", "nessuna"],
    ["Patto Civico", "riprendi"],
    ["Nuova Concordia", "riprendi"],
    ["Fronte dei Comuni", "riprendi"],
  ])("%s → %s", (n, atteso) => {
    expect(azioneDi(partite.find((p) => nome(p) === n)!)).toBe(atteso);
  });
});

describe("maleSe", () => {
  it("inflazione senza colore, gli altri tre salgono male", () => {
    expect(maleSe("inflazione")).toBeNull();
    expect(maleSe("spread")).toBe("sale");
    expect(maleSe("debito_pil")).toBe("sale");
    expect(maleSe("disoccupazione")).toBe("sale");
  });
});

describe("tracce", () => {
  const finiti = (r: ReturnType<typeof tracce>, valori: number[]) =>
    [r.x(0), r.x(1), ...valori.map(r.y)].every(Number.isFinite);
  it("senza serie: nessun punto, coordinate finite", () => {
    const r = tracce([], 290, 40);
    expect(r.n).toBe(0);
    expect(finiti(r, [0])).toBe(true);
  });
  it("una linea vuota non dà NaN", () => {
    expect(finiti(tracce([[]], 290, 40), [0])).toBe(true);
  });
  it("un punto solo: a sinistra, dentro il riquadro", () => {
    const r = tracce([[0.4]], 290, 40);
    expect(r.n).toBe(1);
    expect(r.x(0)).toBe(0);
    expect(r.y(0.4)).toBeGreaterThanOrEqual(0);
    expect(r.y(0.4)).toBeLessThanOrEqual(40);
  });
  it("più punti: dal bordo sinistro al destro, il massimo in alto", () => {
    const r = tracce([[1, 3, 2]], 200, 40);
    expect([r.x(0), r.x(2)]).toEqual([0, 200]);
    expect(r.y(3)).toBe(0);
    expect(r.y(1)).toBe(40);
  });
  it("con zero, lo zero entra nell'intervallo", () => {
    const r = tracce([[10, 20]], 200, 40, true);
    expect(r.y(0)).toBe(40);
  });
});

describe("anniAlGoverno", () => {
  it.each([
    [0, "mai al governo"],
    [2, "2 anni al governo"],
    [1, "1 anno al governo"],
    [5.25, "5 anni e 3 mesi al governo"],
    [0.75, "9 mesi al governo"],
  ])("%d → %s", (a, atteso) => expect(anniAlGoverno(a)).toBe(atteso));
});

describe("voceNuova", () => {
  const voce = (nome: string, anno: number, trimestre: number) =>
    ({ ...albo[0], nome, fine: { anno, trimestre } }) as VoceAlbo;
  it("la voce che prima non c'era, anche se un'omonima finisce dopo", () => {
    const vecchia = voce("Patto Civico", 2035, 2);
    const nuova = voce("Patto Civico", 2028, 2);
    // copie: dopo l'abbandono il guscio rimanda oggetti nuovi, il confronto è sul contenuto
    const prima = [vecchia, voce("Altro", 2040, 1)];
    const dopo = [structuredClone(vecchia), structuredClone(prima[1]), nuova];
    expect(voceNuova(prima, dopo, "Patto Civico")).toBe(nuova);
  });
  it("albo invariato: ricade sulla fine più recente fra le omonime", () => {
    const elenco = [voce("Patto Civico", 2027, 1), voce("Altro", 2040, 1), voce("Patto Civico", 2028, 2)];
    expect(voceNuova(elenco, structuredClone(elenco), "Patto Civico")?.fine).toEqual({ anno: 2028, trimestre: 2 });
  });
  it("più d'una voce nuova: ricade sulla regola del nome", () => {
    const dopo = [voce("Patto Civico", 2027, 1), voce("Patto Civico", 2028, 2)];
    expect(voceNuova([], dopo, "Patto Civico")).toBe(dopo[1]);
  });
  it("nessuna voce con quel nome: null", () => {
    expect(voceNuova(albo, albo, "Patto Civico")).toBeNull();
  });
});
