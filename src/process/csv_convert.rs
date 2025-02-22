use csv::Reader;
use std::fs;
use serde_json::Value;
use crate::opts::OutputFormat;
use std::collections::BTreeMap;
pub fn process_csv(input: &str, output: String, format: OutputFormat) -> anyhow::Result<()> {
    let mut reader = Reader::from_path(input)?;
    let mut ret = Vec::with_capacity(128); // 预分配内存，避免频繁扩容
    let headers = reader.headers()?.clone();
    for result in reader.records() {
        let record = result?;
        let mut map = BTreeMap::new();
        let iter = headers.iter().zip(record.iter());
        for (header, value) in iter {
            map.insert(header.to_string(), value.to_string());
        }
        ret.push(map);
    }
    let output_str = match format {
        OutputFormat::Json => serde_json::to_string(&ret)?,
        OutputFormat::Yaml => serde_yaml::to_string(&ret)?,
        OutputFormat::Toml => {
            let wrapper = BTreeMap::from([("data".to_string(), ret)]);
            toml::to_string(&wrapper)?
        },
    };
    fs::write(output, output_str)?;
    Ok(())
}
