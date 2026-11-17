import { describe, expect, it } from "vitest";
import type { Vista } from "$lib/tipi/Vista";
import { t } from "$lib/testi/t";
import turnoZero from "../../../../.scratch/v1-piano-11-ui/viste/01-turno-zero.json";
import bilancio from "../../../../.scratch/v1-piano-11-ui/viste/04-governo-bilancio.json";
import grave from "../../../../.scratch/v1-piano-11-ui/viste/08-tono-grave.json";
import opposizione from "../../../../.scratch/v1-piano-11-ui/viste/28-opposizione.json";
import registro from "../../../../.scratch/v1-piano-11-ui/viste/25-registro.json";
import { tracciatiDi } from "$lib/tracciati";
import { pagina } from "$lib/esito/esito";
import { schema, soloInPrimaPagina, tondiDi } from "./schema";

const zero = turnoZero as unknown as Vista;
const ordinario = bilancio as unknown as Vista;
const tono = grave as unknown as Vista;
const stati = opposizione as unknown as Record<string, Record<string, Vista>>;
const fine = stati.fine.vista;
const conSfiducia = stati.sfiducia.dopo_comando;

const tipi = (v: Vista, tipo: string) => schema(v).fili.filter((f) => f.tipo === tipo);

describe("schema, governo ordinario (04)", () => {
  const s = schema(ordinario);
  it("un filo proposta per ogni ministero con una proposta, verso il Presidente", () => {
    const attesi = ordinario.ministeri.filter((m) => m.proposta).map((m) => `d:${m.dicastero}`);
    expect(attesi).toHaveLength(6);
    expect(tipi(ordinario, "proposta").map((f) => f.da)).toEqual(attesi);
    expect(tipi(ordinario, "proposta").every((f) => f.a === "pdc")).toBe(true);
  });
  it("due fili di maggioranza, Camera e Senato", () => {
    expect(tipi(ordinario, "maggioranza").map((f) => f.a)).toEqual(["camera", "senato"]);
  });
  it("nessun filo ha un capo senza nodo", () => {
    const ids = new Set(s.nodi.map((n) => n.id));
    for (const f of s.fili) {
      expect(ids.has(f.da), f.id).toBe(true);
      expect(ids.has(f.a), f.id).toBe(true);
    }
  });
  it("ordine dei nodi: Presidente primo nel governo, Mercati primo nel Paese, situazioni in fondo", () => {
    const di = (c: number) => s.nodi.filter((n) => n.colonna === c);
    expect(di(2)[0].tipo).toBe("presidente");
    expect(di(3)[0].tipo).toBe("mercati");
    expect(di(3).at(-1)!.tipo).toBe("situazione");
    expect(di(1).map((n) => n.id)).toEqual(["camera", "senato"]);
  });
  it("gli id dei fili sono unici", () => {
    expect(new Set(s.fili.map((f) => f.id)).size).toBe(s.fili.length);
  });
  it("un cantiere è tratteggiato a puntini, un'agenda con effetto ritardato è in ritardo", () => {
    expect(tipi(ordinario, "cantiere").every((f) => f.cantiere)).toBe(true);
    expect(tipi(ordinario, "agenda").length).toBeGreaterThan(0);
  });
  it("un evento in una regione parte dal nodo della regione, per nome", () => {
    const ev = tipi(ordinario, "evento");
    expect(ev.map((f) => f.da)).toEqual(["r:emilia_romagna"]);
  });
  it("solo le situazioni accese hanno un filo", () => {
    expect(tipi(ordinario, "situazione")).toHaveLength(ordinario.situazioni.filter((x) => x.accesa).length);
  });
  it("un filo spread dai Mercati all'Economia", () => {
    const f = tipi(ordinario, "spread");
    expect(f).toHaveLength(1);
    expect([f[0].da, f[0].a]).toEqual(["mer", "d:economia_finanze"]);
  });
});

describe("schema, grave (08)", () => {
  it("la minaccia di dimissioni è un filo dal ministero al Presidente", () => {
    const f = tipi(tono, "minaccia");
    expect(f.map((x) => [x.da, x.a, x.contrario])).toEqual([["d:sanita", "pdc", true]]);
  });
});

describe("schema, turno zero (01)", () => {
  const s = schema(zero);
  it("Camera e Senato spenti, il filo «si vota a fine turno»", () => {
    expect(s.nodi.filter((n) => n.tipo === "camera").map((n) => n.spento)).toEqual([true, true]);
    expect(tipi(zero, "maggioranza").map((f) => f.etichetta)).toEqual([t("schema.et_si_vota")]);
  });
  it("il governo uscente è centrodestra e destra", () => {
    expect(s.nodi.filter((n) => n.tipo === "partito" && n.alGoverno).map((n) => n.id)).toEqual(["p:centrodestra", "p:destra"]);
  });
  it("il Presidente è «uscente · chi guida»", () => {
    expect(s.nodi.find((n) => n.tipo === "presidente")!.sotto).toMatch(/^uscente · /);
  });
  it("chi guida il governo uscente è il partito con più ministri: Destra, 8 su 10", () => {
    expect(s.nodi.find((n) => n.tipo === "presidente")!.sotto).toBe(t("schema.sotto_uscente", { nome: "Destra" }));
  });
});

describe("schema, all'opposizione (28 fine)", () => {
  it("nessun filo agenda: la Vista non le porta", () => {
    expect(tipi(fine, "agenda")).toHaveLength(0);
  });
  it("un filo intesa per ogni voce di alleati_avversari", () => {
    expect(fine.alleati_avversari.length).toBeGreaterThan(0);
    expect(tipi(fine, "intesa").map((f) => f.da)).toEqual(fine.alleati_avversari.map((a) => `p:${a.partito}`));
  });
  it("un partito sciolto non ha riga e il suo filo non si disegna", () => {
    const v = { ...fine, partiti: fine.partiti.map((p) => (p.id === "sinistra" ? { ...p, sciolto: true } : p)) } as Vista;
    expect(tipi(v, "intesa").map((f) => f.da)).not.toContain("p:sinistra");
    expect(tipi(v, "intesa")).toHaveLength(fine.alleati_avversari.length - 1);
  });
});

describe("schema, sfiducia e posto riservato (28)", () => {
  it("un filo sfiducia dal partito al Presidente e un filo posto dal partito alla Camera", () => {
    expect(tipi(conSfiducia, "sfiducia").map((f) => [f.da, f.a])).toEqual([["p:giocatore", "pdc"]]);
    expect(tipi(conSfiducia, "posto").map((f) => [f.da, f.a])).toEqual([["p:estrema_destra", "camera"]]);
  });
  it("la sfiducia a un ministro arriva al suo dicastero", () => {
    const v = { ...conSfiducia, sfiducia: { dicastero: "sanita", partito: "giocatore" } } as Vista;
    expect(tipi(v, "sfiducia").map((f) => f.a)).toEqual(["d:sanita"]);
  });
  it("senza sfiducia e senza posto nessuno dei due fili", () => {
    const v = { ...stati.sfiducia.prima, posto_riservato_di: null } as Vista;
    expect(tipi(v, "sfiducia")).toHaveLength(0);
    expect(tipi(v, "posto")).toHaveLength(0);
  });
});

describe("schema, dimissioni, posto ed esiti (04)", () => {
  it("le dimissioni chieste sono un filo rosso dal Presidente al ministero", () => {
    const d = ordinario.ministeri[0].dicastero;
    const v = { ...ordinario, ministeri: ordinario.ministeri.map((m, i) => (i === 0 ? { ...m, dimissioni_chieste: true } : m)) } as Vista;
    expect(tipi(v, "dimissioni").map((f) => [f.id, f.da, f.a, f.contrario])).toEqual([[`dim:${d}`, "pdc", `d:${d}`, true]]);
    expect(tipi(ordinario, "dimissioni")).toHaveLength(0);
  });
  it("il posto altrui ha sempre il filo; il tuo solo se libero, in ritardo", () => {
    expect(ordinario.posto_riservato_di).toBe("sinistra");
    expect(tipi(ordinario, "posto").map((f) => [f.da, f.etichetta, f.ritardo])).toEqual([["p:sinistra", t("schema.et_posto"), false]]);
    const libero = { ...ordinario, posto_riservato_di: "giocatore", posto_riservato: true } as Vista;
    expect(tipi(libero, "posto").map((f) => [f.da, f.etichetta, f.ritardo])).toEqual([["p:giocatore", t("schema.et_posto_tuo"), true]]);
    expect(tipi({ ...libero, posto_riservato: false } as Vista, "posto")).toHaveLength(0);
  });
  it("una misura decaduta nel trimestre chiuso non ha il filo della legge", () => {
    expect(tipi(ordinario, "legge").map((f) => f.id)).toEqual(["es:0"]);
    const v = { ...ordinario, esiti: ordinario.esiti.map((e) => ({ ...e, decaduta: "senza voto" })) } as Vista;
    expect(tipi(v, "legge")).toHaveLength(0);
  });
});

describe("tondi dello schema (25, chiusura 3, dopo)", () => {
  const v = (registro as unknown as { dopo: Vista }[])[3].dopo;
  const tondi = (w: Vista) => tondiDi(w, schema(w));
  const cambia = (w: Vista, tipo: string, x: object) => ({ ...w, esito: w.esito.map((e) => (e.tipo === tipo ? { ...e, ...x } : e)) }) as Vista;
  const voce = (T: ReturnType<typeof tondiDi>, tipo: string) => T.elenco.find((e) => e.voce.tipo === tipo)!;

  it("la voce spread ha il tondo sul filo spread", () => {
    const T = tondi(cambia(v, "spread", { in_evidenza: true })), e = voce(T, "spread");
    expect(e.posto).toBe("spread");
    expect(T.fili.get("spread")).toEqual([e.n]);
  });
  it("una legge respinta ha il tondo su es:<misura>", () => {
    const w = { ...cambia(v, "legge_approvata", { tipo: "legge_respinta" }), esiti: v.esiti.map((x) => ({ ...x, approvata: false })) } as Vista;
    const T = tondi(w), e = voce(T, "legge_respinta");
    expect(e.posto).toBe("es:0");
    expect(T.fili.get("es:0")).toEqual([e.n]);
  });
  it("l'evento nato ha il tondo sul filo dell'evento con la stessa famiglia e regione", () => {
    const T = tondi(v), e = voce(T, "evento_nato");
    expect(e.posto).toBe("ev:Emergenza idrogeologica");
    expect(T.fili.get("ev:Emergenza idrogeologica")).toEqual([e.n]);
  });
  it("una voce senza posto sta nell'elenco con posto null e senza tondo", () => {
    const T = tondi(v), e = voce(T, "proposta_respinta");
    expect(e.posto).toBeNull();
    expect([...T.fili.values(), ...T.nodi.values()].flat()).not.toContain(e.n);
  });
  it("senza filo del suo tipo la voce col dicastero va sulla riga del ministero", () => {
    const d = v.ministeri[0].dicastero;
    const T = tondi(cambia(v, "cantiere_fermo", { dicastero: d })), e = voce(T, "cantiere_fermo");
    expect(e.posto).toBe(`d:${d}`);
    expect(T.nodi.get(`d:${d}`)).toEqual([e.n]);
  });
  it("i numeri seguono l'ordine della pagina, testa al numero 1", () => {
    const T = tondi(v), p = pagina(v.esito);
    expect(T.elenco.map((e) => e.n)).toEqual(T.elenco.map((_, i) => i + 1));
    expect(T.elenco.map((e) => e.voce)).toEqual([p.testa!, ...p.colonne].map((i) => v.esito[i]));
    expect(T.elenco[0].voce.tipo).toBe("evento_risolto");
    // una testa che nella chiusura viene dopo prende comunque il numero 1
    const w = tondi(cambia(v, "shock_assorbito", { criticita: "critica", precedenza: 9 }));
    expect(w.elenco[0].voce.tipo).toBe("shock_assorbito");
    expect(w.elenco.filter((e) => e.voce.in_evidenza)).toHaveLength(T.elenco.length);
  });
  it("le voci fuori dalla prima pagina non hanno numero", () => {
    expect(tondi(v).elenco).toHaveLength(v.esito.filter((e) => e.in_evidenza).length);
  });
  it("«Solo quelli in prima pagina»: i fili col tondo coi loro capi, e le righe col tondo", () => {
    const d = v.ministeri[0].dicastero, w = cambia(v, "cantiere_fermo", { dicastero: d });
    const S = schema(w), F = soloInPrimaPagina(S, tondiDi(w, S));
    expect(F.fili.map((f) => f.id)).toEqual(["es:0", "ev:Emergenza idrogeologica"]);
    expect(F.nodi.map((n) => n.id).sort()).toEqual(["camera", "senato", "pdc", "r:emilia_romagna", `d:${d}`].sort());
  });
  it("al turno zero nessun tondo", () => {
    const T = tondi(zero);
    expect(T.elenco).toEqual([]);
    expect(T.fili.size + T.nodi.size).toBe(0);
  });
});

describe("schema, maggioranza come la cornice", () => {
  it("il margine del filo è quello del tracciato, e il rosso segue lo stesso stacco", () => {
    for (const v of [ordinario, tono]) {
      const tr = tracciatiDi(v, []);
      const fili = tipi(v, "maggioranza");
      const margini = fili.map((f) => Number(f.etichetta.replace("maggioranza ", "").replace("−", "-")));
      expect(Math.min(...margini)).toBe(tr[1].valore);
      expect(fili.some((f) => f.contrario)).toBe(tr[1].rosso);
    }
  });
  it("04: +23 / +14 come la cornice", () => {
    expect(tipi(ordinario, "maggioranza").map((f) => f.etichetta)).toEqual(["maggioranza +23", "maggioranza +14"]);
  });
  it("in trattativa la coalizione è quella della cornice", () => {
    const trattativa = (JSON.parse(JSON.stringify(ordinario)) as Vista);
    Object.assign(trattativa, { fase: "trattativa", governa: false, turno: 5, coalizione: ["destra"], richieste: [] });
    const tr = tracciatiDi(trattativa, []);
    const fili = tipi(trattativa, "maggioranza").map((f) => Number(f.etichetta.replace("maggioranza ", "").replace("−", "-")));
    expect(Math.min(...fili)).toBe(tr[1].valore);
  });
  it("all'opposizione (28 fine) il margine è del governo vero, non dei seggi del giocatore", () => {
    const fili = tipi(fine, "maggioranza");
    expect(fili.map((f) => f.etichetta)).toEqual(["maggioranza +139", "maggioranza +71"]);
    expect(fili.some((f) => f.contrario)).toBe(false);
  });
  it("a partita finita il margine è del governo vero", () => {
    const finita = { ...fine, finita: true } as Vista;
    expect(tipi(finita, "maggioranza").map((f) => f.etichetta)).toEqual(["maggioranza +139", "maggioranza +71"]);
  });
});

describe("schema, id dei fili unici (sono le chiavi della tavola)", () => {
  const unici = (v: Vista) => {
    const ids = schema(v).fili.map((f) => f.id);
    expect(new Set(ids).size).toBe(ids.length);
  };
  it("su 04, 08, 01 e 28 fine", () => {
    for (const v of [ordinario, tono, zero, fine]) unici(v);
  });
  it("due eventi omonimi e due esiti della stessa misura restano due fili ciascuno", () => {
    expect(tono.eventi.length).toBeGreaterThan(0);
    const v = { ...tono, eventi: [...tono.eventi, tono.eventi[0]] } as Vista;
    unici(v);
    expect(tipi(v, "evento")).toHaveLength(tono.eventi.length + 1);
    const prima = v.data.trimestre === 1 ? { anno: v.data.anno - 1, trimestre: 4 } : { anno: v.data.anno, trimestre: v.data.trimestre - 1 };
    const esito = { ...(ordinario.esiti[0] ?? {}), misura: 999, nome: "M", data: prima, decaduta: null, approvata: true,
      camera: { si: 1, no: 0 }, senato: { si: 1, no: 0 } };
    const w = { ...v, esiti: [esito, esito] } as Vista;
    unici(w);
    expect(tipi(w, "legge")).toHaveLength(2);
  });
});
