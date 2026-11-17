import { describe, expect, it } from "vitest";
import { render } from "svelte/server";
import type { Vista } from "$lib/tipi/Vista";
import bilancio from "../../../../.scratch/v1-piano-11-ui/viste/04-governo-bilancio.json";
import opposizione from "../../../../.scratch/v1-piano-11-ui/viste/28-opposizione.json";
import registro from "../../../../.scratch/v1-piano-11-ui/viste/25-registro.json";
import turnoZero from "../../../../.scratch/v1-piano-11-ui/viste/01-turno-zero.json";
import { t } from "$lib/testi/t";
import { battutaDi } from "$lib/testi/pungolo";
import Schema from "./Schema.svelte";
import DettaglioSchema from "./DettaglioSchema.svelte";
import { schema, soloInPrimaPagina, tondiDi } from "./schema";

const ordinario = bilancio as unknown as Vista;
const fine = (opposizione as unknown as Record<string, Record<string, Vista>>).fine.vista;
const nulla = () => {};

describe("Schema, render", () => {
  it.each([["04", ordinario], ["28 fine", fine]] as const)("%s: testa, «Come si legge» e il numero dei fili", (_, vista) => {
    const { body } = render(Schema, { props: { vista } });
    expect(body).toContain(`Tavola n. ${vista.turno} · Come funziona lo Stato`);
    expect(body).toContain("Come si legge");
    expect(body).toContain(`${schema(vista).fili.length} fili in questo trimestre`);
  });

  it("il filo scelto: da chi a chi, la frase intera e il rimando alla sezione; la fonte tecnica no", () => {
    const S = schema(ordinario), f = S.fili.find((x) => x.tipo === "proposta")!;
    const { body } = render(DettaglioSchema, { props: { vista: ordinario, nodi: S.nodi, fili: S.fili, tutti: S.fili, tondi: tondiDi(ordinario, S), scelta: { tipo: "filo", id: f.id }, id: "d", onapri: nulla, onchiudi: nulla } });
    expect(body).toContain("→ Presidente del Consiglio");
    expect(body).toContain(f.frase.split("«")[0]);
    expect(body).toContain('href="/partita/cruscotto"');
    expect(body).not.toContain(f.fonte);
  });

  it("la riga scelta: i suoi fili col verso", () => {
    const S = schema(ordinario);
    const { body } = render(DettaglioSchema, { props: { vista: ordinario, nodi: S.nodi, fili: S.fili, tutti: S.fili, tondi: tondiDi(ordinario, S), scelta: { tipo: "riga", id: "pdc" }, id: "d", onapri: nulla, onchiudi: nulla } });
    const suoi = S.fili.filter((f) => f.da === "pdc" || f.a === "pdc");
    expect(body.match(/class="ver/g)).toHaveLength(suoi.length);
    expect(body).toContain("→ Camera");
    const verso = suoi.find((f) => f.a === "pdc")!;
    expect(body).toContain(`← ${S.nodi.find((n) => n.id === verso.da)!.nome}`);
  });

  describe("tondi, notizie e battuta (25, chiusura 3)", () => {
    const v = (registro as unknown as { dopo: Vista }[])[3].dopo;
    const ev = "ev:Emergenza idrogeologica";
    const dettaglio = (w: Vista, id: string) => {
      const S = schema(w);
      return render(DettaglioSchema, { props: { vista: w, nodi: S.nodi, fili: S.fili, tutti: S.fili, tondi: tondiDi(w, S), scelta: { tipo: "filo", id }, id: "d", onapri: nulla, onchiudi: nulla } }).body;
    };
    it("l'elenco delle notizie c'è senza scelta, con una riga per voce in evidenza", () => {
      const { body } = render(Schema, { props: { vista: v } });
      expect(body).toContain(t("schema.notizie"));
      expect(body.match(/<li>/g)).toHaveLength(v.esito.filter((e) => e.in_evidenza).length);
      expect(body).toContain(t("schema.senza_posto"));
      // il conteggio simmetrico: i fili col tondo (es:0 e l'evento)
      expect(body).toContain(t("schema.solo_prima", { n: 2 }));
      // la legenda viene dopo l'elenco: la notizia sta sopra
      expect(body.indexOf(t("schema.notizie"))).toBeLessThan(body.indexOf(t("schema.leg_tondo")));
      expect(t("schema.leg_tondo")).toMatch(/qui sopra/);
    });
    it("il filo col tondo porta la battuta del suo tipo, a biro", () => {
      const body = dettaglio(v, ev);
      expect(body).toContain(t("schema.battuta", { battuta: battutaDi("evento")! }));
    });
    it("nel tono grave la battuta tace", () => {
      const w = { ...v, esito: v.esito.map((e, i) => (i === 0 ? { ...e, tono: "grave" } : e)) } as Vista;
      expect(dettaglio(w, ev)).not.toContain(battutaDi("evento")!);
    });
    it("un filo senza tondo non ha battuta", () => {
      expect(dettaglio(v, "spread")).not.toContain(battutaDi("spread")!);
    });
    it("al turno zero niente notizie e niente filtro", () => {
      const { body } = render(Schema, { props: { vista: turnoZero as unknown as Vista } });
      expect(body).not.toContain(t("schema.notizie"));
      expect(body).not.toContain(t("schema.solo_prima"));
    });
      it("a filtro acceso i conteggi restano quelli del trimestre", () => {
      const d = v.ministeri[0].dicastero;
      const w = { ...v, esito: v.esito.map((e) => (e.tipo === "cantiere_fermo" ? { ...e, dicastero: d } : e)) } as Vista;
      const S = schema(w), T = tondiDi(w, S), F = soloInPrimaPagina(S, T);
      const det = (scelta: { tipo: "filo" | "riga"; id: string }) => render(DettaglioSchema, { props: { vista: w, nodi: F.nodi, fili: F.fili, tutti: S.fili, tondi: T, scelta, id: "d", onapri: nulla, onchiudi: nulla } }).body;
      const body = det({ tipo: "riga", id: `d:${d}` });
      expect(body).toContain(t("schema.n_fili", { n: S.fili.length }));
      // la riga ha fili nel trimestre, solo non in prima pagina
      expect(S.fili.some((f) => f.da === `d:${d}` || f.a === `d:${d}`)).toBe(true);
      expect(body).not.toContain(t("schema.nessun_filo"));
    });
    it("anche la riga dice i suoi numeri in prima pagina", () => {
      const d = v.ministeri[0].dicastero;
      const w = { ...v, esito: v.esito.map((e) => (e.tipo === "cantiere_fermo" ? { ...e, dicastero: d } : e)) } as Vista;
      const S = schema(w), T = tondiDi(w, S), n = T.nodi.get(`d:${d}`)![0];
      const body = render(DettaglioSchema, { props: { vista: w, nodi: S.nodi, fili: S.fili, tutti: S.fili, tondi: T, scelta: { tipo: "riga", id: `d:${d}` }, id: "d", onapri: nulla, onchiudi: nulla } }).body;
      expect(body).toMatch(new RegExp(`${t("schema.in_prima_pagina")}[^]*>${n}</span>`));
    });
    it("nell'elenco numero e notizia sono separati, anche nel nome del pulsante", () => {
      const { body } = render(Schema, { props: { vista: v } });
      expect(body).toMatch(/class="tn[^"]*">7<\/span> <span>/);
    });
  });
});
