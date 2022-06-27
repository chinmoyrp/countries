pub mod csv;
pub mod country;
pub mod handler;

use std::net::SocketAddr;
use std::sync::Arc;
use axum::{Router, Extension, routing::{post}, response::IntoResponse, handler::Handler, http::StatusCode};
use anyhow::Result;
use tower_http::cors::CorsLayer;
use tower::ServiceBuilder;

async fn fallback() -> impl IntoResponse {
     (StatusCode::NOT_FOUND, "404: Not Found!")
}

pub async fn start_server(addr: SocketAddr)  -> Result<()>{
    let countries = csv::read_from_csv("./countries.csv")?;
    let countries = Arc::new(countries);
    
    let cors = CorsLayer::very_permissive();

    let routes = Router::new()
                        .route("/companies", post(handler::countries))
                        .route("/search", post(handler::search))
                        .fallback(fallback.into_service())
                        .layer(Extension(countries))
                        .layer(ServiceBuilder::new().layer(cors));

    axum::Server::try_bind(&addr)?
                 .serve(routes.into_make_service()).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
