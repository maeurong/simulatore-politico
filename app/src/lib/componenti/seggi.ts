// I seggi di un elenco a barre, e la maggioranza: un posto solo per Paese, Partiti e BarraSeggi.
import { nomeDi } from "$lib/partiti";
import type { SeggiPartito } from "$lib/tipi/SeggiPartito";
import type { Vista } from "$lib/tipi/Vista";

export const RAMI = ["camera", "senato"] as const;

/** La metà più uno dei seggi della barra. */
export const maggioranza = (totale: number) => Math.floor(totale / 2) + 1;

/** I seggi di un elenco, ramo per ramo; il totale è la somma delle voci, e a zero la barra non si disegna. */
export const barreSeggi = (v: Vista, elenco: SeggiPartito[] | null) =>
  RAMI.flatMap((ramo) => {
    const righe = (elenco ?? []).map((s) => ({ id: s.partito, nome: nomeDi(v, s.partito), seggi: s[ramo] }));
    const totale = righe.reduce((a, r) => a + r.seggi, 0);
    return totale ? [{ ramo, righe, totale }] : [];
  });
