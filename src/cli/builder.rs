use crate::front_end::{token::{self, TokenKind}, tokenizer::Tokenizer};
use std::{error::Error, fs, path::Path};

pub fn build(p: &Path) -> Result<(), Box<dyn Error>> {
    let source = fs::read_to_string(&p)?;
    let mut tokenizer = Tokenizer::new(&source);
    // TODO: create .into() iterator, and then remove partialeq and pub kind
    let mut token = tokenizer.next().unwrap();
    while token.kind != TokenKind::EOF {
        println!("Current token: {:?}", token);
        token = tokenizer.next().unwrap();
    }
    Ok(())
}
