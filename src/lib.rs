mod routes;
use tokio::net::TcpListener;

pub async fn run() -> Result<(), String> {
    let app = routes::create_routes().await?;
    let bind_addr = &"0.0.0.0:3511";
    let listener = TcpListener::bind(bind_addr)
        .await
        .map_err(|e| format!("Failed to parse address: {}", e))?;
    axum::serve(listener, app.into_make_service())
        .await
        .map_err(|e| format!("Server error: {}", e))?;
    Ok(())
}
