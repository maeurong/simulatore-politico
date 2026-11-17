<script lang="ts">
  // Il Registro, forma D · Indice col filo (25-registro.prototype.html): filtri sopra, colonna dei turni, voci del turno scelto, dettaglio a lato.
  import { untrack } from "svelte";
  import { page } from "$app/state";
  import { periodo } from "$lib/edicola/edicola";
  import { SEZIONE_DI } from "$lib/esito/esito";
  import { etichetta, SEZIONI, type Sezione } from "$lib/ossatura";
  import { t, type Chiave } from "$lib/testi/t";
  import type { Vista } from "$lib/tipi/Vista";
  import Pulsante from "$lib/componenti/Pulsante.svelte";
  import { registroCache } from "./cache.svelte";
  import DettaglioVoce from "./DettaglioVoce.svelte";
  import Riavvolgi from "./Riavvolgi.svelte";
  import Turni, { type Riga } from "./Turni.svelte";
  import { passa, periodoDel, soggetti, tocca, turnoDaUrl, type Filtro, type Soggetto, type Turno } from "./registro";

  let { vista }: { vista: Vista } = $props();

  // un turno che si chiude mentre si legge allunga il giro in corso (la cache lo sa), e qui basta richiamarla al cambio di turno
  const turnoVista = $derived(vista.turno);
  $effect(() => {
    void turnoVista;
    untrack(() => registroCache.carica(vista));
  });

  const TUTTO: Filtro = { criticita: "tutte", sezione: null, soggetto: null };
  let filtro = $state<Filtro>(TUTTO);
  const attivo = $derived(filtro.criticita !== "tutte" || filtro.sezione !== null || filtro.soggetto !== null);

  const turni = $derived(registroCache.turni);
  const letti = $derived(turni.filter((x): x is Turno => "voci" in x));
  const caricando = $derived(turni.length < vista.turno);
  // i turni non letti restano in colonna anche filtrando: un filtro non li può giudicare
  const righe = $derived<Riga[]>(
    turni
      .map((turno) => ({ turno, passanti: "voci" in turno ? turno.voci.filter((v) => passa(v, filtro)).length : 0 }))
      .filter((r) => !attivo || !("voci" in r.turno) || r.passanti > 0)
      .reverse(),
  );

  // solo all'apertura: dopo, il turno lo sceglie il giocatore (il link della banda di «avanti» porta `?turno=N`)
  let scelto = $state<number | undefined>(untrack(() => turnoDaUrl(page.url.searchParams.get("turno"))));
  let scelta = $state<{ turno: number; i: number } | null>(null);
  const corrente = $derived(righe.find((r) => r.turno.turno === scelto)?.turno ?? righe[0]?.turno);

  const sezioni = $derived(SEZIONI.filter((s) => letti.some((x) => x.voci.some((v) => SEZIONE_DI[v.tipo] === s))));
  const elencoSoggetti = $derived(soggetti(letti, vista));
  const chiave = (s: Soggetto) => `${s.tipo}:${s.id}`;
  const GRUPPI = ["partito", "dicastero", "regione", "disposizione", "bilancio"] as const;

  const CRITICITA = [["tutte", "registro.crit_tutte"], ["notevoli", "registro.crit_notevoli"], ["critiche", "registro.crit_critiche"]] as const;

  function scegliTurno(n: number) {
    scelto = n;
    scelta = null;
  }
  // una collegata fuori dal filtro lo toglie: altrimenti il turno non sarebbe in colonna e la voce non in pagina
  function vai(turno: number, i: number) {
    const v = letti.find((x) => x.turno === turno)?.voci[i];
    if (v && !passa(v, filtro)) filtro = TUTTO;
    scelto = turno;
    scelta = { turno, i };
  }

  // le voci che passano il filtro; una riga di dettaglio che porta il soggetto si mostra sotto la voce fusa che non lo porta da sola
  const righeDelFiltro = (v: Turno["voci"][number]) => {
    const s = filtro.soggetto;
    return s && !tocca(v, s) ? v.dettagli.filter((d) => tocca(d, s)) : [];
  };
</script>

<div class="registro">
  <div class="filtri" role="group" aria-label={t("registro.filtri")}>
    <span class="gr">
      <span class="k">{t("registro.criticita")}</span>
      {#each CRITICITA as [valore, chiaveTesto] (valore)}
        <Pulsante forma="piccolo" scelto={filtro.criticita === valore} onclick={() => (filtro = { ...filtro, criticita: valore })}>{t(chiaveTesto)}</Pulsante>
      {/each}
    </span>
    <label class="gr">
      <span class="k">{t("registro.sezione")}</span>
      <select value={filtro.sezione ?? ""} onchange={(e) => (filtro = { ...filtro, sezione: (e.currentTarget.value || null) as Sezione | null })}>
        <option value="">{t("registro.tutte")}</option>
        {#each sezioni as s (s)}<option value={s}>{etichetta(s)}</option>{/each}
      </select>
    </label>
    <label class="gr">
      <span class="k">{t("registro.soggetto")}</span>
      <select value={filtro.soggetto ? chiave(filtro.soggetto) : ""}
        onchange={(e) => (filtro = { ...filtro, soggetto: elencoSoggetti.find((x) => chiave(x.soggetto) === e.currentTarget.value)?.soggetto ?? null })}>
        <option value="">{t("registro.nessuno")}</option>
        {#each GRUPPI as g (g)}
          {@const nomi = elencoSoggetti.filter((x) => x.soggetto.tipo === g)}
          {#if nomi.length}
            <optgroup label={t(`registro.soggetto_${g}` as Chiave)}>
              {#each nomi as { soggetto, nome } (chiave(soggetto))}<option value={chiave(soggetto)}>{nome}</option>{/each}
            </optgroup>
          {/if}
        {/each}
      </select>
    </label>
    {#if attivo}<Pulsante forma="link" onclick={() => (filtro = TUTTO)}>{t("registro.togli_filtri")}</Pulsante>{/if}
  </div>

  <Turni {righe} scelto={corrente?.turno} filtrato={attivo} {caricando} {vista} onscelta={scegliTurno} />

  <section class="pag">
    {#if corrente}
      {@const c = corrente}
      <div class="capo">
        <h3 class="tit">{t("registro.turno_periodo", { n: c.turno, periodo: periodo(periodoDel(c.turno, vista)) })}</h3>
        <Riavvolgi turno={c.turno} {vista} {turni} />
      </div>
      {#if "voci" in c}
        {@const passanti = c.voci.filter((v) => passa(v, filtro))}
        <p class="meta">
          {attivo ? t("registro.voci_di", { k: passanti.length, n: c.voci.length }) : t("registro.voci", { n: c.voci.length })}
          · {t("registro.n_in_prima_pagina", { n: c.voci.filter((v) => v.in_evidenza).length })} · {t("registro.critiche", { n: c.critiche })}
        </p>
        <ul class="voci">
          {#each c.voci as v, i (i)}
            {#if passa(v, filtro)}
              {@const sel = scelta?.turno === c.turno && scelta.i === i}
              <li class={["vc", v.criticita, { sel }]}>
                <button type="button" aria-pressed={sel} onclick={() => (scelta = { turno: c.turno, i })}>
                  <i class="mk" aria-hidden="true"></i>
                  <span class="fa">{#if v.criticita === "critica"}<span class="nascosto">{t("registro.c_critica")}: </span>{/if}{v.fatto}</span>
                  <span class="ca"><i>{t("componenti.perche")}</i> {v.causa}</span>
                  {#each righeDelFiltro(v) as d, k (k)}<span class="rf">{d.testo}</span>{/each}
                </button>
                <span class="tg">
                  {#if v.in_evidenza}<span class="pp">{t("registro.in_prima_pagina")}</span>{/if}
                  {#if v.tono === "grave"}<span class="grave">{t("registro.grave")}</span>{/if}
                  <span class="etc">{etichetta(SEZIONE_DI[v.tipo])}</span>
                </span>
              </li>
            {/if}
          {/each}
        </ul>
      {:else}
        <p class="ph">{t("registro.guasto", { testo: c.guasto })}</p>
      {/if}
    {:else if !caricando}
      <p class="ph">{t("registro.nessuna_voce")}</p>
    {/if}
  </section>

  <aside class="lato" aria-label={t("registro.dettaglio")}>
    <DettaglioVoce {vista} turni={letti} {scelta} onvai={vai} />
  </aside>
</div>

<style>
  .registro { display: grid; grid-template-columns: 244px minmax(0, 1fr) var(--lato); grid-template-rows: auto minmax(0, 1fr); height: 100%; min-height: 0 }
  .registro > * { min-width: 0 }
  .filtri { grid-column: 1 / -1; display: flex; gap: 14px; align-items: center; flex-wrap: wrap; padding: 0 0 7px; border-bottom: var(--forte); font-size: var(--c-sec) }
  .gr { display: flex; gap: 3px; align-items: center }
  .k { font-size: var(--c-min); letter-spacing: .1em; text-transform: uppercase; color: var(--grigio); margin-right: 3px }
  select { border: 1.5px solid var(--ink); padding: 2px 6px; font-size: var(--c-sec); background: var(--carta); color: var(--ink); max-width: 260px; font-family: inherit }
  .pag { padding: 12px var(--canale) 10px; min-height: 0; overflow: hidden auto }
  .capo { display: flex; align-items: baseline; justify-content: space-between; gap: 12px; border-bottom: var(--doppio); padding-bottom: 4px; flex-wrap: wrap }
  .tit { font-family: var(--tit); font-variation-settings: var(--wonk); font-weight: 700; line-height: 1.1; text-wrap: balance; font-size: var(--c-sez) }
  .meta { font-size: var(--c-min); color: var(--grigio); margin: 4px 0 2px }
  .ph { border: 1.5px dashed var(--grigio); color: var(--grigio); padding: 7px 11px; font-size: var(--c-sec); margin-top: 4px }
  .voci { list-style: none }
  .vc { display: grid; grid-template-columns: minmax(0, 1fr) auto; border-top: var(--filo) }
  .vc:first-child { border-top: 0 }
  .vc:hover, .vc.sel { background: var(--velo) }
  .vc.sel { box-shadow: inset 0 0 0 1.5px var(--ink) }
  .vc button { display: grid; grid-template-columns: 12px minmax(0, 1fr); gap: 1px 8px; padding: 6px 0 7px 6px; text-align: left; align-items: start }
  .vc button > :not(.mk) { grid-column: 2 }
  .fa { font-size: var(--c-sec); line-height: 1.3 }
  .critica .fa, .notevole .fa { font-weight: 700 }
  .ca { font-size: var(--c-min); color: var(--grigio); line-height: 1.3 }
  .rf { font-size: var(--c-min); line-height: 1.3; padding-left: 10px; text-indent: -10px }
  .rf::before { content: "· " }
  .mk { width: 9px; height: 9px; margin-top: 4px; flex: none; grid-row: 1 }
  .critica .mk { background: var(--rosso) }
  :global(:root.grave) .critica .mk { background: var(--ink) }
  .notevole .mk { border: 1.5px solid var(--ink) }
  .ordinaria .mk { width: 5px; height: 5px; margin: 6px 2px 0; background: var(--grigio); border-radius: 50% }
  .tg { display: flex; flex-direction: column; align-items: end; gap: 2px; padding: 6px 6px 0 8px }
  .pp { font-size: var(--c-min); white-space: nowrap; border-bottom: 1.5px solid var(--ink) }
  .grave { font-size: var(--c-min); font-weight: 700; background: var(--ink); color: var(--carta); padding: 0 5px; white-space: nowrap }
  .etc { font-size: var(--c-min); white-space: nowrap; color: var(--grigio) }
  .lato { border-left: var(--forte); padding: 12px 0 12px var(--canale); min-height: 0; overflow: hidden auto; display: grid; gap: 0; align-content: start }
</style>
