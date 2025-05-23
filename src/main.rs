use std::env;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Uso: Herrramienta de documentar <ruta_al_archivo_js>");
        std::process::exit(1);
    }
    let ruta = &args[1];

    if !Path::new(ruta).exists() {
        eprintln!("El archivo no existe: {}", ruta);
        std::process::exit(1);
    }

    println!("Ruta al archivo: {}", ruta);
}
