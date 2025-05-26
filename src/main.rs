use std::error::Error;
use std::process;

use documentador::cli::args::read_validate_args;
use documentador::fs::reader::open_file_as_text;
use documentador::processor::process_data;

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
    let path = match read_validate_args() {
        Ok(path) => path,
        Err(e) => {
            eprintln!("Error: {}", e);
            return Err(Box::new(e));
        }
    };
    let file = match open_file_as_text(&path).await {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return Err(Box::new(e));
        }
    };

    match process_data::run(&file).await {
        Ok(_) => println!("Processing completed successfully."),
        Err(e) => {
            eprintln!("Error processing data: {}", e);
            return Err(Box::new(e));
        }
    }

    Ok(())
}
