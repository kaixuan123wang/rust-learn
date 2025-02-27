use clap::Parser;
use std::path::PathBuf;
use super::verify_path;
use crate::CmdExector;
use crate::process::process_http_server;
use enum_dispatch::enum_dispatch;
#[derive(Debug, Parser)]
#[enum_dispatch(CmdExector)]
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

impl CmdExector for ServerOpts {
    async fn execute(self) -> anyhow::Result<()> {
        process_http_server(self.dir, self.port).await?;
        Ok(())
    }
}
