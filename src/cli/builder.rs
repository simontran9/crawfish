use crate::front_end;
use crate::front_end::token::TokenKind; // TODO: remove
use std::error::Error;
use std::fs;
use std::path::Path;

pub fn build(p: &Path) -> Result<(), Box<dyn Error>> {
    let source = fs::read_to_string(&p)?;
    let mut tokenizer = front_end::tokenizer::Tokenizer::new(&source);
    // TODO: create .into() iterator, and then remove partialeq and pub kind
    let mut token = tokenizer.next().unwrap();
    while token.kind != TokenKind::EOF {
        println!("Current token: {:?}", token);
        token = tokenizer.next().unwrap();
    }
    Ok(())
}
