mod error_handling;
mod pages;

use error_handling::AppError;
use pages::{about, index};

use minijinja::Environment;

use axum::{extract::FromRef, routing::get, Router};

#[derive(Clone, FromRef)]
pub struct AppState {
    pub env: Environment<'static>,
}

pub async fn create_routes() -> Result<Router, String> {
    let mut env = Environment::new();
    env.add_template("layout.html", include_str!("../templates/layout.html"))
        .map_err(|e| format!("Failed to add layout.html: {}", e))?;

    env.add_template("index.html", include_str!("../templates/index.html"))
        .map_err(|e| format!("Failed to add index.html: {}", e))?;

    env.add_template("about.html", include_str!("../templates/about.html"))
        .map_err(|e| format!("Failed to add about.html: {}", e))?;

    let app_state = AppState { env };

    Ok(Router::new()
        .route("/", get(index))
        .route("/about", get(about))
        .with_state(app_state))
}
