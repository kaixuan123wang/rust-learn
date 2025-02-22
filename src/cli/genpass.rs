use clap::Parser;


#[derive(Debug, Parser)]
pub struct GenpassOpts {
    #[arg(short, long, help = "Length of the password", default_value_t = 16)]
    pub length: u8,
    #[arg(long, help = "No uppercase", default_value_t = false)]
    pub no_uppercase: bool,
    #[arg(long, help = "No lowercase", default_value_t = false)]
    pub no_lowercase: bool,
    #[arg(long, help = "No numbers", default_value_t = false)]
    pub no_numbers: bool,
    #[arg(long, help = "No symbols", default_value_t = false)]
    pub no_symbols: bool
}