<script lang="ts">
  // Il campione per la prova di riconoscimento (ticket 32, storia 91): le facce senza nome, con un numero sotto; la chiave
  // numero → nome sta in fondo, ripiegata. Caricata da /sviluppo/caricature solo in sviluppo; i testi di prova non stanno in interfaccia.json.
  import Caricatura from "$lib/caricature/Caricatura.svelte";
  import Faccia from "$lib/componenti/Faccia.svelte";
  import { leveDi, rosa, saltiDeiLeader, type Ruolo } from "$lib/caricature/caricatura";
  import v01 from "../../../../.scratch/v1-piano-11-ui/viste/01-turno-zero.json";
  import v03 from "../../../../.scratch/v1-piano-11-ui/viste/03-governo-ordinario.json";

  type Pers = { ruolo: Ruolo; partito: string; nome: string; salto?: number };
  const MISURE = [34, 46, 62, 84] as const;
  const FONDI = ["carta", "giallo", "rosso"] as const;
  const saltiL = saltiDeiLeader(v01);
  const leader: Pers[] = v01.partiti.filter((p) => p.leader).map((p) => ({ ruolo: "leader" as const, partito: p.nome, nome: p.leader, salto: saltiL[p.id] }));
  const ministri: Pers[] = v03.ministeri.map((m) => ({ ruolo: "ministro" as const, partito: v03.partiti.find((p) => p.id === m.ministro.partito)?.nome ?? m.ministro.partito, nome: m.ministro.nome }));
  // nessuna Vista ha un tecnico: tre inventati, per vedere il cartellino
  const tecnici: Pers[] = [["Destra", "Elena Marino"], ["Partito del giocatore", "Fabio Greco"], ["Destra", "Francesca Bruno"]].map(([partito, nome]) => ({ ruolo: "tecnico" as const, partito, nome }));
  const campione = [...leader, ...ministri, ...tecnici];
  const numero = (p: Pers) => campione.indexOf(p) + 1;
  // tre nomi che con l'hash puro hanno la stessa faccia: la rosa li separa
  const candidati = ["Luca Bianchi", "Marco Marino", "Simone Bruno"].map((nome) => ({ partito: "Partito del giocatore", nome }));
  const separate = rosa(candidati);
  const grezze = candidati.map((c) => leveDi("ministro", c.partito, c.nome));

  let grave = $state(false);
  $effect(() => {
    document.documentElement.classList.toggle("grave", grave);
    return () => document.documentElement.classList.remove("grave");
  });
</script>

<main class="pag">
  <h2>Caricature · campione senza nomi</h2>
  <p class="muto">Per ogni faccia: a chi somiglia? Quattro misure, in riga: 34, 46, 62, 84 px. Tre gruppi: sette leader (1–7, con la rosa dell'app), dieci ministri (8–17), tre tecnici inventati (18–20).</p>

  <section class="gruppo" aria-label="Campione">
    {#each campione as p, i (i)}
      <figure>
        <div class="riga">{#each MISURE as m}<Caricatura ruolo={p.ruolo} partito={p.partito} nome="" leve={leveDi(p.ruolo, p.partito, p.nome, p.salto)} misura={m} />{/each}</div>
        <figcaption>{numero(p)}</figcaption>
      </figure>
    {/each}
  </section>

  <section>
    <h3>Una rosa di tre</h3>
    <p class="muto">Sopra l'hash puro, sotto la rosa che separa chi ripete naso e colletto.</p>
    <div class="gruppo">
      {#each [grezze, separate] as serie, k (k)}
        <figure><div class="riga">{#each serie as l, i (i)}<Caricatura ruolo="ministro" partito="Partito del giocatore" nome="" leve={l} misura={62} />{/each}</div><figcaption>{k ? "con la rosa" : "senza"}</figcaption></figure>
      {/each}
    </div>
  </section>

  <section>
    <h3>Fondi della soddisfazione</h3>
    <div class="riga">{#each FONDI as f}<Caricatura ruolo="ministro" partito="Destra" nome="" leve={leveDi("ministro", "Destra", "Simone Gallo")} misura={84} fondo={f} />{/each}</div>
  </section>

  <section>
    <h3>Tono grave</h3>
    <label class="muto"><input type="checkbox" bind:checked={grave} /> tono grave: iniziali nel cerchio, stesso riquadro</label>
    <div class="riga">{#each MISURE as m}<Faccia nome="Anna Ferrucci" ruolo="leader" partito="Estrema sinistra" salto={saltiL.estrema_sinistra} misura={m} />{/each}</div>
  </section>

  <details>
    <summary>Chiave numero → nome</summary>
    <ol class="chiave">{#each campione as p, i (i)}<li value={numero(p)}>{p.nome} · {p.ruolo} · {p.partito}</li>{/each}</ol>
  </details>
</main>

<style>
  .pag { padding: var(--s3) var(--marg); display: grid; gap: var(--s3); align-content: start }
  h2, h3 { font-family: var(--tit); font-variation-settings: var(--wonk); font-weight: 700 }
  h3 { border-bottom: var(--doppio); padding-bottom: 3px }
  .muto { color: var(--grigio); font-size: var(--c-sec) }
  .gruppo { display: flex; flex-wrap: wrap; gap: 14px 22px }
  figure { display: grid; gap: 4px; justify-items: center; padding: 6px 8px; border: var(--filo) }
  .riga { display: flex; align-items: end; gap: 10px }
  figcaption { font-weight: 700; font-variant-numeric: tabular-nums }
  .chiave { columns: 3; padding-left: 2em; font-size: var(--c-sec) }
</style>
