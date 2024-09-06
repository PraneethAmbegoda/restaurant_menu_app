use crate::server::api::v1::handlers::add_item;
use crate::server::api::v1::handlers::get_item;
use crate::server::api::v1::handlers::get_items;
use crate::server::api::v1::handlers::get_menus;
use crate::server::api::v1::handlers::get_tables;
use crate::server::api::v1::handlers::remove_item;
use actix_web::web;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/api/v1/add_item/{table_id}", web::post().to(add_item))
        .route(
            "/api/v1/remove_item/{table_id}/{item_id}",
            web::delete().to(remove_item),
        )
        .route("/api/v1/get_items/{table_id}", web::get().to(get_items))
        .route(
            "/api/v1/get_item/{table_id}/{item_id}",
            web::get().to(get_item),
        )
        .route("/api/v1/tables", web::get().to(get_tables))
        .route("/api/v1/menus", web::get().to(get_menus));
}
