use clap::{Parser};
use learn::{
    Opts, 
    Commands, 
    process_csv, 
    process_genpass, 
    Base64Subcommand, 
    process_base64_encode, 
    process_base64_decode, 
    TextSubcommand, 
    process_text_sign, 
    process_text_verify, 
    process_text_generate, 
    TextSignFormat, 
    HttpSubcommand,
    process_http_server,
};
use zxcvbn::zxcvbn;
use std::fs;

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
            let password = process_genpass(opts.length, opts.no_uppercase, opts.no_lowercase, opts.no_numbers, opts.no_symbols)?;
            let strength = zxcvbn(&password, &[]);
            println!("{}", password);
            println!("{}", strength.score());
        }
        Commands::Base64(subcommand) => {
            match subcommand {
                Base64Subcommand::Encode(opts) => {
                    let encoded = process_base64_encode(&opts.input, opts.format)?;
                    println!("{}", encoded);
                }
                Base64Subcommand::Decode(opts) => {
                    let decode = process_base64_decode(&opts.input, opts.format)?;
                    let decoded = String::from_utf8(decode)?;
                    println!("{}", decoded);
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
                TextSubcommand::Generate(opts) => {
                    let key = process_text_generate(&opts.format)?;
                    match opts.format {
                        TextSignFormat::Blake3 => {
                            let name = opts.output.join("blake3.key");
                            fs::write(name, &key[0])?;
                        }
                        TextSignFormat::Ed25519 => {
                            let name = opts.output.join("ed25519.key");
                            fs::write(name, &key[0])?;
                            let name = opts.output.join("ed25519.pub");
                            fs::write(name, &key[1])?;
                        }
                    }
                }
            }
        }
        Commands::Http(subcommand) => {
            match subcommand {
                HttpSubcommand::Serve(opts) => {
                    process_http_server(opts.dir, opts.port)?;
                }
            }
        }
    }
    Ok(())
}