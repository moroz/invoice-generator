use std::fs::File;
use std::io;
use std::io::BufReader;

pub mod invoice_data;
pub mod renderer;

use invoice_data::InvoiceData;
use renderer::Renderer;

fn main() -> io::Result<()> {
    let source = File::open("./data/sample.json")?;
    let buf_reader = BufReader::new(source);
    let data: InvoiceData = serde_json::from_reader(buf_reader)?;
    let path = Renderer::render_invoice_template(data)?;
    Renderer::compile_latex_file(path)?;

    Ok(())
}
