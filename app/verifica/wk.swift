// Misura in WKWebView (il WebKit di sistema, lo stesso dell'app): carica un URL con la scala data e stampa il JSON
// di un'espressione. Lo compila e lo lancia batteria-webkit.mjs; a mano, per una misura sola:
//   swiftc -O verifica/wk.swift -o verifica/uscita/wk
//   verifica/uscita/wk <url> <larghezza> <altezza> <scala> <espressione-js> [<css-forzato>]
// <css-forzato>: CSS aggiunto alla pagina prima della misura. WK_FOTO=<percorso.png>: anche una fotografia.
// Il deposito è .nonPersistent(): non legge e non scrive il localStorage dell'app.
import AppKit
import WebKit

let a = CommandLine.arguments
let (url, w, h, scala, espr) = (a[1], Double(a[2])!, Double(a[3])!, a[4], a[5])
let css = a.count > 6 ? a[6] : ""
let app = NSApplication.shared
app.setActivationPolicy(.prohibited)
let conf = WKWebViewConfiguration()
conf.websiteDataStore = .nonPersistent()
let imp = "{\"versione\":1,\"velocita\":\"1\",\"movimento\":\"sistema\",\"scala\":\(scala)}"
conf.userContentController.addUserScript(WKUserScript(source: "localStorage.setItem('impostazioni', \(String(reflecting: imp)))", injectionTime: .atDocumentStart, forMainFrameOnly: true))
let win = NSWindow(contentRect: NSRect(x: -5000, y: -5000, width: w, height: h), styleMask: [.borderless], backing: .buffered, defer: false)
let web = WKWebView(frame: NSRect(x: 0, y: 0, width: w, height: h), configuration: conf)
win.contentView = web
win.orderBack(nil)
web.load(URLRequest(url: URL(string: url)!))
func misura() {
  DispatchQueue.main.asyncAfter(deadline: .now() + 3) {
    let js = """
    return await (async () => { await document.fonts.ready;
      if (\(String(reflecting: css)) ) { const s = document.createElement('style'); s.textContent = \(String(reflecting: css)); document.head.append(s); }
      await new Promise((r) => setTimeout(r, 400));
      return JSON.stringify(await (\(espr))); })()
    """
    web.callAsyncJavaScript(js, arguments: [:], in: nil, in: .page) { r in
      switch r {
      case .success(let v): print(v as? String ?? "\(v)")
      case .failure(let e): print("errore: \(e)")
      }
      // WK_FOTO=<percorso.png>: anche una fotografia della finestra
      guard let foto = ProcessInfo.processInfo.environment["WK_FOTO"] else { exit(0) }
      web.takeSnapshot(with: nil) { img, _ in
        if let img, let t = img.tiffRepresentation, let b = NSBitmapImageRep(data: t), let png = b.representation(using: .png, properties: [:]) {
          try? png.write(to: URL(fileURLWithPath: foto))
        }
        exit(0)
      }
    }
  }
}
misura()
app.run()
