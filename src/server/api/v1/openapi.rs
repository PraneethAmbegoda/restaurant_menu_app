#![deny(warnings)]
#![deny(clippy::all)]

use crate::server::api::v1;
use crate::server::data_model::models;
use crate::server::data_model::models::MenuItem;
use serde::Serialize;
use utoipa::OpenApi;
use utoipa::ToSchema;
use utoipa_swagger_ui::SwaggerUi;

/// Success response with a message
#[derive(Serialize, ToSchema)]
pub struct SuccessResponseMessage {
    pub status: String,
    pub message: String,
}

/// Success response with a list of menu items
#[derive(Serialize, ToSchema)]
pub struct SuccessResponseMenuItems {
    pub status: String,
    pub data: Vec<MenuItem>,
}

/// Success response with a menu item
#[derive(Serialize, ToSchema)]
pub struct SuccessResponseMenuItem {
    pub status: String,
    pub data: MenuItem,
}

/// Success response with a list of tables
#[derive(Serialize, ToSchema)]
pub struct SuccessResponseTables {
    pub status: String,
    pub data: Vec<u32>,
}

/// Structure for error responses.
#[derive(Serialize, ToSchema)]
pub struct ErrorResponse {
    pub status: String,
    pub message: String,
}

/// This struct serves as the OpenAPI entry point. It collects all the routes and schemas.
#[derive(OpenApi)]
#[openapi(
    paths(
        v1::handlers::add_item,
        v1::handlers::remove_item,
        v1::handlers::get_items,
        v1::handlers::get_item,
        v1::handlers::get_tables,
        v1::handlers::get_menus,
    ),
    components(schemas(
        models::MenuItem,
        SuccessResponseMessage,
        SuccessResponseMenuItems,
        SuccessResponseMenuItem,
        SuccessResponseTables,
        ErrorResponse
    )),
    tags(
        (name = "Restaurant API", description = "API for managing restaurant orders and menu items")
    )
)]
pub struct ApiDoc;

/// Function to serve OpenAPI documentation via Swagger UI.
pub fn configure_openapi_ui() -> SwaggerUi {
    SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-doc/openapi.json", ApiDoc::openapi())
}
