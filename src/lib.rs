mod cli;
mod process;
mod utils;
use enum_dispatch::enum_dispatch;
pub use cli::{
    Opts, 
    CsvOpts, 
    GenpassOpts, 
    Base64Subcommand, 
    Base64Format, 
    TextSubcommand, 
    TextSignFormat, 
    HttpSubcommand,
    ServerOpts,
    EncodeOpts,
    DecodeOpts,
    SignOpts,
    VerifyOpts,
    GenerateOpts,
    SubCommand,
    JwtDecodeOpts,
    JwtEncodeOpts,
    JwtSubcommand,
};
pub use process::*;
pub use utils::*;

#[allow(async_fn_in_trait)]
#[enum_dispatch]
pub trait CmdExector {
    async fn execute(self) -> anyhow::Result<()>;
}