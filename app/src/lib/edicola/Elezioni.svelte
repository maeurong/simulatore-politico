<script lang="ts">
  // Le elezioni di una partita: barra dei voti, etichette, esito del giocatore. Una sola fonte per il dettaglio d'albo e l'eredità.
  import { numero, percentuale, t } from "$lib/testi/t";
  import type { ElezioneAlbo } from "$lib/tipi/ElezioneAlbo";
  import { periodo } from "./edicola";

  let { elezioni, nome }: { elezioni: ElezioneAlbo[]; nome: string } = $props();
</script>

{#each elezioni as e, k (k)}
  {@const tot = e.voti.reduce((s, v) => s + v[1], 0) || 1}
  {@const mio = e.voti.find((v) => v[0] === nome)}
  {@const [primaQuota, dopoQuota] = t("edicola.esito_giocatore", { nome, camera: e.seggi_giocatore[0], senato: e.seggi_giocatore[1] }).split("{quota}")}
  <div class="el">
    <p class="sec"><b>{t("edicola.voto_di", { periodo: periodo(e.data) })}</b> <span class="muto">{t("edicola.affluenza", { quota: percentuale(e.affluenza) })}</span></p>
    <div class="barra" role="img" aria-label={e.voti.map(([n, q]) => `${n} ${percentuale(q)}`).join(", ")}>
      {#each e.voti as [n, q], j (j)}<i class:mio={n === nome} style:flex={q / tot} title="{n} {percentuale(q)}"></i>{/each}
    </div>
    <div class="etich" aria-hidden="true">
      {#each e.voti as [n, q], j (j)}<span class:mio={n === nome} style:flex={q / tot} data-tagli-ignora={n === nome ? undefined : ""}>{q > 0.07 ? `${n.split(" ").pop()} ${numero(q * 100)}%` : ""}</span>{/each}
    </div>
    <p class="sec">{primaQuota}<b>{mio ? percentuale(mio[1]) : t("edicola.assente")}</b>{dopoQuota}</p>
  </div>
{/each}

<style>
  .sec { font-size: var(--c-sec) }
  .muto { color: var(--grigio) }
  .el { display: grid; gap: 4px; padding-top: 6px; border-top: var(--filo) }
  .barra { display: flex; gap: 2px; height: 16px }
  .barra i { background: var(--grigio); min-width: 0 }
  .barra i:nth-child(even) { background: var(--ink) }
  .barra i.mio { background: var(--rosso) }
  /* contain: le etichette, che ci stiano o no, non allargano il contenitore */
  .etich { display: flex; gap: 2px; font-size: var(--c-min); color: var(--grigio); contain: inline-size }
  /* un'etichetta che non ci sta non si mostra: va a capo dietro un primo elemento vuoto e l'altezza di una riga la taglia */
  /* 1.35em è l'interlinea della radice: il ripiego per un WebKit senza lh */
  .etich span { min-width: 0; display: flex; flex-wrap: wrap; height: 1.35em; height: 1lh; overflow: hidden; white-space: nowrap }
  .etich span::before { content: ""; height: 1.35em; height: 1lh }
  /* quella del giocatore non si stringe */
  .etich span.mio { min-width: max-content; color: var(--rosso); font-weight: 700 }
</style>
