mod routes;

pub async fn run() -> Result<(), String> {
    let app = routes::create_routes().await?;
    let bind_addr = &"0.0.0.0:3511"
        .parse()
        .map_err(|e| format!("Failed to parse address: {}", e))?;
    axum::Server::bind(bind_addr)
        .serve(app.into_make_service())
        .await
        .map_err(|e| format!("Server error: {}", e))
}
