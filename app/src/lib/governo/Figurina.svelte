<script lang="ts">
  // Forma B · Figurine (18-governo.prototype.html): la carta del ministro nell'album, col filo rosso se l'atto è in evidenza.
  // I bottoni di Ferma e Dirotta stanno sopra il pulsante del nome (z-index): li porta il Task 5.
  import { t, numero, type Chiave } from "$lib/testi/t";
  import { nomeDi } from "$lib/partiti";
  import { chiaveDi, fondoDi } from "$lib/caricature/caricatura";
  import { REGIONI } from "$lib/mappa/regioni";
  import { mottoDi } from "$lib/testi/pungolo";
  import type { Vista } from "$lib/tipi/Vista";
  import Faccia from "$lib/componenti/Faccia.svelte";
  import NumeroFreccia from "$lib/componenti/NumeroFreccia.svelte";
  import { euro, type Figurina, type StatoCarta } from "./governo";

  let { vista, f, scelta, grave, id, onscelta }: { vista: Vista; f: Figurina; scelta: boolean; grave: boolean; id: string; onscelta: () => void } = $props();

  const m = $derived(f.ministro);
  const motto = $derived(m && !grave ? mottoDi(m.nome, f.dicastero) : null);
  const regione = $derived(f.agenda?.regione ? (REGIONI.find((r) => r.id === f.agenda!.regione)?.nome ?? f.agenda.regione) : null);
  const rosa = $derived(vista.ministeri.find((x) => x.dicastero === f.dicastero)?.rosa.length ?? 0);
  const soglia = (s: StatoCarta) => s === "preavviso" || s === "dimissioni_chieste";
</script>

<article class="fig" class:ev={f.evidenza.length > 0} class:sel={scelta}>
  <div class="cima">
    <span>{t("governo.numero", { n: f.indice })}</span>
    {#if m && f.voto !== null}
      <span class="voto" class:basso={m.minaccia} title={t("governo.voto", { n: f.voto })}>
        <NumeroFreccia valore={m.soddisfazione} prima={Math.abs(f.freccia) >= 0.5 ? m.soddisfazione - f.freccia : undefined} formato={(x) => numero(x)} male="scende" sottoSoglia={m.soddisfazione < vista.soglie.dimissioni} />
      </span>
    {/if}
  </div>
  {#if f.stati.length}<div class="stati">{#each f.stati as s (s)}<span class="chip" class:rosso={soglia(s)}>{t(`governo.stato_${s}` as Chiave)}</span>{/each}</div>{/if}
  <div class="fz">
    {#if m}<Faccia nome={m.nome} ruolo={m.tecnico ? "tecnico" : "ministro"} partito={chiaveDi(vista, m.partito)} misura={62} fondo={fondoDi(m.soddisfazione, vista.soglie)} dicastero={f.dicastero} />
    {:else}<span class="vuoto" aria-hidden="true">—</span>{/if}
  </div>
  <h4 class="nm"><button type="button" {id} aria-pressed={scelta} onclick={onscelta}>{m ? m.nome : t("governo.senza_ministro")}</button></h4>
  <span class="dc">{f.nomeDicastero}{m ? ` · ${nomeDi(vista, m.partito)}` : ""}</span>
  {#if motto}<span class="motto">{t("governo.motto", { motto })}</span>{/if}
  <div class="at">
    {#if !m}
      <span class="vuota">{t("governo.interim_rosa", { n: rosa })}</span>
    {:else if f.agenda}
      <span class:fermo={f.agenda.stato === "fermata"}>{f.agenda.nome}</span>{regione ? ` · ` : ""}{#if regione}<b>{regione}</b>{/if}
      <span class="muto">· {euro(f.agenda.costo)}</span>
      {#if f.agenda.stato !== "annunciata"}<span class="bollo" class:biro={f.agenda.stato === "dirottata"}>{t(f.agenda.stato === "fermata" ? "governo.atto_fermato" : "governo.atto_dirottato")}</span>{/if}
    {:else if f.senzaAgenda}
      <span class="vuota">{t(`governo.senza_agenda_${f.senzaAgenda.replace(" ", "_")}` as Chiave)}</span>
    {:else}
      <span class="vuota">{t("componenti.nessun_atto")}</span>
    {/if}
  </div>
</article>

<style>
  .fig { position: relative; border: var(--bordo); padding: 6px 8px 8px; display: grid; grid-template-columns: minmax(0, 1fr); gap: 3px; align-content: start; min-width: 0; background: var(--carta) }
  .fig.ev { box-shadow: inset 0 4px 0 var(--rosso) }
  .fig.sel { border-width: 3px; padding: 4.5px 6.5px 6.5px }
  .fig:hover:not(.sel) { box-shadow: inset 0 0 0 1.5px var(--ink) }
  .fig.ev:hover:not(.sel) { box-shadow: inset 0 4px 0 var(--rosso), inset 0 0 0 1.5px var(--ink) }
  .cima { display: flex; justify-content: space-between; align-items: start; gap: 4px; font-size: var(--c-min); color: var(--grigio) }
  .stati { display: flex; flex-wrap: wrap; align-items: start; gap: 2px 4px; min-width: 0 }
  .voto { flex: none }
  .chip { display: inline-block; border: 1.5px solid var(--ink); padding: 0 3px; font-size: var(--c-min); font-weight: 700; line-height: 1.3; color: var(--ink); max-width: 100%; overflow-wrap: anywhere }
  .chip.rosso { border-color: var(--rosso); color: var(--rosso) }
  .voto { font-size: var(--c-num); font-weight: 700; line-height: 1; white-space: nowrap; color: var(--ink) }
  .voto.basso { color: var(--rosso) }
  .fz { justify-self: center }
  .vuoto { display: inline-grid; place-items: center; width: 62px; height: 62px; border: 1.5px dashed var(--grigio); border-radius: 50%; color: var(--grigio); font-size: 19px }
  .nm { font-size: var(--c-sec); line-height: 1.15; font-weight: 700 }
  .nm button { font: inherit; text-align: left; overflow-wrap: normal }
  .nm button::after { content: ""; position: absolute; inset: 0 }
  .dc { font-size: var(--c-min); color: var(--grigio); line-height: 1.2 }
  .motto { font-size: var(--c-min); color: var(--biro); line-height: 1.25 }
  .at { font-size: var(--c-min); line-height: 1.25; border-top: var(--filo); padding-top: 3px; margin-top: 2px }
  .muto { color: var(--grigio) }
  .fermo { text-decoration: line-through; text-decoration-thickness: 1.5px; color: var(--grigio) }
  .vuota { display: inline-block; border: 1.5px dashed var(--grigio); color: var(--grigio); padding: 1px 4px }
  .bollo { display: inline-block; border: 2px solid var(--rosso); color: var(--rosso); font-family: var(--tit); font-weight: 800; font-size: var(--c-min); letter-spacing: .06em; text-transform: uppercase; padding: 0 4px; transform: rotate(-4deg); white-space: nowrap }
  .bollo.biro { border-color: var(--biro); color: var(--biro) }
  :global(:root.grave) .bollo { transform: none }
</style>
