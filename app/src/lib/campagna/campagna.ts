// Campagna: i comandi della coalizione e le righe che la mostrano (ticket 26, storie 62-63).
import { nomeDi } from "$lib/partiti";
import type { Cessione } from "$lib/tipi/Cessione";
import type { Comando } from "$lib/tipi/Comando";
import type { Vista } from "$lib/tipi/Vista";

export const PASSO_CESSIONE = 5;

/** Il comando che dichiara la coalizione di oggi con un partito in più o in meno; le cessioni degli usciti cadono. */
export function dichiara(v: Vista, cessioni: Cessione[], partito: string, entra: boolean): Comando {
  const partiti = entra ? [...new Set([...v.coalizione, partito])] : v.coalizione.filter((id) => id !== partito);
  return { tipo: "dichiara_coalizione", partiti, cessioni: cessioni.filter((c) => partiti.includes(c.partito)) };
}

/** Il comando con la cessione a `partito` portata a `seggi`; 0 la toglie. */
export function cedi(v: Vista, cessioni: Cessione[], partito: string, seggi: number): Comando {
  const altre = cessioni.filter((c) => c.partito !== partito);
  if (seggi <= 0) return { tipo: "dichiara_coalizione", partiti: v.coalizione, cessioni: altre };
  return {
    tipo: "dichiara_coalizione",
    partiti: [...new Set([...v.coalizione, partito])],
    cessioni: [...altre, { partito, seggi }],
  };
}

/** I seggi attesi del giocatore alla Camera sulla proiezione pubblica; null senza proiezione. */
export const seggiAttesi = (v: Vista): number | null => v.proiezione?.find((s) => s.partito === v.giocatore)?.camera ?? null;

/** Una riga per partito IA, nell'ordine di Vista.partiti: dichiarato, disponibile, causa. */
export function righeCoalizione(v: Vista) {
  return v.partiti
    .filter((p) => p.id !== v.giocatore)
    .map((p) => {
      const d = v.disponibilita.find((x) => x.partito === p.id);
      return { id: p.id, nome: nomeDi(v, p.id), dichiarato: v.coalizione.includes(p.id), disponibile: d?.disponibile ?? false, causa: d?.causa ?? "" };
    });
}
