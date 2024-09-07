use crate::server::api::v1;
use crate::server::api::v1::handlers;
use crate::server::in_memory_menu_store::InMemoryMenuStore;
use crate::server::in_memory_order_store::InMemoryOrderStore;
use crate::server::in_memory_table_store::InMemoryTableStore;
use crate::server::main::v1::openapi;
use crate::server::models::Restaurant;
use crate::server::restaurant::SimpleRestaurant;
use actix_web::{web, App, HttpServer};
use std::sync::Arc;

pub async fn main(port: Option<u16>) -> std::io::Result<()> {
    // Default to port 8081 if no port is provided
    let port = port.unwrap_or(8081);

    // Create the restaurant instance using the SimpleRestaurant implementation
    let restaurant = Arc::new(SimpleRestaurant::new(
        Box::new(InMemoryMenuStore::default()),  // Using Default trait
        Box::new(InMemoryOrderStore::default()), // Using Default trait
        Box::new(InMemoryTableStore::default()), // Using Default trait
    ));

    // Set up the shared application state
    let app_state = handlers::AppState {
        restaurant: restaurant as Arc<dyn Restaurant + Send + Sync>, // Coerce the type to the trait object
    };

    // Start the HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone())) // Share the state with the handlers
            .configure(v1::routes::configure_routes) // Register routes
            .service(openapi::configure_openapi_ui()) // Serve OpenAPI docs via Swagger UI
    })
    .bind(format!("127.0.0.1:{}", port))?
    .run()
    .await
}
