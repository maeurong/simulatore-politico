import { beforeEach, describe, expect, it } from "vitest";
import { render } from "svelte/server";
import registroJson from "../../../../.scratch/v1-piano-11-ui/viste/25-registro.json";
import { t } from "$lib/testi/t";
import type { Vista } from "$lib/tipi/Vista";
import type { VoceEsito } from "$lib/tipi/VoceEsito";
import { registroCache } from "./cache.svelte";
import Registro from "./Registro.svelte";
import { turnoDa } from "./registro";

/* eslint-disable @typescript-eslint/no-explicit-any */
const catena = registroJson as any[];
const vista = catena[8].dopo as Vista;
const esc = (s: string) => s.replace(/&/g, "&amp;").replace(/</g, "&lt;").replace(/"/g, "&quot;");
const html = () => render(Registro, { props: { vista } }).body;

beforeEach(() => registroCache.svuota());

describe("Registro", () => {
  it("con tutti i turni letti: il più recente in pagina, «Torna al turno N», il turno zero in colonna, nessun caricamento", () => {
    registroCache.turni = catena.map((c, i) => turnoDa(i, c.dopo.esito as VoceEsito[]));
    const h = html();
    expect(h).toContain(t("registro.torna", { n: 8 }));
    expect(h).toContain(esc(catena[8].dopo.esito[0].fatto));
    expect(h).toContain(t("registro.turno", { n: 0 }));
    expect(h.indexOf(t("registro.turno", { n: 8 }))).toBeLessThan(h.indexOf(t("registro.turno", { n: 0 })));
    expect(h).not.toContain(t("registro.carico"));
  });

  it("un turno che non si legge sta in colonna e lo dice; quello che si legge resta", () => {
    registroCache.turni = catena.map((c, i) => (i === 3 ? { turno: 3, guasto: "disco" } : turnoDa(i, c.dopo.esito as VoceEsito[])));
    const h = html();
    expect(h).toContain(t("registro.non_si_legge"));
    expect(h).toContain(t("registro.torna", { n: 8 }));
  });

  it("a metà lettura la colonna dice che legge", () => {
    registroCache.turni = catena.slice(0, 4).map((c, i) => turnoDa(i, c.dopo.esito as VoceEsito[]));
    expect(html()).toContain(t("registro.carico"));
  });
});
