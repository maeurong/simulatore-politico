<script lang="ts">
  // L'Edicola, forma A · Banco (.scratch/v1-piano-11-ui/prototipi/13-edicola.prototype.html:315-336): fuori dalla partita,
  // senza cornice. In corso sopra, albo sotto, il dettaglio della riga scelta a lato.
  import { onMount, tick } from "svelte";
  import { goto } from "$app/navigation";
  import { comeErrore, guscio, type ErroreGuscio } from "$lib/guscio/chiamate";
  import { partita } from "$lib/partita.svelte";
  import { casa } from "$lib/ossatura";
  import { t } from "$lib/testi/t";
  import type { VoceAlbo } from "$lib/tipi/VoceAlbo";
  import type { VocePartita } from "$lib/tipi/VocePartita";
  import Dettaglio from "$lib/componenti/Dettaglio.svelte";
  import Pulsante from "$lib/componenti/Pulsante.svelte";
  import Impostazioni from "$lib/cornice/Impostazioni.svelte";
  import { anniAlGoverno, esitoTesto, inOrdine, periodo, voceNuova } from "$lib/edicola/edicola";
  import RigaPartita from "$lib/edicola/RigaPartita.svelte";
  import DettaglioPartita from "$lib/edicola/DettaglioPartita.svelte";
  import DettaglioAlbo from "$lib/edicola/DettaglioAlbo.svelte";

  const ora = new Date();
  // null finché gli elenchi non arrivano: un guasto al caricamento non si spaccia per un'edicola vuota
  let partite = $state.raw<VocePartita[] | null>(null);
  let albo = $state.raw<VoceAlbo[] | null>(null);
  let scelta = $state.raw<{ k: "p"; id: string } | { k: "a"; voce: VoceAlbo } | null>(null);
  let guasto = $state<string | null>(null);
  let rifiuto = $state<{ id: string; testo: string } | null>(null);
  /** l'id della partita il cui abbandono è in volo */
  let inVolo = $state<string | null>(null);
  let impostazioni = $state(false);

  const ordinate = $derived(inOrdine(partite ?? []));
  const occupato = $derived(partita.occupato || inVolo !== null);
  // la riga scelta, se c'è ancora; altrimenti la prima partita, poi la prima voce d'albo
  const dettaglio = $derived.by(() => {
    const s = scelta;
    const p = s?.k === "p" ? ordinate.find((v) => v.indice.id === s.id) : undefined;
    if (p) return { k: "p" as const, voce: p };
    if (s?.k === "a" && albo?.includes(s.voce)) return { k: "a" as const, voce: s.voce };
    if (ordinate[0]) return { k: "p" as const, voce: ordinate[0] };
    return albo?.[0] ? { k: "a" as const, voce: albo[0] } : null;
  });

  async function carica(): Promise<ErroreGuscio | null> {
    try {
      [partite, albo] = await Promise.all([guscio.partite(), guscio.albo()]);
      return null;
    } catch (e) {
      return comeErrore(e);
    }
  }
  onMount(async () => {
    const e = await carica();
    if (e) guasto = `${e.testo} ${t("edicola.elenchi_non_caricati")}`;
  });

  function azzera(id: string) {
    rifiuto = null;
    guasto = null;
    partita.guasto = null;
    scelta = { k: "p", id };
  }

  /** Il guasto lo scrive partita in partita.guasto; il rifiuto torna qui e sta accanto alla scheda. */
  async function riprendi(id: string) {
    if (occupato) return;
    azzera(id);
    try {
      const vista = await partita.riprendi(id);
      if (vista) await goto(`/partita/${casa(vista)}`);
    } catch (e) {
      rifiuto = { id, testo: comeErrore(e).testo };
    }
  }

  /** Dalle chiamate del guscio, non da partita.riprendi: l'edicola non resta con una partita aperta nello stato. */
  async function abbandona(voce: VocePartita) {
    const id = voce.indice.id;
    if (occupato) return;
    azzera(id);
    inVolo = id;
    let errore: ErroreGuscio | null = null;
    try {
      await guscio.riprendi(id);
      await guscio.abbandona();
    } catch (e) {
      errore = comeErrore(e);
    }
    // anche dopo un errore: se riprendi è riuscito e abbandona no, la partita resta in corso e l'elenco lo dice
    const elenchi = await carica();
    inVolo = null;
    if (errore?.specie === "rifiuto") rifiuto = { id, testo: errore.testo };
    else if (errore) guasto = `${errore.testo} ${t("cornice.rimasta_com_era")}`;
    else if (elenchi) guasto = `${elenchi.testo} ${t("edicola.elenchi_non_caricati")}`;
    else {
      const nuova = voceNuova(albo ?? [], voce.indice.testata.nome);
      if (nuova) scelta = { k: "a", voce: nuova };
    }
    // i pulsanti della conferma non ci sono più: il fuoco va alla riga scelta
    await tick();
    document.querySelector<HTMLElement>('.edicola [aria-current="true"]')?.focus();
  }

  function chiudiImpostazioni() {
    impostazioni = false;
    document.getElementById("apri-impostazioni")?.focus();
  }
  const nuova = () => goto("/scenario");
</script>

<div class="edicola">
  <header class="testa">
    <span class="nomino">{t("cornice.nomino")}</span>
    <h1 class="dove">{t("edicola.dove")}</h1>
    <span class="fuori">
      <Pulsante forma="link" id="apri-impostazioni" aria-expanded={impostazioni}
        onclick={() => (impostazioni ? chiudiImpostazioni() : (impostazioni = true))}>{t("sommario.impostazioni")}</Pulsante>
    </span>
  </header>
  {#if partita.guasto ?? guasto}<p class="guasto" role="alert">{partita.guasto ?? guasto}</p>{/if}

  <main class="corpo">
    <div class="elenchi">
      <section aria-labelledby="titolo-in-corso">
        <div class="titsez">
          <h2 id="titolo-in-corso" class="tit">{t("edicola.in_corso")}</h2>
          {#if ordinate.length}<span class="sec muto">{t("edicola.partite_in_corso", { n: ordinate.length })}</span>{/if}
          <span class="dx"><Pulsante forma="chiude" onclick={nuova}>{t("edicola.nuova_partita")}</Pulsante></span>
        </div>
        {#each ordinate as voce, i (voce.indice.id)}
          <RigaPartita {voce} prima={i === 0} scelta={dettaglio?.voce === voce} {ora} {occupato} inVolo={inVolo === voce.indice.id}
            onscegli={() => (scelta = { k: "p", id: voce.indice.id })}
            onriprendi={() => riprendi(voce.indice.id)}
            onabbandona={() => abbandona(voce)} />
        {:else}
          {#if partite}<p class="vuoto">{t("edicola.nessuna_in_corso")}</p>{/if}
        {/each}
      </section>

      <section aria-labelledby="titolo-albo">
        <div class="titsez">
          <h2 id="titolo-albo" class="tit">{t("edicola.albo")}</h2>
          <span class="sec muto">{t("edicola.albo_sottotitolo")}</span>
        </div>
        {#each albo ?? [] as a (a)}
          {@const sel = dettaglio?.voce === a}
          <!-- come le righe delle partite: il clic sulla riga è comodità, da tastiera si sceglie col nome -->
          <!-- svelte-ignore a11y_click_events_have_key_events, a11y_no_static_element_interactions -->
          <div class={["rp", { sel }]} onclick={() => (scelta = { k: "a", voce: a })}>
            <span class="n"><button type="button" class="scegli" aria-current={sel ? "true" : undefined}>{a.nome}</button></span>
            <span class="azr"><span class="bollo" class:ink={a.esito !== "Senza seggi"}>{esitoTesto(a.esito)}</span></span>
            <span class="m">
              {t("edicola.riga_albo", { scenario: a.scenario, inizio: periodo(a.inizio), fine: periodo(a.fine), governo: anniAlGoverno(a.anni_al_governo) })}
            </span>
          </div>
        {:else}
          {#if albo}<p class="vuoto">{t("edicola.albo_vuoto")}</p>{/if}
        {/each}
      </section>
    </div>

    {#if impostazioni}
      <div class="lato"><Impostazioni onchiudi={chiudiImpostazioni} /></div>
    {:else}
      <Dettaglio aria-label={t("edicola.dove")}>
        {#if dettaglio?.k === "p"}
          <DettaglioPartita voce={dettaglio.voce} {ora} {occupato} inVolo={inVolo === dettaglio.voce.indice.id}
            rifiuto={rifiuto?.id === dettaglio.voce.indice.id ? rifiuto.testo : null}
            onriprendi={() => riprendi(dettaglio.voce.indice.id)}
            onabbandona={() => abbandona(dettaglio.voce)} />
        {:else if dettaglio?.k === "a"}
          {#key dettaglio.voce}<DettaglioAlbo voce={dettaglio.voce} />{/key}
        {:else if partite && albo}
          <div class="tagliando">
            <p class="occ">{t("edicola.primo_numero")}</p>
            <p class="tit">{t("edicola.nessuna_partita")}</p>
            <p class="sec">{t("edicola.invito")}</p>
            <span><Pulsante forma="chiude" onclick={nuova}>{t("edicola.nuova_partita")}</Pulsante></span>
          </div>
        {/if}
      </Dettaglio>
    {/if}
  </main>
</div>

<style>
  /* la pagina sta ferma; il corpo scorre in verticale quando non basta, mai di lato */
  .edicola { position: fixed; inset: 0; display: flex; flex-direction: column; overflow: hidden }
  .testa { flex: none; display: flex; align-items: end; gap: 16px; padding: 12px var(--marg) 8px; border-bottom: var(--doppio) }
  .nomino { font-family: var(--tit); font-variation-settings: var(--wonk); font-weight: 800; font-size: var(--c-pag); color: var(--rosso); line-height: 1; white-space: nowrap }
  .dove { font-family: var(--tit); font-size: var(--c-sez); font-weight: 600; line-height: 1.1 }
  .fuori { margin-left: auto; display: flex; gap: 16px; align-items: center; padding-bottom: 3px }
  .guasto { flex: none; padding: 4px var(--marg); border-bottom: var(--filo); font-size: var(--c-sec); font-weight: 700; color: var(--rosso) }

  .corpo { flex: 1; min-height: 0; display: flex; gap: var(--canale); padding: 14px var(--marg) 16px; overflow: hidden auto }
  .elenchi { flex: 1; min-width: 0; display: grid; gap: 18px; align-content: start }
  .lato { flex: none; width: var(--lato); min-width: 0 }

  .titsez { display: flex; align-items: baseline; gap: 12px; border-bottom: var(--forte); padding-bottom: 4px; margin-bottom: 2px }
  .titsez .dx { margin-left: auto }
  .tit { font-family: var(--tit); font-variation-settings: var(--wonk); font-weight: 700; line-height: 1.1; text-wrap: balance; font-size: var(--c-riq) }
  .sec { font-size: var(--c-sec) }
  .muto { color: var(--grigio) }
  .vuoto { font-size: var(--c-sec); color: var(--grigio); padding: 10px 8px }

  /* le righe dell'albo: la stessa riga di RigaPartita */
  .rp { display: grid; grid-template-columns: minmax(0, 1fr) auto; gap: 2px 14px; padding: 8px; border-bottom: var(--filo); cursor: pointer; align-items: center }
  .rp:hover { background: var(--velo) }
  .rp.sel { background: var(--velo); box-shadow: inset 0 0 0 1.5px var(--ink) }
  .rp .n { font-family: var(--tit); font-weight: 700; font-size: var(--c-riq); line-height: 1.15 }
  .scegli { text-align: left }
  .rp .m { font-size: var(--c-sec); color: var(--grigio) }
  .azr { display: flex; gap: 8px; align-items: center; grid-row: span 2 }
  .bollo {
    display: inline-block; border: var(--doppio); border-color: var(--rosso); color: var(--rosso); font-weight: 700; letter-spacing: .12em;
    text-transform: uppercase; padding: 1px 9px; transform: rotate(-5deg); font-size: var(--c-min); white-space: nowrap;
  }
  .bollo.ink { border-color: var(--ink); color: var(--ink) }

  .tagliando { border: 2px dashed var(--ink); padding: 12px 14px; display: grid; gap: 8px; align-content: start; justify-items: start }
  .tagliando .tit { font-size: var(--c-sez) }
  .occ { font-size: var(--c-min); letter-spacing: .1em; text-transform: uppercase; font-weight: 700; color: var(--rosso) }
</style>
