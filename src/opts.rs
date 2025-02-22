use clap::Parser;
use std::path::Path;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Parser)]
pub enum Commands {
    #[command(name = "csv", about = "Show CSV, or convert CSV to other formats")]
    Csv(CsvOpts),
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, help = "Input file", value_parser = verify_input_file)]
    pub input: String,
    #[arg(short, long, help = "Output file", default_value = "output.json")]
    pub output: String,
    #[arg(short, long, help = "Delimiter", default_value_t = ',')]
    pub delimiter: char,
    // -h为clap的help，所以header不能使用-h
    #[arg(long, help = "CSV has header or not", default_value_t = true)]
    pub header: bool,
}

fn verify_input_file(path: &str) -> Result<String, String> {
    if !Path::new(path).exists() {
        Err(format!("File {} does not exist", path))
    } else {
        Ok(path.into())
    }
}