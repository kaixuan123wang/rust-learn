use clap::Parser;
use std::path::PathBuf;
use super::verify_path;

#[derive(Debug, Parser)]
pub enum HttpSubcommand {
    #[command(about = "Get a resource")]
    Serve(ServerOpts),
}

#[derive(Debug, Parser)]
pub struct ServerOpts {
    #[arg(short, long, help = "Server address", value_parser = verify_path, default_value = ".")]
    pub dir: PathBuf,
    #[arg(short, long, help = "Server port", default_value_t = 8080)]
    pub port: u16,
}