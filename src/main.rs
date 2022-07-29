use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;

pub mod invoice_data;

fn resolve_out_dir() -> io::Result<PathBuf> {
    let cwd = env::current_dir()?;
    let default = PathBuf::from("/");
    let parent = cwd.parent().unwrap_or(&default);
    Ok(parent.join("out"))
}

fn main() -> io::Result<()> {
    let path = resolve_out_dir()?;
    println!("{:?}", path);
    fs::create_dir_all(path)?;
    println!("Hello, world!");

    Ok(())
}
