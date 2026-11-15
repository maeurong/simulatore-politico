<script lang="ts">
  // Dirotta a lato (18-governo.prototype.html, funzione dirotta): gli atti del dicastero col costo in soddisfazione, e per Sviluppo economico e Infrastrutture la regione bersaglio.
  // Atto e regione si scelgono insieme; il costo di ognuno lo dà il core (costoDirotta) e il totale è la somma. Una nuova apertura annulla il giro di richieste della vecchia.
  import { onMount, untrack } from "svelte";
  import { t, segnato } from "$lib/testi/t";
  import { comeErrore, guscio } from "$lib/guscio/chiamate";
  import { partita } from "$lib/partita.svelte";
  import { chiaveDi, fondoDi } from "$lib/caricature/caricatura";
  import { REGIONI } from "$lib/mappa/regioni";
  import type { Comando } from "$lib/tipi/Comando";
  import type { Vista } from "$lib/tipi/Vista";
  import Faccia from "$lib/componenti/Faccia.svelte";
  import Pulsante from "$lib/componenti/Pulsante.svelte";
  import Rifiuto from "$lib/componenti/Rifiuto.svelte";
  import SelettoreRegione from "$lib/mappa/SelettoreRegione.svelte";
  import { attiDirotta, conRegione, costiDirotta, euro, totaleDirotta, type Figurina } from "./governo";

  let { vista, f, manda, rifiuto, onchiudi }: { vista: Vista; f: Figurina; manda: (c: Comando) => Promise<boolean>; rifiuto: string | null; onchiudi: () => void } = $props();

  const min = $derived(vista.ministeri.find((x) => x.dicastero === f.dicastero)!);
  const a = $derived(f.agenda!);
  const m = $derived(f.ministro!);
  const cr = $derived(conRegione(min));
  let atto = $state(untrack(() => f.agenda!.atto));
  let regione = $state<string | null>(untrack(() => f.agenda!.regione));
  let costi = $state.raw<Awaited<ReturnType<typeof costiDirotta>> | null>(null);
  let radice: HTMLElement;
  onMount(() => radice.focus());

  $effect(() => {
    const d = f.dicastero;
    let vivo = true;
    costi = null;
    untrack(() => costiDirotta((x, r) => guscio.costoDirotta(d, x, r), attiDirotta(min).map((x) => x.id), cr ? REGIONI.map((r) => r.id).filter((r) => r !== a.regione) : [], () => vivo))
      .then((c) => { if (vivo) costi = c; })
      .catch((e) => { if (vivo) { partita.guasto = comeErrore(e).testo; onchiudi(); } });
    return () => { vivo = false; };
  });

  // il costo è una variazione (negativa = soddisfazione persa); con segno e arrotondato una volta, come «Ferma»
  const num = (c: number | string | undefined) => (typeof c === "number" ? c : null);
  const voce = (n: number) => (n === 0 ? t("governo.dirotta_gratis") : segnato(n));
  const etichetta = (c: number | string | undefined) => (c === undefined ? "…" : typeof c === "number" ? voce(totaleDirotta(c, null)) : "");
  // la mappa tinge per soddisfazione persa: un guadagno è classe zero
  const persa = (c: number | string | undefined) => (typeof c === "number" ? Math.max(0, Math.round(-c)) : null);
  const grandezza = (id: string) => vista.grandezze.find((g) => g.id === id)?.nome ?? id;
  const effetto = (e: { grandezza: string; migliora: boolean; ritardo: number }) =>
    t("governo.effetto", { nome: grandezza(e.grandezza), verso: t(e.migliora ? "governo.effetto_meglio" : "governo.effetto_peggio"), quando: e.ritardo ? t("governo.effetto_fra", { n: e.ritardo }) : t("governo.effetto_subito") });

  const cambia = $derived(atto !== a.atto || (cr && regione !== a.regione));
  const totale = $derived(totaleDirotta(atto !== a.atto ? num(costi?.atti[atto]) : null, cr && regione && regione !== a.regione ? num(costi?.regioni[regione]) : null));
  const rifiutoRegione = $derived(cr && regione && typeof costi?.regioni[regione] === "string" ? (costi!.regioni[regione] as string) : undefined);
  const blocco = $derived(!costi ? t("governo.dirotta_calcolo") : rifiutoRegione ?? (cambia ? undefined : t("governo.dirotta_scegli")));
  const azione = $derived(t("governo.dirotta_atto", { costo: voce(totale) }));
  const valoriRegioni = $derived(Object.fromEntries(REGIONI.map((r) => [r.id, r.id === a.regione ? null : persa(costi?.regioni[r.id])])));

  const fai = () => manda({ tipo: "dirotta", dicastero: f.dicastero, atto: atto !== a.atto ? atto : null, regione: cr && regione !== a.regione ? regione : null }).then((ok) => ok && onchiudi());
  const tasto = (e: KeyboardEvent) => { if (e.key === "Escape") { e.preventDefault(); onchiudi(); } };
</script>

<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
<section class="carta" bind:this={radice} tabindex="-1" aria-label={t("governo.dirotta_titolo", { dicastero: f.nomeDicastero })} onkeydown={tasto}>
  <div class="su">
    <Faccia nome={m.nome} ruolo={m.tecnico ? "tecnico" : "ministro"} partito={chiaveDi(vista, m.partito)} misura={62} fondo={fondoDi(m.soddisfazione, vista.soglie)} dicastero={f.dicastero} />
    <div>
      <div class="occ">{t("governo.dirotta_titolo", { dicastero: f.nomeDicastero })}</div>
      <h4 class="tit">{m.nome}</h4>
      <div class="sec muto">{t("governo.dirotta_spiega")}</div>
    </div>
  </div>

  <div class="sez"><span>{t("governo.dirotta_atti")}</span><span>{t("governo.dirotta_costo")}</span></div>
  <div class="atti">
    {#each min.atti as x (x.id)}
      {@const cur = x.id === a.atto}
      {@const no = cur ? null : typeof costi?.atti[x.id] === "string" ? (costi.atti[x.id] as string) : !x.pagabile ? (x.prerequisito_mancante ?? t("governo.dirotta_non_pagabile")) : null}
      <button type="button" class="atto" class:on={atto === x.id} aria-pressed={atto === x.id} aria-disabled={no ? "true" : undefined} onclick={() => { if (!no) atto = x.id; }}>
        <span><b>{x.nome}</b>{#if cur} <span class="chip">{t("governo.dirotta_in_agenda")}</span>{/if}</span>
        <span class="nf">{cur || no ? "" : etichetta(costi?.atti[x.id])}</span>
        <small>{euro(x.costo)}{x.turni > 1 ? ` · ${t("governo.dirotta_cantiere", { n: x.turni })}` : ""} · {x.effetti.map(effetto).join(" · ")}{#if no} · <span class="male">{no}</span>{/if}</small>
      </button>
    {/each}
  </div>

  {#if cr}
    <div class="sez"><span>{t("governo.dirotta_regione")}</span><span>{t("governo.dirotta_regione_sua", { regione: REGIONI.find((r) => r.id === a.regione)?.nome ?? "" })}</span></div>
    <p class="min muto">{a.motivo}</p>
    <SelettoreRegione valori={valoriRegioni} bind:scelta={regione} {azione} spento={blocco} legenda={t("governo.dirotta_legenda")} formato={(_, id) => etichetta(costi?.regioni[id])} senzaAzioni onconferma={fai} onannulla={onchiudi} />
  {/if}
  {#if rifiuto}<Rifiuto testo={rifiuto} />{/if}
  <div class="az">
    <Pulsante forma="bordo" spento={cr && !regione ? t("mappa.scegli") : blocco} onclick={fai}>{azione}</Pulsante>
    <Pulsante forma="link" onclick={onchiudi}>{t("governo.dirotta_chiudi")}</Pulsante>
  </div>
</section>

<style>
  .carta { border: var(--forte); background: var(--carta); padding: 10px 12px; display: grid; gap: 7px; align-content: start }
  .su { display: grid; grid-template-columns: auto minmax(0, 1fr); gap: 12px; align-items: center }
  .tit { font-family: var(--tit); font-variation-settings: var(--wonk); font-weight: 700; line-height: 1.1; text-wrap: balance; font-size: var(--c-sez) }
  .occ { font-size: var(--c-min); letter-spacing: .1em; text-transform: uppercase; font-weight: 700; color: var(--rosso) }
  .sec { font-size: var(--c-sec) }
  .min { font-size: var(--c-min) }
  .muto { color: var(--grigio) }
  .male { color: var(--rosso) }
  .sez { font-size: var(--c-min); letter-spacing: .1em; text-transform: uppercase; font-weight: 700; color: var(--grigio); border-bottom: 1.5px solid var(--ink); padding-bottom: 2px; margin-top: 4px; display: flex; justify-content: space-between; gap: 8px }
  .atti { display: grid; gap: 0 }
  .atto { display: grid; grid-template-columns: minmax(0, 1fr) auto; gap: 1px 10px; text-align: left; padding: 4px 6px; border-bottom: var(--filo); font-size: var(--c-sec) }
  .atto small { grid-column: 1 / -1; font-size: var(--c-min); color: var(--grigio) }
  .atto:hover { background: var(--velo) }
  .atto.on { background: var(--giallo) }
  :global(:root.grave) .atto.on { background: var(--ink); color: var(--carta) }
  :global(:root.grave) .atto.on small { color: inherit }
  .atto[aria-disabled="true"] { color: var(--grigio); cursor: default }
  .atto[aria-disabled="true"]:hover { background: none }
  .nf { font-weight: 700; font-variant-numeric: tabular-nums; white-space: nowrap }
  .chip { display: inline-block; border: 1.5px solid var(--ink); padding: 0 6px; font-size: var(--c-min); font-weight: 700; line-height: 1.4 }
  .az { position: sticky; bottom: -8px; z-index: 1; display: flex; gap: var(--s3); align-items: center; flex-wrap: wrap; background: var(--carta); border-top: var(--forte); padding: 6px 0 2px; margin-top: 2px }
</style>
