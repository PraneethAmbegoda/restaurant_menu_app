use crate::server::api::v1;
use crate::server::data_model::models;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

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
    components(schemas(models::MenuItem)),
    tags(
        (name = "Restaurant API", description = "API for managing restaurant orders and menu items")
    )
)]
pub struct ApiDoc;

/// Function to serve OpenAPI documentation via Swagger UI.
pub fn configure_openapi_ui() -> SwaggerUi {
    SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-doc/openapi.json", ApiDoc::openapi())
}
