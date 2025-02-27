use clap::Parser;
use crate::CmdExector;
use crate::process::{process_jwt_encode, process_jwt_decode};
use enum_dispatch::enum_dispatch;
use serde::{Serialize, Deserialize};

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExector)]
pub enum JwtSubcommand {
    #[command(about = "Encode a JWT")]
    Encode(JwtEncodeOpts),
    #[command(about = "Decode a JWT")]
    Decode(JwtDecodeOpts),
}


#[derive(Debug, Parser, Serialize, Deserialize)]
pub struct JwtEncodeOpts {
    #[arg(long, help = "The name of the JWT")]
    pub name: String,
    #[arg(long, help = "The age of the JWT")]
    pub age: u8,
}

#[derive(Debug, Parser)]
pub struct JwtDecodeOpts {
    #[arg(long, help = "The JWT to decode")]
    jwt: String,
}

impl CmdExector for JwtEncodeOpts {
    async fn execute(self) -> anyhow::Result<()> {
        process_jwt_encode(self.name, self.age).await?;
        Ok(())
    }
}

impl CmdExector for JwtDecodeOpts {
    async fn execute(self) -> anyhow::Result<()> {
        process_jwt_decode(self.jwt).await?;
        Ok(())
    }
}



