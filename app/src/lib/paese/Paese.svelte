<script lang="ts">
  // Paese, forma A · Atlante (23-paese.prototype.html): i livelli, la mappa e la lista; il dettaglio a lato tiene il sondaggio, o la scheda della regione scelta.
  import { tick } from "svelte";
  import { t, numero, percentuale } from "$lib/testi/t";
  import type { Chiave } from "$lib/testi/t";
  import { periodoDi } from "$lib/aula/aula";
  import { coalizioneDi, coloreDi, nomeDi } from "$lib/partiti";
  import type { Vista } from "$lib/tipi/Vista";
  import Mappa from "$lib/mappa/Mappa.svelte";
  import FormaSegno from "$lib/mappa/FormaSegno.svelte";
  import type { Segno } from "$lib/mappa/Mappa.svelte";
  import SchedaRegione from "./SchedaRegione.svelte";
  import Sondaggio from "./Sondaggio.svelte";
  import { LIVELLI, affluenzaPerRegione, classi, consensoPerRegione, lista, livelloManca, segniPerRegione, valoriDi, votoPerRegione, type Livello, type Pericolo } from "./paese";

  let { vista }: { vista: Vista } = $props();
  const pre = $props.id();
  const TIPI: Pericolo[] = ["totale", "sisma", "idro", "incendi"];

  let livello = $state<Livello>("consenso");
  let chi = $state("coalizione");
  let tipo = $state<Pericolo>("totale");
  let ordine = $state<"nome" | "valore">("nome");
  let scelta = $state<string | null>(null);
  // un partito sciolto o sparito dopo una Vista nuova non resta nel menu
  $effect(() => { if (chi !== "coalizione" && !vista.partiti.some((p) => p.id === chi && !p.sciolto)) chi = "coalizione"; });
  // senza elezioni il voto non c'è: dopo una Vista nuova la linguetta barrata non resta scelta
  $effect(() => { if (livello === "voto" && livelloManca("voto", vista)) livello = "consenso"; });
  async function indietro() {
    const id = scelta;
    scelta = null;
    await tick();
    document.getElementById(`${pre}-r-${id}`)?.focus();
  }

  const voto = $derived(livello === "voto" ? votoPerRegione(vista) : null);
  const valori = $derived<Record<string, number | null>>(
    livello === "voto" ? {}
    : livello === "affluenza" ? affluenzaPerRegione(vista)
    : livello === "consenso" ? consensoPerRegione(vista, chi === "coalizione" ? coalizioneDi(vista) : [chi])
    : valoriDi(vista, livello, tipo),
  );
  const segni = $derived(segniPerRegione(vista));
  const righe = $derived(lista(valori, livello === "voto" ? "nome" : ordine));
  const presenti = $derived(righe.map((r) => r.valore).filter((x): x is number => x !== null));
  const scala = $derived(classi(presenti, livello));
  const classe = $derived(Object.fromEntries(righe.map((r) => [r.id, r.valore === null ? null : scala.classe(r.valore)])));
  const grave = $derived(vista.eventi.some((e) => e.tono === "grave"));
  // nel tono grave i partiti sono tutti grigi: il retino, e il nome in lista
  const colore = (id: string) => (grave ? null : coloreDi(id));
  const tinta = $derived(voto ? Object.fromEntries(Object.entries(voto).map(([id, r]) => [id, colore(r.primo)])) : undefined);
  const primoNome = (id: string) => (voto?.[id] ? nomeDi(vista, voto[id].primo) : null);
  const primi = $derived(voto ? [...new Set(Object.values(voto).map((r) => r.primo))] : []);
  const RETINO = "var(--retino), var(--carta)";
  const campo = (id: string) => (voto ? (voto[id] ? (tinta?.[id] ?? RETINO) : "var(--carta)") : classe[id] === null ? "var(--carta)" : `var(--classe-${classe[id]})`);
  const etichetta = (id: string) => { const n = primoNome(id); return n ? t("paese.voto_primo", { nome: n }) : t("mappa.senza_dato"); };
  // ogni tipo una volta col numero: otto opere nella stessa regione non entrano nella colonna
  const conta = (id: string) => [...(segni[id] ?? []).reduce((m, s) => m.set(s, (m.get(s) ?? 0) + 1), new Map<Segno, number>())];
  const tipiSegno = $derived([...new Set(Object.values(segni).flat())] as Segno[]);
  const nomeLivello = (l: Livello) => t(`paese.livello_${l}` as Chiave);
  const pc = (x: number) => percentuale(x, 1);
  const formato = (x: number): string =>
    livello === "disoccupazione" ? t("paese.fmt_disoccupazione", { x: numero(x, 1) })
    : livello === "reddito" ? t("paese.fmt_reddito", { x: numero(x / 1000, 1) })
    : livello === "pericolosita" ? numero(x, 2)
    : pc(x);
  const nota = $derived(
    ({ consenso: "paese.nota_consenso", affluenza: "paese.nota_affluenza", pericolosita: "paese.nota_pericolosita", voto: "paese.nota_voto", disoccupazione: "paese.nota_chiusura", reddito: "paese.nota_chiusura" } as Record<Livello, Chiave>)[livello],
  );
  const estremi = $derived(livello === "pericolosita" ? { min: 0, max: 1 } : presenti.length ? { min: Math.min(...presenti), max: Math.max(...presenti) } : null);
</script>

<div class="atlante">
  <div class="col">
    <div class="livelli" role="group" aria-label={t("paese.livelli")}>
      {#each LIVELLI as l (l)}
        {@const manca = livelloManca(l, vista)}
        {#if manca}
          <span class="barrata">
            <button type="button" class="chiusa" aria-disabled="true" title={t(manca)} aria-describedby="{pre}-{l}">{nomeLivello(l)}</button>
            <span id="{pre}-{l}" class="perche nascosto">{t(manca)}</span>
          </span>
        {:else}
          <button type="button" class:qui={livello === l} aria-pressed={livello === l} onclick={() => (livello = l)}>{nomeLivello(l)}</button>
        {/if}
      {/each}
      <span class="sub">
        {#if livello === "consenso"}
          <label>{t("paese.di")}
            <select bind:value={chi}>
              <option value="coalizione">{t("paese.coalizione")}</option>
              {#each vista.partiti.filter((p) => !p.sciolto) as p (p.id)}<option value={p.id}>{p.nome}</option>{/each}
            </select>
          </label>
        {:else if livello === "pericolosita"}
          <label>{t("paese.tipo")}
            <select bind:value={tipo}>{#each TIPI as k (k)}<option value={k}>{t(`paese.tipo_${k}` as Chiave)}</option>{/each}</select>
          </label>
        {/if}
        <span class="muto">{livello === "consenso" || livello === "pericolosita" ? "· " : ""}{t(nota)}</span>
      </span>
    </div>

    <div class="carta">
      <div class="mappa">
        <Mappa {valori} {segni} {scelta} {classe} {tinta} etichetta={voto ? etichetta : undefined} ordine={righe.map((r) => r.id)} nome={t("mappa.regioni")} {formato} onscelta={(id) => (scelta = id)} />
      </div>
      <div class="lista" role="group" aria-label={t("mappa.regioni")}>
        <div class="h">
          <button type="button" class:on={ordine === "nome"} aria-pressed={ordine === "nome"} onclick={() => (ordine = "nome")}>{t("paese.col_regione")}</button>
          {#if voto}<span class="dx">{nomeLivello(livello)}</span>
          {:else}<button type="button" class="dx" class:on={ordine === "valore"} aria-pressed={ordine === "valore"} onclick={() => (ordine = "valore")}>{nomeLivello(livello)}</button>{/if}
          <span class="dx">{t("paese.col_segni")}</span>
        </div>
        {#each righe as r (r.id)}
          <button type="button" id="{pre}-r-{r.id}" class="r" class:sel={scelta === r.id} aria-pressed={scelta === r.id} onclick={() => (scelta = r.id)}>
            <span class="n"><i style:background={campo(r.id)}></i>{r.nome}</span>
            <span class="x">{voto ? (primoNome(r.id)?.split(" ")[0] ?? "—") : r.valore === null ? "—" : formato(r.valore)}</span>
            <span class="s">
              {#each conta(r.id) as [s, n] (s)}
                <svg width="12" height="12" viewBox="-7 -7 14 14" aria-label={t(`mappa.segno_${s}`)} role="img"><FormaSegno {s} /></svg>{#if n > 1}<small>×{n}</small>{/if}
              {/each}
            </span>
          </button>
        {/each}
      </div>
    </div>

    <div class="legenda">
      {#if voto}
        {#each primi as id (id)}<span class="sg"><i class="sw" style:background={colore(id) ?? RETINO}></i>{nomeDi(vista, id)}</span>{/each}
        <span>{t("paese.voto_legenda", { periodo: periodoDi(vista, vista.elezioni.at(-1)!.data) })}</span>
      {:else if estremi && scala.soglie.length}
        <span class="cl">{#each [0, 1, 2, 3, 4] as c (c)}<i style:background="var(--classe-{c})"></i>{/each}</span>
        <span>{formato(estremi.min)} · {t("paese.fino_a", { x: formato(estremi.max) })}</span>
      {:else if estremi}
        <span class="cl"><i style:background="var(--classe-0)"></i></span><span>{t("paese.classe_unica", { x: formato(estremi.min) })}</span>
      {:else}<span>{t("mappa.senza_dato")}</span>{/if}
      {#each tipiSegno as s (s)}
        <span class="sg"><svg width="12" height="12" viewBox="-7 -7 14 14" aria-hidden="true"><FormaSegno {s} /></svg>{t(`mappa.segno_${s}`)}</span>
      {/each}
    </div>
  </div>

  <aside class="lato">{#if scelta}<SchedaRegione {vista} regione={scelta} {indietro} />{:else}<Sondaggio {vista} />{/if}</aside>
</div>

<style>
  .atlante { display: grid; grid-template-columns: minmax(0, 1fr) var(--lato); gap: 0 var(--canale); min-height: 0 }
  .atlante > * { min-width: 0 }
  .col { display: grid; gap: 8px; align-content: start }
  .lato { border-left: var(--forte); padding-left: var(--canale); display: grid; gap: 8px; align-content: start }
  .muto { color: var(--grigio) }

  .livelli { display: flex; flex-wrap: wrap; align-items: end; border-bottom: var(--forte) }
  .livelli button { padding: 4px 11px; font-weight: 700; color: var(--grigio); border: 1.5px solid transparent; border-bottom: 0; font-size: var(--c-sec) }
  .livelli button:hover { color: var(--ink) }
  .livelli button.qui { color: var(--ink); border-color: var(--ink); background: var(--carta); margin-bottom: -2px; padding-bottom: 6px }
  .livelli button.chiusa { text-decoration: line-through; font-weight: 400; cursor: help }
  .livelli button.chiusa:hover { color: var(--grigio) }
  .barrata { position: relative; display: flex }
  .chiusa:focus-visible + .perche, .barrata:hover .perche {
    width: auto; height: auto; overflow: visible; clip-path: none; left: 0; top: calc(100% + 2px); z-index: 3;
    background: var(--ink); color: var(--carta); font-size: var(--c-min); font-weight: 400; padding: 3px 8px;
  }
  .sub { margin-left: auto; display: flex; gap: 6px; align-items: center; padding-bottom: 4px; font-size: var(--c-sec) }
  select { font: inherit; font-size: var(--c-sec); border: var(--bordo); background: var(--carta); padding: 1px 4px }

  .carta { display: grid; grid-template-columns: minmax(0, 1fr) minmax(250px, 290px); gap: 0 18px; align-items: start }
  .mappa { height: clamp(380px, 68vh, 640px) }
  .lista { display: grid; align-content: start; font-size: var(--c-sec) }
  .h, .r { display: grid; grid-template-columns: minmax(0, 1fr) 64px 70px; gap: 6px; align-items: center; padding: 2px 4px }
  .h { font-size: var(--c-min); color: var(--grigio); font-weight: 700; border-bottom: var(--bordo) }
  .h button.on { color: var(--ink); text-decoration: underline }
  .dx { text-align: right }
  .r { border-bottom: var(--filo); line-height: 1.25 }
  .r:hover { background: var(--velo) }
  .r.sel { background: var(--velo); box-shadow: inset 0 0 0 1.5px var(--ink) }
  .r .n { display: flex; gap: 6px; align-items: center; min-width: 0; overflow-wrap: normal }
  .r .n i { width: 10px; height: 10px; border: 1px solid var(--ink); flex: none }
  .x { text-align: right; font-weight: 700; white-space: nowrap }
  .s { display: flex; gap: 2px; justify-content: flex-end; align-items: center; white-space: nowrap }
  .s small { font-size: var(--c-min); margin-right: 2px }

  .legenda { display: flex; gap: 10px; align-items: center; flex-wrap: wrap; font-size: var(--c-min); color: var(--grigio) }
  .cl { display: flex }
  .sw { display: inline-block; flex: none; width: 10px; height: 10px; border: 1px solid var(--ink) }
  .cl i { width: 34px; height: 10px; border: 1px solid var(--ink); border-right: 0 }
  .cl i:last-child { border-right: 1px solid var(--ink) }
  .sg { display: inline-flex; align-items: center; gap: 3px }
</style>
