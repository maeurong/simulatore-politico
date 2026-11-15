import { describe, expect, it } from "vitest";
import type { Vista } from "$lib/tipi/Vista";
import aula from "../../../../.scratch/v1-piano-11-ui/viste/19-aula.json";
import ordinario from "../../../../.scratch/v1-piano-11-ui/viste/03-governo-ordinario.json";
import { bozzaDelComando, bozzaInvariata, cambia, comandoBozza, etichettaBozza, forma, leggiAnteprima, misuraNominata, perDicastero, rifiutoDellaBozza, statoGradino } from "./bozza";

const stati = aula as unknown as Record<string, { vista: Vista }>;
const v03 = ((ordinario as unknown as { vista?: Vista }).vista ?? ordinario) as unknown as Vista;
const opp = stati.opposizione_libero.vista;
const salario = { disposizione: "salario_minimo", gradino: "11" };
const disp = (v: Vista, id: string) => v.disposizioni.find((d) => d.id === id)!;

describe("cambia", () => {
  it("un gradino diverso dal vigente entra; il vigente lascia la bozza vuota", () => {
    const b = cambia(v03, [], "salario_minimo", "11");
    expect(b).toEqual([salario]);
    expect(cambia(v03, b, "salario_minimo", "nessuno")).toEqual([]);
  });
  it("due gradini della stessa disposizione tengono l'ultimo", () => {
    expect(cambia(v03, cambia(v03, [], "salario_minimo", "11"), "salario_minimo", "9")).toEqual([{ disposizione: "salario_minimo", gradino: "9" }]);
  });
  it("il gradino già in bozza la toglie; le altre disposizioni restano", () => {
    const b = cambia(v03, cambia(v03, [], "salario_minimo", "11"), "regime_pensioni", "retributivo");
    expect(cambia(v03, b, "salario_minimo", "11")).toEqual([{ disposizione: "regime_pensioni", gradino: "retributivo" }]);
  });
});

describe("forma", () => {
  it("riforma, costituzionale, elettorale, null", () => {
    expect(forma(v03, [salario])).toBe("riforma");
    expect(forma(v03, [{ disposizione: "numero_parlamentari", gradino: "300_150" }])).toBe("costituzionale");
    expect(forma(v03, [{ disposizione: "soglia_lista", gradino: "0" }])).toBe("elettorale");
    expect(forma(v03, [])).toBeNull();
  });
});

describe("etichettaBozza, comandoBozza", () => {
  it("governo: presenta la legge; costituzionale: proponi; opposizione: posto riservato", () => {
    expect(etichettaBozza(v03, [salario])).toBe("aula.presenta_riforma");
    expect(etichettaBozza(v03, [{ disposizione: "soglia_lista", gradino: "0" }])).toBe("aula.presenta_elettorale");
    expect(etichettaBozza(v03, [{ disposizione: "numero_parlamentari", gradino: "300_150" }])).toBe("aula.proponi_costituzionale");
    expect(etichettaBozza(opp, [salario])).toBe("aula.porta_nel_posto");
    expect(etichettaBozza(v03, [])).toBeNull();
  });
  it("il comando: presenta_legge al governo, presenta_proposta all'opposizione, stessi valori", () => {
    expect(comandoBozza(v03, [salario])).toEqual({ tipo: "presenta_legge", valori: [salario] });
    expect(comandoBozza(opp, [salario])).toEqual({ tipo: "presenta_proposta", valori: [salario] });
  });
});

describe("statoGradino", () => {
  const dopo = stati.dopo_elettorale.vista;
  const coalizioni = disp(dopo, "coalizioni");
  const g = (d: typeof coalizioni, id: string) => d.gradini.find((x) => x.id === id)!;
  it("il gradino vigente, in arrivo, in bozza, libero, bloccato", () => {
    expect(coalizioni.legge).toBe("L. 1/2027");
    expect(statoGradino(coalizioni, [], g(coalizioni, "no"))).toBe("in_vigore");
    const inv = disp(dopo, "incentivo_investimenti");
    expect(statoGradino(inv, [], g(inv, "0"))).toBe("in_arrivo");
    expect(statoGradino(inv, [], g(inv, "10"))).toBe("libero");
    expect(statoGradino(inv, [{ disposizione: inv.id, gradino: "10" }], g(inv, "10"))).toBe("in_bozza");
    expect(statoGradino(inv, [], { ...g(inv, "10"), bloccato: true })).toBe("bloccato");
  });
});

describe("perDicastero", () => {
  it("niente elettorali, i dicasteri nell'ordine della Vista, la Costituzione ultima", () => {
    const r = perDicastero(v03);
    expect(r.flatMap((x) => x.voci).some((d) => d.norma === "elettorale")).toBe(false);
    expect(r.at(-1)!.voci.every((d) => d.norma === "costituzionale")).toBe(true);
    const ids = r.flatMap((x) => (x.dicastero ? [x.dicastero] : []));
    expect(ids).toEqual(v03.dicasteri.map((d) => d.id).filter((i) => ids.includes(i)));
    expect(r.flatMap((x) => x.voci)).toHaveLength(v03.disposizioni.filter((d) => d.norma !== "elettorale").length);
  });
});

describe("misuraNominata", () => {
  it("dal testo vero del core («{disposizione} è già in esame») alla misura che la contiene", () => {
    const v = stati.piena.vista;
    expect(misuraNominata(v, "Salario minimo legale è già in esame")?.id).toBe(3);
    expect(misuraNominata(v, "Numero dei parlamentari è già in esame")?.id).toBe(2);
  });
  it("altri testi, o una disposizione non in calendario: null", () => {
    const v = stati.piena.vista;
    expect(misuraNominata(v, "calendario pieno")).toBeNull();
    expect(misuraNominata(v, "Ticket sanitario è già in esame")).toBeNull();
  });
});

describe("rifiutoDellaBozza", () => {
  const r = { testo: "calendario pieno", tipo: "presenta_legge" as const, misura: null, vista: v03, bozza: JSON.stringify([salario]) };
  it("vale per la bozza in cui è nato, non per un'altra", () => {
    expect(rifiutoDellaBozza(r, v03, [salario])).toBe(r);
    expect(rifiutoDellaBozza(r, v03, [])).toBeNull();
    expect(rifiutoDellaBozza(r, v03, [{ disposizione: "regime_pensioni", gradino: "retributivo" }])).toBeNull();
    expect(rifiutoDellaBozza(r, stati.piena.vista, [salario])).toBeNull();
  });
});

describe("bozzaDelComando, rifiutoDellaBozza", () => {
  const valori = [salario];
  const compra = { tipo: "compra", acquisto: { test_misura: { valori } } } as const;
  it("il compra porta il JSON dei valori come la presentazione", () => {
    expect(bozzaDelComando(compra)).toBe(JSON.stringify(valori));
    expect(bozzaDelComando({ tipo: "presenta_legge", valori })).toBe(JSON.stringify(valori));
    expect(bozzaDelComando({ tipo: "compra", acquisto: "centro_studi" })).toBeNull();
    expect(bozzaDelComando({ tipo: "dimettiti" })).toBeNull();
  });
  it("il rifiuto del compra resta visibile sulla stessa bozza e sparisce se cambia", () => {
    const r = { vista: v03, misura: null, bozza: bozzaDelComando(compra) };
    expect(rifiutoDellaBozza(r, v03, valori)).toBe(r);
    expect(rifiutoDellaBozza(r, v03, [])).toBeNull();
  });
});

describe("bozzaInvariata", () => {
  it("vera solo se i valori mandati sono quelli della bozza di adesso: cambiata in volo non si svuota", () => {
    const mandata = comandoBozza(v03, [salario]);
    expect(bozzaInvariata(mandata, [salario])).toBe(true);
    expect(bozzaInvariata(mandata, [salario, { disposizione: "regime_pensioni", gradino: "retributivo" }])).toBe(false);
    expect(bozzaInvariata(mandata, [])).toBe(false);
    expect(bozzaInvariata({ tipo: "dimettiti" }, [salario])).toBe(false);
  });
});

describe("leggiAnteprima", () => {
  const ok = { nome: "x" } as unknown as import("$lib/tipi/Anteprima").Anteprima;
  it("un'anteprima riuscita: nessun errore", () => {
    expect(leggiAnteprima(ok)).toEqual({ ante: ok, errore: null });
  });
  it("un rifiuto toglie l'anteprima e porta il testo del core senza il prefisso", () => {
    expect(leggiAnteprima({ specie: "rifiuto", testo: "comando non ammesso: calendario pieno" })).toEqual({ ante: null, errore: "calendario pieno" });
    expect(leggiAnteprima({ specie: "guasto", testo: "motore spento" })).toEqual({ ante: null, errore: "motore spento" });
  });
  it("il rifiuto «è già in esame» dell'anteprima nomina la misura in esame", () => {
    const { errore } = leggiAnteprima({ specie: "rifiuto", testo: "comando non ammesso: Salario minimo legale è già in esame" });
    expect(misuraNominata(stati.piena.vista, errore!)?.id).toBe(3);
  });
});
