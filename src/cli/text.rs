
use clap::Parser;
use super::verify_file;
use std::str::FromStr;
use std::fmt;

#[derive(Debug, Parser)]
pub enum TextSubcommand {
    #[command(name = "sign", about = "Sign a message with a private/shared key")]
    Sign(SignOpts),
    #[command(name = "verify", about = "Verify a signed string")]
    Verify(VerifyOpts),
}

#[derive(Debug, Parser)]
pub struct SignOpts {
    #[arg(short, long, help = "Input file", value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(short, long, help = "Private key file", value_parser = verify_file, default_value = "-")]
    pub key: String,
    #[arg(long, default_value = "blake3", value_parser = parse_format)]
    pub format: TextSignFormat,
}

#[derive(Debug, Parser)]
pub struct VerifyOpts {
    #[arg(short, long, help = "Input file", value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(short, long, help = "Public key file", value_parser = verify_file, default_value = "-")]
    pub key: String,
    #[arg(short, long, help = "Signature file")]
    pub sig: String,
    #[arg(long, default_value = "blake3", value_parser = parse_format)]
    pub format: TextSignFormat,
}

#[derive(Debug, Clone, Copy)]
pub enum TextSignFormat {
    Blake3,
    Ed25519,
}
fn parse_format(s: &str) -> Result<TextSignFormat, anyhow::Error> {
    s.parse()
}

impl FromStr for TextSignFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "blake3" => Ok(TextSignFormat::Blake3),
            "ed25519" => Ok(TextSignFormat::Ed25519),
            _ => Err(anyhow::anyhow!("Invalid format: {}", s)),
        }
    }
}
impl fmt::Display for TextSignFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<String>::into(*self))
    }
}
impl From<TextSignFormat> for String {
    fn from(format: TextSignFormat) -> Self {
        match format {
            TextSignFormat::Blake3 => "blake3".to_string(),
            TextSignFormat::Ed25519 => "ed25519".to_string(),
        }
    }
}



