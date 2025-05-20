use crate::front_end::tokenizer::Tokenizer;
use std::{error::Error, fs, path::Path};

pub fn build(p: &Path) -> Result<(), Box<dyn Error>> {
    let source = fs::read_to_string(&p)?;
    let tokenizer = Tokenizer::new(&source);
    Ok(())
}
