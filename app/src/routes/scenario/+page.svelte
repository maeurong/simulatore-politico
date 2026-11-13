<script lang="ts">
  // Scenario ed editor, forma A · Banco (.scratch/v1-piano-11-ui/prototipi/14-editor.prototype.html:386-394): fuori dalla
  // partita, senza cornice. L'editor scorre a sinistra, il riepilogo dal vivo sta fermo a lato.
  import { onMount, tick } from "svelte";
  import { goto } from "$app/navigation";
  import { comeErrore, guscio } from "$lib/guscio/chiamate";
  import { t } from "$lib/testi/t";
  import type { CatalogoScenario } from "$lib/tipi/CatalogoScenario";
  import type { Cursori } from "$lib/tipi/Cursori";
  import type { RiepilogoScenario } from "$lib/tipi/RiepilogoScenario";
  import type { SuMisura } from "$lib/tipi/SuMisura";
  import Conferma from "$lib/componenti/Conferma.svelte";
  import Pulsante from "$lib/componenti/Pulsante.svelte";
  import Rifiuto from "$lib/componenti/Rifiuto.svelte";
  import Cursore from "$lib/editor/Cursore.svelte";
  import Disposizioni from "$lib/editor/Disposizioni.svelte";
  import LeggeElettorale from "$lib/editor/LeggeElettorale.svelte";
  import Riepilogo from "$lib/editor/Riepilogo.svelte";
  import { base, codaRiepiloghi, editor, ritocca } from "$lib/editor/editor.svelte";

  let catalogo = $state.raw<CatalogoScenario | null>(null);
  let salvati = $state.raw<string[]>([]);
  /** il riepilogo dell'Italia di oggi: lo scarto dei conti si misura su questo */
  let oggi = $state.raw<RiepilogoScenario | null>(null);
  let guasto = $state<string | null>(null);
  let rifiutoCarica = $state<string | null>(null);
  let rifiutoSalva = $state<string | null>(null);
  let nome = $state(editor.sorgente ?? editor.scenario?.nome ?? "");
  /** un caricamento o un salvataggio in volo: i comandi dell'editor e «Fonda il partito ›» sono spenti */
  let caricaOSalva = $state(false);
  let campo = $state<HTMLInputElement>();

  const ordinarie = $derived(catalogo?.disposizioni.filter((d) => d.norma === "ordinaria") ?? []);
  const cambiate = $derived(ordinarie.filter((d) => editor.scenario && d.id in editor.scenario.disposizioni).length);
  const esiste = $derived(salvati.includes(nome.trim()));
  const daSalvare = $derived(editor.mosso && nome.trim() !== "");

  // una richiesta in volo per volta, l'ultima vince; il riepilogo resta l'ultimo valido finché non arriva il nuovo
  const chiedi = codaRiepiloghi(guscio.riepilogoScenario);
  // il guasto del riepilogo sta in editor: il successo successivo lo toglie
  const inTesta = $derived(guasto ?? (editor.guasto && `${editor.guasto} ${t("editor.guasto_riepilogo")}`));

  onMount(async () => {
    try {
      [catalogo, salvati, oggi] = await Promise.all([guscio.catalogoScenario(), guscio.scenari(), guscio.riepilogoScenario(null)]);
    } catch (e) {
      guasto = `${comeErrore(e).testo} ${t("editor.guasto_catalogo")}`;
      return;
    }
    if (editor.scenario) chiedi(editor.scenario);
    else {
      editor.riepilogo = oggi;
      editor.rifiuto = editor.guasto = null;
    }
  });

  // mentre un caricamento o un salvataggio sono in volo i comandi dell'editor sono spenti: la risposta non deve coprire una modifica
  function parti(scenario: SuMisura | null, sorgente: string | null, mosso: boolean) {
    if (caricaOSalva) return;
    guasto = rifiutoCarica = rifiutoSalva = null;
    editor.scenario = scenario;
    editor.sorgente = sorgente;
    editor.mosso = mosso;
    nome = sorgente ?? "";
    chiedi(scenario);
  }
  const cambia = (s: SuMisura) => {
    if (caricaOSalva) return;
    guasto = rifiutoSalva = null;
    editor.scenario = s;
    editor.mosso = true;
    chiedi(s);
  };
  const muovi = (k: string, v: number) => cambia(ritocca(editor.scenario, catalogo!, (s) => (s.cursori[k as keyof Omit<Cursori, "legge_elettorale">] = v)));

  async function carica(n: string) {
    if (caricaOSalva) return;
    const giro = editor.giro;
    caricaOSalva = true;
    try {
      const s = await guscio.caricaScenario(n);
      caricaOSalva = false;
      // si è usciti all'edicola mentre era in volo: la risposta appartiene all'editor lasciato
      if (giro === editor.giro) parti(s, n, false);
    } catch (e) {
      if (giro !== editor.giro) return;
      const x = comeErrore(e);
      if (x.specie === "rifiuto") rifiutoCarica = x.testo;
      else guasto = `${x.testo} ${t("editor.guasto_carica")}`;
    } finally {
      caricaOSalva = false;
    }
  }

  async function salva() {
    if (caricaOSalva || !editor.scenario) return;
    const n = nome.trim();
    const s = { ...editor.scenario, nome: n };
    const giro = editor.giro;
    caricaOSalva = true;
    rifiutoSalva = guasto = null;
    try {
      await guscio.salvaScenario(s);
      if (giro !== editor.giro) return;
      editor.scenario = s;
      editor.sorgente = n;
      editor.mosso = false;
      if (!salvati.includes(n)) salvati = [...salvati, n];
    } catch (e) {
      const x = comeErrore(e);
      if (x.specie === "rifiuto") rifiutoSalva = x.testo;
      else guasto = `${x.testo} ${t("editor.guasto_salva")}`;
    } finally {
      caricaOSalva = false;
    }
    // i pulsanti della conferma non ci sono più: il fuoco torna al nome
    await tick();
    campo?.focus();
  }

  function esci() {
    editor.azzera();
    goto("/");
  }
</script>

<div class="editor">
  <header class="testa">
    {#if editor.mosso}
      <Conferma forma="link" neutro etichetta={t("editor.torna_edicola")} conferma={t("editor.esci_comunque")} annulla={t("editor.resta")}
        spiegazione={t("editor.modifiche_perse")} onconferma={esci} />
    {:else}
      <Pulsante forma="link" onclick={esci}>{t("editor.torna_edicola")}</Pulsante>
    {/if}
    <span class="nomino">{t("cornice.nomino")}</span>
    <h1 class="dove">{t("editor.dove", { sotto: t("editor.dove_scenario") })}</h1>
    <p class="fuori stato">
      {#if !editor.mosso && !editor.scenario}{t("editor.stato_scenario")} <b>{t("editor.scenario_base")}</b>
      {:else if !editor.mosso}{t("editor.stato_su_misura")} <b>{editor.sorgente}</b>
      {:else}{t("editor.stato_modificato")} <b>{editor.sorgente ? t("editor.nome_modificato", { nome: editor.sorgente }) : t("editor.nuovo_senza_nome")}</b>
        {t("editor.non_salvato")}{/if}
    </p>
  </header>
  {#if inTesta}<p class="guasto" role="alert">{inTesta}</p>{/if}

  <main class="corpo">
    {#if catalogo}
      <div class="sx">
        <section aria-labelledby="titolo-scenario">
          <div class="titsez"><h2 id="titolo-scenario" class="tit">{t("editor.titolo_scenario")}</h2></div>
          <div class="chips">
            <button type="button" class="chip" aria-pressed={!editor.scenario && !editor.mosso} aria-disabled={caricaOSalva || undefined} onclick={() => parti(null, null, false)}>{t("editor.scenario_base")}</button>
            <span class="min muto" aria-hidden="true">·</span>
            {#each salvati as n (n)}
              <button type="button" class="chip" aria-pressed={editor.sorgente === n && !editor.mosso} aria-disabled={caricaOSalva || undefined} onclick={() => carica(n)}>{n}</button>
            {/each}
            <button type="button" class="chip nuovo" aria-disabled={caricaOSalva || undefined} onclick={() => parti(base(catalogo!), null, true)}>{t("editor.nuovo_su_misura")}</button>
          </div>
          {#if rifiutoCarica}<Rifiuto testo={rifiutoCarica} />{/if}
          <p class="min muto">{t("editor.nota_scenari")}</p>
        </section>

        <section aria-labelledby="titolo-paese">
          <div class="titsez"><h2 id="titolo-paese" class="tit">{t("editor.titolo_paese")}</h2><span class="dx min muto">{t("editor.valore_di_oggi")}</span></div>
          <div class="curs">
            {#each catalogo.cursori as cur (cur.nome)}
              <Cursore {cur} valore={editor.scenario?.cursori[cur.nome as keyof Cursori] as number ?? cur.valore} fermo={caricaOSalva} onvalore={(v) => muovi(cur.nome, v)} />
            {/each}
          </div>
        </section>

        <section aria-labelledby="titolo-regole">
          <div class="titsez"><h2 id="titolo-regole" class="tit">{t("editor.titolo_regole")}</h2></div>
          <LeggeElettorale {catalogo} scenario={editor.scenario} rifiuto={editor.rifiuto} fermo={caricaOSalva} onscenario={cambia} />
        </section>

        <section aria-labelledby="titolo-leggi">
          <div class="titsez"><h2 id="titolo-leggi" class="tit">{t("editor.titolo_leggi")}</h2></div>
          <p class="min muto">{t("editor.disposizioni_nota", { n: ordinarie.length })}
            {#if cambiate}<b class="male">{t("editor.cambiate", { n: cambiate })}</b>{/if}</p>
          <Disposizioni {catalogo} scenario={editor.scenario} voci={ordinarie} gruppi fermo={caricaOSalva} onscenario={cambia} />
        </section>
      </div>

      <aside class="lato" aria-label={t("editor.riepilogo_vivo")}>
        {#if editor.riepilogo && oggi}
          <Riepilogo riepilogo={editor.riepilogo} {oggi} partiti={catalogo.partiti} spento={!editor.valido} />
        {/if}
        <div class="salva">
          {#if !editor.scenario && !editor.mosso}
            <p class="sec">{t("editor.si_gioca_base")}</p>
          {:else}
            <div class="riga">
              <input bind:this={campo} bind:value={nome} placeholder={t("editor.nome_scenario")} aria-label={t("editor.nome_scenario")}
                aria-describedby={rifiutoSalva ? "rifiuto-salva" : undefined} oninput={() => (rifiutoSalva = null)} />
              {#if esiste && daSalvare}
                <Conferma neutro etichetta={t("editor.salva")} conferma={t("editor.conferma_sostituisci", { nome: nome.trim() })}
                  spiegazione={t("editor.esiste_gia")} onconferma={salva} />
              {:else}
                <Pulsante forma="bordo" spento={!editor.mosso ? t("editor.salvato") : !daSalvare ? t("editor.manca_il_nome") : undefined}
                  aria-busy={caricaOSalva || undefined} onclick={salva}>{t("editor.salva")}</Pulsante>
              {/if}
            </div>
            {#if rifiutoSalva}<span id="rifiuto-salva"><Rifiuto testo={rifiutoSalva} /></span>{/if}
            {#if editor.mosso}<p class="min muto">{t("editor.salvare_non_serve")}</p>{/if}
          {/if}
          <div class="riga">
            <Pulsante forma="chiude" spento={editor.valido ? undefined : t("editor.prima_una_legge")} aria-disabled={caricaOSalva ? "true" : undefined}
              onclick={() => caricaOSalva || goto("/fondazione")}>{t("editor.fonda_il_partito")}</Pulsante>
          </div>
        </div>
      </aside>
    {/if}
  </main>
</div>

<style>
  /* la pagina sta ferma; scorrono l'editor e, se non basta, il riepilogo */
  .editor { position: fixed; inset: 0; display: flex; flex-direction: column; overflow: hidden }
  /* testata, guasto, corpo e titoli in comune con edicola e fondazione: fondamenta.css */
  .stato { font-size: var(--c-sec); color: var(--grigio) }
  .stato b { color: var(--ink) }

  .sx { display: grid; gap: 16px; align-content: start; min-width: 0; min-height: 0; overflow: hidden auto; padding-right: 6px }
  .lato { display: grid; gap: 10px; align-content: start; min-width: 0; min-height: 0; overflow: hidden auto }

  .male { color: var(--rosso) }

  .chips { display: flex; gap: 6px; flex-wrap: wrap; align-items: center; margin-bottom: 2px }
  .chip { border: 1.5px solid var(--ink); padding: 3px 10px; font-size: var(--c-sec); font-weight: 700; background: var(--carta) }
  .chip:hover { background: var(--velo) }
  .chip[aria-pressed="true"] { background: var(--ink); color: var(--carta) }
  .chip[aria-disabled="true"] { border-style: dashed; color: var(--grigio); cursor: default; background: var(--carta) }
  .chip.nuovo { border-style: dashed }
  .curs { display: grid; grid-template-columns: repeat(2, minmax(0, 1fr)); gap: 12px var(--canale) }

  .salva { display: grid; gap: 6px; border-top: var(--doppio); padding-top: 8px }
  .riga { display: flex; gap: 6px; align-items: center; flex-wrap: wrap }
  .riga input { font: inherit; border: 1.5px solid var(--ink); background: var(--carta); padding: 3px 6px; min-width: 0; flex: 1 }
</style>
