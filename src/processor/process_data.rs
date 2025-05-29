use crate::models::errors::{ProcessError, ProcessErrorKind};
use crate::models::document::DocumentInfo;

pub async fn run(text: &str) -> Result<(), ProcessError> {
    let doc = DocumentInfo::new(text);
    if doc.title.is_empty() {
        return Err(ProcessError::new(
            401,
            "El archivo no tiene titulo",
            ProcessErrorKind::MissingTittle,
            "processor::process_data::run",
        ));
    }
    println!("Título encontrado: {}", doc.title);
    Ok(())
}
