<script lang="ts">
  // Cruscotto all'opposizione, forma A · Ordine del giorno (28-opposizione.prototype.html): l'evento del governo in sola lettura, le leve,
  // le misure con la tua linea sulla riga, il programma e la maggioranza avversaria. A lato il dettaglio della riga scelta.
  import { untrack } from "svelte";
  import { t, numero } from "$lib/testi/t";
  import { partita } from "$lib/partita.svelte";
  import { comeErrore, testoRifiuto } from "$lib/guscio/chiamate";
  import { chiLaPorta, quandoMisura, votaOra } from "$lib/aula/aula";
  import Linea from "$lib/aula/Linea.svelte";
  import Evento from "$lib/cruscotto/Evento.svelte";
  import Rifiuto from "$lib/componenti/Rifiuto.svelte";
  import { nomeDi } from "$lib/partiti";
  import type { Vista } from "$lib/tipi/Vista";
  import type { VoceMisura } from "$lib/tipi/VoceMisura";
  import Leve from "./Leve.svelte";
  import Programma from "./Programma.svelte";
  import RigaOdg from "./RigaOdg.svelte";
  import { misureDelGoverno } from "./opposizione";
  import type { Scelta } from "./scelta";

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
  const sotto = (m: VoceMisura, conQuando: boolean) =>
    [chiLaPorta(vista, m, (id) => nomeDi(vista, id)), forbice(m), conQuando ? quandoMisura(vista, m) : ""].filter(Boolean).join(" · ");
  const occupato = $derived(partita.occupato ? t("campagna.comando_in_corso") : undefined);

  // raw: la Vista dentro si confronta per identità; il rifiuto vale per la Vista e la misura in cui è nato
  let rifiuto = $state.raw<{ testo: string; vista: Vista | null; misura: number } | null>(null);
  async function linea(m: VoceMisura, si: boolean | null) {
    if (partita.occupato) return;
    rifiuto = null;
    try {
      await partita.comanda({ tipo: "linea", misura: m.id, si });
    } catch (e) {
      rifiuto = { testo: testoRifiuto(comeErrore(e)), vista: partita.vista, misura: m.id };
    }
  }
</script>

<div class="cr">
  <div class="odg">
    {#each vista.eventi as evento, i (i)}<Evento {vista} {evento} indice={i} puoRispondere={false} />{/each}
    <Leve {vista} {scelta} onscegli={(s) => { scelta = s; }} />
    {#each gruppi as g (g.titolo)}
      {@const n = g.voci.filter((m) => m.linea_giocatore === null).length}
      <section>
        <h2 class="grp"><span>{t(g.titolo)}</span>{#if n}<span>{t("opposizione.n_da_posizione", { n })}</span>{/if}</h2>
        <ul>
          {#each g.voci as m (m.id)}
            <RigaOdg titolo={m.nome} sotto={sotto(m, g.quando)} segno={m.linea_giocatore !== null ? "fatto" : m.forbice.in_bilico ? "urg" : ""}
              scelta={scelta?.tipo === "misura" && scelta.id === String(m.id)} onscegli={() => { scelta = { tipo: "misura", id: String(m.id) }; }}>
              <Linea {vista} misura={m} {occupato} onlinea={(si) => linea(m, si)}>
                {#if rifiuto?.vista === vista && rifiuto.misura === m.id}<Rifiuto testo={rifiuto.testo} />{/if}
              </Linea>
            </RigaOdg>
          {/each}
        </ul>
      </section>
    {/each}
    <Programma {vista} />
  </div>
  <aside class="lato" aria-label={t("aula.dettaglio")}>
    {#if !scelta}<p class="muto">{t("cruscotto.scegli_riga")}</p>{/if}
  </aside>
</div>

<style>
  .cr { display: grid; grid-template-columns: minmax(0, 1fr) var(--lato); grid-template-rows: minmax(0, 1fr); gap: 10px var(--canale); height: 100%; min-height: 0 }
  .odg { display: grid; gap: 10px; align-content: start; min-width: 0; min-height: 0; overflow: hidden auto }
  .lato { border-left: var(--forte); padding-left: var(--canale); display: grid; gap: 8px; align-content: start; min-height: 0; overflow: hidden auto; padding-bottom: 8px }
  .muto { color: var(--grigio) }
  .grp { display: flex; justify-content: space-between; gap: 8px; font-size: var(--c-min); letter-spacing: .1em; text-transform: uppercase; font-weight: 700; color: var(--grigio); padding-bottom: 2px; border-bottom: 1.5px solid var(--ink) }
  ul { list-style: none }
</style>
