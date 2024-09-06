#![deny(warnings)]
#![deny(clippy::all)]

use restaurant_menu_app::server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Call the async main function from the server module
    server::main::main().await
}
