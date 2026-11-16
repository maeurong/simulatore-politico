<script lang="ts">
  // Il dettaglio della riga scelta nel lato (prototipo 21, posti dello schema 19 da 36-scelte-11d-conti): bilancio ed economia qui, le soglie in DettaglioSoglie.
  import { tick } from "svelte";
  import { numero, segnato, t, type Chiave } from "$lib/testi/t";
  import { periodo } from "$lib/edicola/edicola";
  import { euro } from "$lib/governo/governo";
  import { apribile } from "$lib/ossatura";
  import { partita } from "$lib/partita.svelte";
  import type { Stima } from "$lib/tipi/Stima";
  import type { Vista } from "$lib/tipi/Vista";
  import Pulsante from "$lib/componenti/Pulsante.svelte";
  import DettaglioSoglie from "./DettaglioSoglie.svelte";
  import Spread from "./Spread.svelte";
  import Forte from "./Forte.svelte";
  import Ventaglio from "./Ventaglio.svelte";
  import { percorso, procedura, reale, trimestrePrima } from "./conti";

  let { vista, scelta = $bindable(), torna }: { vista: Vista; scelta: string | null; torna: () => void } = $props();

  const k = $derived(scelta ?? "");
  const [tipo, id] = $derived(k.split(":"));
  const b = $derived(vista.bilancio);
  const cs = $derived(vista.acquisti.centro_studi);
  const ue = $derived(vista.regole_ue);
  let capo = $state<HTMLElement>();
  // dentro il lato la riga nuova toglie il bottone premuto: il fuoco va a «Torna»
  async function apri(nuova: string) {
    scelta = nuova;
    await tick();
    capo?.querySelector("button")?.focus();
  }
  const cifra = (s: Stima, c: keyof Stima) => `${c === "crescita_pil" ? segnato(s[c], 1) : numero(s[c], 1)}%`;
</script>

<div class="capo-lato" bind:this={capo}><Pulsante forma="link" onclick={torna}>{t("conti.torna")}</Pulsante></div>

{#if tipo === "dic"}
  {@const d = b.dicasteri.find((x) => x.id === id)!}
  {@const nome = vista.dicasteri.find((x) => x.id === id)?.nome ?? id}
  <h4 class="tit">{nome}</h4>
  <p class="sec"><Forte testo={t("conti.dic_reale", { anno: b.anno })} vals={[t("conti.dic_perdita", { nome, perdita: reale(d.perdita_reale).testo })]} /></p>
  <dl class="dati">
    <dt>{t("conti.d_spesa")}</dt><dd><Forte testo={t("conti.all_anno")} vals={[euro(d.spesa)]} /></dd>
    <dt>{t("conti.d_salari")}</dt><dd>{euro(d.salari)}</dd>
    <dt>{t("conti.d_acquisti")}</dt><dd>{euro(d.acquisti)}</dd>
    <dt>{t("conti.d_margine")}</dt><dd>{euro(d.margine)}</dd>
  </dl>
  {#if apribile("bilancio", vista)}
    <a class="link" href="/partita/bilancio?dicastero={encodeURIComponent(id)}">{t("conti.indicizzalo")}</a>
  {:else}
    <p class="sec muto">{t("conti.si_indicizza")}</p>
  {/if}
{:else if tipo === "irpef"}
  {@const s = b.aliquote.irpef[+id]}
  {@const p = b.perdita_reale_scaglioni[+id]}
  <h4 class="tit">{s.fino_a === null ? t("conti.irpef_oltre") : t("conti.irpef_fino", { n: numero(s.fino_a) })}</h4>
  <p class="sec">{#if p === undefined}{t("conti.irpef_ultimo")}{:else}<Forte testo={t("conti.irpef_erosa")} vals={[`${numero(p, 1)}%`]} />{/if}</p>
{:else if k === "fondo"}
  <h4 class="tit">{t("conti.fondo_titolo", { anno: b.anno })}</h4>
  <p class="sec"><Forte testo={t("conti.fondo_restano", { totale: euro(b.fondo_riserva) })} vals={[euro(b.fondo_residuo)]} /></p>
  {#if vista.eventi.length}
    <h5 class="grp"><span>{t("conti.eventi_aperti")}</span></h5>
    {#each vista.eventi as e, i (i)}<div class="r fermo"><span class="n">{e.nome}</span><span class="x">{euro(e.costo)}</span></div>{/each}
  {/if}
{:else if tipo === "stat"}
  {@const c = id as keyof Stima}
  {@const rev = vista.statistica.revisione}
  <h4 class="tit">{t(`conti.stat_${c}` as Chiave)}</h4>
  <dl class="dati">
    <dt>{t("conti.prima_stima")}</dt><dd><b>{cifra(vista.statistica.prima_stima, c)}</b> · {periodo(vista.statistica.trimestre)}</dd>
    {#if cs}<dt>{t("conti.cs_titolo")}</dt><dd><b>{cifra(cs.stima, c)}</b></dd>{/if}
    {#if rev}<dt>{t("conti.rivista")}</dt><dd>{periodo(rev.trimestre)}: {numero(rev.prima_stima[c], 1)} → <b>{numero(rev.riveduta[c], 1)}</b></dd>{/if}
  </dl>
  <p class="sec muto">{t("conti.stat_spiega")}</p>
{:else if k === "spread"}
  <h4 class="tit">{t("conti.spread_titolo", { n: numero(vista.spread_pb) })}</h4>
  <Spread serie={partita.serie} soglia={vista.soglie.crisi_debito_pb} W={360} H={120} />
  <p class="sec">{t("conti.spread_spiega", { n: numero(vista.soglie.crisi_debito_pb) })}</p>
{:else if k === "debito"}
  <h4 class="tit">{t("conti.debito_titolo", { n: numero(vista.debito.debito_pil, 1) })}</h4>
  <p class="sec">{t("conti.debito_spiega")}</p>
  {#if cs}
    <h5 class="grp"><span>{t("conti.debito_studi")}</span></h5>
    <Ventaglio previsione={cs.previsione} k="debito" W={360} H={80} />
    {@const f = cs.previsione.at(-1)}
    {#if f}<p class="sec"><Forte testo={t("conti.prev_fine", { periodo: t(`conti.trim_${f.data.trimestre}` as Chiave, { anno: f.data.anno }), sigma: numero(f.debito.sigma, 1) })} vals={[`${numero(f.debito.centro, 1)}%`]} /></p>{/if}
  {:else}
    <p class="sec muto">{t("conti.debito_senza")}</p>
  {/if}
{:else if k === "ue"}
  {@const pde = procedura(ue)}
  <h4 class="tit">{t("conti.regole_ue")}</h4>
  <dl class="dati">
    <dt>{t("conti.ue_procedura")}</dt>
    <dd><b class:male={pde.male}>{pde.testo}</b>{#if ue.procedura !== "nessuna"}&nbsp;{ue.turni_in_stato === 0 ? t("conti.da_questo_turno") : t("conti.da_turni", { n: ue.turni_in_stato })}{/if}</dd>
    <dt>{t("conti.conto_controllo")}</dt><dd>{t("conti.del_pil", { n: numero(ue.conto_controllo, 1) })}</dd>
    <dt>{t("conti.ue_ultimo")}</dt><dd>{percorso(ue.ultima_deviazione)}</dd>
    <dt>{t("conti.prossimo_controllo")}</dt>
    <dd>{ue.proiezione ? t("conti.ue_conto", { proiezione: t("conti.proiezione", { anno: ue.proiezione.anno, percorso: percorso(ue.proiezione.deviazione) }), n: numero(ue.proiezione.conto, 1) }) : t("conti.manca_un_anno")}</dd>
    <dt>{t("conti.ue_coesione")}</dt><dd>{#if ue.coesione_sospesa}<b class="male">{t("conti.ue_sospesi")}</b>{:else}{t("conti.ue_attivi")}{/if}</dd>
    <dt>{t("conti.ue_energia")}</dt><dd>{ue.margine_energia !== null ? t("conti.ue_energia_chiesto", { anno: ue.margine_energia }) : t("conti.ue_energia_no")}</dd>
  </dl>
  <p class="sec muto">{t("conti.ue_spiega")}</p>
{:else if k === "mes" && vista.debito.mes}
  {@const m = vista.debito.mes}
  <h4 class="tit">{t("conti.mes_titolo")}</h4>
  <p class="sec">{t("conti.mes_spiega", { periodo: periodo(trimestrePrima(m.fine)) })}</p>
  {#each m.vincoli as v, i (i)}<div class="r fermo"><span class="n">{v.disposizione}</span><span class="x">{v.gradino}</span></div>{/each}
{:else if k === "ristr" && vista.debito.ristrutturazione}
  {@const r = vista.debito.ristrutturazione}
  <h4 class="tit">{t("conti.ristr_titolo")}</h4>
  <dl class="dati">
    <dt>{t("conti.ristr_taglio")}</dt><dd>{periodo(r.taglio)}</dd>
    <dt>{t("conti.ristr_premio")}</dt><dd>{t("conti.ristr_premio_pb", { n: numero(r.premio_pb) })}</dd>
    <dt>{t("conti.ristr_redditi")}</dt><dd>{r.fine_perdita ? t("conti.ristr_calano", { periodo: periodo(trimestrePrima(r.fine_perdita)) }) : t("conti.ristr_finita")}</dd>
  </dl>
{:else if tipo === "red"}
  {@const r = vista.redditi.find((x) => x.fascia === id)!}
  <h4 class="tit">{t(`conti.fascia_${r.fascia}`)}</h4>
  <dl class="dati">
    <dt>{t("conti.red_reddito")}</dt><dd><Forte testo={t("conti.red_euro")} vals={[`${numero(r.reddito)} €`]} /></dd>
    <dt>{t("conti.red_trimestre")}</dt><dd>{segnato(r.crescita, 1)}%</dd>
    <dt>{t("conti.red_attese")}</dt><dd>{segnato(r.attesa, 1)}%</dd>
  </dl>
  <p class="sec muto">{t("conti.red_spiega")}</p>
{:else}
  <DettaglioSoglie {vista} scelta={k} {apri} />
{/if}

<style>
  .capo-lato { display: flex }
  .fermo:hover { background: none }
  .fermo .x { white-space: normal }
</style>
