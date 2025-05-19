use std::path::Path;

#[derive(Debug)]
pub enum BruhError {
    Ya,
}
pub fn build(p: &Path) -> Result<(), BruhError> {
    if 2 == 2 {
            // match fs::read_to_string(&source_path) {
    // }
        Ok(println!("We building!"))
    } else {
        Err(BruhError::Ya)
    }
}