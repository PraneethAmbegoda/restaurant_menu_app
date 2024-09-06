use actix_web::{web, App, HttpServer};
use restaurant_app::server::{
    handlers, menu_store::InMemoryMenuStore, models::SimpleRestaurant,
    order_store::InMemoryOrderStore, table_store::InMemoryTableStore,
};
use std::sync::Arc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Set up in-memory stores
    let menu_store = Arc::new(InMemoryMenuStore::new());
    let order_store = Arc::new(InMemoryOrderStore::new());
    let table_store = Arc::new(InMemoryTableStore::new());

    // Create the restaurant instance using the SimpleRestaurant implementation
    let restaurant = Arc::new(SimpleRestaurant::new(
        Box::new(Arc::clone(&menu_store)),
        Box::new(Arc::clone(&order_store)),
        Box::new(Arc::clone(&table_store)),
    ));

    // Set up the shared application state
    let app_state = handlers::AppState {
        restaurant: Arc::clone(&restaurant),
    };

    // Start the HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone())) // Share the state with the handlers
            .configure(handlers::configure_routes) // Register routes
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
