import { describe, expect, it, vi } from "vitest";
import type { Vista } from "$lib/tipi/Vista";
import type { Spesa } from "$lib/tipi/Spesa";
import type { VoceAnteprimaCampagna } from "$lib/tipi/VoceAnteprimaCampagna";
import { LEVE, MASSIMO_PROMESSE, MASSIMO_SPESE, PASSI, PASSO_CESSIONE, cedi, conto, dichiara, quotaNelBersaglio, rivali, righeCoalizione, aggiungiSpesa, seggiAttesi, sessioneSpese, taglio, posizioniVicine, promesseFatte, prometti } from "./campagna";
import turnoZeroJson from "../../../../.scratch/v1-piano-11-ui/viste/01-turno-zero.json";
import campagnaJson from "../../../../.scratch/v1-piano-11-ui/viste/26-campagna.json";

const turnoZero = turnoZeroJson as unknown as Vista;
const tagliata = (campagnaJson as unknown as { tagliata_prima: Vista }).tagliata_prima;
const piano = campagnaJson as unknown as { piano_scelto: Vista; piano_dopo: Vista };
const dichiarata = (campagnaJson as unknown as { dichiarata: Vista }).dichiarata;

describe("dichiara", () => {
  it("aggiunge il partito alla coalizione di oggi", () => {
    expect(dichiara(turnoZero, [], "centrodestra", true)).toEqual({ tipo: "dichiara_coalizione", partiti: ["centrodestra"], cessioni: [] });
  });
  it("toglie il partito e fa cadere le sue cessioni", () => {
    expect(dichiara(dichiarata, [{ partito: "destra", seggi: 10 }], "destra", false)).toEqual({
      tipo: "dichiara_coalizione", partiti: ["centrodestra"], cessioni: [],
    });
  });
  it("tiene le cessioni degli altri", () => {
    const c = dichiara(dichiarata, [{ partito: "destra", seggi: 10 }], "centrodestra", false);
    expect(c).toEqual({ tipo: "dichiara_coalizione", partiti: ["destra"], cessioni: [{ partito: "destra", seggi: 10 }] });
  });
});

describe("cedi", () => {
  it("aggiunge la cessione, e il partito che cede entra nei partiti", () => {
    expect(cedi(turnoZero, [], "centro", 10)).toEqual({ tipo: "dichiara_coalizione", partiti: ["centro"], cessioni: [{ partito: "centro", seggi: 10 }] });
  });
  it("cambia la cessione che c'è", () => {
    const c = cedi(dichiarata, [{ partito: "destra", seggi: 5 }], "destra", 10);
    expect(c).toMatchObject({ partiti: ["centrodestra", "destra"], cessioni: [{ partito: "destra", seggi: 10 }] });
  });
  it("0 la toglie, e non porta dentro chi non c'era", () => {
    expect(cedi(turnoZero, [{ partito: "centro", seggi: 10 }], "centro", 0)).toEqual({ tipo: "dichiara_coalizione", partiti: [], cessioni: [] });
  });
  it("il passo è cinque seggi", () => {
    expect(PASSO_CESSIONE).toBe(5);
  });
});

describe("righeCoalizione", () => {
  it("al turno zero: sette righe, Centrodestra e Destra disponibili, il Centro no con la causa del core", () => {
    const r = righeCoalizione(turnoZero);
    expect(r).toHaveLength(7);
    expect(r.filter((x) => x.disponibile).map((x) => x.id)).toEqual(["centrodestra", "destra"]);
    expect(r.find((x) => x.id === "centro")).toEqual({ id: "centro", nome: "Centro", dichiarato: false, disponibile: false, causa: "posizione troppo lontana dalla tua" });
    expect(r.every((x) => !x.dichiarato)).toBe(true);
  });
  it("coalizione dichiarata: due righe dichiarate", () => {
    expect(righeCoalizione(dichiarata).filter((x) => x.dichiarato).map((x) => x.id)).toEqual(["centrodestra", "destra"]);
  });
});

describe("seggiAttesi", () => {
  it("i seggi alla Camera del giocatore sul sondaggio pubblico", () => {
    expect(seggiAttesi(turnoZero)).toBe(80);
  });
  it("senza proiezione, null", () => {
    expect(seggiAttesi({ ...turnoZero, proiezione: null })).toBeNull();
  });
});

const spesa = (euro: number, condizione: "pensionato" | "studente" | "autonomo" = "pensionato"): Spesa => ({ bersaglio: { condizione }, leva: "persuadere", euro });

describe("costanti", () => {
  it("tre leve e tre bersagli", () => {
    expect(LEVE).toEqual(["mobilitare", "persuadere", "convincere"]);
    expect(MASSIMO_SPESE).toBe(3);
  });
});

describe("taglio", () => {
  it("due da 500.000 con 985.000: la prima intera, la seconda scende a 485.000", () => {
    expect(taglio([spesa(500_000), spesa(500_000, "studente")], 985_000)).toEqual([
      { indice: 0, paga: 500_000, esito: "intera" },
      { indice: 1, paga: 485_000, esito: "scende" },
    ]);
  });
  it("due da 500.000 con 400.000: la prima scende a 400.000, la seconda cade", () => {
    expect(taglio([spesa(500_000), spesa(500_000, "studente")], 400_000)).toEqual([
      { indice: 0, paga: 400_000, esito: "scende" },
      { indice: 1, paga: 0, esito: "cade" },
    ]);
  });
  it("la cassa basta: tutte intere; nessuna spesa: vuoto; cassa 0: tutte cadono", () => {
    expect(taglio([spesa(100), spesa(200, "studente")], 300).map((x) => x.esito)).toEqual(["intera", "intera"]);
    expect(taglio([], 1000)).toEqual([]);
    expect(taglio([spesa(100), spesa(200, "studente")], 0).map((x) => x.esito)).toEqual(["cade", "cade"]);
  });
});

describe("conto", () => {
  it("tagliata_prima: cassa 985.000, sondaggio 15.000 pagato, spese 1.000.000, resto negativo", () => {
    expect(conto(tagliata, tagliata.campagna.spese)).toEqual({ cassa: 985_000, sondaggio: 15_000, spese: 1_000_000, resto: -15_000 });
  });
  it("senza sondaggio comprato, sondaggio 0", () => {
    expect(conto(turnoZero, []).sondaggio).toBe(0);
  });
});

describe("quotaNelBersaglio", () => {
  it("la regione 14 è la Campania: legge l'incrocio dei votanti, e senza privato la privata è null", () => {
    const q = quotaNelBersaglio({ ...tagliata, acquisti: { ...tagliata.acquisti, sondaggio_privato: null } }, { regione: 14 });
    const j = tagliata.partiti.findIndex((p) => p.id === tagliata.giocatore);
    const quote = tagliata.sondaggio.regioni[14].quote;
    expect(tagliata.sondaggio.regioni[14].id).toBe("campania");
    expect(q.pubblica).toBeCloseTo(quote[j] / (1 - quote[quote.length - 1]), 10);
    expect(q.privata).toBeNull();
  });
  it("una condizione, col privato comprato: tutte e due", () => {
    const q = quotaNelBersaglio(tagliata, { condizione: "pensionato" });
    expect(q.pubblica).toBeGreaterThan(0);
    expect(q.privata).toBeGreaterThan(0);
  });
});

describe("rivali", () => {
  it("sette partiti IA dalla cassa più ricca: la Sinistra con 8.135.300", () => {
    const r = rivali(turnoZero);
    expect(r).toHaveLength(7);
    expect(r[0]).toMatchObject({ id: "sinistra", nome: "Sinistra", cassa: 8_135_300 });
    expect(r.map((x) => x.cassa)).toEqual([...r.map((x) => x.cassa)].sort((a, b) => b - a));
  });
});

describe("aggiungiSpesa", () => {
  it("in coda, persuadere, 100.000 €", () => {
    expect(aggiungiSpesa([spesa(5)], { regione: 14 })).toEqual([spesa(5), { bersaglio: { regione: 14 }, leva: "persuadere", euro: 100_000 }]);
  });
  it("un bersaglio già preso si aggiunge lo stesso: il rifiuto «Bersaglio ripetuto» è del core", () => {
    const una = aggiungiSpesa([], { regione: 14 });
    expect(aggiungiSpesa(una, { regione: 14 })).toHaveLength(2);
  });
});

describe("sessioneSpese", () => {
  // anteprime risolte a mano, una per chiamata, nell'ordine
  function banco() {
    const attese: { risolvi: (r: VoceAnteprimaCampagna[]) => void; rifiuta: (e: unknown) => void }[] = [];
    const anteprima = vi.fn((_s: Spesa[]) => new Promise<VoceAnteprimaCampagna[]>((risolvi, rifiuta) => attese.push({ risolvi, rifiuta })));
    const manda = vi.fn();
    const mostra = vi.fn();
    return { attese, anteprima, manda, mostra, sessione: sessioneSpese(anteprima, manda, mostra) };
  }
  const voce = (euro: number): VoceAnteprimaCampagna => ({ bersaglio: { condizione: "pensionato" }, leva: "persuadere", euro, intensita: "poco", causa: "" });
  const svuota = () => new Promise((r) => setTimeout(r, 0));

  it("tre importi con la prima anteprima in volo: due chiamate, mostrata l'ultima", async () => {
    const { attese, anteprima, mostra, sessione } = banco();
    sessione.cambia([spesa(1)]);
    sessione.cambia([spesa(2)]);
    sessione.cambia([spesa(3)]);
    expect(anteprima).toHaveBeenCalledTimes(1);
    attese[0].risolvi([voce(1)]);
    await svuota();
    expect(anteprima).toHaveBeenCalledTimes(2);
    expect(anteprima.mock.calls[1][0]).toEqual([spesa(3)]);
    attese[1].risolvi([voce(3)]);
    await svuota();
    expect(mostra).toHaveBeenCalledTimes(1);
    expect(mostra).toHaveBeenLastCalledWith({ voci: [voce(3)] });
  });

  it("all'uscita dal campo il comando parte se l'ultima anteprima è riuscita, una volta sola", async () => {
    const { attese, manda, sessione } = banco();
    sessione.cambia([spesa(5)]);
    attese[0].risolvi([voce(5)]);
    await svuota();
    sessione.esci();
    sessione.esci();
    expect(manda).toHaveBeenCalledTimes(1);
    expect(manda).toHaveBeenCalledWith([spesa(5)]);
  });

  it("anteprima rifiutata: niente comando, il rifiuto va a chi mostra", async () => {
    const { attese, manda, mostra, sessione } = banco();
    sessione.cambia([spesa(5)]);
    attese[0].rifiuta({ specie: "rifiuto", testo: "comando non ammesso: Spesa oltre la cassa" });
    await svuota();
    sessione.esci();
    expect(manda).not.toHaveBeenCalled();
    expect(mostra).toHaveBeenLastCalledWith({ rifiuto: "Spesa oltre la cassa" });
  });

  it("uscita con l'anteprima in volo: il comando parte quando arriva, se è riuscita", async () => {
    const { attese, manda, sessione } = banco();
    sessione.cambia([spesa(5)]);
    sessione.esci();
    expect(manda).not.toHaveBeenCalled();
    attese[0].risolvi([voce(5)]);
    await svuota();
    expect(manda).toHaveBeenCalledWith([spesa(5)]);
  });

  it("la lettura all'apertura chiede l'anteprima ma l'uscita non manda niente", async () => {
    const { attese, anteprima, manda, mostra, sessione } = banco();
    sessione.cambia([spesa(5)], false);
    attese[0].risolvi([voce(5)]);
    await svuota();
    sessione.esci();
    expect(anteprima).toHaveBeenCalledTimes(1);
    expect(mostra).toHaveBeenCalledWith({ voci: [voce(5)] });
    expect(manda).not.toHaveBeenCalled();
  });

  it("comando non partito (occupato): la bozza resta sporca e riparte una volta sola, quando si riprova", async () => {
    const { attese, manda, sessione } = banco();
    manda.mockReturnValueOnce(Promise.resolve(false));
    sessione.cambia([spesa(5)]);
    attese[0].risolvi([voce(5)]);
    await svuota();
    sessione.esci();
    await svuota();
    expect(manda).toHaveBeenCalledTimes(1);
    sessione.riprova();
    sessione.riprova();
    await svuota();
    expect(manda).toHaveBeenCalledTimes(2);
    expect(manda).toHaveBeenLastCalledWith([spesa(5)]);
  });

  it("una modifica dopo il comando non partito prende il posto della vecchia: riprova non manda la vecchia", async () => {
    const { attese, manda, sessione } = banco();
    manda.mockReturnValueOnce(Promise.resolve(false));
    sessione.cambia([spesa(5)]);
    attese[0].risolvi([voce(5)]);
    await svuota();
    sessione.esci();
    await svuota();
    sessione.cambia([spesa(6)]);
    sessione.riprova();
    expect(manda).toHaveBeenCalledTimes(1);
  });

  it("uscita senza modifiche: niente comando", () => {
    const { manda, sessione } = banco();
    sessione.esci();
    expect(manda).not.toHaveBeenCalled();
  });
});

describe("posizione e promesse", () => {
  it("quattro passi dalla dichiarata di oggi: da 4,5 si arriva a 4, 4,25, 4,75, 5", () => {
    expect(PASSI).toEqual([-0.5, -0.25, 0.25, 0.5]);
    expect(posizioniVicine(turnoZero)).toEqual([4, 4.25, 4.75, 5]);
  });
  it("promesseFatte: nessuna al turno zero; due in campagna (piano_scelto); dopo il voto (piano_dopo) sono in sospeso, quindi nessuna", () => {
    expect(promesseFatte(turnoZero)).toEqual([]);
    expect(promesseFatte(piano.piano_scelto).map((p) => `${p.disposizione}: ${p.gradino}`)).toEqual(["Reddito minimo: universale", "Età pensionabile: 64 anni"]);
    expect(promesseFatte(piano.piano_dopo)).toEqual([]);
  });
  it("prometti: le coppie (disposizione, gradino) nel comando; il massimo è due", () => {
    expect(prometti([{ disposizione: "a", gradino: "b" }])).toEqual({ tipo: "prometti", promesse: [["a", "b"]] });
    expect(MASSIMO_PROMESSE).toBe(2);
  });
});
