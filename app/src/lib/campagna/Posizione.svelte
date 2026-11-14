<script lang="ts">
  // Posizione (26-campagna.prototype.html, forma A): l'asse, i quattro passi, la conferma sul posto. Anteprima e soglie: 11g.
  import { t, numero, segnato } from "$lib/testi/t";
  import { partita } from "$lib/partita.svelte";
  import { comeErrore, testoRifiuto } from "$lib/guscio/chiamate";
  import type { Vista } from "$lib/tipi/Vista";
  import Conferma from "$lib/componenti/Conferma.svelte";
  import Pulsante from "$lib/componenti/Pulsante.svelte";
  import Rifiuto from "$lib/componenti/Rifiuto.svelte";
  import Segnaposto from "$lib/componenti/Segnaposto.svelte";
  import { PASSI, posizioniVicine } from "./campagna";

  let { vista }: { vista: Vista } = $props();
  const io = $derived(vista.partiti.find((p) => p.id === vista.giocatore));
  const altri = $derived(vista.partiti.filter((p) => p.id !== vista.giocatore && !p.sciolto));
  const vicine = $derived(posizioniVicine(vista));
  const x = (p: number) => `${4 + ((p - 1) / 6) * 92}%`;

  // la bozza è il passo scelto, in memoria; raw: il rifiuto vale per la Vista in cui è nato
  let scelta = $state<number | null>(null);
  let rifiuto = $state.raw<{ testo: string; vista: Vista | null } | null>(null);

  async function sposta() {
    if (scelta === null || partita.occupato) return;
    rifiuto = null;
    try {
      if (await partita.comanda({ tipo: "riposiziona", posizione: scelta })) scelta = null;
    } catch (e) {
      rifiuto = { testo: testoRifiuto(comeErrore(e)), vista: partita.vista };
    }
  }
</script>

<section aria-label={t("campagna.posizione")}>
  <div class="asse" aria-hidden="true">
    <div class="lin"></div>
    {#each [1, 2, 3, 4, 5, 6, 7] as k (k)}<i class="tk" style:left={x(k)}><span>{k}</span></i>{/each}
    {#each altri as p (p.id)}<i class="alt" style:left={x(p.posizione_dichiarata)} title="{p.nome} {numero(p.posizione_dichiarata, 1)}"></i>{/each}
    {#if io}
      <i class="pt" style:left={x(io.posizione_percepita)}></i><i class="pt d" style:left={x(io.posizione_dichiarata)}></i>
    {/if}
  </div>
  <div class="min muto">{t("campagna.pos_legenda")}</div>
  {#if io}
    <dl class="dist">
      <dt>{t("campagna.dichiarata")}</dt><dd>{numero(io.posizione_dichiarata, 2)}</dd>
      <dt>{t("campagna.percepita")}</dt><dd>{numero(io.posizione_percepita, 2)}</dd>
    </dl>
  {/if}
  <Segnaposto sezione={t("campagna.pos_anteprima_vuoto")} fetta="11g" />
  <div class="az" role="group" aria-label={t("campagna.pos_gruppo")}>
    {#each vicine as p, i (p)}
      <Pulsante forma="bordo" scelto={scelta === p} onclick={() => (scelta = p)}>{segnato(PASSI[i], 2)}</Pulsante>
    {/each}
  </div>
  {#if scelta !== null}
    <div class="az">
      <Conferma etichetta={t("campagna.sposta_a", { posizione: numero(scelta, 2) })} conferma={t("campagna.conferma_spostamento")}
        spiegazione={t("campagna.una_volta")} spento={partita.occupato ? t("campagna.comando_in_corso") : undefined} onconferma={sposta} />
    </div>
  {/if}
  {#if rifiuto && rifiuto.vista === partita.vista}<div class="rif"><Rifiuto testo={rifiuto.testo} /></div>{/if}
  <p class="min muto">{t("campagna.pos_nota")}</p>
</section>

<style>
  .min { font-size: var(--c-min) }
  .muto { color: var(--grigio) }
  .asse { position: relative; height: 46px; margin: 6px 0 2px; overflow: hidden }
  .lin { position: absolute; left: 0; right: 0; top: 22px; border-top: 1.5px solid var(--ink) }
  .tk { position: absolute; top: 17px; height: 11px; border-left: 1.5px solid var(--ink) }
  .tk span { position: absolute; top: 12px; left: -4px; font-size: var(--c-min); color: var(--grigio) }
  .pt { position: absolute; top: 15px; width: 14px; height: 14px; margin-left: -7px; border: 2px solid var(--ink); border-radius: 50%; background: var(--carta) }
  .pt.d { background: var(--ink) }
  .alt { position: absolute; top: 19px; width: 8px; height: 8px; margin-left: -4px; background: var(--grigio) }
  .dist { display: grid; grid-template-columns: auto 1fr; gap: 2px 10px; font-size: var(--c-sec); margin: 4px 0 }
  .dist dt { color: var(--grigio) }
  .az { display: flex; flex-wrap: wrap; gap: 6px; align-items: center; margin: 6px 0 }
  .rif { border: 1.5px solid var(--rosso); padding: 4px 8px; margin: 2px 0 }
</style>
