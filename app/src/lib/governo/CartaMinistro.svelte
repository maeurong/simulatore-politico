<script lang="ts">
  // La carta del ministro nel dettaglio a lato (storia 100): chi è, quanto è soddisfatto e perché, dove sta rispetto a te, cosa vuole, cosa ha in agenda.
  // Sotto «In agenda»: Ferma e Dirotta (diretti), cantieri, dimissioni. Dopo Ferma o Dirotta riusciti la carta gira (Web Animations con la molla del prototipo 18)
  // e sul retro dice cosa è successo; lo stato del retro si legge da agenda.stato, quindi al rientro la carta è già girata. Nel tono grave e col movimento spento il retro compare senza rotazione.
  import { t, numero, segnato, elenco, type Chiave } from "$lib/testi/t";
  import { nomeDi } from "$lib/partiti";
  import { chiaveDi, fondoDi } from "$lib/caricature/caricatura";
  import { REGIONI } from "$lib/mappa/regioni";
  import { mottoDi } from "$lib/testi/pungolo";
  import { partita } from "$lib/partita.svelte";
  import type { Comando } from "$lib/tipi/Comando";
  import type { Vista } from "$lib/tipi/Vista";
  import Conferma from "$lib/componenti/Conferma.svelte";
  import Girevole from "./Girevole.svelte";
  import Faccia from "$lib/componenti/Faccia.svelte";
  import NumeroFreccia from "$lib/componenti/NumeroFreccia.svelte";
  import Pulsante from "$lib/componenti/Pulsante.svelte";
  import Rifiuto from "$lib/componenti/Rifiuto.svelte";
  import Cantieri from "./Cantieri.svelte";
  import Rosa from "./Rosa.svelte";
  import { etichettaFerma, euro, obiettivo, posizioni, prezzoDimissioni, sulRetro, type Figurina } from "./governo";

  let { vista, f, grave, manda, rifiuto, ondirotta, anima, volta, onrigira }: {
    vista: Vista; f: Figurina; grave: boolean; manda: (c: Comando) => Promise<boolean>; rifiuto: string | null; ondirotta: () => void;
    anima: boolean; volta: boolean; onrigira: () => void;
  } = $props();

  const m = $derived(f.ministro);
  const min = $derived(vista.ministeri.find((x) => x.dicastero === f.dicastero)!);
  const s = $derived(vista.soglie);
  const motto = $derived(m && !grave ? mottoDi(m.nome, f.dicastero) : null);
  const pos = $derived(m ? posizioni(vista, m) : null);
  const ob = $derived(obiettivo(vista, f.dicastero));
  const causa = (c: string) => t(`governo.causa_${c}` as Chiave);
  const regione = (id: string | null) => (id ? (REGIONI.find((r) => r.id === id)?.nome ?? id) : null);
  const pc = (x: number) => Math.min(10, Math.max(0, x)) * 10;
  // shortcut: copia riduzione_rodaggio del core (ministri/mod.rs:128-133); passare a una capacità effettiva nella Vista quando il core la espone
  const capacita = $derived(m ? (m.rodaggio ? Math.max(1, m.capacita - 1) : m.capacita) : 0);
  const puo = $derived(!!m && f.agenda?.stato === "annunciata" && !min.dimissioni_chieste);
  const stato = $derived(f.agenda?.stato ?? "annunciata");

  const girata = $derived(!grave && sulRetro(f) && !volta);
  const motivi = $derived(elenco(f.evidenza.map((e) => t(`governo.evidenza_${e}` as Chiave)), ", "));
</script>

{#if !m}
  <div class="carta">
    <div class="su">
      <span class="vuoto" aria-hidden="true">—</span>
      <div>
        <div class="occ">{f.nomeDicastero}</div>
        <h4 class="tit">{t("governo.carta_interim")}</h4>
        {#if min.causa_dimissioni}<div class="sec muto">{t("governo.interim_causa", { causa: causa(min.causa_dimissioni) })}</div>{/if}
      </div>
    </div>
    <p class="sec">{t("governo.interim_nota")}</p>
    <Rosa {vista} {min} {manda} />
    <Cantieri {min} puoFermare={false} {manda} />
    {#if rifiuto}<Rifiuto testo={rifiuto} />{/if}
  </div>
{:else}
  <Girevole {girata} {anima} conRetro={!grave && sulRetro(f)} chiave={f.dicastero}>
  {#snippet fronte()}
  <div class="carta">
    <div class="su">
      <Faccia nome={m.nome} ruolo={m.tecnico ? "tecnico" : "ministro"} partito={chiaveDi(vista, m.partito)} misura={84} fondo={fondoDi(m.soddisfazione, s)} dicastero={f.dicastero} />
      <div>
        <div class="occ">{f.nomeDicastero}</div>
        <h4 class="tit">{m.nome}</h4>
        <div class="sec muto">
          {nomeDi(vista, m.partito)} ·
          <span class="cap" role="img" aria-label={t(m.rodaggio ? "governo.capacita_rodaggio" : "governo.capacita_nome", { c: m.capacita, r: capacita })}>
            {#each [1, 2, 3, 4, 5] as i (i)}<i class:on={i <= capacita} class:meno={i > capacita && i <= m.capacita}></i>{/each}
          </span>
        </div>
        <div class="chips">{#each f.stati as st (st)}<span class="chip" class:rosso={st === "preavviso" || st === "dimissioni_chieste"}>{t(`governo.stato_${st}` as Chiave)}</span> {/each}</div>
      </div>
    </div>
    {#if motto}<p class="motto">{t("governo.motto", { motto })}</p>{/if}

    <div class="sez"><span>{t("governo.soddisfazione")}</span>
      <NumeroFreccia valore={m.soddisfazione} prima={Math.abs(f.freccia) >= 0.5 ? m.soddisfazione - f.freccia : undefined} formato={(x) => numero(x)} male="scende" sottoSoglia={m.soddisfazione < s.dimissioni} />
    </div>
    <div class="sodd" aria-hidden="true">
      <i class:basso={m.soddisfazione < s.minaccia} style:width="{m.soddisfazione}%"></i>
      <b style:left="{s.dimissioni}%"></b><b style:left="{s.minaccia}%"></b>
    </div>
    <div class="min muto riga"><span>{t("governo.soglie", { dimissioni: s.dimissioni, minaccia: s.minaccia })}</span><span>{t("governo.su_cento", { n: numero(m.soddisfazione) })}</span></div>
    {#if m.variazioni.length}
      <div class="cause">
        {#each m.variazioni as v, i (i)}
          <span>{v.turno === vista.turno ? t("governo.questo_turno", { causa: causa(v.causa) }) : causa(v.causa)}</span>
          <span class={v.delta < 0 ? "male" : "bene"}>{segnato(v.delta, 1)}</span>
        {/each}
      </div>
    {:else}<p class="min muto">{t("governo.nessuna_variazione")}</p>{/if}

    <div class="sez"><span>{t("governo.posizione")}</span></div>
    <div class="asse">
      <span style:left="{pc(pos!.ministro)}%"></span>
      <em style:left="{pc(pos!.ministro)}%">{t("governo.asse_nome", { nome: m.nome.split(" ").at(-1) ?? "", n: numero(pos!.ministro, 1) })}</em>
      <span class="tu" style:left="{pc(pos!.tua)}%"></span>
      <b style:left="0">{t("governo.asse_estremo_sinistro")}</b><b style:right="0">{t("governo.asse_estremo_destro")}</b>
      <b class="tua" style:left="{pc(pos!.tua)}%">{t("governo.asse_tu", { n: numero(pos!.tua, 1) })}</b>
    </div>

    {#if ob.nome}
      <div class="sez"><span>{t("governo.obiettivo")}</span></div>
      <p class="sec">{t("governo.obiettivo_testo", { nome: ob.nome, valore: numero(ob.valore ?? 0, 1), obiettivo: numero(ob.obiettivo ?? 0, 1), verso: t(`governo.verso_${ob.verso}` as Chiave) })}</p>
    {/if}

    <div class="sez"><span>{t("governo.in_agenda")}</span>{#if f.agenda}<span>{euro(f.agenda.costo)}</span>{/if}</div>
    {#if f.agenda}
      <div class="sec"><span class:fermo={f.agenda.stato === "fermata"}>{f.agenda.nome}</span>{#if f.agenda.regione} · <b>{regione(f.agenda.regione)}</b>{/if}</div>
      <p class="min muto">{f.agenda.motivo}</p>
      {#if f.evidenza.length}<p class="min male">{t("governo.in_evidenza", { motivi })}</p>{/if}
    {:else}
      <p class="sec"><span class="vuota">{f.senzaAgenda ? t(`governo.senza_agenda_${f.senzaAgenda.replace(" ", "_")}` as Chiave) : t("componenti.nessun_atto")}</span></p>
    {/if}
    {#if puo}
      <div class="az">
        <Pulsante forma="bordo" aria-disabled={partita.occupato ? "true" : undefined} title={t("governo.ferma_titolo")} onclick={() => manda({ tipo: "ferma", dicastero: f.dicastero })}>{etichettaFerma(f.agenda!.costo_ferma)}</Pulsante>
        <Pulsante forma="bordo" aria-disabled={partita.occupato ? "true" : undefined} onclick={ondirotta}>{t("governo.dirotta")}</Pulsante>
      </div>
    {/if}
    {#if rifiuto}<Rifiuto testo={rifiuto} />{/if}
    <p class="min muto">
      {t("governo.margine", { euro: euro(min.margine_residuo) })}{#if min.proposta} · {t("governo.proposta", { motivo: min.proposta.motivo })} <a class="link" href="/partita/cruscotto">{t("governo.cruscotto")}</a>{/if}
    </p>
    <Cantieri {min} puoFermare={!min.dimissioni_chieste} {manda} />
    {#if min.dimissioni_chieste}
      <p class="sec"><b>{t("governo.dimissioni_chieste")}</b></p>
    {:else}
      <div><Conferma etichetta={t("governo.dimissioni_chiedi")} conferma={t("governo.dimissioni_conferma")}
        spiegazione={t("governo.dimissioni_spiega", { prezzo: prezzoDimissioni(vista, min), atto: min.agenda ? t("governo.dimissioni_atto") : "" })}
        onconferma={() => manda({ tipo: "chiedi_dimissioni", dicastero: f.dicastero })} /></div>
    {/if}
  </div>
  {/snippet}
  {#snippet retro()}
    <div class="carta retro">
      <Faccia nome={m.nome} ruolo={m.tecnico ? "tecnico" : "ministro"} partito={chiaveDi(vista, m.partito)} misura={62} fondo={fondoDi(m.soddisfazione, s)} dicastero={f.dicastero} />
      <span class="bollo" class:biro={stato === "dirottata"}>{t(stato === "fermata" ? "governo.atto_fermato" : "governo.atto_dirottato")}</span>
      <b>{f.agenda?.nome}{#if f.agenda?.regione} · {regione(f.agenda.regione)}{/if}</b>
      <span class="sec">{t("governo.retro_voto", { n: numero(m.soddisfazione) })}</span>
      <button type="button" class="link" onclick={onrigira}>{t("governo.rigira")}</button>
    </div>
  {/snippet}
  </Girevole>
{/if}

<style>
  .carta.retro { place-content: center; justify-items: center; text-align: center; gap: 8px }
  .carta.retro .bollo { font-size: var(--c-sez); padding: 2px 12px }
  .bollo { display: inline-block; border: 2px solid var(--rosso); color: var(--rosso); font-family: var(--tit); font-weight: 800; letter-spacing: .06em; text-transform: uppercase; transform: rotate(-4deg); white-space: nowrap }
  .bollo.biro { border-color: var(--biro); color: var(--biro) }
  :global(:root.grave) .bollo { transform: none }
  .az { display: flex; gap: 6px; flex-wrap: wrap }
  .carta { border: var(--forte); background: var(--carta); padding: 10px 12px; display: grid; gap: 7px; align-content: start }
  .su { display: grid; grid-template-columns: auto minmax(0, 1fr); gap: 12px; align-items: center }
  .su .tit { font-size: var(--c-sez) }
  .tit { font-family: var(--tit); font-variation-settings: var(--wonk); font-weight: 700; line-height: 1.1; text-wrap: balance }
  .occ { font-size: var(--c-min); letter-spacing: .1em; text-transform: uppercase; font-weight: 700; color: var(--rosso) }
  .vuoto { display: inline-grid; place-items: center; width: 84px; height: 84px; border: 1.5px dashed var(--grigio); border-radius: 50%; color: var(--grigio); font-size: 24px }
  .sec { font-size: var(--c-sec) }
  .min { font-size: var(--c-min) }
  .muto { color: var(--grigio) }
  .bene { color: var(--verde) }
  .male { color: var(--rosso) }
  .motto { font-size: var(--c-min); color: var(--biro); line-height: 1.25 }
  .chips { margin-top: 3px }
  .chip { display: inline-block; border: 1.5px solid var(--ink); padding: 0 6px; font-size: var(--c-min); font-weight: 700; white-space: nowrap; line-height: 1.4 }
  .chip.rosso { border-color: var(--rosso); color: var(--rosso) }
  .cap { display: inline-flex; gap: 2px; vertical-align: middle }
  .cap i { width: 8px; height: 8px; border-radius: 50%; border: 1.5px solid var(--ink) }
  .cap i.on { background: var(--ink) }
  .cap i.meno { background: var(--retino) }
  .sez { font-size: var(--c-min); letter-spacing: .1em; text-transform: uppercase; font-weight: 700; color: var(--grigio); border-bottom: 1.5px solid var(--ink); padding-bottom: 2px; margin-top: 4px; display: flex; justify-content: space-between; align-items: baseline; gap: 8px }
  .sez :global(.nf) { font-size: var(--c-num); letter-spacing: 0; text-transform: none; color: var(--ink); line-height: 1 }
  .sodd { position: relative; height: 10px; border: 1px solid var(--grigio); background: var(--velo) }
  .sodd i { position: absolute; inset: 0 auto 0 0; background: var(--ink) }
  .sodd i.basso { background: var(--rosso) }
  .sodd b { position: absolute; top: -3px; bottom: -3px; width: 1.5px; background: var(--rosso) }
  .riga { display: flex; justify-content: space-between; gap: 8px }
  .cause { display: grid; grid-template-columns: minmax(0, 1fr) auto; gap: 1px 10px; font-size: var(--c-min) }
  .cause .male { font-weight: 700 }
  .asse { position: relative; height: 22px; border-bottom: 1.5px solid var(--ink); margin: 2px 6px 14px }
  .asse span { position: absolute; bottom: -5px; width: 10px; height: 10px; margin-left: -5px; border-radius: 50%; background: var(--ink) }
  .asse span.tu { background: var(--carta); border: 2px solid var(--biro) }
  .asse em { position: absolute; bottom: 8px; font-style: normal; font-size: var(--c-min); transform: translateX(-50%); white-space: nowrap }
  .asse b { position: absolute; top: 24px; font-size: var(--c-min); font-weight: 400; color: var(--grigio) }
  .asse b.tua { transform: translateX(-50%); color: var(--biro) }
  .fermo { text-decoration: line-through; text-decoration-thickness: 1.5px; color: var(--grigio) }
  .vuota { display: inline-block; border: 1.5px dashed var(--grigio); color: var(--grigio); padding: 2px 8px }
  .link { text-decoration: underline; text-underline-offset: 2px; white-space: nowrap }
  .link:hover { text-decoration-thickness: 2px }
</style>
