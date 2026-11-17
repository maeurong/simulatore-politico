<script lang="ts">
  // Cruscotto all'opposizione, forma A · Ordine del giorno (28-opposizione.prototype.html): l'evento del governo in sola lettura, le leve,
  // le misure con la tua linea sulla riga, il programma e la maggioranza avversaria. A lato il dettaglio della riga scelta.
  import { tick, untrack } from "svelte";
  import { t, numero } from "$lib/testi/t";
  import { costoDellaLinea } from "$lib/aula/linea";
  import { partita } from "$lib/partita.svelte";
  import { comeErrore, guscio, testoRifiuto } from "$lib/guscio/chiamate";
  import { chiLaPorta, quandoMisura, votaOra } from "$lib/aula/aula";
  import Linea from "$lib/aula/Linea.svelte";
  import Evento from "$lib/cruscotto/Evento.svelte";
  import Rifiuto from "$lib/componenti/Rifiuto.svelte";
  import { nomeDi } from "$lib/partiti";
  import type { Comando } from "$lib/tipi/Comando";
  import type { Vista } from "$lib/tipi/Vista";
  import type { VoceMisura } from "$lib/tipi/VoceMisura";
  import Leve from "./Leve.svelte";
  import DettaglioReferendum from "./DettaglioReferendum.svelte";
  import DettaglioSfiducia from "./DettaglioSfiducia.svelte";
  import Programma from "./Programma.svelte";
  import RigaOdg from "./RigaOdg.svelte";
  import ScegliMinistro from "./ScegliMinistro.svelte";
  import { misureDelGoverno } from "./opposizione";
  import type { Scelta } from "./scelta";
  import { caricaAnteprime, dicasteriConMinistro, quantiPassano, type Anteprime } from "./sfiducia";

  let { vista }: { vista: Vista } = $props();

  // la riga scelta vive in memoria; al turno nuovo si azzera (una Vista nuova dello stesso turno, dopo un comando, non la tocca)
  let scelta = $state<Scelta | null>(null);
  const turno = $derived(vista.turno);
  $effect.pre(() => {
    void turno;
    untrack(() => { scelta = null; });
  });

  const misure = $derived(misureDelGoverno(vista));
  const gruppi = $derived([
    { titolo: "cruscotto.si_vota" as const, voci: misure.filter((m) => votaOra(vista, m)), quando: false },
    { titolo: "aula.scheda_in_aula" as const, voci: misure.filter((m) => !votaOra(vista, m)), quando: true },
  ].filter((g) => g.voci.length));

  const forbice = (m: VoceMisura) => t("cruscotto.forbice", {
    c0: numero(m.forbice.camera[0]), c1: numero(m.forbice.camera[1]), mc: numero(m.forbice.maggioranza_camera),
    s0: numero(m.forbice.senato[0]), s1: numero(m.forbice.senato[1]), ms: numero(m.forbice.maggioranza_senato),
  });
  // la riga dice anche il default e quanto costa la linea oggi scelta (28-opposizione.prototype.html:704)
  function sotto(m: VoceMisura, conQuando: boolean): string {
    const c = m.costo_linea;
    const mia = m.linee.find((l) => l.partito === vista.giocatore)?.si ?? false;
    const oggi = c && (m.linea_giocatore === true ? c.si : m.linea_giocatore === false ? c.no : c.default);
    return [
      chiLaPorta(vista, m, (id) => nomeDi(vista, id)), forbice(m), conQuando ? quandoMisura(vista, m) : "",
      c ? `${t("cruscotto.default")} ${t("aula.come_da_posizione", { linea: t(mia ? "aula.si" : "aula.no") }).toLowerCase()}` : "",
      oggi ? costoDellaLinea(vista, oggi) : "",
    ].filter(Boolean).join(" · ");
  }
  const occupato = $derived(partita.occupato ? t("campagna.comando_in_corso") : undefined);

  // raw: la Vista dentro si confronta per identità; il rifiuto vale per la Vista e la riga in cui è nato (`chiave`)
  let rifiuto = $state.raw<{ testo: string; vista: Vista | null; chiave: string } | null>(null);
  const rifiutoDi = (chiave: string) => (rifiuto?.vista === vista && rifiuto.chiave === chiave ? rifiuto.testo : null);
  let lato: HTMLElement;
  // il bottone premuto sparisce con la risposta (la Vista dice «presentata»): se il fuoco stava nel dettaglio passa al dettaglio.
  // Il secondo comando mentre il primo è in volo non parte (partita.comanda dà null) e non lascia rifiuto.
  async function manda(c: Comando, chiave: string): Promise<boolean> {
    if (partita.occupato) return false;
    rifiuto = null;
    const dentro = !!lato?.contains(document.activeElement);
    let fatto = false;
    try {
      fatto = (await partita.comanda(c)) !== null;
    } catch (e) {
      rifiuto = { testo: testoRifiuto(comeErrore(e)), vista: partita.vista, chiave };
    }
    await tick();
    if (dentro && !lato.contains(document.activeElement)) lato.focus({ preventScroll: true });
    return fatto;
  }
  const linea = (m: VoceMisura, si: boolean | null) => manda({ tipo: "linea", misura: m.id, si }, `linea:${m.id}`);

  // le anteprime della sfiducia, legate alla Vista che le ha chieste: governo alla scelta della riga, i ministri (uno alla volta) alla prima apertura del turno
  let anteprime = $state.raw<Anteprime>(new Map());
  let giro: { vista: Vista; chiesti: Set<string | null> } | null = null;
  const bisogno = $derived(scelta?.tipo === "sfiducia" ? "governo" : scelta?.tipo === "ministro" ? "ministri" : null);
  $effect(() => {
    const v = vista;
    const b = bisogno;
    untrack(() => {
      if (giro?.vista !== v) { giro = { vista: v, chiesti: new Set() }; anteprime = new Map(); }
      if (!b) return;
      const da = (b === "governo" ? [null] : dicasteriConMinistro(v)).filter((d) => !giro!.chiesti.has(d));
      da.forEach((d) => giro!.chiesti.add(d));
      void caricaAnteprime(v, da, guscio.anteprimaSfiducia, () => partita.vista, (d, a) => { anteprime = new Map(anteprime).set(d, a); });
    });
  });
  const conMinistro = $derived(dicasteriConMinistro(vista));
  const n = $derived(quantiPassano(anteprime, conMinistro));
  const passano = $derived(n === null ? null : { n, m: conMinistro.length });
  const dicastero = $derived(scelta?.tipo === "ministro" ? scelta.id : null);
  const misuraScelta = $derived(scelta?.tipo === "misura" ? misure.find((m) => String(m.id) === scelta!.id) : undefined);
</script>

<div class="cr">
  <div class="odg">
    {#each vista.eventi as evento, i (i)}<Evento {vista} {evento} indice={i} puoRispondere={false} />{/each}
    <Leve {vista} {scelta} {passano} onscegli={(s) => { scelta = s; }} />
    {#each gruppi as g (g.titolo)}
      {@const n = g.voci.filter((m) => m.linea_giocatore === null).length}
      <section>
        <h2 class="grp"><span>{t(g.titolo)}</span>{#if n}<span>{t("opposizione.n_da_posizione", { n })}</span>{/if}</h2>
        <ul>
          {#each g.voci as m (m.id)}
            <RigaOdg titolo={m.nome} sotto={sotto(m, g.quando)} segno={m.linea_giocatore !== null ? "fatto" : m.forbice.in_bilico ? "urg" : ""}
              scelta={scelta?.tipo === "misura" && scelta.id === String(m.id)} onscegli={() => { scelta = { tipo: "misura", id: String(m.id) }; }}>
              {#snippet azioni()}<Linea {vista} misura={m} {occupato} compatta onlinea={(si) => linea(m, si)} />{/snippet}
              {@const r = rifiutoDi(`linea:${m.id}`)}{#if r}<Rifiuto testo={r} />{/if}
            </RigaOdg>
          {/each}
        </ul>
      </section>
    {/each}
    <Programma {vista} />
  </div>
  <aside class="lato" bind:this={lato} tabindex="-1" aria-label={t("aula.dettaglio")}>
    {#if scelta?.tipo === "sfiducia"}
      <h3 class="tit">{t("opposizione.leva_sfiducia_governo")}</h3>
      <DettaglioSfiducia {vista} dicastero={null} anteprima={anteprime.get(null)} rifiuto={rifiutoDi("sfiducia:")} manda={(c) => manda(c, "sfiducia:")} />
    {:else if scelta?.tipo === "ministro"}
      <h3 class="tit">{t("opposizione.leva_sfiducia_ministro")}</h3>
      <ScegliMinistro {vista} scelto={dicastero} {anteprime} onscegli={(id) => { scelta = { tipo: "ministro", id }; }} />
      {#if dicastero}
        <h4 class="occ">{vista.dicasteri.find((d) => d.id === dicastero)?.nome ?? dicastero}</h4>
        <DettaglioSfiducia {vista} {dicastero} anteprima={anteprime.get(dicastero)} rifiuto={rifiutoDi(`sfiducia:${dicastero}`)} manda={(c) => manda(c, `sfiducia:${dicastero}`)} />
      {/if}
    {:else if scelta?.tipo === "referendum"}
      {@const abrogativo = scelta.id === "abrogativo"}
      <h3 class="tit">{abrogativo ? t("opposizione.leva_abrogativo") : t("opposizione.leva_confermativo", { nome: vista.referendum.find((r) => r.tipo === "confermativo")?.nome ?? "" })}</h3>
      <DettaglioReferendum {vista} quale={abrogativo ? "abrogativo" : "confermativo"} manda={(c) => manda(c, c.tipo === "promuovi_referendum" ? `ref:${c.disposizione}` : "confermativo")}
        rifiuto={rifiuto?.vista === vista && (rifiuto.chiave.startsWith("ref:") || rifiuto.chiave === "confermativo") ? { testo: rifiuto.testo, chiave: rifiuto.chiave } : null} />
    {:else if misuraScelta}
      <h3 class="tit">{misuraScelta.nome}</h3>
      <p class="sec muto">{[chiLaPorta(vista, misuraScelta, (id) => nomeDi(vista, id)), forbice(misuraScelta), quandoMisura(vista, misuraScelta)].join(" · ")}</p>
      <Linea {vista} misura={misuraScelta} {occupato} onlinea={(si) => linea(misuraScelta, si)}>
        {@const r = rifiutoDi(`linea:${misuraScelta.id}`)}{#if r}<Rifiuto testo={r} />{/if}
      </Linea>
    {:else}<p class="muto">{t("cruscotto.scegli_riga")}</p>{/if}
  </aside>
</div>

<style>
  .cr { display: grid; grid-template-columns: minmax(0, 1fr) var(--lato); grid-template-rows: minmax(0, 1fr); gap: 10px var(--canale); height: 100%; min-height: 0 }
  .odg { display: grid; gap: 10px; align-content: start; min-width: 0; min-height: 0; overflow: hidden auto }
  .lato { border-left: var(--forte); padding-left: var(--canale); display: grid; gap: 8px; align-content: start; min-height: 0; overflow: hidden auto; padding-bottom: 8px }
  .lato:focus-visible { outline-offset: 2px }
  .muto { color: var(--grigio) }
  .sec { font-size: var(--c-sec) }
  .tit { font-family: var(--tit); font-variation-settings: var(--wonk); font-weight: 700; line-height: 1.1; text-wrap: balance; font-size: var(--c-riq) }
  .occ { font-size: var(--c-min); letter-spacing: .1em; text-transform: uppercase; font-weight: 700 }
  .grp { display: flex; justify-content: space-between; gap: 8px; font-size: var(--c-min); letter-spacing: .1em; text-transform: uppercase; font-weight: 700; color: var(--grigio); padding-bottom: 2px; border-bottom: 1.5px solid var(--ink) }
  ul { list-style: none }
</style>
