<script lang="ts">
  // Il dettaglio a lato della voce scelta (17-cruscotto.prototype.html, `dettaglio`): ciò che serve a decidere.
  // Proposta di fondi: il ministro, il motivo, il cartellino di `anteprima_accogli` e le tre azioni. Riforma: il rimando «Anteprima della misura ›».
  // Voto: i due emicicli. Decreto d'evento: i gradini della norma scelta, il cartellino di `anteprima_decreto` e «Emana il decreto», diretto.
  // Le anteprime vanno una in volo per volta e vince l'ultima (come la bozza dell'Aula); il rifiuto del core sta accanto alla scelta.
  import { tick, untrack } from "svelte";
  import { t, numero } from "$lib/testi/t";
  import { nomeDi } from "$lib/partiti";
  import { partita } from "$lib/partita.svelte";
  import { chiaveDi } from "$lib/caricature/caricatura";
  import { comeErrore, guscio, testoRifiuto, type ErroreGuscio } from "$lib/guscio/chiamate";
  import { ultimoVince } from "$lib/editor/editor.svelte";
  import { formaDi } from "$lib/aula/aula";
  import { cambia, leggiAnteprima, numeraRichieste, type Bozza } from "$lib/aula/bozza";
  import Scala from "$lib/aula/Scala.svelte";
  import Cartellino from "$lib/anteprima/Cartellino.svelte";
  import type { Anteprima } from "$lib/tipi/Anteprima";
  import type { Comando } from "$lib/tipi/Comando";
  import type { Vista } from "$lib/tipi/Vista";
  import Emiciclo from "$lib/componenti/Emiciclo.svelte";
  import Faccia from "$lib/componenti/Faccia.svelte";
  import Pulsante from "$lib/componenti/Pulsante.svelte";
  import Rifiuto from "$lib/componenti/Rifiuto.svelte";
  import type { VoceOdg } from "./cruscotto";

  let { vista, voce, norma, riporta }: { vista: Vista; voce: VoceOdg | null; norma: string | null; riporta: (da: Element | null) => void } = $props();

  const occupato = $derived(partita.occupato ? t("campagna.comando_in_corso") : undefined);
  const nome = (id: string) => nomeDi(vista, id);
  const ministero = $derived(vista.ministeri.find((m) => m.dicastero === voce?.dicastero));
  const proposta = $derived(voce?.azioni === "proposta" ? (ministero?.proposta ?? null) : null);
  const fondi = $derived(!!proposta && !proposta.disposizione);
  const misura = $derived(vista.calendario.find((m) => m.id === voce?.misura));
  const disposizione = $derived(vista.disposizioni.find((d) => d.id === norma));
  const camera = $derived(vista.seggi.reduce((s, x) => s + x.camera, 0));
  const senato = $derived(vista.seggi.reduce((s, x) => s + x.senato, 0));
  const emendamento = $derived(misura?.emendamenti.find((e) => e.partito === voce?.partito));

  // il decreto d'evento: una disposizione, il gradino scelto
  let bozza = $state<Bozza>([]);
  // la proposta di riforma: il cartellino si apre a un gesto, come «Anteprima della misura» dell'Aula
  let aperto = $state(false);
  let piede = $state<HTMLElement>();
  async function scambia(e: MouseEvent, apri: boolean) {
    const aveva = e.currentTarget === document.activeElement;
    aperto = apri;
    if (aveva && !apri) {
      await tick();
      piede?.querySelector("button")?.focus({ preventScroll: true });
    }
  }

  // una anteprima in volo per volta: l'ultima chiesta vince, una risposta vecchia non copre la nuova
  let ante = $state.raw<Anteprima | null>(null);
  let errore = $state<string | null>(null);
  let inCalcolo = $state(false);
  const richieste = numeraRichieste();
  const chiedi = ultimoVince(
    ({ f }: { f: () => Promise<Anteprima | ErroreGuscio>; n: number }) => f(),
    (r, { n }) => {
      if (!richieste.vale(n)) return;
      inCalcolo = false;
      const letta = leggiAnteprima(r);
      ({ ante, errore } = letta);
      if (letta.guasto) partita.guasto = letta.guasto;
    },
  );
  const richiesta = $derived.by((): (() => Promise<Anteprima | ErroreGuscio>) | null => {
    if (norma) return bozza.length ? () => guscio.anteprimaDecreto({ tipo: "disposizioni", valori: bozza }) : null;
    if (!proposta || !voce?.dicastero) return null;
    const dicastero = voce.dicastero;
    if (fondi) return () => guscio.anteprimaAccogli(dicastero);
    const { disposizione: d, gradino: g } = proposta;
    return aperto && d && g ? () => guscio.anteprima([{ disposizione: d, gradino: g }]) : null;
  });
  $effect(() => {
    const f = richiesta;
    void vista; // una Vista nuova cambia ciò che il core risponde, anche a richiesta ferma
    const n = richieste.nuova();
    untrack(() => {
      if (!f) {
        ante = null;
        errore = null;
        inCalcolo = false;
        return;
      }
      inCalcolo = true;
      chiedi({ f, n });
    });
  });

  // raw: la Vista dentro si confronta per identità; il rifiuto vale per la Vista in cui è nato
  let rifiutato = $state.raw<{ testo: string; tipo: Comando["tipo"]; vista: Vista | null } | null>(null);
  const rifiuto = $derived(rifiutato?.vista === vista ? rifiutato : null);
  async function manda(c: Comando): Promise<boolean> {
    if (partita.occupato) return false;
    rifiutato = null;
    const da = document.activeElement;
    try {
      const ok = (await partita.comanda(c)) !== null;
      if (ok && c.tipo === "emana_decreto") bozza = [];
      riporta(da);
      return ok;
    } catch (e) {
      rifiutato = { testo: testoRifiuto(comeErrore(e)), tipo: c.tipo, vista: partita.vista };
      riporta(da);
      return false;
    }
  }
  const dettaglio = (azione: string) => t("cruscotto.dal_dettaglio", { azione });
  const ministro = $derived(ministero?.ministro ?? null);
  const emana = $derived<Comando>({ tipo: "emana_decreto", decreto: { tipo: "disposizioni", valori: bozza } });
  const nonEmanabile = $derived(occupato ?? (inCalcolo ? t("aula.ante_in_calcolo") : !bozza.length ? t("cruscotto.decreto_scegli_gradino") : !ante ? t("cruscotto.decreto_rifiutato") : undefined));
</script>

{#snippet cartellino(valori: Bozza, nota?: string, indietro?: string)}
  {#if ante}
    <Cartellino {vista} misura={ante} {valori} {inCalcolo} {nota} {indietro} ontorna={(e) => scambia(e, false)} manda={manda} rifiuto={rifiuto?.tipo === "compra" ? rifiuto : null} />
  {:else if errore}
    <Rifiuto testo={errore} />
  {:else if inCalcolo}
    <p class="sec muto">{t("aula.ante_in_calcolo")}</p>
  {/if}
{/snippet}

{#if norma && disposizione}
  <span class="occ">{t("cruscotto.decreto_occ")}</span>
  <h3 class="tit">{disposizione.nome}</h3>
  <p class="sec">{t("cruscotto.decreto_scegli")}</p>
  <Scala d={disposizione} {bozza} onscegli={(g) => (bozza = cambia(vista, bozza, disposizione.id, g))} />
  <div class="az">
    <Pulsante forma="chiude" spento={nonEmanabile} onclick={() => manda(emana)}>{t("cruscotto.decreto_emana")}</Pulsante>
  </div>
  {#if rifiuto && rifiuto.tipo === "emana_decreto"}<Rifiuto testo={rifiuto.testo} />{/if}
  {@render cartellino(bozza)}
{:else if !voce}
  <p class="sec muto">{t("cruscotto.scegli_riga")}</p>
{:else if proposta && voce.dicastero}
  {@const dicastero = voce.dicastero}
  <span class="occ">{t(fondi ? "cruscotto.proposta_fondi" : "cruscotto.proposta_riforma")}</span>
  <h3 class="tit">{voce.titolo}</h3>
  <div class="chi">
    {#if ministro}<Faccia nome={ministro.nome} ruolo={ministro.tecnico ? "tecnico" : "ministro"} partito={chiaveDi(vista, ministro.partito)} dicastero={dicastero} misura={46} />{/if}
    <span><b>{ministro?.nome ?? t("governo.senza_ministro")}</b>
      {#if ministro}<br /><span class="min muto">{t("cruscotto.ministro_soddisfazione", { partito: nome(ministro.partito), n: numero(Math.round(ministro.soddisfazione)) })}</span>{/if}</span>
  </div>
  <p class="sec"><i class="muto">{t("componenti.perche")}</i> {proposta.motivo}</p>
  <div class="az" role="group" aria-label={dettaglio(voce.titolo)}>
    <Pulsante forma="piccolo" spento={occupato} aria-label={dettaglio(t("cruscotto.accogli_di", { nome: voce.titolo }))} onclick={() => manda({ tipo: "accogli", dicastero })}>{t("aula.accogli")}</Pulsante>
    <Pulsante forma="piccolo" spento={proposta.rinviata ? t("cruscotto.gia_rinviata") : occupato} aria-label={dettaglio(t("cruscotto.rinvia_di", { nome: voce.titolo }))} onclick={() => manda({ tipo: "rinvia", dicastero })}>{t("cruscotto.rinvia")}</Pulsante>
    <Pulsante forma="piccolo" spento={occupato} aria-label={dettaglio(t("cruscotto.respingi_di", { nome: voce.titolo }))} onclick={() => manda({ tipo: "respingi", dicastero })}>{t("aula.respingi")}</Pulsante>
  </div>
  <p class="min muto">{t("cruscotto.se_ignori", { default: voce.default })}</p>
  {#if rifiuto && rifiuto.tipo !== "compra"}<Rifiuto testo={rifiuto.testo} />{/if}
  {#if fondi}
    {@render cartellino([])}
  {:else if aperto}
    {@render cartellino([{ disposizione: proposta.disposizione!, gradino: proposta.gradino! }], t("anteprima.nota_proposta"), t("anteprima.torna_alla_proposta"))}
  {:else}
    <p class="sec">{t("anteprima.nota_proposta")}</p>
    <div bind:this={piede}><Pulsante forma="link" onclick={(e) => scambia(e, true)}>{t("aula.anteprima")}</Pulsante></div>
  {/if}
{:else if voce.azioni === "fiducia" && misura}
  <span class="occ">{formaDi(misura)}</span>
  <h3 class="tit">{voce.titolo}</h3>
  <div class="emicicli">
    <Emiciclo ramo="Camera" totale={camera} si={misura.forbice.camera} maggioranza={misura.forbice.maggioranza_camera} />
    <Emiciclo ramo="Senato" totale={senato} si={misura.forbice.senato} maggioranza={misura.forbice.maggioranza_senato} />
  </div>
  {#if misura.forbice.in_bilico}<span class="chip rosso">{t("aula.in_bilico")}</span>{/if}
{:else}
  <h3 class="tit">{voce.titolo}</h3>
  <p class="sec">{voce.sotto}</p>
  {#if emendamento}<p class="sec"><i class="muto">{t("componenti.perche")}</i> {emendamento.motivo}</p>{/if}
  {#if voce.default}<p class="min muto">{t("cruscotto.se_ignori", { default: voce.default })}</p>{/if}
{/if}

<style>
  .occ { font-size: var(--c-min); letter-spacing: .1em; text-transform: uppercase; font-weight: 700; color: var(--ink) }
  .tit { font-family: var(--tit); font-variation-settings: var(--wonk); font-weight: 700; line-height: 1.1; text-wrap: balance; font-size: var(--c-sez) }
  .sec { font-size: var(--c-sec) }
  .min { font-size: var(--c-min) }
  .muto { color: var(--grigio) }
  .chi { display: flex; gap: 10px; align-items: center }
  .az { display: flex; gap: 6px; align-items: center; flex-wrap: wrap; border-top: var(--filo); padding-top: 8px }
  .emicicli { display: grid; grid-template-columns: 1.5fr 1fr; gap: 8px; align-items: end }
  .chip { justify-self: start; border: 1.5px solid var(--rosso); color: var(--rosso); padding: 0 6px; font-size: var(--c-min); font-weight: 700; line-height: 1.4 }
</style>
