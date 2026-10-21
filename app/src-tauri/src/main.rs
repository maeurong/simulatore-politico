fn main() {
    if let Err(errore) = app_lib::run() {
        eprintln!("errore: {errore}");
        std::process::exit(1);
    }
}
