// La legge elettorale come bozza (storia 109): le leggi pronte del catalogo e i gradini che il core non ammette.
import type { ValoreProposto } from "$lib/tipi/ValoreProposto";
import type { Vista } from "$lib/tipi/Vista";
import type { VoceMenu } from "$lib/tipi/VoceMenu";

/** Una legge pronta di CatalogoScenario.leggi_elettorali: il gradino di ogni disposizione elettorale. */
export type LeggePronta = Record<string, string>;

const elettorali = (v: Vista) => v.disposizioni.filter((d) => d.norma === "elettorale");

/** Solo ciò che cambia rispetto al vigore, nell'ordine delle disposizioni. */
export const bozzaDiLegge = (v: Vista, legge: LeggePronta): ValoreProposto[] =>
  elettorali(v).flatMap((d) => (legge[d.id] !== undefined && legge[d.id] !== d.in_vigore ? [{ disposizione: d.id, gradino: legge[d.id] }] : []));

/** La legge pronta prende il posto della parte elettorale della bozza; il resto resta. */
export const conLegge = (v: Vista, b: ValoreProposto[], legge: LeggePronta): ValoreProposto[] => {
  const ids = new Set(elettorali(v).map((d) => d.id));
  return [...b.filter((x) => !ids.has(x.disposizione)), ...bozzaDiLegge(v, legge)];
};

/** Bozza e vigore compongono, insieme, proprio questa legge. */
export const eLegge = (v: Vista, b: ValoreProposto[], legge: LeggePronta): boolean =>
  elettorali(v).every((d) => (b.find((x) => x.disposizione === d.id)?.gradino ?? d.in_vigore) === legge[d.id]);

/** I gradini spenti dal menu, col motivo: chiave `disposizione|gradino`. */
export const spenti = (menu: VoceMenu[]): Record<string, string> =>
  Object.fromEntries(menu.filter((m) => !m.ammesso).map((m) => [`${m.disposizione}|${m.gradino}`, m.motivo ?? "non ammesso"]));
