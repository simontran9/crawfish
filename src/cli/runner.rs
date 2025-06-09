use std::error::Error;
use std::path::Path;

pub fn run(_path: &Path) -> Result<(), Box<dyn Error>> {
    Err("Runner not implemented yet".into())
}
