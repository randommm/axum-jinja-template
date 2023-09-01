use axum_jinja_template::run;

#[tokio::main]
async fn main() -> Result<(), String> {
    run().await
}
