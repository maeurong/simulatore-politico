<script lang="ts">
  // La scheda del partito nel lato (24-partiti.prototype.html, forma A): chi è, dove sta, che indole ha, che intesa ha con te, i seggi, il carisma, la cassa.
  import { t, numero, percentuale, segnato, type Chiave } from "$lib/testi/t";
  import { coloreDi, quotaVotanti } from "$lib/partiti";
  import { saltiDeiLeader } from "$lib/caricature/caricatura";
  import type { Vista } from "$lib/tipi/Vista";
  import type { VocePartito } from "$lib/tipi/VocePartito";
  import Caselle from "$lib/componenti/Caselle.svelte";
  import Faccia from "$lib/componenti/Faccia.svelte";
  import Pulsante from "$lib/componenti/Pulsante.svelte";
  import Segnaposto from "$lib/componenti/Segnaposto.svelte";
  import Avvisi from "./Avvisi.svelte";
  import AsseDeiPartiti from "./AsseDeiPartiti.svelte";
  import { CAUSE, avvisi, caselleFascia, cassaBreve, distanza, intesaDi, mosseDi, seggiDi, staPerSciogliersi, ultimoVoto } from "./schieramento";

  let { vista, partito: p, indietro }: { vista: Vista; partito: VocePartito; indietro: () => void } = $props();

  const tu = $derived(p.id === vista.giocatore);
  const mio = $derived(vista.partiti.find((x) => x.id === vista.giocatore));
  const colore = $derived(coloreDi(p.id));
  const lista = $derived(avvisi(vista, p));
  const dist = $derived(distanza(p));
  const intesa = $derived(intesaDi(vista, p.id));
  const seggi = $derived(seggiDi(vista, p.id));
  const voto = $derived(ultimoVoto(vista, p.id));
  const mosse = $derived(mosseDi(vista, p.id));
  const sond = $derived(quotaVotanti(vista.sondaggio, vista.partiti.indexOf(p)));
  const coppia = (s: { camera: number; senato: number } | null) => (s ? `${s.camera} · ${s.senato}` : "—");
  const causa = (c: keyof typeof CAUSE) => t(CAUSE[c]);
</script>

<Pulsante forma="link" onclick={indietro}>‹ {t("partiti.camere")}</Pulsante>
<div class="testa">
  <Faccia nome={tu ? t("partiti.tu") : p.leader} ruolo={tu ? undefined : "leader"} partito={p.nome} salto={saltiDeiLeader(vista)[p.id]} misura={84} />
  <div class="chi">
    <h4 class="tit" class:sciolto={p.sciolto}><i class="sw" class:nato={colore === null} style:background={colore ?? undefined}></i>{tu ? t("partiti.tuo") : p.nome}</h4>
    <p class="ld">{#if tu}{t("partiti.s_leader_tu")}{:else}{t("partiti.s_leader")} <b>{p.leader}</b>{/if}</p>
    <Avvisi {lista} />
  </div>
</div>
{#if p.sciolto}<p class="sec"><b>{t("partiti.s_sciolto")}</b> {t("partiti.s_sciolto_motivo")}</p>{/if}
{#if colore === null}<Segnaposto sezione={t("partiti.ph_origine")} fetta="11f" />{/if}

<h5 class="grp"><span>{t("partiti.s_posizione")}</span><span>{[t("partiti.s_dichiarata", { x: numero(p.posizione_dichiarata, 1) }), ...(!tu && mio ? [t("partiti.s_tu_a", { x: numero(mio.posizione_dichiarata, 1) })] : [])].join(" · ")}</span></h5>
<AsseDeiPartiti partiti={tu || !mio ? [p] : [p, mio]} giocatore={vista.giocatore} larghezza={300} />
<p class="sec">{dist.verso === "ferma" ? t("partiti.s_distanza_ferma") : t("partiti.s_distanza", { x: numero(p.posizione_percepita, 2), verso: t(`partiti.verso_${dist.verso}` as Chiave), n: numero(dist.punti, 2) })}</p>
<Segnaposto sezione={t("partiti.ph_scissione")} fetta="11g" />

{#if !tu}
  <h5 class="grp"><span>{t("partiti.s_indole")}</span></h5>
  <p class="sec">{t(`partiti.voc_lunga_${p.vocazione}`)}; {t(`partiti.reg_lunga_${p.regola}`)}.</p>
{/if}

{#if intesa && "alleato" in intesa}
  {@const a = intesa.alleato}
  <h5 class="grp"><span>{t("partiti.s_intesa")}</span><span>{t("partiti.s_intesa_su", { n: numero(a.intesa, 1) })}</span></h5>
  <span class="ints"><Caselle piene={Math.round(a.intesa / 10)} su={10} rosso={a.ultimatum} nome={t("partiti.intesa", { n: Math.round(a.intesa) })} /></span>
  <Segnaposto sezione={t("partiti.ph_soglie")} fetta="11g" />
  {#if a.ultimatum}
    <p class="sec"><b>{a.causa_ultimatum ? t("partiti.s_ultimatum_per", { causa: causa(a.causa_ultimatum) }) : `${t("partiti.s_ultimatum")}.`}</b> <a class="vai" href="/partita/cruscotto">{t("partiti.s_rispondi")}</a></p>
  {/if}
  {#if a.causa_principale}<p class="min muto">{t("partiti.s_pesa", { causa: causa(a.causa_principale) })}</p>{/if}
  {#if a.variazioni.length}
    <div class="coppie">
      {#each a.variazioni as x, i (i)}<span>{causa(x.causa)} <span class="muto">· {t("partiti.s_turno", { n: x.turno })}</span></span><b>{segnato(x.delta)}</b>{/each}
    </div>
  {/if}
{:else if intesa}
  {@const f = intesa.fascia}
  <h5 class="grp"><span>{t("partiti.s_intesa_col_governo")}</span><span>{t("partiti.s_in_fasce")}</span></h5>
  <span class="ints"><Caselle piene={caselleFascia(f.fascia)} su={3} rosso={f.fascia === "ultimatum"} nome={t("partiti.col_governo", { fascia: t(`partiti.fascia_${f.fascia}` as Chiave) })} /></span>
  {#if f.causa_ultimatum}<p class="sec">{t("partiti.s_ultimatum_per", { causa: causa(f.causa_ultimatum) })}</p>{/if}
  <p class="min muto">{t("partiti.s_incrinato")}</p>
{/if}

<h5 class="grp"><span>{t("partiti.s_seggi")}</span><span>{t("partiti.s_camera_senato")}</span></h5>
<dl class="dati">
  <dt>{t("partiti.in_carica")}</dt><dd><b>{coppia(seggi.carica)}</b></dd>
  <dt>{t("partiti.se_oggi")}</dt>
  <dd>{coppia(seggi.oggi)}{" "}{#if seggi.oggi && seggi.carica} <span class="muto min">{t("partiti.s_alla_camera", { n: segnato(seggi.oggi.camera - seggi.carica.camera) })}</span>{/if}</dd>
  <dt>{t("partiti.sondaggio")}</dt><dd>{sond === null ? "—" : percentuale(sond, 1)} <span class="muto min">{t("partiti.s_media")}</span></dd>
  {#if voto}<dt>{t("partiti.s_ultimo_voto")}</dt><dd>{percentuale(voto.quota, 1)} · {vista.elezioni[0].data.anno}{#if staPerSciogliersi(vista, p)}{" "}<span class="male min">· {t("partiti.s_senza_seggio")}</span>{/if}</dd>{/if}
  {#if !tu}<dt>{t("partiti.s_carisma")}</dt><dd>{segnato(p.valence, 2)} <span class="muto min">{t("partiti.s_carisma_nota")}</span></dd>{/if}
  <dt>{t("partiti.cassa")}</dt><dd><b>{cassaBreve(p.cassa)}</b></dd>
</dl>

{#if p.promesse.length}
  <h5 class="grp"><span>{t("partiti.s_promesse")}</span><span>{t("partiti.s_si_verificano")}</span></h5>
  <div class="coppie">
    {#each p.promesse as x, i (i)}<span>{x.disposizione}: <b>{x.gradino}</b></span><span class="muto">{x.in_sospeso ? t("partiti.s_dal_voto_prima") : t("partiti.s_in_campagna")}</span>{/each}
  </div>
{/if}
{#if mosse.length}
  <h5 class="grp"><span>{t("partiti.s_mosse")}</span></h5>
  {#each mosse as m, i (i)}<p class="mossa">{m}</p>{/each}
{/if}

<style>
  .sec { font-size: var(--c-sec) }
  .min { font-size: var(--c-min) }
  .muto { color: var(--grigio) }
  .male { color: var(--rosso) }
  .ints { font-size: var(--c-min); color: var(--grigio) }
  .tit { font-family: var(--tit); font-variation-settings: var(--wonk); font-weight: 700; line-height: 1.1; font-size: var(--c-sez); display: flex; gap: 6px; align-items: baseline }
  .tit.sciolto { text-decoration: line-through; color: var(--grigio) }
  .sw { width: 10px; height: 10px; border: 1px solid var(--ink); flex: none }
  .sw.nato { background: var(--retino), var(--carta) }
  .testa { display: flex; gap: 10px; align-items: flex-start }
  .chi { display: grid; gap: 3px; min-width: 0 }
  .ld { font-size: var(--c-min); color: var(--grigio) }
  .ld b { color: var(--ink) }
  .grp { display: flex; justify-content: space-between; gap: 8px; margin-top: 6px; padding-bottom: 2px; border-bottom: var(--bordo); font-size: var(--c-min); letter-spacing: .1em; text-transform: uppercase; font-weight: 700; color: var(--grigio) }
  .grp > :last-child:not(:first-child) { letter-spacing: 0; text-transform: none; font-weight: 400; text-align: right }
  .vai { color: inherit; text-decoration: underline; text-underline-offset: 2px; white-space: nowrap }
  .coppie { display: grid; grid-template-columns: minmax(0, 1fr) auto; gap: 1px 10px; font-size: var(--c-min) }
  .dati { display: grid; grid-template-columns: auto minmax(0, 1fr); gap: 3px 12px; font-size: var(--c-sec); align-items: baseline }
  .dati dt { color: var(--grigio) }
  .mossa { font-size: var(--c-min); border-left: 2px solid var(--biro); padding-left: 6px; line-height: 1.3 }
</style>
