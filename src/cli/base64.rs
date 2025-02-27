use clap::Parser;
use super::verify_file;
use std::str::FromStr;
use std::fmt;
use crate::CmdExector;
use crate::process::process_base64_encode;
use crate::process::process_base64_decode;
use enum_dispatch::enum_dispatch;
#[derive(Debug, Parser)]
#[enum_dispatch(CmdExector)]
pub enum Base64Subcommand {
    #[command(name = "encode", about = "Encode a string to base64")]
    Encode(EncodeOpts),
    #[command(name = "decode", about = "Decode a base64 string")]
    Decode(DecodeOpts),
}

#[derive(Debug, Parser)]
pub struct EncodeOpts {
    #[arg(short, long, help = "Input file", value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(long, help = "Output format", default_value = "standard")]
    pub format: Base64Format,
}

#[derive(Debug, Parser)]
pub struct DecodeOpts {
    #[arg(short, long, help = "Input file", value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(long, help = "Output format", default_value = "standard")]
    pub format: Base64Format,
}

#[derive(Debug, Clone, Copy)]
pub enum Base64Format {
    Standard,
    UrlSafe,
}

impl FromStr for Base64Format {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_lowercase().as_str() {
            "standard" => Base64Format::Standard,
            "urlsafe" => Base64Format::UrlSafe,
            _ => anyhow::bail!("Invalid base64 format: {}", s),
        })
    }
}
impl From<Base64Format> for &str {
    fn from(format: Base64Format) -> Self {
        match format {
            Base64Format::Standard => "standard",
            Base64Format::UrlSafe => "urlsafe",
        }
    }
}
impl fmt::Display for Base64Format {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}

impl CmdExector for EncodeOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let encoded = process_base64_encode(&self.input, self.format)?;
        println!("{}", encoded);
        Ok(())
    }
}

impl CmdExector for DecodeOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let decoded = process_base64_decode(&self.input, self.format)?;
        println!("{}", String::from_utf8(decoded)?);
        Ok(())
    }
}
