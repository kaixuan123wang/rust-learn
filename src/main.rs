use clap::Parser;
use learn::Opts;
use learn::CmdExector;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let args = Opts::parse();
    args.command.execute().await?;
    Ok(())
}