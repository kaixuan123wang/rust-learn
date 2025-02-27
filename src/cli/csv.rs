use clap::Parser;
use std::path::Path;
use std::str::FromStr;
use std::fmt;
use std::fmt::Display;
use super::verify_file;
use crate::CmdExector;
use crate::process::process_csv;
#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, help = "Input file", value_parser = verify_file)]
    pub input: String,
    #[arg(short, long, help = "Output file")]
    pub output: Option<String>,
    #[arg(short, long, help = "Delimiter", default_value_t = ',')]
    pub delimiter: char,
    #[arg(long, help = "CSV has header or not", default_value_t = true)]
    pub header: bool,
    #[arg(long, help = "Output format",value_parser=parse_format, default_value = "json")]
    pub format: OutputFormat,
}
impl CmdExector for CsvOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let output = if let Some(output) = self.output {
            output
        } else {
            format!("output.{}", self.format)
        };
        process_csv(&self.input, output, self.format)?;
        Ok(())
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