use crate::invoice_data::InvoiceData;
use chrono::prelude::*;
use handlebars::{to_json, Handlebars};
use serde_json::value::{Map, Value as Json};
use std::env;
use std::fs;
use std::fs::File;
use std::io;
use std::path::PathBuf;
use std::process::Command;

fn resolve_out_dir() -> io::Result<PathBuf> {
    let cwd = env::current_dir();
    let default = PathBuf::from("/");
    let parent = cwd.unwrap_or(default);
    Ok(parent.join("out"))
}

fn generate_file_name() -> String {
    let now = Local::now();
    now.format("%Y%m%d%H%M.tex").to_string()
}

#[test]
fn test_generate_file_name() {
    generate_file_name();
}

fn generate_file_name_with_path() -> io::Result<PathBuf> {
    let path = resolve_out_dir()?.join(generate_file_name());
    Ok(path)
}

fn init_templates<'a>() -> Handlebars<'a> {
    let mut handlebars = Handlebars::new();
    handlebars
        .register_template_file("template", "./templates/layout.hbs")
        .unwrap();
    handlebars
}

fn prepare_assigns(invoice_data: &InvoiceData) -> Map<String, Json> {
    let mut data = Map::new();

    data.insert("buyer".to_string(), to_json(&invoice_data.buyer));
    data.insert("seller".to_string(), to_json(&invoice_data.seller));
    data.insert("currency".to_string(), to_json(&invoice_data.currency));
    data.insert("sold_on".to_string(), to_json(&invoice_data.sold_on));
    data.insert("issued_at".to_string(), to_json(&invoice_data.issued_at));

    data
}

pub fn render_invoice_template(data: InvoiceData) -> io::Result<PathBuf> {
    let outdir = resolve_out_dir()?;
    fs::create_dir_all(outdir)?;
    let file_name = generate_file_name_with_path()?;
    let handlebars = init_templates();
    let data = prepare_assigns(&data);
    let mut file = File::create(&file_name)?;
    handlebars
        .render_to_write("template", &data, &mut file)
        .unwrap();

    Ok(file_name)
}

pub fn compile_latex_file(path: PathBuf) -> io::Result<()> {
    let outdir = resolve_out_dir()?;
    let outdir = &outdir.as_path().display();
    let job = path.as_path().display();
    Command::new("xelatex")
        .arg(format!("-output-dir={}", outdir))
        .arg(format!("{}", job))
        .output()?;
    Ok(())
}
