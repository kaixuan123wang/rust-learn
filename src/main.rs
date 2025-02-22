use clap::{Parser, Subcommand};
use std::path::Path;
use std::fs;
use serde::{Serialize, Deserialize};
use csv::Reader;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Player {
    name: String,
    position: String,
    nationality: String,
    #[serde(rename = "DOB")]
    dob: String,
    #[serde(rename = "Kit Number")]
    kit: u8,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Opts {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Parser)]
enum Commands {
    #[command(name = "csv", about = "Show CSV, or convert CSV to other formats")]
    Csv(CsvOpts),
}

#[derive(Debug, Parser)]
struct CsvOpts {
    #[arg(short, long, help = "Input file", value_parser = verify_input_file)]
    input: String,
    #[arg(short, long, help = "Output file", default_value = "output.json")]
    output: String,
    #[arg(short, long, help = "Delimiter", default_value_t = ',')]
    delimiter: char,
    // -h为clap的help，所以header不能使用-h
    #[arg(long, help = "CSV has header or not", default_value_t = true)]
    header: bool,
}

fn verify_input_file(path: &str) -> Result<String, String> {
    if !Path::new(path).exists() {
        Err(format!("File {} does not exist", path))
    } else {
        Ok(path.into())
    }
}

fn main() -> anyhow::Result<()> {
    let args = Opts::parse();
    match args.command {
        Commands::Csv(opts) => {
            let mut reader = Reader::from_path(opts.input)?;
            let mut ret = Vec::with_capacity(128);
            for result in reader.deserialize() {
                let player: Player = result?;
                ret.push(player);
            }
            let json = serde_json::to_string_pretty(&ret)?;
            fs::write(opts.output, json)?;
        }
    }
    Ok(())
}