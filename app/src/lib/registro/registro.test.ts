import { describe, expect, it } from "vitest";
import registroJson from "../../../../.scratch/v1-piano-11-ui/viste/25-registro.json";
import type { Vista } from "$lib/tipi/Vista";
import type { VoceEsito } from "$lib/tipi/VoceEsito";
import { t } from "$lib/testi/t";
import { cancella, collegate, corregge, passa, periodoDel, riveduta, soggetti, testoConferma, tocca, origineDi, turnoDa, turnoDaUrl, type Filtro } from "./registro";

/* eslint-disable @typescript-eslint/no-explicit-any */
const catena = registroJson as any[];
const turni = catena.map((c, i) => turnoDa(i, c.dopo.esito as VoceEsito[]));
const vista = catena[8].dopo as Vista;
const tutte: Filtro = { criticita: "tutte", sezione: null, soggetto: null };
const voce = (tipo: string, turno: number, pred: (v: VoceEsito) => boolean = () => true) => turni[turno].voci.find((v) => v.tipo === tipo && pred(v))!;

describe("passa", () => {
  it("critiche tiene solo le critiche, notevoli anche le notevoli", () => {
    const v = turni[4].voci;
    expect(v.filter((x) => passa(x, { ...tutte, criticita: "critiche" })).every((x) => x.criticita === "critica")).toBe(true);
    expect(v.filter((x) => passa(x, { ...tutte, criticita: "critiche" })).length).toBe(turni[4].critiche);
    expect(v.filter((x) => passa(x, { ...tutte, criticita: "notevoli" })).every((x) => x.criticita !== "ordinaria")).toBe(true);
    expect(v.filter((x) => passa(x, tutte)).length).toBe(v.length);
  });
  it("la sezione guarda SEZIONE_DI", () => {
    expect(passa(voce("prima_stima", 4), { ...tutte, sezione: "conti" })).toBe(true);
    expect(passa(voce("prima_stima", 4), { ...tutte, sezione: "aula" })).toBe(false);
  });
  it("un partito passa anche da una voce fusa che lo ha solo in un dettaglio", () => {
    const fusa = voce("emendamento_respinto", 4);
    expect(fusa.partito).toBeNull();
    expect(passa(fusa, { ...tutte, soggetto: { tipo: "partito", id: "centrodestra" } })).toBe(true);
    expect(passa(fusa, { ...tutte, soggetto: { tipo: "partito", id: "sinistra" } })).toBe(false);
  });
  it("il bilancio è una misura senza disposizioni", () => {
    const f = { ...tutte, soggetto: { tipo: "bilancio", id: "3" } } as Filtro;
    expect(passa(voce("bilancio_approvato", 4), f)).toBe(true);
    expect(passa(voce("legge_respinta", 5, (v) => v.misura === 1), f)).toBe(false);
  });
});

describe("soggetti", () => {
  const s = soggetti(turni, vista);
  const nome = (tipo: string, id: string) => s.find((x) => x.soggetto.tipo === tipo && x.soggetto.id === id)?.nome;
  it("nomi di partiti, disposizioni e bilancio con l'anno del trimestre della voce", () => {
    expect(nome("partito", "centrodestra")).toBe(vista.partiti.find((p) => p.id === "centrodestra")!.nome);
    expect(nome("disposizione", "autonomia_differenziata")).toBe(vista.disposizioni.find((d) => d.id === "autonomia_differenziata")!.nome);
    expect(nome("bilancio", "3")).toBe("Legge di bilancio · 2027");
  });
  it("le regioni vengono da REGIONI", () => {
    const r = s.filter((x) => x.soggetto.tipo === "regione");
    for (const x of r) expect(x.nome).not.toBe(x.soggetto.id);
  });
  it("un id che la Vista non ha resta col suo id", () => {
    const v = { ...voce("legge_respinta", 5), disposizioni: ["sconosciuta"] };
    expect(soggetti([turnoDa(5, [v])], vista).find((x) => x.soggetto.tipo === "disposizione")?.nome).toBe("sconosciuta");
  });
});

describe("corregge", () => {
  it("la revisione del turno 4 dà la prima stima del turno 3", () => {
    const r = corregge(voce("revisione", 4), 4, turni);
    expect(r).toBe(voce("prima_stima", 3));
  });
  it("un'altra voce, o il turno zero senza turno prima, no", () => {
    expect(corregge(voce("prima_stima", 4), 4, turni)).toBeNull();
    expect(corregge(voce("revisione", 4), 4, turni.slice(4))).toBeNull();
  });
});

describe("collegate", () => {
  it("la regione compare in due turni: l'evento nato del 2 e quello risolto del 3 si collegano, e nient'altro", () => {
    const nato = voce("evento_nato", 2, (x) => x.regione === "calabria");
    const risolto = voce("evento_risolto", 3, (x) => x.regione === "calabria");
    const c = collegate(nato, 2, turni);
    expect(c.stesse).toEqual([[3, risolto]]);
    expect(c.nelleRighe).toEqual([]);
    expect(collegate(risolto, 3, turni).stesse).toEqual([[2, nato]]);
  });
  it("a mano: stesse per campo, nelleRighe solo per una riga di una voce fusa, mai due volte, mai lo stesso turno", () => {
    const base = { tipo: "atto_eseguito", partito: null, dicastero: null, regione: null, misura: null, disposizioni: [], dettagli: [] };
    const riga = (r: string | null, d: string[] = []) => ({ testo: "", partito: null, dicastero: null, regione: r, misura: null, disposizioni: d });
    const mia = { ...base, tipo: "legge_approvata", disposizioni: ["cuneo"] } as unknown as VoceEsito;
    const stessa = { ...base, tipo: "legge_respinta", disposizioni: ["cuneo", "altra"] } as unknown as VoceEsito;
    const fusa = { ...base, dettagli: [riga("umbria"), riga(null, ["cuneo"])] } as unknown as VoceEsito;
    const estranea = { ...base, partito: "sinistra", disposizioni: ["altra"], dettagli: [riga("umbria")] } as unknown as VoceEsito;
    const stessoTurno = { ...base, disposizioni: ["cuneo"] } as unknown as VoceEsito;
    const t = [turnoDa(1, [mia, stessoTurno]), turnoDa(2, [stessa, estranea]), turnoDa(3, [fusa])];
    expect(collegate(mia, 1, t)).toEqual({ stesse: [[2, stessa]], nelleRighe: [[3, fusa]] });
  });
});

describe("cancella", () => {
  it("dal 5 all'8 quattro chiusure", () => {
    expect(cancella(5, 8, turni).chiusure).toBe(4);
    expect(cancella(8, 8, turni).chiusure).toBe(1);
  });
  it("i voti e i governi nati nel mezzo", () => {
    const t = [turnoDa(2, [{ tipo: "risultato_del_voto", tono: "neutro", criticita: "critica" } as unknown as VoceEsito]), turnoDa(3, [{ tipo: "governo_formato", criticita: "notevole" } as unknown as VoceEsito]), turnoDa(4, [])];
    expect(cancella(3, 4, t)).toEqual({ chiusure: 2, voti: [], governi: [3] });
    expect(cancella(2, 4, t)).toEqual({ chiusure: 3, voti: [2], governi: [3] });
  });
});

describe("periodoDel", () => {
  it("il turno 0 è ottobre–dicembre 2026, il 4 lo stesso trimestre dell'anno dopo, e conta all'indietro dalla Vista", () => {
    expect(periodoDel(0, vista)).toEqual({ anno: 2026, trimestre: 4 });
    expect(periodoDel(4, vista)).toEqual({ anno: 2027, trimestre: 4 });
    expect(periodoDel(8, vista)).toEqual({ anno: 2028, trimestre: 4 });
    expect(periodoDel(1, vista)).toEqual({ anno: 2027, trimestre: 1 });
  });
});

describe("riveduta", () => {
  it("la prima stima del 3 è riveduta dalla revisione del 4; un'altra voce, o l'ultimo turno, no", () => {
    expect(riveduta(voce("prima_stima", 3), 3, turni)).toBe(voce("revisione", 4));
    expect(riveduta(voce("revisione", 4), 4, turni)).toBeNull();
    expect(riveduta(voce("prima_stima", 8), 8, turni)).toBeNull();
  });
});

describe("tocca", () => {
  it("vale anche per una riga di dettaglio", () => {
    const fusa = voce("emendamento_respinto", 4);
    expect(fusa.dettagli.some((d) => tocca(d, { tipo: "partito", id: "centrodestra" }))).toBe(true);
  });
});

describe("testoConferma", () => {
  const voto = (n: number) => turnoDa(n, [{ tipo: "risultato_del_voto", tono: "neutro", criticita: "critica", dettagli: [], disposizioni: [] } as unknown as VoceEsito]);
  const gov = (n: number) => turnoDa(n, [{ tipo: "governo_formato", tono: "neutro", criticita: "notevole", dettagli: [], disposizioni: [] } as unknown as VoceEsito]);
  const v9 = { ...vista, turno: 9, finita: false } as Vista;
  const vuoto = (n: number) => turnoDa(n, []);
  const nonLetti = (n: number) => t("registro.conferma_non_letti", { n });

  it("una chiusura sola: dice la chiusura, il trimestre e il turno in corso", () => {
    const x = testoConferma(8, v9, turni, 0);
    expect(x).toContain("apertura del turno 8");
    expect(x).toContain("la chiusura del turno 8");
    expect(x).toContain("turno 9 in corso");
    expect(x).toContain("nessun voto e nessun governo");
  });
  it("più chiusure: il conto e l'elisione davanti a 8 e 11", () => {
    expect(testoConferma(5, v9, turni, 0)).toContain("4 chiusure, dal turno 5 all’8");
    expect(testoConferma(3, v9, turni, 0)).toContain("6 chiusure, dal turno 3 all’8");
    const v12 = { ...v9, turno: 12 } as Vista;
    const t11 = Array.from({ length: 11 }, (_, i) => vuoto(i));
    expect(testoConferma(9, v12, t11, 0)).toContain("3 chiusure, dal turno 9 all’11");
  });
  it("il voto e il governo in mezzo si nominano, col trimestre", () => {
    const ts = [vuoto(0), vuoto(1), voto(2), gov(3), vuoto(4)];
    const x = testoConferma(2, { ...v9, turno: 5 } as Vista, ts, 0);
    expect(x).toContain("il voto di");
    expect(x).toContain("la nascita del governo");
    expect(x).not.toContain("nessun voto");
  });
  it("il numero del riavvolgimento è l'indice più uno; senza indice la frase cade e il resto resta", () => {
    expect(testoConferma(8, v9, turni, 0)).toContain("il primo");
    expect(testoConferma(8, v9, turni, 2)).toContain("il terzo");
    expect(testoConferma(8, v9, turni, 6)).toContain("il 7°");
    const senza = testoConferma(8, v9, turni, null);
    expect(senza).not.toContain("sarà il");
    expect(senza).toContain("Con le stesse scelte");
  });
  it("a partita finita dice l'eredità in bozza, non il turno in corso", () => {
    const x = testoConferma(8, { ...v9, finita: true } as Vista, turni, 0);
    expect(x).toContain("l’eredità in bozza");
    expect(x).not.toContain("in corso");
  });
  it("un turno che non si legge fra N e l'ultimo lo dice, invece di sottostimare in silenzio", () => {
    const ts = turni.map((x) => (x.turno === 6 ? { turno: 6, guasto: "disco" } : x));
    expect(testoConferma(5, v9, ts, 0)).toContain(nonLetti(1));
    expect(testoConferma(7, v9, ts, 0)).not.toContain(nonLetti(1));
    // non ancora letti contano come non letti
    expect(testoConferma(5, v9, turni.slice(0, 7), 0)).toContain(nonLetti(2));
  });
});

describe("turnoDaUrl", () => {
  it("un numero intero apre quel turno", () => expect(turnoDaUrl("16")).toBe(16));
  it("assente, vuoto o non intero non apre niente", () => {
    for (const x of [null, "", "x", "-1", "1.5", "16a"]) expect(turnoDaUrl(x)).toBeUndefined();
  });
});

describe("origineDi", () => {
  const voce = (o: Partial<VoceEsito>): VoceEsito => ({
    tipo: "spread", criticita: "ordinaria", precedenza: 0, in_evidenza: false, tono: "ordinario", fatto: "", causa: "",
    partito: null, dicastero: null, regione: null, misura: null, disposizioni: [], famiglia: null, dettagli: [], ...o,
  });
  const registro = [
    turnoDa(5, [voce({ tipo: "scissione", partito: "nuovo_1", fatto: "Dal Centro esce una lista." })]),
    turnoDa(6, [voce({ tipo: "spread" })]),
    { turno: 7, guasto: "disco" },
    turnoDa(9, [voce({ tipo: "cambio_di_leader", partito: "nuovo_1" })]),
    turnoDa(11, [voce({ tipo: "cambio_di_leader", partito: "nuovo_1" }), voce({ tipo: "cambio_di_leader", partito: "centro" })]),
  ];
  it("il turno e il fatto della scissione che lo nomina, il leader nuovo dall'ultimo cambio", () => {
    expect(origineDi(registro, "nuovo_1")).toEqual({ nato: { turno: 5, fatto: "Dal Centro esce una lista." }, leader: 11 });
  });
  it("il partito di protesta nomina il suo partito; senza voce non c'è origine, senza cambio non c'è leader", () => {
    const r = [turnoDa(4, [voce({ tipo: "partito_di_protesta", partito: "p", fatto: "Nasce la protesta." })])];
    expect(origineDi(r, "p")).toEqual({ nato: { turno: 4, fatto: "Nasce la protesta." }, leader: null });
    expect(origineDi(r, "altro")).toEqual({ nato: null, leader: null });
  });
  it("cerca anche nei dettagli di una voce fusa, col testo del dettaglio", () => {
    const d = { testo: "Nasce la lista X.", partito: "x", dicastero: null, regione: null, misura: null, disposizioni: [] };
    expect(origineDi([turnoDa(2, [voce({ tipo: "scissione", partito: "y", fatto: "Due liste.", dettagli: [d] })])], "x").nato).toEqual({ turno: 2, fatto: "Nasce la lista X." });
  });
});
