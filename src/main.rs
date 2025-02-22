use clap::{Parser};
use learn::{Opts, Commands, process_csv, process_genpass};

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
        Commands::Genpass(opts) => {
            process_genpass(opts.length, opts.no_uppercase, opts.no_lowercase, opts.no_numbers, opts.no_symbols)?;
        }
    }
    Ok(())
}