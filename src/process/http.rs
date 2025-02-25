use std::path::PathBuf;

pub fn process_http_server(path: PathBuf, port: u16) -> anyhow::Result<()> {
    println!("path: {:?}", path);
    println!("port: {}", port);
    Ok(())
}