use std::path::Path;

#[derive(Debug)]
pub enum RunError {
    NAH,
}

pub fn run(p: &Path) -> Result<(), RunError> {
    if 2 == 2 {
        Ok(println!("We running!"))
    } else {
        Err(RunError::NAH)
    }
}
