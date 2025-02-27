use std::{net::SocketAddr, path::{PathBuf}, fs};
use tracing::{info, warn};
use axum::{routing::get, Router, extract::{State, Path}};
use tokio::net::TcpListener;
use std::sync::Arc;
use tokio::fs::File;
use tokio::io::BufReader;
use tokio::io::AsyncReadExt;
use axum::http::StatusCode;
use tower_http::services::ServeDir;
#[derive(Debug)]
pub struct HttpServeState {
    path: PathBuf,
}

pub async fn process_http_server(path: PathBuf, port: u16) -> anyhow::Result<()> {
    info!("Starting {:?} on port {}", path, port);
    let state = HttpServeState { path: path.clone() };
    let addr: SocketAddr = SocketAddr::from(([0, 0, 0, 0], port));
    let router = Router::new()
        .route("/{*path}", get(file_handler))
        .nest_service("/tower", ServeDir::new(path))
        .with_state(Arc::new(state));
    
    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, router).await?;
    Ok(())
}

async fn file_handler(
    State(state): State<Arc<HttpServeState>>,
    Path(path): Path<String>,
) -> Result<String, StatusCode> {
    let file_path = state.path.join(path);
    if file_path.is_dir() {
        let files = fs::read_dir(&file_path).map_err(|_| StatusCode::NOT_FOUND)?;
        let mut file_list = String::new();
        for file in files {
            let file = file.unwrap();
            file_list.push_str(&format!("<a href=\"{}\">{}</a><br> <br>", file.path().display(), file.path().file_name().unwrap().to_str().unwrap()));
        }
        return Ok(file_list);
    } else {
        let file = File::open(&file_path).await;
    if let Err(e) = file {
        warn!("File not found: {:?}", &file_path);
        return Err(StatusCode::NOT_FOUND);
    }
        let mut reader = BufReader::new(file.unwrap());
        let mut content = String::new();
        reader.read_to_string(&mut content).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        Ok(content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_file_handler() {
        let state = HttpServeState { path: PathBuf::from(".") };
        let path = String::from("Cargo.toml");
        let result = file_handler(State(Arc::new(state)), Path(path)).await;
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap().trim().starts_with("[package]"), true);
    }
}
