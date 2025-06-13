use crate::front_end;
use std::error::Error;
use std::fs;
use std::path::Path;

pub fn build(p: &Path) -> Result<(), Box<dyn Error>> {
    let source = fs::read_to_string(&p)?;
    let lexer = front_end::lexer::Lexer::new(&source);
    Ok(())
}
