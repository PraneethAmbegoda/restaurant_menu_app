#![deny(warnings)]
#![deny(clippy::all)]

use crate::server::api::v1;
use crate::server::data_model::models;
use crate::server::data_model::models::MenuItem;
use serde::Serialize;
use utoipa::OpenApi;
use utoipa::ToSchema;
use utoipa_swagger_ui::SwaggerUi;

/// Struct representing a success response containing a message.
///
/// This struct is used for API responses that only contain a success message
/// and no additional data, such as the result of a POST or DELETE request.
#[derive(Serialize, ToSchema)]
pub struct SuccessResponseMessage {
    /// Status of the response, typically "ok" for success.
    pub status: String,
    /// The message detailing the result of the request.
    pub message: String,
}

/// Struct representing a success response with a list of menu items.
///
/// This is used in API responses that return a list of `MenuItem`s, such as a request
/// for all available menu items.
#[derive(Serialize, ToSchema)]
pub struct SuccessResponseMenuItems {
    /// Status of the response, typically "ok" for success.
    pub status: String,
    /// A list of menu items returned by the request.
    pub data: Vec<MenuItem>,
}

/// Struct representing a success response with a single menu item.
///
/// This is used in API responses that return a single `MenuItem`, such as a request
/// for a specific menu item.
#[derive(Serialize, ToSchema)]
pub struct SuccessResponseMenuItem {
    /// Status of the response, typically "ok" for success.
    pub status: String,
    /// The specific menu item returned by the request.
    pub data: MenuItem,
}

/// Struct representing a success response with a list of tables.
///
/// This is used in API responses that return a list of available tables
/// in the restaurant.
#[derive(Serialize, ToSchema)]
pub struct SuccessResponseTables {
    /// Status of the response, typically "ok" for success.
    pub status: String,
    /// A list of table IDs available in the restaurant.
    pub data: Vec<u32>,
}

/// Struct representing an error response.
///
/// This is used in API responses where an error occurred,
/// such as a failed request due to invalid data.
#[derive(Serialize, ToSchema)]
pub struct ErrorResponse {
    /// Status of the response, typically "error" for error cases.
    pub status: String,
    /// The error message detailing the issue.
    pub message: String,
}

/// Struct representing the OpenAPI documentation entry point.
///
/// This struct collects all the API routes and schemas to generate OpenAPI
/// documentation for the restaurant API.
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

/// Configures and serves the OpenAPI documentation via Swagger UI.
///
/// This function sets up Swagger UI at the `/swagger-ui` endpoint,
/// which provides a user interface for exploring and interacting with
/// the restaurant API's documentation.
pub fn configure_openapi_ui() -> SwaggerUi {
    SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-doc/openapi.json", ApiDoc::openapi())
}
