use crate::models::errors::ProcessError;

pub async fn run(text: &str) -> Result<(), ProcessError>{
    println!("Processing text: {}", text);
    Ok(())
}