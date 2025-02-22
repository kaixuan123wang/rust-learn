use clap::{Parser};
use learn::{Opts, Commands, process_csv, OutputFormat};

fn main() -> anyhow::Result<()> {
    let args = Opts::parse();
    match args.command {
        Commands::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output.clone()
            } else {
                format!("output.{}", opts.format)
            };
            process_csv(&opts.input, output, opts.format)?;
        }
        
    }
    Ok(())
}