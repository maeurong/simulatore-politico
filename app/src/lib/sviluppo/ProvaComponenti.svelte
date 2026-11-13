<script lang="ts">
  // Pagina di prova dei componenti di base, caricata da /sviluppo/componenti solo in sviluppo:
  // i testi qui sono di prova e non stanno in interfaccia.json.
  import { numero, percentuale, t } from "$lib/testi/t";
  import Pulsante from "$lib/componenti/Pulsante.svelte";
  import Conferma from "$lib/componenti/Conferma.svelte";
  import NumeroFreccia from "$lib/componenti/NumeroFreccia.svelte";
  import Dettaglio from "$lib/componenti/Dettaglio.svelte";
  import Rifiuto from "$lib/componenti/Rifiuto.svelte";
  import Segnaposto from "$lib/componenti/Segnaposto.svelte";
  import Caselle from "$lib/componenti/Caselle.svelte";
  import VoceCausa from "$lib/componenti/VoceCausa.svelte";
  import BarraSeggi from "$lib/componenti/BarraSeggi.svelte";
  import Faccia from "$lib/componenti/Faccia.svelte";
  import { coalizioneDi, nomeDi, quotaIncrocio } from "$lib/partiti";
  import Mappa, { type Segno } from "$lib/mappa/Mappa.svelte";
  import SelettoreRegione from "$lib/mappa/SelettoreRegione.svelte";
  import { REGIONI } from "$lib/mappa/regioni";
  import Impostazioni from "$lib/cornice/Impostazioni.svelte";
  import Tracciato from "$lib/componenti/Tracciato.svelte";
  import { tracciatiDi, type Tracciato as T } from "$lib/tracciati";
  import type { Vista } from "$lib/tipi/Vista";
  import v01 from "../../../../.scratch/v1-piano-11-ui/viste/01-turno-zero.json";
  import v03 from "../../../../.scratch/v1-piano-11-ui/viste/03-governo-ordinario.json";
  import v05 from "../../../../.scratch/v1-piano-11-ui/viste/05-crisi.json";
  import v06 from "../../../../.scratch/v1-piano-11-ui/viste/06-opposizione-governo-ia.json";
  import v07 from "../../../../.scratch/v1-piano-11-ui/viste/07-partita-finita-eredita.json";

  let grave = $state(false);
  let scelta = $state<"ferma" | "dirotta" | null>(null);
  let confermate = $state(0);
  let rifiuto = $state(false);
  let impostazioni = $state(false);
  let escAlContenitore = $state(0);
  const scegli = (s: "ferma" | "dirotta") => (scelta = scelta === s ? null : s);

  $effect(() => {
    document.documentElement.classList.toggle("grave", grave);
    return () => document.documentElement.classList.remove("grave");
  });

  const inchiostri = [
    ["carta", "Il foglio: l’unico fondo.", "#F4ECD8 · —"],
    ["ink", "Testo, filetti forti, pieni.", "#1F1A17 · 14,6 : 1"],
    ["grigio", "Testo secondario, filetti sottili.", "#5E5750 · 6,0 : 1"],
    ["rosso", "Testata, ciò che va male, il critico.", "#B3122B · 5,9 : 1"],
    ["verde", "Ciò che va bene.", "#2E6B45 · 5,4 : 1"],
    ["biro", "Note a biro, motti, rimandi allo schema.", "#1D4E89 · 7,1 : 1"],
    ["giallo", "Evidenziatore: solo sotto l’inchiostro, mai testo né filetto.", "#FFD23F · 1,2 : 1"],
  ];
  const filetti = [
    ["border-top:var(--filo)", "1 px grigio · fra le righe"],
    ["border-top:var(--bordo)", "1,5 px · riquadri e caselle"],
    ["border-top:var(--forte)", "2 px · pulsanti"],
    ["border-top:var(--doppio)", "doppio · testa e piede di pagina"],
    ["height:12px;background:var(--retino);border:var(--bordo)", "retino · zona di pericolo, vignette"],
    ["height:12px;background:var(--velo)", "velo · riga scelta, passaggio del puntatore"],
    ["height:12px;border:1.5px dashed var(--grigio)", "tratteggio · spento o non ancora valido"],
  ];
  const uno = (x: number) => numero(x, 1);

  const dalleViste: [string, unknown][] = [["Turno zero · 01", v01], ["Al governo · 03", v03], ["Crisi · 05", v05], ["All’opposizione · 06", v06], ["Partita finita · 07", v07]];
  const SPREAD = { stacco: 550, passo: 75, rovesciata: true };
  const prova = (x: Partial<T>): T => ({ nome: "tracciato.conti", valore: 0, testo: "", ramo: null, scala: SPREAD, rosso: false, grigio: false, rimando: null, prima: null, ...x });
  const diProva: [string, T[]][] = [
    ["Con la variazione", [
      prova({ nome: "tracciato.consenso_coalizione", valore: 56.68, testo: "56,7%", scala: { stacco: 42, passo: 4, rovesciata: false }, prima: 53.1 }),
      prova({ nome: "tracciato.maggioranza", valore: 14, testo: "+14", ramo: "Senato", scala: { stacco: 0, passo: 3, rovesciata: false }, prima: 15 }),
      prova({ valore: 205, testo: "205", prima: 196 }),
    ]],
    ["Spread alla soglia e al tetto", [prova({ valore: 550, testo: "550" }), prova({ valore: 1000, testo: "1.000" })]],
    ["Caselle larghe, col retino", [prova({ nome: "tracciato.maggioranza", valore: 2, testo: "+2", ramo: "Camera", scala: { stacco: 0, passo: 3, rovesciata: false } })]],
  ];
  let riempi = $state(2);

  const v03t = v03 as unknown as Vista;
  // il consenso della coalizione per regione, dalla Vista 03; Molise e Sardegna senza dato, per vedere il tratteggio
  const coalizione = coalizioneDi(v03t).map((id) => v03t.partiti.findIndex((p) => p.id === id));
  const consenso: Record<string, number | null> = Object.fromEntries(
    v03t.sondaggio.regioni.map((r) => [r.id, coalizione.reduce((s, j) => s + (quotaIncrocio(r, j) ?? 0), 0)]),
  );
  const conVuoti = { ...consenso, molise: null, sardegna: null };
  const segniProva: Record<string, Segno[]> = {
    liguria: ["cantiere", "atto", "emergenza"], valle_d_aosta: ["atto", "opera"], molise: ["cantiere", "emergenza"], lombardia: ["cantiere"], sicilia: ["emergenza"], toscana: ["opera"],
  };
  let regScelta = $state<string | null>(null);
  let selScelta = $state<string | null>(null);
  let spesa = $state("");
  const camera = (v: Vista) => v.seggi.map((s) => ({ id: s.partito, nome: nomeDi(v, s.partito), seggi: s.camera }));
  const coalCamera = (v: Vista) => v.seggi.filter((s) => coalizioneDi(v).includes(s.partito)).reduce((x, s) => x + s.camera, 0);
  const totCamera = (v: Vista) => v.seggi.reduce((x, s) => x + s.camera, 0);
</script>

<div class="camp">
  <section>
    <h3 class="tit">Inchiostri</h3>
    {#each inchiostri as [v, ruolo, valori] (v)}
      <div class="ink"><i style="background:var(--{v})"></i><b>{v}</b><code>{valori}</code><small>{ruolo}</small></div>
    {/each}
    <p class="min muto nota">Contrasto sulla carta. Nel tono grave rosso, verde e biro diventano inchiostro e il giallo sparisce.</p>
    <p class="nota"><Pulsante forma="piccolo" scelto={grave} onclick={() => (grave = !grave)}>Tono grave</Pulsante></p>
    <h4 class="tit">Filetti e campiture</h4>
    <div class="fil">
      {#each filetti as [stile, nome] (nome)}<i style={stile}></i><span>{nome}</span>{/each}
    </div>
  </section>

  <section>
    <h3 class="tit">Caratteri</h3>
    {#if !grave}<div class="car uno"><code>72 · testata</code><span class="nomino">{t("cornice.nomino")}</span></div>{/if}
    <div class="car"><code>44 · apertura</code><span><span class="tit" style="font-size:var(--c-apre)">Piove sul Po e sui conti</span></span></div>
    <div class="car"><code>30 · pagina</code><span><span class="tit" style="font-size:var(--c-pag)">Come funziona lo Stato</span></span></div>
    <div class="car"><code>23 · sezione</code><span><span class="tit" style="font-size:var(--c-sez)">Il borsino dei ministri</span></span></div>
    <div class="car"><code>18 · riquadro</code><span><span class="tit" style="font-size:var(--c-riq)">Emergenza idrogeologica</span></span></div>
    <div class="car"><code>19 · numero</code><b style="font-size:var(--c-num)">56,5% · +24 / +15 · 1.000</b></div>
    <div class="car"><code>16 · attacco</code><span style="font-size:var(--c-lead)">Il fondo di riserva copre un miliardo.</span></div>
    <div class="car"><code>14 · corpo</code><span>Formazione pagata dalle imprese</span></div>
    <div class="car"><code>13 · secondo</code><span class="sec muto">Simone Ricci · Centrodestra</span></div>
    <div class="car"><code>12 · minimo</code><span class="min muto">vicino alla sua posizione</span></div>
    <div class="car"><code>12 · occhiello</code><span class="occ">Emergenza · Marche</span></div>
    {#if !grave}<div class="car"><code>13 · motto</code><span class="sec motto">«Meno sigle, più primari contenti.»</span></div>{/if}
  </section>

  <section>
    <h3 class="tit">Componenti</h3>
    <h4 class="tit primo">Numero con freccia</h4>
    <div class="fila grande">
      <NumeroFreccia valore={71} prima={61} formato={numero} male="scende" />
      <NumeroFreccia valore={51} prima={53.7} formato={numero} formatoVariazione={uno} male="scende" />
      <NumeroFreccia valore={33} prima={45.2} formato={numero} formatoVariazione={uno} male="scende" sottoSoglia />
      <NumeroFreccia valore={60} prima={60} formato={numero} male="scende" />
      <NumeroFreccia valore={86} prima={83} formato={(x) => `${numero(x)} pb`} male="sale" />
      <NumeroFreccia valore={24} prima={20} formato={numero} />
      <NumeroFreccia valore={38} formato={numero} sottoSoglia />
    </div>
    <p class="min muto">La freccia dice il verso, il colore il giudizio (lo spread che sale è rosso); senza giudizio resta grigia. Sotto soglia la variazione va in negativo.</p>

    <h4 class="tit">Pulsante d’azione</h4>
    <div class="fila">
      <Pulsante forma="chiude">{t("cornice.chiudi_il_turno")}</Pulsante>
      <Pulsante forma="bordo">Presenta</Pulsante>
      <Pulsante forma="bordo" scelto={scelta === "ferma"} onclick={() => scegli("ferma")}>{t("componenti.ferma")}</Pulsante>
      <Pulsante forma="bordo" scelto={scelta === "dirotta"} onclick={() => scegli("dirotta")}>{t("componenti.dirotta")}</Pulsante>
      <Pulsante forma="piccolo">{t("componenti.ferma")}</Pulsante>
      <Pulsante forma="piccolo" scelto>Scelto</Pulsante>
    </div>
    <div class="fila">
      <Pulsante forma="bordo" spento="Camere sciolte" onclick={() => (rifiuto = true)}>Poni la fiducia</Pulsante>
      <Pulsante forma="piccolo" spento="niente da fermare: margine esaurito">{t("componenti.ferma")}</Pulsante>
    </div>
    <div class="fila">
      <Pulsante forma="link">Altre 14 notizie nel registro ›</Pulsante>
      <Pulsante forma="link" spento="Turno zero: nessun turno passato">Registro</Pulsante>
    </div>

    <h4 class="tit">Conferma sul posto</h4>
    <div class="fila">
      <Conferma etichetta={t("componenti.chiedi_le_dimissioni")} conferma={t("componenti.conferma_le_dimissioni")} onconferma={() => confermate++} />
    </div>
    <div class="fila">
      <Conferma etichetta="Torna al turno 1" conferma="Conferma: cancella 1 turno" spiegazione="Il turno 2 si perde." onconferma={() => confermate++} />
    </div>
    <!-- contenitore che chiude con Esc, come il pannello delle impostazioni: l'Esc della conferma non deve arrivarci -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div class="fila contenitore" onkeydown={(e) => { if (e.key === "Escape") escAlContenitore++; }}>
      <Conferma etichetta="Cancella la bozza" conferma="Conferma: cancella la bozza" onconferma={() => confermate++} />
      <span class="min muto">Esc arrivati al contenitore: {escAlContenitore}</span>
    </div>
    <p class="min muto">Confermate: {confermate}. Tab, Invio, Esc.</p>

    <h4 class="tit">Rifiuto</h4>
    <div class="fila">
      <Pulsante forma="bordo" onclick={() => (rifiuto = !rifiuto)}>Presenta il decreto</Pulsante>
      {#if rifiuto}<Rifiuto testo="Il decreto supera il fondo di riserva di 1,2 miliardi." />{/if}
    </div>

    <h4 class="tit">Segnaposto</h4>
    <Segnaposto sezione={t("sommario.paese")} fetta="11c" />
  </section>
</div>

<section class="tracciati">
  <h3 class="tit">Tracciati</h3>
  {#each dalleViste as [nome, v] (nome)}
    <div class="stato"><b class="sec">{nome}</b><div class="trks">{#each tracciatiDi(v as Vista, []) as x (x.nome)}<Tracciato tracciato={x} />{/each}</div></div>
  {/each}
  {#each diProva as [nome, tre] (nome)}
    <div class="stato" style={nome.startsWith("Caselle") ? "--cw:14px;--ch:13px;--ret:var(--retino)" : undefined}>
      <b class="sec">{nome} <span class="min muto">· di prova</span></b><div class="trks">{#each tre as x, i (i)}<Tracciato tracciato={x} />{/each}</div>
    </div>
  {/each}
  <div class="stato">
    <b class="sec">A scatti <span class="min muto">· di prova</span></b>
    <div class="trks"><Tracciato tracciato={prova({ nome: "tracciato.maggioranza", valore: riempi, testo: String(riempi), ramo: "Camera", scala: { stacco: 0, passo: 3, rovesciata: false }, rosso: riempi < 0 })} /></div>
    <span class="fila"><Pulsante forma="piccolo" onclick={() => (riempi = riempi === 2 ? 20 : riempi === 20 ? -12 : 2)}>Cambia: {riempi}</Pulsante></span>
  </div>
</section>

<section class="nuovi">
  <h3 class="tit">Fascia, voce con causa, barra dei seggi, faccia</h3>
  <div class="stato"><b class="sec">Fascia a tre</b>
    <div class="fila sec"><Caselle piene={3} su={3} nome="rischio alto" rosso /><Caselle piene={2} su={3} nome="intesa media" /><Caselle piene={0} su={3} nome="intesa assente" /></div></div>
  <div class="stato"><b class="sec">Barra a dieci, filo dopo la quarta</b>
    <div class="fila sec"><Caselle piene={7} su={10} filo={4} nome="intesa 70" /><Caselle piene={3} su={10} filo={4} nome="intesa 30" rosso /><Caselle piene={10} su={10} filo={4} nome="intesa 100" /><Caselle piene={0} su={10} filo={4} nome="intesa 0" /></div></div>
  <div class="stato"><b class="sec">Voce con causa</b>
    <div style="max-width:420px"><VoceCausa fatto="Sinistra porta nel posto riservato «Regime delle pensioni»" causa="nessun seggio dei partiti alleati la voterebbe" /><VoceCausa fatto="Senza causa: solo il fatto" /></div></div>
  <div class="stato"><b class="sec">Barra dei seggi · Camera</b>
    <div style="max-width:520px;display:grid;gap:16px">
      <BarraSeggi righe={camera(v03t)} totale={totCamera(v03t)} segno={coalCamera(v03t)} />
      <BarraSeggi righe={[...camera(v03t), { id: "nuovo_8", nome: "Voce del Paese", seggi: 12 }]} totale={totCamera(v03t) + 12} />
      <BarraSeggi righe={[{ id: "destra", nome: "Destra", seggi: 0 }]} totale={400} />
    </div></div>
  <div class="stato"><b class="sec">Faccia · iniziali</b>
    <div class="fila"><Faccia nome="Luca Benedetti" misura={34} /><Faccia nome="Luca Benedetti" misura={46} /><Faccia nome="Luca Benedetti" misura={62} /><Faccia nome="Luca Benedetti" misura={84} /><Faccia nome="" misura={34} /></div></div>
</section>

<section class="mappe">
  <h3 class="tit">Mappa e selettore delle regioni</h3>
  <div class="stato"><b class="sec">Mappa · consenso, con segni</b>
    <div class="mp"><Mappa valori={conVuoti} segni={segniProva} scelta={regScelta} ordine={REGIONI.map((r) => r.id)} nome="Le venti regioni" formato={(x) => percentuale(x)} onscelta={(id) => (regScelta = id)} /></div>
    <span class="min muto">Un Tab entra, le frecce girano le venti; Invio sceglie. Molise e Sardegna senza dato.</span></div>
  <div class="stato"><b class="sec">Mappa · sola lettura</b>
    <div class="mp"><Mappa valori={consenso} scelta={null} ordine={REGIONI.map((r) => r.id)} nome="Le venti regioni" solaLettura onscelta={() => {}} /></div><span></span></div>
  <div class="stato"><b class="sec">Selettore</b>
    <div class="sl"><SelettoreRegione valori={consenso} bind:scelta={selScelta} azione="Spendi in {REGIONI.find((r) => r.id === selScelta)?.nome ?? '…'}" onconferma={() => (spesa = selScelta ?? "")} onannulla={() => (selScelta = null)} /></div>
    <span class="min muto">Confermato: {spesa || "nulla"}.</span></div>
</section>

<div class="lato">
  <div class="corpo">
    <Segnaposto sezione={t("sommario.governo")} fetta="11b" />
    <p class="nota"><Pulsante forma="link" scelto={impostazioni} onclick={() => (impostazioni = !impostazioni)}>{t("impostazioni.titolo")}</Pulsante></p>
  </div>
  {#if impostazioni}
  <Impostazioni onchiudi={() => (impostazioni = false)} />
  {:else}
  <Dettaglio aria-label="Dettaglio di prova">
    <div class="occ">Sanità</div>
    <h4 class="tit" style="font-size:var(--c-sez)">Sara Ferraro</h4>
    <p class="sec muto">Destra</p>
    {#each { length: 12 }, i (i)}
      <p class="sec">Riga {i + 1} del riquadro a lato: largo var(--lato), scorre in verticale.</p>
    {/each}
    <div class="fila"><Pulsante forma="bordo">{t("componenti.ferma")}</Pulsante><Pulsante forma="bordo">{t("componenti.dirotta")}</Pulsante></div>
  </Dettaglio>
  {/if}
</div>

<style>
  .tit { font-family: var(--tit); font-variation-settings: var(--wonk); font-weight: 700; line-height: 1.1; text-wrap: balance }
  :global(:root.grave) .tit { font-weight: 600 }
  .occ { font-size: var(--c-min); letter-spacing: .1em; text-transform: uppercase; font-weight: 700; color: var(--rosso) }
  .muto { color: var(--grigio) }
  .sec { font-size: var(--c-sec) }
  .min { font-size: var(--c-min) }
  .motto { font-style: italic; color: var(--biro) }
  .nomino { font-family: var(--tit); font-variation-settings: var(--wonk); font-weight: 800; color: var(--rosso); font-size: var(--c-test); line-height: 1.05; white-space: nowrap }

  .camp { display: grid; grid-template-columns: repeat(3, minmax(0, 1fr)); gap: 10px var(--canale); padding: var(--s3) var(--marg) 14px; align-content: start }
  .camp > * { min-width: 0 }
  h3 { font-size: var(--c-sez); border-bottom: var(--doppio); padding-bottom: 3px; margin-bottom: var(--s2) }
  h4 { font-size: var(--c-riq); margin: var(--s3) 0 5px }
  h4.primo { margin-top: 0 }
  .nota { margin-top: 6px }

  .ink { display: grid; grid-template-columns: 26px 1fr auto; gap: 2px 10px; align-items: center; padding: var(--s1) 0; border-top: var(--filo); font-size: var(--c-sec) }
  .ink i { width: 26px; height: 26px; border: var(--bordo); grid-row: span 2 }
  .ink small { grid-column: 2/-1; color: var(--grigio); font-size: var(--c-min) }
  code { font: inherit; font-size: var(--c-min); color: var(--grigio) }
  .car { display: grid; grid-template-columns: 54px minmax(0, 1fr); gap: 0 10px; align-items: baseline; padding: 3px 0; border-top: var(--filo) }
  .car.uno { grid-template-columns: 1fr }
  .car > span:last-child { overflow: hidden; white-space: nowrap; text-overflow: ellipsis }
  .fil { display: grid; grid-template-columns: 70px 1fr; gap: 6px 10px; align-items: center; font-size: var(--c-min); color: var(--grigio) }
  .fil i { display: block; height: 0 }

  .fila { display: flex; gap: 6px; margin-top: var(--s2); flex-wrap: wrap; align-items: center }
  .grande { gap: 18px; font-size: 17px; margin-top: 0 }
  .contenitore { border: 1.5px dashed var(--grigio); padding: var(--s1) var(--s2) }

  .mappe { padding: 0 var(--marg) var(--s3) }
  .mp { height: 420px }
  .sl { max-width: 440px }
  .nuovi { padding: 0 var(--marg) var(--s3) }
  .tracciati { padding: 0 var(--marg) var(--s3) }
  .stato { display: grid; grid-template-columns: 200px minmax(0, 1fr) auto; gap: var(--s3); align-items: center; padding: var(--s2) 0; border-top: var(--filo) }
  .trks { display: flex; flex-wrap: wrap; row-gap: var(--s2) }

  /* flex, non grid: il 30% di --lato si misura sulla riga intera, come nella sezione del prototipo */
  .lato { display: flex; gap: var(--canale); padding: 0 var(--marg) var(--marg); height: 640px }
  .corpo { flex: 1; min-width: 0 }
</style>
