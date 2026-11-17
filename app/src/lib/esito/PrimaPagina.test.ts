import { beforeEach, describe, expect, it } from "vitest";
import { render } from "svelte/server";
import type { Vista } from "$lib/tipi/Vista";
import { t } from "$lib/testi/t";
import { tracciatiDi, tracciatiRivelati } from "$lib/tracciati";
import { ordineRivelazione, pagina } from "./esito";
import { rivelazione } from "./rivelazione.svelte";
import PrimaPagina from "./PrimaPagina.svelte";
import registroJson from "../../../../.scratch/v1-piano-11-ui/viste/25-registro.json";
import primaPaginaJson from "../../../../.scratch/v1-piano-11-ui/viste/16-prima-pagina.json";

/* eslint-disable @typescript-eslint/no-explicit-any */
const chiusura = (n: number) => (registroJson as any)[n].dopo as Vista;
const voto9 = (primaPaginaJson as any).voto_9.vista as Vista;
const votoOpp = (primaPaginaJson as any).voto_opp.vista as Vista;
const html = (vista: Vista) => render(PrimaPagina, { props: { vista, id: "partita-di-prova" } }).body;
const conta = (h: string, ago: string | RegExp) => h.match(new RegExp(ago, "g"))?.length ?? 0;

beforeEach(() => rivelazione.chiudi());

describe("PrimaPagina", () => {
  const v = chiusura(3);
  const evidenza = ordineRivelazione(v.esito);

  it("ferma (Riprendi) è composta: ogni voce in evidenza c'è, nessuna in attesa, testata e piede", () => {
    const h = html(v);
    for (const i of evidenza) expect(h).toContain(v.esito[i].fatto.replace(/&/g, "&amp;").replace(/</g, "&lt;"));
    expect(conta(h, 'class="[^"]*\\battesa\\b')).toBe(0);
    expect(h).toContain(t("prima.esito"));
    expect(h).toContain(t("prima.conteggio", { n: evidenza.length, fuori: v.esito.length - evidenza.length }));
    expect(h).toContain(t("prima.vai_cruscotto"));
    expect(h).not.toContain(t("prima.salta"));
  });

  it("a metà rivelazione le voci non ancora passate restano al loro posto, in attesa; si salta finché dura", () => {
    rivelazione.inizia(chiusura(2), [], evidenza.length);
    rivelazione.avanza();
    rivelazione.avanza();
    const h = html(v);
    expect(conta(h, 'class="[^"]*\\battesa\\b')).toBe(evidenza.length - 2);
    expect(h).toContain(t("prima.salta"));
  });

  it("la testa può comparire dopo una colonna: in attesa finché la sua posizione nell'ordine non è passata", () => {
    const v8 = chiusura(8);
    const ordine = ordineRivelazione(v8.esito);
    const { testa } = pagina(v8.esito);
    expect(ordine.indexOf(testa!)).toBeGreaterThan(0);
    rivelazione.inizia(chiusura(7), [], ordine.length);
    rivelazione.avanza();
    expect(html(v8)).toMatch(/<article class="voce testa[^"]*\battesa\b/);
    for (let j = 0; j < ordine.indexOf(testa!); j++) rivelazione.avanza();
    expect(html(v8)).toMatch(/<article class="voce testa(?![^"]*\battesa\b)/);
  });

  it("l'evento nato aperto dice dove si decide: sul cruscotto al governo, al governo se sei all'opposizione", () => {
    expect(html(v)).toContain(t("prima.si_decide_sul_cruscotto"));
    const opp = html({ ...v, governa: false } as Vista);
    expect(opp).toContain(t("prima.lo_decide_il_governo"));
    expect(opp).not.toContain(t("prima.si_decide_sul_cruscotto"));
  });

  it("senza voci in evidenza: la testata, una riga e il rimando al registro", () => {
    const h = html({ ...v, esito: v.esito.map((x) => ({ ...x, in_evidenza: false })) } as Vista);
    expect(h).toContain(t("prima.nessuna_notizia"));
    const critiche = v.esito.filter((x) => x.criticita === "critica").length;
    expect(h).toContain(t(critiche === 1 ? "prima.altre_una_critica" : "prima.altre_critiche", { n: v.esito.length, k: critiche }));
    expect(h).toContain(t("prima.esito"));
  });

  it("tono grave: edizione straordinaria, niente titolo e niente vignetta; l'occhiello sale a titolo", () => {
    const g = { ...v, esito: v.esito.map((x, i) => ({ ...x, tono: i === evidenza[0] ? "grave" : x.tono })) } as Vista;
    const h = html(g);
    expect(h).toContain(t("prima.esito_grave"));
    expect(h).not.toContain(t("prima.motto"));
    expect(h).not.toContain("<figure");
    expect(h).toContain(`<h2 class="tit`);
  });

  it("partita finita: il piede porta all'eredità", () => {
    const h = html({ ...v, finita: true } as Vista);
    expect(h).toContain(t("prima.vai_eredita"));
    expect(h).not.toContain(t("prima.vai_cruscotto"));
  });

  it("in trattativa il piede porta alla trattativa", () => {
    expect(html({ ...v, fase: "trattativa" } as Vista)).toContain(t("prima.vai_trattativa"));
  });

  it("la pagina del voto mostra il risultato a barre al posto della vignetta", () => {
    const h = html(voto9);
    expect(h).toContain(t("prima.coalizione_nera"));
    expect(h).toMatch(/<figure class="ris/);
  });
});

describe("piede e voci senza titolo", () => {
  const v = chiusura(3);
  const base = { ...v.esito[0], in_evidenza: false, criticita: "ordinaria" } as Vista["esito"][number];
  const con = (esito: Vista["esito"]) => html({ ...v, esito } as Vista);

  it("il rimando al registro dice le critiche fuori pagina, al singolare e al plurale", () => {
    const crit = { ...base, criticita: "critica" } as Vista["esito"][number];
    const una = con([crit, base]);
    expect(una).toContain("Altre 2 notizie nel registro, una critica ›");
    expect(una).toContain('href="/partita/registro"');
    expect(con([crit, crit, base])).toContain("Altre 3 notizie nel registro, 2 critiche ›");
    expect(con([crit])).toContain("Un’altra notizia nel registro, una critica ›");
    expect(con([base])).toContain("Un’altra notizia nel registro ›");
    expect(con([base, base])).toContain("Altre 2 notizie nel registro ›");
  });

  it("una voce senza titolo (sondaggio ordinario) mostra il solo occhiello, che sale a titolo", () => {
    const h = con([{ ...base, tipo: "sondaggio", in_evidenza: true, fatto: "Il consenso cala", famiglia: null } as Vista["esito"][number]]);
    expect(h).toMatch(/<h2 class="tit[^"]*">(<!--[^>]*-->)*Il consenso cala<\/h2>/);
    expect(h).not.toContain('class="occh');
  });

  it("la rivelazione non parte prima che la pagina sia resa: il timer lo guida PrimaPagina solo con l'id (SSR: senza id niente corpo)", () => {
    expect(render(PrimaPagina, { props: { vista: v } }).body).not.toContain("giornale");
  });
});

describe("Risultato", () => {
  it("la didascalia della coalizione col premio c'è solo se c'è un premio", () => {
    expect(html(voto9)).toContain(t("prima.coalizione_nera"));
    expect(votoOpp.elezioni.at(-1)!.premio).toBeNull();
    expect(html(votoOpp)).not.toContain(t("prima.coalizione_nera"));
  });
  it("ogni partito con voti o seggi, senza soglia", () => {
    const e = voto9.elezioni.at(-1)!;
    const piccolo = { partito: e.voti[0].partito, quota: 0.001, camera: 0, senato: 0 };
    const h = html({ ...voto9, elezioni: [{ ...e, voti: [...e.voti, { ...piccolo, partito: "estrema_sinistra" }, { ...piccolo, quota: 0, camera: 1, partito: "centro" }].filter((x, i, a) => a.findIndex((y) => y.partito === x.partito) === i || i >= e.voti.length) }] } as Vista);
    expect(conta(h, 'class="r[ "]')).toBeGreaterThanOrEqual(e.voti.length);
  });
});

describe("uscire dalla prima pagina a metà rivelazione (punto 2 da sorvegliare)", () => {
  const dopo = chiusura(3);
  const prima = chiusura(2);

  it("chiudi porta i tracciati al valore nuovo e la pagina composta e ferma", () => {
    rivelazione.inizia(prima, [], ordineRivelazione(dopo.esito).length);
    rivelazione.avanza();
    rivelazione.avanza();
    rivelazione.avanza();
    expect(rivelazione.ferma).toBe(false);
    rivelazione.chiudi();
    expect(tracciatiRivelati(dopo, [], rivelazione)).toEqual(tracciatiDi(dopo, []));
    expect(rivelazione.ferma).toBe(true);
    expect(conta(html(dopo), 'class="[^"]*\\battesa\\b')).toBe(0);
  });
});
