use base64::prelude::BASE64_STANDARD;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use crate::Base64Format;
use crate::get_reader;

pub fn process_base64_encode(input: &str, format: Base64Format) -> anyhow::Result<String> {
    let mut reader =  get_reader(&input)?;

    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;

    let result = match format {
        Base64Format::Standard => BASE64_STANDARD.encode(buf),
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.encode(buf),
    };
    Ok(result)
}

pub fn process_base64_decode(input: &str, format: Base64Format) -> anyhow::Result<Vec<u8>> {
    let mut reader =  get_reader(&input)?;

    let mut buf = String::new();   
    reader.read_to_string(&mut buf)?;
    let buf = buf.trim();

    let decode = match format {
        Base64Format::Standard => BASE64_STANDARD.decode(buf)?,
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.decode(buf)?,
    };
    Ok(decode)
}
