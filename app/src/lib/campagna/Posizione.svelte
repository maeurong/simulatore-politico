<script lang="ts">
  // Posizione (26-campagna.prototype.html, forma A): l'asse, il metro della distanza, i quattro passi, la conferma sul posto.
  import { t, numero, segnato } from "$lib/testi/t";
  import { partita } from "$lib/partita.svelte";
  import { guscio, comeErrore, testoRifiuto } from "$lib/guscio/chiamate";
  import { ultimoVince } from "$lib/editor/editor.svelte";
  import type { AnteprimaRiposizionamento } from "$lib/tipi/AnteprimaRiposizionamento";
  import type { Vista } from "$lib/tipi/Vista";
  import Conferma from "$lib/componenti/Conferma.svelte";
  import Pulsante from "$lib/componenti/Pulsante.svelte";
  import Rifiuto from "$lib/componenti/Rifiuto.svelte";
  import { PASSI, distanzaDi, posizioniVicine, sogliaDi } from "./campagna";

  let { vista }: { vista: Vista } = $props();
  const io = $derived(vista.partiti.find((p) => p.id === vista.giocatore));
  const altri = $derived(vista.partiti.filter((p) => p.id !== vista.giocatore && !p.sciolto));
  const vicine = $derived(posizioniVicine(vista));
  const x = (p: number) => `${4 + ((p - 1) / 6) * 92}%`;

  // la bozza è il passo scelto, in memoria; raw: il rifiuto vale per la Vista in cui è nato
  let scelta = $state<number | null>(null);
  let rifiuto = $state.raw<{ testo: string; vista: Vista | null } | null>(null);

  // l'anteprima del passo scelto: una in volo, vince l'ultima; vale solo se è ancora il passo scelto
  let anteprima = $state.raw<{ posizione: number; esito: AnteprimaRiposizionamento | { rifiuto: string } } | null>(null);
  const chiedi = ultimoVince(
    (p: number) => guscio.anteprimaRiposizionamento(p),
    (r, posizione) => (anteprima = { posizione, esito: "distanza" in r ? r : { rifiuto: testoRifiuto(comeErrore(r)) } }),
  );
  const corrente = $derived(anteprima && anteprima.posizione === scelta ? anteprima.esito : null);
  const nuova = $derived(corrente && "distanza" in corrente ? corrente : null);
  const rifiutata = $derived(corrente && "rifiuto" in corrente ? corrente.rifiuto : null);
  const d = $derived(distanzaDi(vista));
  const w = (x: number) => `${Math.min(100, (x / (vista.campagna.soglia_scissione * 1.2)) * 100)}%`;
  const dNuova = $derived(nuova ? sogliaDi(nuova.distanza, vista.campagna) : null);

  const spiega = $derived(
    nuova
      ? [
          t(nuova.distanza > d ? "campagna.dist_sale" : nuova.distanza < d ? "campagna.dist_scende" : "campagna.dist_resta", { d: numero(nuova.distanza, 2) }),
          dNuova === "scissione" ? t("campagna.dist_scissione") : dNuova === "minaccia" ? t("campagna.dist_minaccia") : "",
          t("campagna.una_volta"),
        ].filter(Boolean).join(" ")
      : "",
  );

  function scegli(p: number) {
    scelta = p;
    rifiuto = null;
    chiedi(p);
  }

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
    {#if nuova}<i class="pt nuovo" style:left={x(nuova.percepita)}></i><i class="pt nuovo d" style:left={x(nuova.dichiarata)}></i>{/if}
  </div>
  <div class="min muto">{t("campagna.pos_legenda")}</div>
  {#if io}
    <dl class="dist">
      <dt>{t("campagna.dichiarata")}</dt><dd>{numero(io.posizione_dichiarata, 2)}{#if nuova}{" "}→ <b>{numero(nuova.dichiarata, 2)}</b>{/if}</dd>
      <dt>{t("campagna.percepita")}</dt><dd>{numero(io.posizione_percepita, 2)}{#if nuova}{" "}→ <b>{numero(nuova.percepita, 2)}</b>{/if}</dd>
      <dt>{t("campagna.distanza")}</dt>
      <dd>
        <b>{numero(d, 2)}</b>{#if nuova}{" "}→ <b>{numero(nuova.distanza, 2)}</b>{/if}
        {#if sogliaDi(d, vista.campagna) !== "sotto"}<span class="no">{t("campagna.minaccia_accesa")}</span>{/if}
      </dd>
    </dl>
    <div class="metro" role="img" aria-label={t("campagna.metro_etichetta")}>
      <i style:width={w(d)}></i>{#if nuova}<i class="nuovo" style:width={w(nuova.distanza)}></i>{/if}
      <b style:left={w(vista.campagna.minaccia_scissione)}><span>{t("campagna.metro_minaccia")}</span></b>
      <b style:left={w(vista.campagna.soglia_scissione)}><span>{t("campagna.metro_scissione")}</span></b>
    </div>
  {/if}
  <div class="az" role="group" aria-label={t("campagna.pos_gruppo")}>
    {#each vicine as p, i (p)}
      <Pulsante forma="bordo" scelto={scelta === p} onclick={() => scegli(p)}>{segnato(PASSI[i], 2)}</Pulsante>
    {/each}
  </div>
  {#if rifiutata}<div class="rif"><Rifiuto testo={rifiutata.replace("comando non ammesso: ", "")} /></div>{/if}
  {#if scelta !== null && nuova}
    <div class="az">
      <Conferma etichetta={t("campagna.sposta_a", { posizione: numero(scelta, 2) })} conferma={t("campagna.conferma_spostamento")}
        spiegazione={spiega} spento={partita.occupato ? t("campagna.comando_in_corso") : undefined} onconferma={sposta} />
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
  .pt.nuovo { border-color: var(--biro); opacity: .8 }
  .pt.nuovo.d { background: var(--biro) }
  .metro { position: relative; height: 14px; border: 1.5px solid var(--ink); margin: 4px 0 16px }
  .metro i { position: absolute; top: 0; bottom: 0; left: 0; background: var(--ink) }
  .metro i.nuovo { background: var(--biro); opacity: .55 }
  .metro b { position: absolute; top: -3px; bottom: -3px; border-left: 2px solid var(--rosso) }
  .metro b span { position: absolute; top: 16px; left: -14px; font-size: var(--c-min); color: var(--rosso); white-space: nowrap; font-weight: 400 }
  .no { color: var(--rosso); font-weight: 700; font-size: var(--c-min) }
  .alt { position: absolute; top: 19px; width: 8px; height: 8px; margin-left: -4px; background: var(--grigio) }
  .dist { display: grid; grid-template-columns: auto 1fr; gap: 2px 10px; font-size: var(--c-sec); margin: 4px 0 }
  .dist dt { color: var(--grigio) }
  .az { display: flex; flex-wrap: wrap; gap: 6px; align-items: center; margin: 6px 0 }
  .rif { border: 1.5px solid var(--rosso); padding: 4px 8px; margin: 2px 0 }
</style>
