use clap::Parser;
use std::path::{Path, PathBuf};
use crate::CmdExector;
use crate::process::{process_csv, process_genpass};
use enum_dispatch::enum_dispatch;
mod csv;
mod genpass;
mod base64;
mod text;
mod http;

pub use self::base64::*;
pub use self::csv::*;
pub use self::genpass::*;
pub use self::text::*;
pub use self::http::*;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub command: SubCommand,
}

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExector)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show CSV, or convert CSV to other formats")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "Generate a password")]
    Genpass(GenpassOpts),
    #[command(subcommand, about = "Show base64 encoded text")]
    Base64(Base64Subcommand),
    #[command(subcommand, about = "Show text, or sign text")]
    Text(TextSubcommand),
    #[command(subcommand, about = "Start a HTTP server")]
    Http(HttpSubcommand),
}

fn verify_file(path: &str) -> Result<String, String> {
    if path == "-" || Path::new(path).exists() {
        // 如果输入为-，则认为是标准输入stdin传入
        Ok(path.into())
    } else {
        Err(format!("File {} does not exist", path))
    }
}

pub fn verify_path(path: &str) -> Result<PathBuf, String> {
    let p = Path::new(path);
    if p.exists() && p.is_dir() {
        Ok(path.into())        
    } else {
        Err(format!("File {} is not a directory", path))
    }
}

#[cfg(test)]
mod tests {
    use super::verify_file;
    #[test]
    fn test_verify_file() {
        let result = verify_file("assets/juventus.csv");
        assert_eq!(result, Ok("assets/juventus.csv".into()));
    }

    #[test]
    fn test_verify_file_stdin() {
        let result = verify_file("-");
        assert_eq!(result, Ok("-".into()));
    }

    #[test]
    fn test_verify_file_not_exist() {
        let result = verify_file("not_exist.txt");
        assert_eq!(result, Err("File not_exist.txt does not exist".to_string()));
    }
}




