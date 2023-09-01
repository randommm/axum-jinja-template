use axum::{
    extract::State,
    response::{Html, IntoResponse},
};

use minijinja::{context, Environment};

use super::AppError;

pub async fn index(State(env): State<Environment<'static>>) -> Result<impl IntoResponse, AppError> {
    let tmpl = env.get_template("index.html")?;
    let content = tmpl.render(context!())?;
    Ok(Html(content))
}

pub async fn about(State(env): State<Environment<'static>>) -> Result<impl IntoResponse, AppError> {
    let tmpl = env.get_template("about.html")?;
    let content = tmpl.render(context!())?;
    Ok(Html(content))
}
