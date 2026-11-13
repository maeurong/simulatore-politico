<script lang="ts">
  // Il riepilogo dal vivo (14-editor.prototype.html:347-371): avvisi, i tre conti con lo scarto da oggi, il peso
  // delle disposizioni cambiate, i sei posti. I testi degli avvisi dai campi dell'Avviso, come Avviso::testo
  // (crates/sim-core/src/riepilogo.rs:73-96).
  import { elenco, numero, percentuale, segnato, t } from "$lib/testi/t";
  import type { Avviso } from "$lib/tipi/Avviso";
  import type { PartitoCatalogo } from "$lib/tipi/PartitoCatalogo";
  import type { PosizioneAttesa } from "$lib/tipi/PosizioneAttesa";
  import type { RiepilogoScenario } from "$lib/tipi/RiepilogoScenario";

  type Props = {
    riepilogo: RiepilogoScenario;
    /** il riepilogo dell'Italia di oggi, per lo scarto */
    oggi: RiepilogoScenario;
    partiti: PartitoCatalogo[];
    /** fermo all'ultimo valido davanti a un rifiuto */
    spento: boolean;
  };
  let { riepilogo: r, oggi, partiti, spento }: Props = $props();

  function avviso(a: Avviso): { grave: boolean; titolo: string; nota: string } {
    switch (a.tipo) {
      case "senza_maggioranza": {
        const posti = elenco(a.posizioni.map((x) => numero(x, 1)), "; ");
        return { grave: true, titolo: t("editor.avviso_senza_maggioranza", { n: a.posizioni.length, posti }), nota: t("editor.avviso_senza_maggioranza_nota") };
      }
      case "situazioni_accese": {
        const n = a.nomi.length;
        return { grave: false, titolo: t("editor.avviso_accese", { n, nomi: elenco(a.nomi, ", ") }), nota: t("editor.avviso_accese_nota", { n }) };
      }
      case "crisi_del_debito":
        return { grave: true, titolo: t("editor.avviso_crisi", { spread: numero(a.spread_pb) }), nota: t("editor.avviso_crisi_nota", { soglia: numero(a.soglia_pb) }) };
    }
  }

  /** Lo scarto da oggi, arrotondato come il valore: ciò che si legge zero è «come oggi». Salire è male per tutti e tre. */
  function scarto(x: number, y: number, decimali: number) {
    const s = segnato(x - y, decimali);
    if (!/[1-9]/.test(s)) return { testo: t("editor.come_oggi"), classe: "muto" };
    return { testo: t("editor.su_oggi", { scarto: s }), classe: x > y ? "male" : "bene" };
  }
  const conti = $derived([
    { k: t("editor.spread"), n: numero(r.spread_pb), u: t("editor.punti_base"), d: scarto(r.spread_pb, oggi.spread_pb, 0) },
    { k: t("editor.interessi"), n: `${numero(r.interessi_pil, 1)}%`, u: t("editor.del_pil"), d: scarto(r.interessi_pil, oggi.interessi_pil, 1) },
    { k: t("editor.disavanzo"), n: `${numero(r.disavanzo_pil, 1)}%`, u: t("editor.del_pil"), d: scarto(r.disavanzo_pil, oggi.disavanzo_pil, 1) },
  ]);
  const peso = $derived(segnato(r.saldo_disposizioni_pil, 2));

  const nomi = $derived(new Map(partiti.map((p) => [p.id, p.nome])));
  const coalizione = (p: PosizioneAttesa) =>
    p.coalizione.map((id) => (id === "giocatore" ? t("editor.voi_minuscolo") : (nomi.get(id) ?? id))).join(", ");
</script>

<div class="riep" class:spento>
  <div>
    <p class="occ">{t("editor.riepilogo_vivo")}</p>
    <h2 class="tit">{t("editor.se_si_fondasse_oggi")}</h2>
  </div>
  <div class="avv">
    {#each r.avvisi as a (a.tipo)}
      {@const x = avviso(a)}
      <p class={["av", { lieve: !x.grave }]}><b>{x.titolo}</b>{x.nota}</p>
    {/each}
  </div>
  <div class="conti">
    {#each conti as c (c.k)}
      <div class="conto"><p class="k">{c.k}</p><p class="n">{c.n}</p><p class="d">{c.u} · <span class={c.d.classe}>{c.d.testo}</span></p></div>
    {/each}
  </div>
  {#if /[1-9]/.test(peso)}
    <p class="sec">{t("editor.peso_prima")} <b class={r.saldo_disposizioni_pil > 0 ? "male" : "bene"}>{t("editor.peso_saldo", { saldo: peso })}</b> {t("editor.peso_dopo")}</p>
  {/if}
  <table class="posti">
    <thead>
      <tr><th>{t("editor.posto")}</th><th>{t("editor.voi")}</th><th>{t("editor.coalizione")}</th><th>{t("editor.camera")}</th><th>{t("editor.senato")}</th><th>{t("editor.maggioranza_breve")}</th></tr>
    </thead>
    <tbody>
      {#each r.posizioni as p (p.posizione)}
        <tr class={{ no: !p.maggioranza }}>
          <td><b>{numero(p.posizione, 1)}</b></td>
          <td><span class="qb" style:width="{p.quota_attesa * 70}px"></span>{percentuale(p.quota_attesa)}</td>
          <td>{percentuale(p.quota_coalizione)}</td>
          <td>{numero(p.seggi_camera)}</td>
          <td>{numero(p.seggi_senato)}</td>
          <td>{#if p.maggioranza}<span class="bene">{t("editor.si")}</span>{:else}<b class="male">{t("editor.no")}</b>{/if}</td>
        </tr>
        <tr class={{ no: !p.maggioranza }}><td></td><td class="co" colspan="5">{t("editor.con", { partiti: coalizione(p) })}</td></tr>
      {/each}
    </tbody>
  </table>
  <p class="min muto">{t("editor.nota_posti")}</p>
</div>

<style>
  .riep { display: grid; gap: 8px; align-content: start; transition: opacity calc(var(--t-press) * var(--k)) var(--e-work) }
  .riep.spento { opacity: .55 }
  .occ { font-size: var(--c-min); letter-spacing: .1em; text-transform: uppercase; font-weight: 700; color: var(--rosso) }
  .tit { font-family: var(--tit); font-variation-settings: var(--wonk); font-weight: 700; line-height: 1.1; text-wrap: balance; font-size: var(--c-sez) }
  .sec { font-size: var(--c-sec) }
  .min { font-size: var(--c-min) }
  .muto { color: var(--grigio) }
  .bene { color: var(--verde) }
  .male { color: var(--rosso) }

  .avv { display: grid; gap: 6px }
  .av { border-left: 4px solid var(--rosso); padding: 3px 8px; font-size: var(--c-sec) }
  .av.lieve { border-left-color: var(--grigio) }
  .av b { display: block }

  .conti { display: grid; grid-template-columns: repeat(3, minmax(0, 1fr)); gap: 8px }
  .conto { border-top: var(--forte); padding-top: 3px; min-width: 0 }
  .conto .k { font-size: var(--c-min); color: var(--grigio); letter-spacing: .06em; text-transform: uppercase }
  .conto .n { font-family: var(--tit); font-weight: 700; font-size: var(--c-pag); line-height: 1.05 }
  .conto .d { font-size: var(--c-min) }
  /* inline-block: WebKit non va a capo davanti a un nowrap in riga e lo lascia sporgere */
  .conto .d span { display: inline-block; white-space: nowrap }

  .riep { container-type: inline-size }
  /* fixed e colonne in percentuale: la tabella è larga quanto il pannello in ogni motore, qualunque sia la misura dei caratteri */
  .posti { width: 100%; table-layout: fixed; border-collapse: collapse; font-size: var(--c-sec) }
  /* misurate: in minuscolo a 300 px e in maiuscolo a 380 px ogni colonna sta nella sua quota */
  .posti th:nth-child(1) { width: 13% }
  .posti th:nth-child(2) { width: 21% }
  .posti th:nth-child(3) { width: 22% }
  .posti th:nth-child(4) { width: 16% }
  .posti th:nth-child(5) { width: 15% }
  .posti th:nth-child(6) { width: 13% }
  .posti th { font-size: var(--c-min); text-transform: uppercase; color: var(--grigio); font-weight: 700; text-align: right; padding: 2px 4px; border-bottom: var(--filo) }
  .posti th:first-child, .posti td:first-child { text-align: left }
  .posti td { padding: 3px 4px; border-bottom: var(--filo); text-align: right; white-space: nowrap }
  .posti tr.no td { background: repeating-linear-gradient(45deg, transparent 0 6px, color-mix(in srgb, var(--rosso) 8%, transparent) 6px 7px) }
  .posti .co { font-size: var(--c-min); color: var(--grigio); text-align: left; white-space: normal }
  /* il lato stretto (scala alta, 1280): l'intestazione maiuscola non entra, scende in minuscolo e stringe.
     In em (380 px sul corpo di 14 px): sotto zoom WebKit confronta la larghezza ingrandita, e con i px la regola non scattava */
  @container (width < 27.2em) {
    .posti th { text-transform: none }
    .posti th, .posti td { padding: 2px }
  }
  .qb { display: inline-block; height: 8px; background: var(--rosso); vertical-align: middle; margin-right: 4px }
</style>
