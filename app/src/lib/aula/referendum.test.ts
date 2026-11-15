import { describe, expect, it } from "vitest";
import type { EsitoReferendum } from "$lib/tipi/EsitoReferendum";
import type { Vista } from "$lib/tipi/Vista";
import type { VistaReferendum } from "$lib/tipi/VistaReferendum";
import opposizione from "../../../../.scratch/v1-piano-11-ui/viste/28-opposizione.json";
import { esitoDi, misuraDelConfermativo } from "./referendum";

const esito = (o: Partial<EsitoReferendum>): EsitoReferendum =>
  ({ tipo: "abrogativo", nome: "x", data: { anno: 2028, trimestre: 2 }, si: 5, no: 7, votanti: 12, aventi_diritto: 20, quorum: true, approvato: false, decaduto: null, ...o });

describe("esitoDi", () => {
  it("decaduto prima di tutto; l'abrogativo senza quorum non conta; poi approvato o respinto", () => {
    expect(esitoDi(esito({ decaduto: "sospeso", quorum: false }))).toBe("decaduto");
    expect(esitoDi(esito({ quorum: false, approvato: true }))).toBe("senza_quorum");
    expect(esitoDi(esito({ approvato: true }))).toBe("approvato");
    expect(esitoDi(esito({}))).toBe("respinto");
  });
  it("il confermativo non ha quorum", () => {
    expect(esitoDi(esito({ tipo: "confermativo", quorum: false, approvato: true }))).toBe("approvato");
  });
  it("sul campione dell'opposizione: il tetto al contante a 5.000 € non raggiunge il quorum", () => {
    const e = (opposizione as any).referendum.chiusure[2].vista.esiti_referendum[0] as EsitoReferendum;
    expect(esitoDi(e)).toBe("senza_quorum");
  });
});

describe("misuraDelConfermativo", () => {
  const r = { nome: "Numero dei parlamentari: 300 deputati" } as VistaReferendum;
  const voto = (o: object) => ({ misura: 7, nome: r.nome, forma: "costituzionale", deliberazione: 2, ...o });
  it("la costituzionale della seconda deliberazione con lo stesso nome", () => {
    expect(misuraDelConfermativo({ esiti: [voto({ deliberazione: 1, misura: 3 }), voto({})] } as unknown as Vista, r)).toBe(7);
  });
  it("senza esito che torni, nessuna misura: la scelta non si offre", () => {
    expect(misuraDelConfermativo({ esiti: [voto({ nome: "altro" }), voto({ forma: "riforma" })] } as unknown as Vista, r)).toBeNull();
  });
});
