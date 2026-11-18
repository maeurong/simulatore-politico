<script lang="ts">
  // La casa del turno in trattativa (ticket 27). Incarico al giocatore: risultato, spartizione e lato; guida e Capo dello Stato sono le pagine di Ribaltone.
  import { onMount } from "svelte";
  import { t, type Chiave } from "$lib/testi/t";
  import { periodoDi } from "$lib/aula/aula";
  import { partita } from "$lib/partita.svelte";
  import { comeErrore, testoRifiuto } from "$lib/guscio/chiamate";
  import type { Vista } from "$lib/tipi/Vista";
  import Evento from "$lib/cruscotto/Evento.svelte";
  import { coloreDi } from "$lib/partiti";
  import Camere from "$lib/partiti/Camere.svelte";
  import Alleati from "./Alleati.svelte";
  import Ribaltone from "./Ribaltone.svelte";
  import Risultato from "./Risultato.svelte";
  import Spartizione from "./Spartizione.svelte";
  import { passaggio, ribaltoneDi, type EsitoManda } from "./trattativa";

  let { vista }: { vista: Vista } = $props();

  /** «occupato»: un clic con un comando in volo non parte. */
  async function manda(comando: Parameters<typeof partita.comanda>[0]): Promise<EsitoManda> {
    if (partita.occupato) return "occupato";
    try {
      return (await partita.comanda(comando)) ? "ok" : "guasto";
    } catch (e) {
      return { rifiuto: testoRifiuto(comeErrore(e)) };
    }
  }
  // la pagina si rimonta quando l'incarico cambia: se è stato un comando di Ribaltone, il fuoco riparte dalla testa
  let testa = $state<HTMLElement>();
  onMount(() => {
    if (!passaggio.fuoco) return;
    passaggio.fuoco = false;
    testa?.focus();
  });

  const coalizione = $derived(ribaltoneDi(vista));
  const insieme = $derived([0, 1].map((i) => (coalizione ?? []).reduce((a, x) => a + (i ? x.senato : x.camera), 0)));
  const periodoVoto = $derived.by(() => { const d = vista.elezioni.at(-1)?.data; return d ? periodoDi(vista, d) : ""; });
  const periodo = $derived(t(`cornice.trimestre_${vista.data.trimestre}` as Chiave, { anno: vista.data.anno }));
</script>

{#if vista.incarico === null}
<p class="sec muto">{t("trattativa.nessun_incarico")}</p>
{:else}
<div class="trt">
  <div class="capo">
    <h3 class="tit" tabindex="-1" bind:this={testa}>{t("trattativa.titolo")}</h3>
    <span class="sec muto">{t("cornice.periodo_turno", { periodo, turno: vista.turno })} · {t(vista.incarico === "giocatore" ? "trattativa.incarico_a_te" : vista.incarico === "guida" ? "trattativa.ribaltone_a" : "trattativa.incarico_altrui")}</span>
  </div>
  {#if vista.incarico === "giocatore"}
    <div class="pag">
      {#if vista.eventi.length}<p class="evt">{t("trattativa.nessuno_risponde")}</p>{/if}
      {#each vista.eventi as evento, i (i)}<Evento {vista} {evento} indice={i} puoRispondere={false} />{/each}
      <Risultato {vista} />
      <Spartizione {vista} {manda} />
    </div>
    <aside class="lato">
      <Alleati {vista} />
    </aside>
  {:else}
    <div class="pag"><Ribaltone {vista} {manda} /></div>
    <aside class="lato">
      {#if coalizione}
        <section class="rib" aria-labelledby="rib-t">
          <h4 class="sez-l" id="rib-t">{t("ribaltone.lato")}</h4>
          {#each coalizione as x (x.id)}
            <div class="prom" class:primo={x.primo}>
              <span><i class="sw" style:background={coloreDi(x.id) ?? undefined} class:nato={coloreDi(x.id) === null}></i>{x.nome}{#if x.primo}{" "}<span class="chip">{t("ribaltone.primo")}</span>{/if}</span>
              <span class="num">{x.camera} · {x.senato}</span>
            </div>
          {/each}
          <div class="prom"><span class="muto">{t("ribaltone.insieme")}</span><span class="num">{insieme[0]} · {insieme[1]}</span></div>
          <p class="min muto">{t("ribaltone.nota_lato", { periodo: periodoVoto })}</p>
        </section>
      {/if}
      <Camere {vista} />
    </aside>
  {/if}
</div>
{/if}

<style>
  .trt { display: grid; grid-template-columns: minmax(0, 1fr) var(--lato); gap: 10px var(--canale); align-content: start }
  .trt > * { min-width: 0 }
  .capo { grid-column: 1 / -1; display: flex; align-items: baseline; justify-content: space-between; gap: 10px; border-bottom: var(--doppio); padding-bottom: 3px }
  .tit { font-family: var(--tit); font-variation-settings: var(--wonk); font-weight: 700; line-height: 1.1; text-wrap: balance }
  h3.tit { font-size: var(--c-riq) }
  .pag { display: grid; gap: 10px; align-content: start }
  .lato { border-left: var(--forte); padding-left: var(--canale); display: grid; gap: 8px; align-content: start }
  .rib { display: grid; gap: 4px; align-content: start }
  .sez-l { font-size: var(--c-min); letter-spacing: .1em; text-transform: uppercase; font-weight: 700; color: var(--grigio); border-bottom: 1.5px solid var(--ink); padding-bottom: 2px }
  .prom { display: grid; grid-template-columns: minmax(0, 1fr) auto; font-size: var(--c-sec); gap: 2px 8px; align-items: baseline; padding: 3px 0; border-bottom: var(--filo) }
  .prom.primo { font-weight: 700 }
  .num { font-weight: 700; white-space: nowrap }
  .min { font-size: var(--c-min) }
  .sw { display: inline-block; width: 10px; height: 10px; border: 1px solid var(--ink); margin-right: 5px; vertical-align: -1px }
  .sw.nato { background: var(--retino), var(--carta) }
  .chip { display: inline-block; border: 1.5px solid var(--ink); padding: 0 6px; font-size: var(--c-min); font-weight: 700; white-space: nowrap; line-height: 1.4 }
  .sec { font-size: var(--c-sec) }
  .muto { color: var(--grigio) }
  .evt { font-size: var(--c-sec); border-left: 3px solid var(--rosso); padding-left: 8px }
  :global(:root.grave) .evt { border-color: var(--ink) }
</style>
