use clap::Parser;
use std::path::Path;
mod csv;
mod genpass;
mod base64;
pub use self::csv::{CsvOpts, OutputFormat};
pub use self::genpass::GenpassOpts;
pub use self::base64::Base64Subcommand;
pub use self::base64::Base64Format;

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
    #[command(name = "genpass", about = "Generate a password")]
    Genpass(GenpassOpts),
    #[command(subcommand)]
    Base64(Base64Subcommand),
}

fn verify_input_file(path: &str) -> Result<String, String> {
    if path == "-" || Path::new(path).exists() {
        // 如果输入为-，则认为是标准输入stdin传入
        Ok(path.into())
    } else {
        Err(format!("File {} does not exist", path))
    }
}

#[cfg(test)]
mod tests {
    use super::verify_input_file;
    #[test]
    fn test_verify_input_file() {
        let result = verify_input_file("assets/juventus.csv");
        assert_eq!(result, Ok("assets/juventus.csv".into()));
    }

    #[test]
    fn test_verify_input_file_stdin() {
        let result = verify_input_file("-");
        assert_eq!(result, Ok("-".into()));
    }

    #[test]
    fn test_verify_input_file_not_exist() {
        let result = verify_input_file("not_exist.txt");
        assert_eq!(result, Err("File not_exist.txt does not exist".to_string()));
    }
}




