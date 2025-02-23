use clap::{Parser};
use learn::{Opts, Commands, process_csv, process_genpass, Base64Subcommand, process_base64_encode, process_base64_decode, TextSubcommand, process_text_sign, process_text_verify};

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
        Commands::Base64(subcommand) => {
            match subcommand {
                Base64Subcommand::Encode(opts) => {
                    process_base64_encode(&opts.input, opts.format)?;
                }
                Base64Subcommand::Decode(opts) => {
                    process_base64_decode(&opts.input, opts.format)?;
                }
            }
        }
        Commands::Text(subcommand) => {
            match subcommand {
                TextSubcommand::Sign(opts) => {
                    process_text_sign(&opts.input, &opts.key, opts.format)?;
                }
                TextSubcommand::Verify(opts) => {
                    process_text_verify(&opts.input, &opts.key, opts.format)?;
                }
            }
        }
    }
    Ok(())
}