<script lang="ts">
  // La cassa nel dettaglio a lato (26-campagna.prototype.html, forma A): il conto, l'avviso della campagna tagliata, il sondaggio privato, i rivali.
  import { t, elenco, numero } from "$lib/testi/t";
  import { partita } from "$lib/partita.svelte";
  import { coloreDi } from "$lib/partiti";
  import { comeErrore, testoRifiuto } from "$lib/guscio/chiamate";
  import type { Spesa } from "$lib/tipi/Spesa";
  import type { Vista } from "$lib/tipi/Vista";
  import Pulsante from "$lib/componenti/Pulsante.svelte";
  import Rifiuto from "$lib/componenti/Rifiuto.svelte";
  import { conto, nomeBersaglio, rivali, taglio } from "./campagna";

  let { vista, spese }: { vista: Vista; spese: Spesa[] } = $props();

  // l'avviso si ricalcola a ogni Vista nuova: la cassa può scendere sotto la bozza
  const c = $derived(conto(vista, spese));
  const tagli = $derived(taglio(spese, c.cassa).filter((x) => x.esito !== "intera"));
  const voci = $derived(
    tagli.map((x) => {
      const nome = nomeBersaglio(vista, spese[x.indice].bersaglio);
      return x.esito === "cade" ? t("campagna.voce_cade", { nome }) : t("campagna.voce_scende", { nome, euro: numero(x.paga) });
    }),
  );
  const rival = $derived(rivali(vista));
  const massimo = $derived(Math.max(1, ...rival.map((r) => r.cassa)));
  const prezzo = $derived(vista.acquisti.prezzi.find((p) => p.acquisto === "sondaggio_privato")?.euro);
  const privato = $derived(vista.acquisti.sondaggio_privato);

  // raw: il rifiuto vale per la Vista in cui è nato
  let rifiuto = $state.raw<{ testo: string; vista: Vista | null } | null>(null);
  async function compra() {
    if (partita.occupato) return;
    rifiuto = null;
    try {
      await partita.comanda({ tipo: "compra", acquisto: "sondaggio_privato" });
    } catch (e) {
      rifiuto = { testo: testoRifiuto(comeErrore(e)), vista: partita.vista };
    }
  }
</script>

<dl class="conto">
  <dt>{t("campagna.conto_cassa")}</dt><dd>{numero(c.cassa)}&nbsp;€</dd>
  {#if c.sondaggio}<dt>{t("campagna.conto_sondaggio")}</dt><dd class="muto">{numero(c.sondaggio)}&nbsp;€</dd>{/if}
  {#each spese as s, i (i)}
    <dt>{nomeBersaglio(vista, s.bersaglio)} · {t(`leva.${s.leva}`).toLowerCase()}</dt><dd class="neg">−{numero(s.euro)}&nbsp;€</dd>
  {/each}
  <dt class="tot">{t("campagna.conto_resta")}</dt><dd class="tot" class:neg={c.resto < 0}>{numero(c.resto)}&nbsp;€</dd>
</dl>

{#if tagli.length}
  <div class="avv" role="status">
    <span><b>{t("campagna.non_basta")}</b> {t("campagna.non_basta_corpo", { chiede: numero(c.spese), cassa: numero(c.cassa), voci: elenco(voci, ", ") })}</span>
    <span class="min">{t("campagna.scissione_cassa")}</span>
  </div>
{/if}

<h4 class="h5">{t("campagna.sondaggio_privato")}</h4>
{#if privato}
  <p class="sec">{t("campagna.sondaggio_comprato", { n: numero(privato.istituti[0]?.campione ?? 0) })}</p>
{:else}
  <p class="sec">{t("campagna.sondaggio_spiega")}</p>
{/if}
{#if prezzo !== undefined}
  <Pulsante forma="bordo" style="white-space: normal; text-align: left" aria-disabled={partita.occupato ? "true" : undefined} onclick={compra}>{t("campagna.compra_sondaggio", { euro: numero(prezzo) })}</Pulsante>
{/if}
{#if rifiuto && rifiuto.vista === partita.vista}<div class="rif"><Rifiuto testo={rifiuto.testo} /></div>{/if}

<h4 class="h5">{t("campagna.rivali")}</h4>
<div class="riv">
  {#each rival as r (r.id)}
    <span>{r.nome}</span><i class="b" style:width="{(r.cassa / massimo) * 100}%" style:background={coloreDi(r.id) ?? "var(--retino)"}></i><span class="x">{numero(r.cassa)}&nbsp;€</span>
    {#if r.promesse.length}<span class="pm">{t("campagna.promesse_sospese", { voci: r.promesse.map((p) => `${p.disposizione}: ${p.gradino}`).join("; ") })}</span>{/if}
  {/each}
</div>
<p class="min muto">{t("campagna.rivali_nota")}</p>

<style>
  .sec { font-size: var(--c-sec) }
  .min { font-size: var(--c-min) }
  .muto { color: var(--grigio) }
  .h5 { font-size: var(--c-min); letter-spacing: .1em; text-transform: uppercase; color: var(--grigio); font-weight: 700; margin: 12px 0 4px }
  .conto { display: grid; grid-template-columns: 1fr auto; gap: 2px 10px; font-size: var(--c-sec) }
  .conto dd { text-align: right; font-variant-numeric: tabular-nums }
  .tot { border-top: var(--bordo); font-weight: 700; padding-top: 2px }
  .neg { color: var(--rosso) }
  .avv { border: 1.5px solid var(--rosso); padding: 7px 10px; font-size: var(--c-sec); margin: 6px 0; display: grid; gap: 3px }
  .avv b { color: var(--rosso) }
  .rif { margin-top: 4px; border: 1.5px solid var(--rosso); padding: 4px 8px }
  .riv { display: grid; grid-template-columns: 110px minmax(0, 1fr) 84px; gap: 2px 8px; align-items: center; font-size: var(--c-sec) }
  .b { height: 10px; border: 1px solid var(--ink) }
  .x { text-align: right; font-weight: 700; white-space: nowrap; font-variant-numeric: tabular-nums }
  .pm { grid-column: 2 / -1; font-size: var(--c-min); color: var(--grigio); margin: -1px 0 3px }
</style>
