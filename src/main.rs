use clap::{Parser};
use learn::{Opts, Commands, process_csv};



fn main() -> anyhow::Result<()> {
    let args = Opts::parse();
    match args.command {
        Commands::Csv(opts) => process_csv(&opts.input, &opts.output)?,
        
    }
    Ok(())
}