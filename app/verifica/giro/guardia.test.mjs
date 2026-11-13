import { describe, expect, it } from "vitest";
import { rifiuto } from "./guardia.mjs";

const PORTA = 1420;
// cmd.mjs: dal loopback, senza Origin
const locale = { remoteAddress: "127.0.0.1", host: "127.0.0.1:1420", origin: undefined, method: "POST" };
const prova = (cambi = {}, u = "/__coda") => rifiuto({ ...locale, ...cambi }, u, PORTA);

describe("guardia del telecomando", () => {
  it("lascia passare cmd.mjs, che non manda Origin", () => {
    expect(prova()).toBeNull();
  });

  it("lascia passare la pagina dell'app, che ha l'origine del server", () => {
    expect(prova({ origin: "http://127.0.0.1:1420" }, "/__comando")).toBeNull();
    expect(prova({ host: "localhost:1420", origin: "http://localhost:1420" }, "/__diario")).toBeNull();
  });

  it("lascia passare il loopback in IPv6, mappato e nativo", () => {
    expect(prova({ remoteAddress: "::ffff:127.0.0.1" })).toBeNull();
    expect(prova({ remoteAddress: "::1", host: "[::1]:1420" })).toBeNull();
  });

  it.each([undefined, "10.127.0.1", "192.168.1.20", "::ffff:10.0.0.5"])("rifiuta con 403 l'indirizzo remoto %s", (remoteAddress) => {
    expect(prova({ remoteAddress })).toBe(403);
  });

  it.each(["malvagio.example:1420", "127.0.0.1:5173", "127.0.0.1", undefined])("rifiuta con 403 l'Host %s", (host) => {
    expect(prova({ host })).toBe(403);
  });

  it.each(["http://malvagio.example", "http://127.0.0.1:5173", "null"])("rifiuta con 403 l'Origin %s", (origin) => {
    expect(prova({ origin })).toBe(403);
  });

  it.each(["/__coda", "/__comando", "/__diario"])("rifiuta con 405 un GET su %s", (u) => {
    expect(prova({ method: "GET" }, u)).toBe(405);
  });

  it("lascia leggere l'esito con un GET", () => {
    expect(prova({ method: "GET" }, "/__esito")).toBeNull();
  });

  it("mette l'indirizzo e l'origine prima del metodo", () => {
    expect(prova({ method: "GET", remoteAddress: "10.0.0.5" })).toBe(403);
    expect(prova({ method: "GET", origin: "null" })).toBe(403);
  });
});
