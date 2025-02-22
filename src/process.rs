use serde::{Serialize, Deserialize};
use csv::Reader;
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Player {
    pub name: String,
    pub position: String,
    pub nationality: String,
    #[serde(rename = "DOB")]
    pub dob: String,
    #[serde(rename = "Kit Number")]
    pub kit: u8,
}

pub fn process_csv(input: &str, output: &str) -> anyhow::Result<()> {
    let mut reader = Reader::from_path(input)?;
    let mut ret = Vec::with_capacity(128); // 预分配内存，避免频繁扩容
    for result in reader.deserialize() {
        let player: Player = result?;
        ret.push(player);
    }
    let json = serde_json::to_string_pretty(&ret)?;
    fs::write(output, json)?;
    Ok(())
}
