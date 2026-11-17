import type { AnteprimaSfiducia } from "$lib/tipi/AnteprimaSfiducia";
import type { Vista } from "$lib/tipi/Vista";

/**
 * L'anteprima della sfiducia per la Vista `v`: la risposta vale solo se `v` è ancora la Vista corrente quando arriva.
 * undefined = superata, non si scrive niente; null = guasto o rifiuto, il tracciato resta vuoto.
 */
export async function anteprimaPer(v: Vista, chiedi: () => Promise<AnteprimaSfiducia>, corrente: () => Vista | null): Promise<AnteprimaSfiducia | null | undefined> {
  let r: AnteprimaSfiducia | null;
  try {
    r = await chiedi();
  } catch {
    r = null;
  }
  return corrente() === v ? r : undefined;
}
