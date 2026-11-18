<script lang="ts">
  // Promesse (26-campagna.prototype.html, forma A): fino a due, scelte su tutto il catalogo, la conferma sul posto. L'intesa che si perde, dall'anteprima del core.
  import { t, elenco, numero } from "$lib/testi/t";
  import { partita } from "$lib/partita.svelte";
  import { guscio, comeErrore, testoRifiuto } from "$lib/guscio/chiamate";
  import { ultimoVince } from "$lib/editor/editor.svelte";
  import { nomeDi } from "$lib/partiti";
  import type { VoceCostoPromessa } from "$lib/tipi/VoceCostoPromessa";
  import type { Vista } from "$lib/tipi/Vista";
  import Conferma from "$lib/componenti/Conferma.svelte";
  import Pulsante from "$lib/componenti/Pulsante.svelte";
  import Rifiuto from "$lib/componenti/Rifiuto.svelte";
  import { MASSIMO_PROMESSE, coppie, prometti, promesseFatte } from "./campagna";

  let { vista }: { vista: Vista } = $props();
  const fatte = $derived(promesseFatte(vista));

  // la bozza vive in memoria; al rientro vale la Vista. raw: il rifiuto vale per la Vista in cui è nato
  let scelte = $state<{ disposizione: string; gradino: string }[]>([]);
  let disposizione = $state("");
  let aperta = $state(false);
  let rifiuto = $state.raw<{ testo: string; vista: Vista | null } | null>(null);

  const disp = (id: string) => vista.disposizioni.find((d) => d.id === id);
  const nomeGradino = (id: string, g: string) => disp(id)?.gradini.find((x) => x.id === g)?.nome ?? g;
  const voce = (s: { disposizione: string; gradino: string }) => `${disp(s.disposizione)?.nome ?? s.disposizione}: ${nomeGradino(s.disposizione, s.gradino)}`;
  const libere = $derived(vista.disposizioni.filter((d) => !scelte.some((s) => s.disposizione === d.id)));
  const aperta_su = $derived(disp(disposizione));

  // l'anteprima delle scelte di ora: una in volo, vince l'ultima; vale solo se la chiave è ancora quella delle scelte
  const chiave = (c: [string, string][]) => c.map(([d, g]) => `${d}=${g}`).join("|");
  let anteprima = $state.raw<{ chiave: string; vista: Vista | null; esito: VoceCostoPromessa[] | { rifiuto: string } } | null>(null);
  const chiedi = ultimoVince(
    (c: [string, string][]) => guscio.anteprimaPromesse(c),
    (r, c) => (anteprima = { chiave: chiave(c), vista: partita.vista, esito: Array.isArray(r) ? r : { rifiuto: testoRifiuto(comeErrore(r)) } }),
  );
  const corrente = $derived(anteprima && anteprima.chiave === chiave(coppie(scelte)) && anteprima.vista === partita.vista ? anteprima.esito : null);
  const perdite = $derived(corrente && Array.isArray(corrente) ? corrente.filter((v) => v.disposizioni.length > 0) : null);
  const rifiutata = $derived(corrente && !Array.isArray(corrente) ? corrente.rifiuto : null);
  const chiediOra = () => scelte.length && chiedi(coppie(scelte));

  function aggiungi(gradino: string) {
    scelte = [...scelte, { disposizione, gradino }];
    disposizione = "";
    aperta = false;
    rifiuto = null;
    chiediOra();
  }
  function togli(i: number) {
    scelte = scelte.filter((_, k) => k !== i);
    aperta = false;
    rifiuto = null;
    chiediOra();
  }
  async function manda() {
    if (partita.occupato) return;
    rifiuto = null;
    try {
      if (await partita.comanda(prometti(scelte))) scelte = [];
    } catch (e) {
      rifiuto = { testo: testoRifiuto(comeErrore(e)), vista: partita.vista };
    }
  }
</script>

<section aria-label={t("campagna.promesse_titolo")}>
  {#if fatte.length}
    {#each fatte as p (p.disposizione)}<div class="pr"><span class="n">{p.disposizione}: {p.gradino}</span></div>{/each}
    <div class="fatto">{t("campagna.promesse_fatte")}</div>
  {:else}
    {#each scelte as s, i (s.disposizione)}
      <div class="pr">
        <span class="n">{voce(s)}</span>
        <Pulsante forma="link" aria-label={t("campagna.togli_promessa_di", { nome: disp(s.disposizione)?.nome ?? s.disposizione })} onclick={() => togli(i)}>{t("campagna.togli_spesa")}</Pulsante>
        <span class="min muto">{t("campagna.oggi", { gradino: nomeGradino(s.disposizione, disp(s.disposizione)?.in_vigore ?? "") })}</span>
      </div>
    {:else}
      <p class="sec muto">{t("campagna.nessuna_promessa")}</p>
    {/each}
    {#if rifiuto && rifiuto.vista === partita.vista}<div class="rif"><Rifiuto testo={rifiuto.testo} /></div>{/if}
    {#if scelte.length < MASSIMO_PROMESSE}
      <h4 class="h5">{t("campagna.promessa_n", { n: scelte.length + 1 })}</h4>
      <select aria-label={t("campagna.disposizione_n", { n: scelte.length + 1 })} bind:value={disposizione}>
        <option value="">{t("campagna.una_disposizione")}</option>
        {#each libere as d (d.id)}<option value={d.id}>{d.nome}</option>{/each}
      </select>
      {#if aperta_su}
        <div class="chips" role="group" aria-label={t("campagna.gradino_n", { n: scelte.length + 1 })}>
          {#each aperta_su.gradini as g (g.id)}
            <Pulsante forma="piccolo" onclick={() => aggiungi(g.id)}>{g.nome}{g.id === aperta_su.in_vigore ? ` · ${t("campagna.in_vigore")}` : ""}</Pulsante>
          {/each}
        </div>
      {/if}
    {/if}
    {#if rifiutata}<div class="rif"><Rifiuto testo={rifiutata} /></div>{/if}
    {#if vista.governa}
      <div class="avv">
        <span><b>{t("campagna.costa_intesa")}</b> {t("campagna.costa_intesa_corpo")}</span>
        {#if perdite?.length}
          {#each perdite as v (v.partito)}
            {@const su = elenco(v.disposizioni.map((id) => disp(id)?.nome ?? id), ", ")}
            <span>{v.intesa < 0 ? t("campagna.intesa_perde", { nome: nomeDi(vista, v.partito), n: numero(-v.intesa, 0), su }) : t("campagna.intesa_zero", { nome: nomeDi(vista, v.partito), su })}</span>
          {/each}
        {:else if perdite}
          <span>{t("campagna.intesa_nessuna")}</span>
        {/if}
      </div>
    {/if}
    {#if scelte.length}
      <div class="az">
        <Conferma bind:aperta etichetta={t("campagna.prometti")} conferma={t(scelte.length === 1 ? "campagna.conferma_promessa" : "campagna.conferma_promesse")}
          spiegazione={t("campagna.prometti_spiega", { voci: elenco(scelte.map((s) => `«${voce(s)}»`), ", ") })} spento={partita.occupato ? t("campagna.comando_in_corso") : undefined} onconferma={manda} />
      </div>
    {/if}
    <p class="min muto">{t("campagna.promesse_nota")}</p>
  {/if}
</section>

<style>
  .sec { font-size: var(--c-sec) }
  .min { font-size: var(--c-min) }
  .muto { color: var(--grigio) }
  .h5 { font-size: var(--c-min); font-weight: 700; text-transform: uppercase; letter-spacing: .06em; margin: 8px 0 3px }
  .pr { display: grid; grid-template-columns: minmax(0, 1fr) auto; gap: 2px 8px; align-items: center; padding: 5px 2px 6px; border-top: var(--filo); background: var(--velo) }
  .pr .min { grid-column: 1 / -1 }
  .n { font-weight: 700; font-size: var(--c-sec); min-width: 0 }
  select { font: inherit; font-size: var(--c-sec); border: 1.5px solid var(--ink); background: var(--carta); padding: 2px 4px; color: var(--ink); max-width: 100% }
  .chips { display: flex; flex-wrap: wrap; gap: 4px; margin-top: 4px }
  .fatto { border: 1.5px solid var(--verde); color: var(--verde); padding: 4px 8px; font-size: var(--c-sec); font-weight: 700; margin-top: 4px }
  :global(:root.grave) .fatto { border-color: var(--ink); color: var(--ink) }
  .avv { border: 1.5px solid var(--rosso); padding: 7px 10px; font-size: var(--c-sec); margin: 6px 0; display: grid; gap: 2px }
  .avv b { color: var(--rosso) }
  .rif { border: 1.5px solid var(--rosso); padding: 4px 8px; margin: 4px 0 }
  .az { display: flex; flex-wrap: wrap; gap: 6px; align-items: center; margin: 6px 0 }
</style>
