import { describe, expect, it } from "vitest";
import type { Vista } from "$lib/tipi/Vista";
import type { VoceEsito } from "$lib/tipi/VoceEsito";
import { eventoAperto, grave, ordineRivelazione, pagina, scattaDopo, SEZIONE_DI, testa, TRACCIATO_DI } from "./esito";
import titoliJson from "../testi/pungolo.json";
import { titoloDi } from "../testi/pungolo";
import registroJson from "../../../../.scratch/v1-piano-11-ui/viste/25-registro.json";
import primaPaginaJson from "../../../../.scratch/v1-piano-11-ui/viste/16-prima-pagina.json";
import contiJson from "../../../../.scratch/v1-piano-11-ui/viste/21-conti.json";
import v08 from "../../../../.scratch/v1-piano-11-ui/viste/08-tono-grave.json";
import v03 from "../../../../.scratch/v1-piano-11-ui/viste/03-governo-ordinario.json";

/* eslint-disable @typescript-eslint/no-explicit-any */
const chiusura = (n: number) => (registroJson as any)[n].dopo as Vista;
const c3 = chiusura(3).esito;
const c8 = chiusura(8).esito;
const voto9 = (primaPaginaJson as any).voto_9.vista as Vista;
const ricondotto = (contiJson as any).ricondotto.vista as Vista;

// voce scritta a mano: il campione non ha una critica fuori pagina
const voce = (o: Partial<VoceEsito>): VoceEsito => ({
  tipo: "spread", criticita: "ordinaria", precedenza: 0, in_evidenza: false, tono: "ordinario", fatto: "", causa: "",
  partito: null, dicastero: null, regione: null, misura: null, disposizioni: [], famiglia: null, dettagli: [], ...o,
});

describe("testa", () => {
  it("la critica con la precedenza più alta fra le voci in evidenza", () => {
    expect(c8[testa(c8)!].tipo).toBe("sfiducia_presentata");
    expect(c8[testa(c8)!].precedenza).toBe(3);
  });
  it("a parità la prima", () => {
    expect(testa(c3)).toBe(0);
    expect(c3[0].tipo).toBe("evento_risolto");
  });
  it("le voci fuori pagina non vincono, nemmeno se più critiche", () => {
    expect(testa([voce({ criticita: "critica", precedenza: 9 }), voce({ in_evidenza: true, criticita: "notevole" })])).toBe(1);
  });
  it("una notevole batte le ordinarie, una critica le notevoli", () => {
    const es = [voce({ in_evidenza: true }), voce({ in_evidenza: true, criticita: "notevole", precedenza: 5 }), voce({ in_evidenza: true, criticita: "critica" })];
    expect(testa(es)).toBe(2);
    expect(testa(es.slice(0, 2))).toBe(1);
  });
  it("senza voci in evidenza è null", () => {
    expect(testa([])).toBeNull();
    expect(testa([voce({})])).toBeNull();
  });
});

describe("pagina", () => {
  it("colonne nell'ordine della chiusura senza la testa; fuori sono le voci non in evidenza", () => {
    expect(pagina(c3)).toEqual({ testa: 0, colonne: [1, 3, 4, 7, 13, 14], fuori: 10, critFuori: 0 });
    expect(pagina(c8)).toEqual({ testa: 16, colonne: [0, 1, 2, 14, 15, 18], fuori: 12, critFuori: 0 });
    expect(pagina(voto9.esito)).toEqual({ testa: 9, colonne: [3, 10, 11], fuori: 9, critFuori: 0 });
  });
  it("critFuori conta le critiche non in evidenza", () => {
    const es = [voce({ in_evidenza: true }), voce({ criticita: "critica" }), voce({ criticita: "critica" }), voce({ criticita: "notevole" })];
    expect(pagina(es)).toEqual({ testa: 0, colonne: [], fuori: 3, critFuori: 2 });
  });
  it("zero voci in evidenza: testa null, colonne vuote", () => {
    expect(pagina([voce({}), voce({})])).toEqual({ testa: null, colonne: [], fuori: 2, critFuori: 0 });
    expect(pagina([])).toEqual({ testa: null, colonne: [], fuori: 0, critFuori: 0 });
  });
});

describe("bilancio_ricondotto", () => {
  it("la voce del campione vero entra in pagina e ha il suo titolo", () => {
    const i = ricondotto.esito.findIndex((x) => x.tipo === "bilancio_ricondotto");
    expect(i).toBeGreaterThan(-1);
    expect(pagina(ricondotto.esito).colonne).toContain(i);
    expect(titoloDi("p", ricondotto.esito[i], 7, 0)).toBeTruthy();
  });
});

describe("grave", () => {
  it("vero sul 08, evento grave aperto", () => {
    expect(grave(v08 as unknown as Vista)).toBe(true);
  });
  it("vero anche a evento chiuso, se una voce dell'esito è grave: il 03 ha un'emergenza grave risolta", () => {
    const x = v03 as unknown as Vista;
    expect(x.eventi.every((e) => e.tono === "ordinario")).toBe(true);
    expect(grave(x)).toBe(true);
  });
  it("falso senza voci né eventi gravi", () => {
    expect(grave(chiusura(3))).toBe(false);
  });
});

const TIPI = Object.values(import.meta.glob("../tipi/TipoVoce.ts", { query: "?raw", import: "default", eager: true }))[0] as string;
const tipiVoce = () => [...TIPI.split("export type TipoVoce =")[1].matchAll(/"([a-z_]+)"/g)].map((m) => m[1]);

describe("SEZIONE_DI", () => {
  it("copre ogni tipo, e solo quelli", () => {
    expect(Object.keys(SEZIONE_DI).sort()).toEqual(tipiVoce().sort());
  });
  it("evento risolto e emergenza lieve rimandano ai Conti (prototipo 25), l'evento nato al cruscotto", () => {
    expect(SEZIONE_DI.evento_risolto).toBe("conti");
    expect(SEZIONE_DI.evento_nato).toBe("cruscotto");
  });
});

describe("tracciati che scattano", () => {
  it("ogni tipo di TRACCIATO_DI esiste", () => {
    const tipi = new Set(tipiVoce());
    for (const lista of Object.values(TRACCIATO_DI)) for (const t of lista) expect(tipi.has(t)).toBe(true);
  });
  it("ordineRivelazione: gli indici in evidenza nell'ordine della chiusura", () => {
    expect(ordineRivelazione(c3)).toEqual([0, 1, 3, 4, 7, 13, 14]);
  });
  it("scattaDopo punta alla prima voce che muove il tracciato", () => {
    expect(scattaDopo(c3, "spread")).toBe(6);
    expect(c3[6].tipo).toBe("spread");
    expect(scattaDopo(c3, "consenso")).toBe(8);
    expect(c3[8].tipo).toBe("sondaggio");
  });
  it("scattaDopo è null se nessuna voce muove il tracciato", () => {
    expect(scattaDopo(c3, "margine")).toBeNull();
  });
  it("nel voto il margine lo muove il risultato del voto", () => {
    const i = scattaDopo(voto9.esito, "margine")!;
    expect(voto9.esito[i].tipo).toBe("risultato_del_voto");
  });
});

describe("i testi coprono i tipi", () => {
  it("pungolo.json ha una chiave per tipo (evento_* per famiglia)", () => {
    const chiavi = Object.keys(titoliJson.titoli);
    for (const t of tipiVoce()) {
      if (t === "evento_nato" || t === "evento_risolto") expect(chiavi.filter((k) => k.startsWith(`${t}|`))).toHaveLength(5);
      else expect(chiavi).toContain(t);
    }
  });
});

describe("eventoAperto", () => {
  const v = chiusura(3);
  const nato = v.esito.find((x) => x.tipo === "evento_nato")!;
  it("l'evento nato con la stessa famiglia e la stessa regione (id nella voce, nome nell'evento) è aperto", () => {
    expect(nato.regione).toBe("emilia_romagna");
    expect(eventoAperto(v, nato)).toBe(true);
  });
  it("un'altra regione, un'altra famiglia o nessun evento aperto: no", () => {
    expect(eventoAperto(v, { ...nato, regione: "calabria" })).toBe(false);
    expect(eventoAperto(v, { ...nato, famiglia: "situazione" })).toBe(false);
    expect(eventoAperto({ ...v, eventi: [] }, nato)).toBe(false);
  });
  it("solo l'evento nato: l'evento risolto non rimanda a un evento aperto", () => {
    expect(eventoAperto(v, { ...nato, tipo: "evento_risolto" })).toBe(false);
  });
});
