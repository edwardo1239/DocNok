use std::error::Error;
use std::process;

use documentador::cli::args::read_validate_args;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    if let Err(e) = run().await {
        eprintln!("Error: {}", e);
        if let Some(source) = e.source() {
            eprintln!("Caused by: {}", source);
        }
        process::exit(1);
    }
    Ok(())
}

async fn run() -> Result<(), Box<dyn Error + Send + Sync>> {
    let path = read_validate_args();

    Ok(())
}
