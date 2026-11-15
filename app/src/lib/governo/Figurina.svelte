<script lang="ts">
  // Forma B · Figurine (18-governo.prototype.html): la carta del ministro nell'album, col filo rosso se l'atto è in evidenza.
  // I bottoni di Ferma e Dirotta stanno sopra il pulsante del nome (z-index); il clic sceglie anche la carta, così il dettaglio mostra cosa è successo.
  import { t, numero, type Chiave } from "$lib/testi/t";
  import { nomeDi } from "$lib/partiti";
  import { chiaveDi, fondoDi } from "$lib/caricature/caricatura";
  import { REGIONI } from "$lib/mappa/regioni";
  import { mottoDi } from "$lib/testi/pungolo";
  import type { Vista } from "$lib/tipi/Vista";
  import Faccia from "$lib/componenti/Faccia.svelte";
  import NumeroFreccia from "$lib/componenti/NumeroFreccia.svelte";
  import { partita } from "$lib/partita.svelte";
  import type { Comando } from "$lib/tipi/Comando";
  import Girevole from "./Girevole.svelte";
  import Pulsante from "$lib/componenti/Pulsante.svelte";
  import { etichettaFerma, euro, statoPrincipale, sulRetro, type Figurina, type StatoCarta } from "./governo";

  let { vista, f, scelta, grave, id, onscelta, manda, ondirotta, anima, volta, onrigira }: {
    vista: Vista; f: Figurina; scelta: boolean; grave: boolean; id: string; onscelta: () => void;
    manda: (c: Comando) => Promise<boolean>; ondirotta: () => void;
    anima: boolean; volta: boolean; onrigira: () => void;
  } = $props();

  const m = $derived(f.ministro);
  const motto = $derived(m && !grave ? mottoDi(m.nome, f.dicastero) : null);
  const regione = $derived(f.agenda?.regione ? (REGIONI.find((r) => r.id === f.agenda!.regione)?.nome ?? f.agenda.regione) : null);
  const rosa = $derived(vista.ministeri.find((x) => x.dicastero === f.dicastero)?.rosa.length ?? 0);
  const puo = $derived(!!m && f.agenda?.stato === "annunciata" && !f.stati.includes("dimissioni_chieste"));
  const girata = $derived(!grave && sulRetro(f) && !volta);
  const stato = $derived(statoPrincipale(f));
  const soglia = (s: StatoCarta) => s === "preavviso" || s === "dimissioni_chieste";
</script>

<article class="fig">
<Girevole {girata} {anima} conRetro={!grave && sulRetro(f)} chiave={f.dicastero}>
{#snippet fronte()}
<div class="fr" class:ev={f.evidenza.length > 0} class:sel={scelta}>
  <div class="cima">
    <div class="sx"><span class="n">{t("governo.numero", { n: f.indice })}</span>
      {#if stato}<span class="chip" class:rosso={soglia(stato)}>{t(`governo.stato_${stato}` as Chiave)}</span>{/if}
    </div>
    {#if m && f.voto !== null}
      <span class="voto" class:basso={m.minaccia} title={t("governo.voto", { n: f.voto })}>
        <NumeroFreccia valore={m.soddisfazione} prima={Math.abs(f.freccia) >= 0.5 ? m.soddisfazione - f.freccia : undefined} formato={(x) => numero(x)} male="scende" sottoSoglia={m.soddisfazione < vista.soglie.dimissioni} />
      </span>
    {/if}
  </div>
  <div class="fz">
    {#if m}<Faccia nome={m.nome} ruolo={m.tecnico ? "tecnico" : "ministro"} partito={chiaveDi(vista, m.partito)} misura={62} fondo={fondoDi(m.soddisfazione, vista.soglie)} dicastero={f.dicastero} />
    {:else}<span class="vuoto" aria-hidden="true">—</span>{/if}
  </div>
  <h4 class="nm"><button type="button" id={girata ? undefined : id} aria-pressed={scelta} onclick={onscelta}>{m ? m.nome : t("governo.senza_ministro")}</button></h4>
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
  {#if puo}
    <div class="az">
      <Pulsante forma="piccolo" aria-disabled={partita.occupato ? "true" : undefined} title={t("governo.ferma_titolo")} onclick={() => { onscelta(); manda({ tipo: "ferma", dicastero: f.dicastero }); }}>{etichettaFerma(f.agenda!.costo_ferma)}</Pulsante>
      <Pulsante forma="piccolo" aria-disabled={partita.occupato ? "true" : undefined} onclick={() => { onscelta(); ondirotta(); }}>{t("governo.dirotta")}</Pulsante>
    </div>
  {/if}
</div>
{/snippet}
{#snippet retro()}
  {#if m && f.agenda}
    <div class="rt" class:sel={scelta}>
      <Faccia nome={m.nome} ruolo={m.tecnico ? "tecnico" : "ministro"} partito={chiaveDi(vista, m.partito)} misura={46} fondo={fondoDi(m.soddisfazione, vista.soglie)} dicastero={f.dicastero} />
      <button type="button" class="nome" id={girata ? id : undefined} aria-pressed={scelta} onclick={onscelta}>{m.nome}</button>
      <span class="bollo" class:biro={f.agenda.stato === "dirottata"}>{t(f.agenda.stato === "fermata" ? "governo.atto_fermato" : "governo.atto_dirottato")}</span>
      <span class="min"><b>{f.agenda.nome}</b>{regione ? ` · ${regione}` : ""}</span>
      <span class="voto" class:basso={m.minaccia}>
        <NumeroFreccia valore={m.soddisfazione} prima={Math.abs(f.freccia) >= 0.5 ? m.soddisfazione - f.freccia : undefined} formato={(x) => numero(x)} male="scende" sottoSoglia={m.soddisfazione < vista.soglie.dimissioni} />
      </span>
      <button type="button" class="link" onclick={onrigira}>{t("governo.rigira_corto")}</button>
    </div>
  {/if}
{/snippet}
</Girevole>
</article>

<style>
  .fig { min-width: 0 }
  .fr, .rt { position: relative; border: var(--bordo); padding: 6px 8px 8px; display: grid; grid-template-columns: minmax(0, 1fr); gap: 3px; align-content: start; min-width: 0; background: var(--carta) }
  .rt { place-content: center; justify-items: center; text-align: center; gap: 6px }
  .nome { font-size: var(--c-sec); font-weight: 700; line-height: 1.15 }
  .link { text-decoration: underline; text-underline-offset: 2px; font-size: var(--c-min) }
  .fr.ev { box-shadow: inset 0 4px 0 var(--rosso) }
  .fr.sel, .rt.sel { border-width: 3px; padding: 4.5px 6.5px 6.5px }
  .fr:hover:not(.sel) { box-shadow: inset 0 0 0 1.5px var(--ink) }
  .fr.ev:hover:not(.sel) { box-shadow: inset 0 4px 0 var(--rosso), inset 0 0 0 1.5px var(--ink) }
  .cima { display: grid; grid-template-columns: minmax(0, 1fr) auto; align-items: start; gap: 2px 4px; font-size: var(--c-min); color: var(--grigio) }
  .sx { display: contents }
  .n { white-space: nowrap }
  .sx .chip { grid-row: 2; grid-column: 1 / -1; justify-self: start }
  .cima .voto { grid-row: 1; grid-column: 2 }
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
  .az { position: relative; z-index: 1; display: flex; flex-wrap: wrap; gap: 3px; margin-top: 2px }
  .az :global(.btn) { padding: 1px 4px }
  .muto { color: var(--grigio) }
  .fermo { text-decoration: line-through; text-decoration-thickness: 1.5px; color: var(--grigio) }
  .vuota { display: inline-block; border: 1.5px dashed var(--grigio); color: var(--grigio); padding: 1px 4px }
  .bollo { display: inline-block; border: 2px solid var(--rosso); color: var(--rosso); font-family: var(--tit); font-weight: 800; font-size: var(--c-min); letter-spacing: .06em; text-transform: uppercase; padding: 0 4px; transform: rotate(-4deg); white-space: nowrap }
  .bollo.biro { border-color: var(--biro); color: var(--biro) }
  :global(:root.grave) .bollo { transform: none }
</style>
