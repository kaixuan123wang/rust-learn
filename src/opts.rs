use clap::Parser;
use std::{path::Path, str::FromStr, fmt};
use std::fmt::Display;

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
    #[arg(short, long, help = "Output file")]
    pub output: Option<String>,
    #[arg(short, long, help = "Delimiter", default_value_t = ',')]
    pub delimiter: char,
    // -h为clap的help，所以header不能使用-h
    #[arg(long, help = "CSV has header or not", default_value_t = true)]
    pub header: bool,

    #[arg(long, help = "Output format",value_parser=parse_format, default_value = "json")]
    pub format: OutputFormat,
}

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Yaml,
    Toml,
}
impl FromStr for OutputFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            "toml" => Ok(OutputFormat::Toml),
            _ => anyhow::bail!("Invalid output format: {}", s),
        }
    }
}

impl Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OutputFormat::Json => write!(f, "json"),
            OutputFormat::Yaml => write!(f, "yaml"),
            OutputFormat::Toml => write!(f, "toml"),
        }
    }
}

fn parse_format(s: &str) -> Result<OutputFormat, anyhow::Error> {
    s.parse()
}

fn verify_input_file(path: &str) -> Result<String, String> {
    if !Path::new(path).exists() {
        Err(format!("File {} does not exist", path))
    } else {
        Ok(path.into())
    }
}