#![deny(warnings)]
#![deny(clippy::all)]

#[allow(unused_imports)]
use crate::server::api::v1::openapi::{
    ErrorResponse, SuccessResponseMenuItem, SuccessResponseMenuItems, SuccessResponseMessage,
    SuccessResponseTables,
};
use crate::server::data_model::models::Restaurant;
use crate::server::utils::param_validation::parse_path_param;
use crate::server::utils::response::restaurant_error_to_response;
use crate::server::utils::response::{error_response, success_message_response, success_response};

use actix_web::{web, Responder};
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub restaurant: Arc<dyn Restaurant + Send + Sync>,
}

/// Adds a menu item to the specified table.
///
/// # Arguments
///
/// * `data` - Application state that contains the restaurant.
/// * `params` - Path parameters containing the table ID and menu item ID.
///
/// # Responses
///
/// * `200` - Menu item added successfully.
/// * `404` - Table or menu item not found.
/// * `400` - Bad request.
/// * `500` - Internal server error.
#[utoipa::path(
    post,
    path = "/api/v1/add_item/{table_id}/{menu_item_id}",
    responses(
        (status = 200, description = "Menu item added successfully", body = SuccessResponseMessage),
        (status = 404, description = "Table or menu item not found", body = ErrorResponse),
        (status = 400, description = "Bad request", body = ErrorResponse),
        (status = 500, description = "Internal server error")
    ),
    params(
        ("table_id" = u32, description = "ID of the table"),
        ("menu_item_id" = u32, description = "ID of the menu item")
    )
)]
pub async fn add_item(
    data: web::Data<AppState>,
    params: web::Path<(String, String)>,
) -> impl Responder {
    let restaurant = &data.restaurant;
    let table_id = match parse_path_param(&params.0, "table ID") {
        Ok(id) => id,
        Err(e) => return error_response(400, &e), // Return the error response if validation fails
    };

    let item_id = match parse_path_param(&params.1, "item ID") {
        Ok(id) => id,
        Err(e) => return error_response(400, &e), // Return the error response if validation fails
    };
    match restaurant.add_item(table_id, item_id) {
        Ok(_) => success_message_response(&format!(
            "Menu item with item id: {} added successfully for table with table id {}",
            item_id, table_id
        )),
        Err(e) => restaurant_error_to_response(e),
    }
}

/// Removes a menu item from the specified table.
///
/// # Arguments
///
/// * `data` - Application state that contains the restaurant.
/// * `params` - Path parameters containing the table ID and menu item ID.
///
/// # Responses
///
/// * `200` - Menu item removed successfully.
/// * `404` - Table or menu item not found.
/// * `400` - Bad request.
/// * `500` - Internal server error.
#[utoipa::path(
    delete,
    path = "/api/v1/remove_item/{table_id}/{menu_item_id}",
    responses(
        (status = 200, description = "Menu item removed successfully", body = SuccessResponseMessage),
        (status = 404, description = "Table or menu item not found", body = ErrorResponse),
        (status = 400, description = "Bad request", body = ErrorResponse),
        (status = 500, description = "Internal server error")
    ),
    params(
        ("table_id" = u32, description = "ID of the table"),
        ("menu_item_id" = u32, description = "ID of the menu item to remove")
    )
)]
pub async fn remove_item(
    data: web::Data<AppState>,
    params: web::Path<(String, String)>,
) -> impl Responder {
    let table_id = match parse_path_param(&params.0, "table ID") {
        Ok(id) => id,
        Err(e) => return error_response(400, &e), // Return the error response if validation fails
    };

    let item_id = match parse_path_param(&params.1, "item ID") {
        Ok(id) => id,
        Err(e) => return error_response(400, &e), // Return the error response if validation fails
    };
    let restaurant = &data.restaurant;
    match restaurant.remove_item(table_id, item_id) {
        Ok(_) => success_message_response(&format!(
            "Menu item with item id:{} removed from table with table id:{} successfully",
            item_id, table_id
        )),
        Err(e) => restaurant_error_to_response(e),
    }
}

/// Retrieves all menu items added to the specified table.
///
/// # Arguments
///
/// * `data` - Application state that contains the restaurant.
/// * `table_id` - Path parameter containing the table ID.
///
/// # Responses
///
/// * `200` - List of menu items added for the table.
/// * `404` - Table not found or no menu items added to the table.
/// * `400` - Bad request.
/// * `500` - Internal server error.
#[utoipa::path(
    get,
    path = "/api/v1/get_items/{table_id}",
    responses(
        (status = 200, description = "List of menu items added for the table", body = SuccessResponseMenuItems),
        (status = 404, description = "Table not found or no menu items added to the table", body = ErrorResponse),
        (status = 400, description = "Bad request", body = ErrorResponse),
        (status = 500, description = "Internal server error")
    ),
    params(
        ("table_id" = u32, description = "ID of the table")
    )
)]
pub async fn get_items(data: web::Data<AppState>, table_id: web::Path<String>) -> impl Responder {
    let restaurant = &data.restaurant;
    let table_id = match parse_path_param(&table_id, "table ID") {
        Ok(id) => id,
        Err(e) => return error_response(400, &e), // Return the error response if validation fails
    };
    match restaurant.get_items(table_id) {
        Ok(items) => success_response(items),
        Err(e) => restaurant_error_to_response(e),
    }
}

/// Retrieves details of a specific menu item added to the specified table.
///
/// # Arguments
///
/// * `data` - Application state that contains the restaurant.
/// * `params` - Path parameters containing the table ID and menu item ID.
///
/// # Responses
///
/// * `200` - Menu item details.
/// * `404` - Table or menu item not found or the menu item not added to the table.
/// * `400` - Bad Request.
/// * `500` - Internal server error.
#[utoipa::path(
    get,
    path = "/api/v1/get_item/{table_id}/{menu_item_id}",
    responses(
        (status = 200, description = "Menu item details", body = SuccessResponseMenuItem),
        (status = 404, description = "Table or menu item not found or menu item not added to the table", body = ErrorResponse),
        (status = 400, description = "Bad request", body = ErrorResponse),
        (status = 500, description = "Internal server error")
    ),
    params(
        ("table_id" = u32, description = "ID of the table"),
        ("menu_item_id" = u32, description = "ID of the menu item")
    )
)]
pub async fn get_item(
    data: web::Data<AppState>,
    params: web::Path<(String, String)>,
) -> impl Responder {
    let restaurant = &data.restaurant;
    let table_id = match parse_path_param(&params.0, "table ID") {
        Ok(id) => id,
        Err(e) => return error_response(400, &e), // Return the error response if validation fails
    };

    let item_id = match parse_path_param(&params.1, "item ID") {
        Ok(id) => id,
        Err(e) => return error_response(400, &e), // Return the error response if validation fails
    };
    match restaurant.get_item(table_id, item_id) {
        Ok(item) => success_response(item),
        Err(e) => restaurant_error_to_response(e),
    }
}

/// Retrieves a list of all available tables in the restaurant.
///
/// # Arguments
///
/// * `data` - Application state that contains the restaurant.
///
/// # Responses
///
/// * `200` - List of available tables.
/// * `500` - Internal server error.
#[utoipa::path(
    get,
    path = "/api/v1/tables",
    responses(
        (status = 200, description = "List of available tables", body = SuccessResponseTables),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_tables(data: web::Data<AppState>) -> impl Responder {
    let restaurant = &data.restaurant;
    match restaurant.get_all_tables() {
        Ok(tables) => success_response(tables),
        Err(e) => restaurant_error_to_response(e),
    }
}

/// Retrieves a list of all available menu items in the restaurant.
///
/// # Arguments
///
/// * `data` - Application state that contains the restaurant.
///
/// # Responses
///
/// * `200` - List of available menus.
/// * `500` - Internal server error.
#[utoipa::path(
    get,
    path = "/api/v1/menus",
    responses(
        (status = 200, description = "List of available menus", body = SuccessResponseMenuItems),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_menus(data: web::Data<AppState>) -> impl Responder {
    let restaurant = &data.restaurant;
    match restaurant.get_all_menus() {
        Ok(menus) => success_response(menus),
        Err(e) => restaurant_error_to_response(e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::server::api::v1::routes::configure_routes;
    use crate::server::data_model::models::{
        MenuItem, MockMenuStore, MockOrderStore, MockTableStore,
    };
    use crate::server::restaurant::SimpleRestaurant;
    use actix_web::{http::StatusCode, test, web, App};
    use mockall::predicate::*;
    use serde_json::Value;

    #[actix_rt::test]
    async fn test_add_item_success() {
        let mut mock_table_store = MockTableStore::new();
        let mut mock_order_store = MockOrderStore::new();
        let mut mock_menu_store = MockMenuStore::new();

        mock_table_store
            .expect_get_all_tables()
            .returning(|| Ok(vec![1, 2, 3]));

        mock_menu_store.expect_get_all_menus().returning(|| {
            Ok(vec![MenuItem {
                id: 1,
                name: "Burger".to_string(),
                cooking_time: 10,
            }])
        });

        mock_order_store
            .expect_add_item()
            .with(eq(1), eq(1))
            .returning(|_, _| Ok(()));

        let restaurant = Arc::new(SimpleRestaurant {
            table_store: Box::new(mock_table_store),
            order_store: Box::new(mock_order_store),
            menu_store: Box::new(mock_menu_store),
        });

        let app_state = AppState { restaurant };
        let mut app = test::init_service(
            App::new()
                .app_data(web::Data::new(app_state))
                .configure(configure_routes),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/api/v1/add_item/1/1")
            .to_request();
        let resp = test::call_service(&mut app, req).await;

        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn test_add_item_table_not_found() {
        let mut mock_table_store = MockTableStore::new();
        let mut mock_menu_store = MockMenuStore::new();
        let mut mock_order_store = MockOrderStore::new();

        mock_table_store
            .expect_get_all_tables()
            .returning(|| Ok(vec![2, 3])); // Table 1 doesn't exist

        mock_menu_store.expect_get_all_menus().returning(|| {
            Ok(vec![MenuItem {
                id: 1,
                name: "Burger".to_string(),
                cooking_time: 10,
            }])
        });

        // Default expectation for add_item (won't be called)
        mock_order_store.expect_add_item().returning(|_, _| Ok(()));

        let restaurant = Arc::new(SimpleRestaurant {
            table_store: Box::new(mock_table_store),
            order_store: Box::new(mock_order_store),
            menu_store: Box::new(mock_menu_store),
        });

        let app_state = AppState { restaurant };
        let mut app = test::init_service(
            App::new()
                .app_data(web::Data::new(app_state))
                .configure(configure_routes),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/api/v1/add_item/1/1")
            .to_request();
        let resp = test::call_service(&mut app, req).await;

        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[actix_rt::test]
    async fn test_add_item_menu_item_not_found() {
        let mut mock_table_store = MockTableStore::new();
        let mut mock_menu_store = MockMenuStore::new();
        let mut mock_order_store = MockOrderStore::new();

        mock_table_store
            .expect_get_all_tables()
            .returning(|| Ok(vec![1, 2, 3])); // Table 1 doesn't exist

        mock_menu_store.expect_get_all_menus().returning(|| {
            Ok(vec![MenuItem {
                id: 10,
                name: "Burger".to_string(),
                cooking_time: 10,
            }])
        });

        // Default expectation for add_item (won't be called)
        mock_order_store.expect_add_item().returning(|_, _| Ok(()));

        let restaurant = Arc::new(SimpleRestaurant {
            table_store: Box::new(mock_table_store),
            order_store: Box::new(mock_order_store),
            menu_store: Box::new(mock_menu_store),
        });

        let app_state = AppState { restaurant };
        let mut app = test::init_service(
            App::new()
                .app_data(web::Data::new(app_state))
                .configure(configure_routes),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/api/v1/add_item/1/1")
            .to_request();
        let resp = test::call_service(&mut app, req).await;

        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[actix_rt::test]
    async fn test_remove_item_success() {
        let mut mock_table_store = MockTableStore::new();
        let mut mock_order_store = MockOrderStore::new();

        mock_table_store
            .expect_get_all_tables()
            .returning(|| Ok(vec![1, 2, 3]));

        mock_order_store
            .expect_remove_item()
            .with(eq(1), eq(1))
            .returning(|_, _| Ok(()));

        let restaurant = Arc::new(SimpleRestaurant {
            table_store: Box::new(mock_table_store),
            order_store: Box::new(mock_order_store),
            menu_store: Box::new(MockMenuStore::new()),
        });

        let app_state = AppState { restaurant };
        let mut app = test::init_service(
            App::new()
                .app_data(web::Data::new(app_state))
                .configure(configure_routes),
        )
        .await;

        let req = test::TestRequest::delete()
            .uri("/api/v1/remove_item/1/1")
            .to_request();
        let resp = test::call_service(&mut app, req).await;

        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn test_remove_item_table_not_found() {
        let mut mock_table_store = MockTableStore::new();
        let mut mock_order_store = MockOrderStore::new();

        mock_table_store
            .expect_get_all_tables()
            .returning(|| Ok(vec![2, 3])); // Table 1 doesn't exist

        // Default expectation for remove_item (won't be called)
        mock_order_store
            .expect_remove_item()
            .returning(|_, _| Ok(()));

        let restaurant = Arc::new(SimpleRestaurant {
            table_store: Box::new(mock_table_store),
            order_store: Box::new(mock_order_store),
            menu_store: Box::new(MockMenuStore::new()),
        });

        let app_state = AppState { restaurant };
        let mut app = test::init_service(
            App::new()
                .app_data(web::Data::new(app_state))
                .configure(configure_routes),
        )
        .await;

        let req = test::TestRequest::delete()
            .uri("/api/v1/remove_item/1/1")
            .to_request();
        let resp = test::call_service(&mut app, req).await;

        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[actix_rt::test]
    async fn test_get_items_success() {
        let mut mock_table_store = MockTableStore::new();
        let mut mock_order_store = MockOrderStore::new();
        let mut mock_menu_store = MockMenuStore::new();

        mock_table_store
            .expect_get_all_tables()
            .returning(|| Ok(vec![1, 2, 3]));

        mock_order_store
            .expect_get_item_ids()
            .with(eq(1))
            .returning(|_| Ok(vec![1]));

        mock_menu_store.expect_get_all_menus().returning(|| {
            Ok(vec![
                MenuItem {
                    id: 1,
                    name: "Burger".to_string(),
                    cooking_time: 10,
                },
                MenuItem {
                    id: 2,
                    name: "Pizza".to_string(),
                    cooking_time: 15,
                },
            ])
        });

        let restaurant = Arc::new(SimpleRestaurant {
            table_store: Box::new(mock_table_store),
            order_store: Box::new(mock_order_store),
            menu_store: Box::new(mock_menu_store),
        });

        let app_state = AppState { restaurant };
        let mut app = test::init_service(
            App::new()
                .app_data(web::Data::new(app_state))
                .configure(configure_routes),
        )
        .await;

        let req = test::TestRequest::get()
            .uri("/api/v1/get_items/1")
            .to_request();
        let resp = test::call_service(&mut app, req).await;

        assert_eq!(resp.status(), StatusCode::OK);

        let json_response: Value = test::read_body_json(resp).await;
        let items: Vec<MenuItem> = serde_json::from_value(json_response["data"].clone()).unwrap();

        assert_eq!(items.len(), 1);
        assert_eq!(items[0].id, 1);
        assert_eq!(items[0].name, "Burger");
    }

    #[actix_rt::test]
    async fn test_get_items_table_not_found() {
        let mut mock_table_store = MockTableStore::new();
        let mut mock_order_store = MockOrderStore::new();
        let mut mock_menu_store = MockMenuStore::new();

        mock_table_store
            .expect_get_all_tables()
            .returning(|| Ok(vec![2, 3])); // Table 1 doesn't exist

        // Default expectation for get_item_ids (won't be called)
        mock_order_store
            .expect_get_item_ids()
            .returning(|_| Ok(vec![]));

        // Default expectation for get_all_menus (won't be called)
        mock_menu_store
            .expect_get_all_menus()
            .returning(|| Ok(vec![]));

        let restaurant = Arc::new(SimpleRestaurant {
            table_store: Box::new(mock_table_store),
            order_store: Box::new(mock_order_store),
            menu_store: Box::new(mock_menu_store),
        });

        let app_state = AppState { restaurant };
        let mut app = test::init_service(
            App::new()
                .app_data(web::Data::new(app_state))
                .configure(configure_routes),
        )
        .await;

        let req = test::TestRequest::get()
            .uri("/api/v1/get_items/1")
            .to_request();
        let resp = test::call_service(&mut app, req).await;

        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[actix_rt::test]
    async fn test_get_item_success() {
        let mut mock_table_store = MockTableStore::new();
        let mut mock_order_store = MockOrderStore::new();
        let mut mock_menu_store = MockMenuStore::new();

        mock_table_store
            .expect_get_all_tables()
            .returning(|| Ok(vec![1, 2, 3]));

        mock_order_store
            .expect_get_item_id()
            .with(eq(1), eq(1))
            .returning(|_, _| Ok(1));

        mock_menu_store.expect_get_all_menus().returning(|| {
            Ok(vec![
                MenuItem {
                    id: 1,
                    name: "Burger".to_string(),
                    cooking_time: 10,
                },
                MenuItem {
                    id: 2,
                    name: "Pizza".to_string(),
                    cooking_time: 15,
                },
            ])
        });

        let restaurant = Arc::new(SimpleRestaurant {
            table_store: Box::new(mock_table_store),
            order_store: Box::new(mock_order_store),
            menu_store: Box::new(mock_menu_store),
        });

        let app_state = AppState { restaurant };
        let mut app = test::init_service(
            App::new()
                .app_data(web::Data::new(app_state))
                .configure(configure_routes),
        )
        .await;

        let req = test::TestRequest::get()
            .uri("/api/v1/get_item/1/1")
            .to_request();
        let resp = test::call_service(&mut app, req).await;

        assert_eq!(resp.status(), StatusCode::OK);

        let json_response: Value = test::read_body_json(resp).await;
        let item: MenuItem = serde_json::from_value(json_response["data"].clone()).unwrap();

        assert_eq!(item.id, 1);
        assert_eq!(item.name, "Burger");
    }

    #[actix_rt::test]
    async fn test_get_item_table_not_found() {
        let mut mock_table_store = MockTableStore::new();
        let mut mock_order_store = MockOrderStore::new();
        let mut mock_menu_store = MockMenuStore::new();

        mock_table_store
            .expect_get_all_tables()
            .returning(|| Ok(vec![2, 3])); // Table 1 doesn't exist

        // Default expectation for get_item_id (won't be called)
        mock_order_store
            .expect_get_item_id()
            .returning(|_, _| Ok(0));

        // Default expectation for get_all_menus (won't be called)
        mock_menu_store
            .expect_get_all_menus()
            .returning(|| Ok(vec![]));

        let restaurant = Arc::new(SimpleRestaurant {
            table_store: Box::new(mock_table_store),
            order_store: Box::new(mock_order_store),
            menu_store: Box::new(mock_menu_store),
        });

        let app_state = AppState { restaurant };
        let mut app = test::init_service(
            App::new()
                .app_data(web::Data::new(app_state))
                .configure(configure_routes),
        )
        .await;

        let req = test::TestRequest::get()
            .uri("/api/v1/get_item/1/1")
            .to_request();
        let resp = test::call_service(&mut app, req).await;

        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[actix_rt::test]
    async fn test_add_item_invalid_table_id() {
        let mut mock_table_store = MockTableStore::new();
        let mock_order_store = MockOrderStore::new();
        let mut mock_menu_store = MockMenuStore::new();

        // Setting up expectations for valid tables and menus (won't be used in this case)
        mock_table_store
            .expect_get_all_tables()
            .returning(|| Ok(vec![1, 2, 3]));

        mock_menu_store.expect_get_all_menus().returning(|| {
            Ok(vec![MenuItem {
                id: 1,
                name: "Burger".to_string(),
                cooking_time: 10,
            }])
        });

        let restaurant = Arc::new(SimpleRestaurant {
            table_store: Box::new(mock_table_store),
            order_store: Box::new(mock_order_store),
            menu_store: Box::new(mock_menu_store),
        });

        let app_state = AppState { restaurant };
        let mut app = test::init_service(
            App::new()
                .app_data(web::Data::new(app_state))
                .configure(configure_routes),
        )
        .await;

        // Invalid table ID
        let req = test::TestRequest::post()
            .uri("/api/v1/add_item/invalid/1")
            .to_request();
        let resp = test::call_service(&mut app, req).await;

        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    #[actix_rt::test]
    async fn test_add_item_invalid_item_id() {
        let mut mock_table_store = MockTableStore::new();
        let mock_order_store = MockOrderStore::new();
        let mut mock_menu_store = MockMenuStore::new();

        // Setting up expectations for valid tables and menus
        mock_table_store
            .expect_get_all_tables()
            .returning(|| Ok(vec![1, 2, 3]));

        mock_menu_store.expect_get_all_menus().returning(|| {
            Ok(vec![MenuItem {
                id: 1,
                name: "Burger".to_string(),
                cooking_time: 10,
            }])
        });

        let restaurant = Arc::new(SimpleRestaurant {
            table_store: Box::new(mock_table_store),
            order_store: Box::new(mock_order_store),
            menu_store: Box::new(mock_menu_store),
        });

        let app_state = AppState { restaurant };
        let mut app = test::init_service(
            App::new()
                .app_data(web::Data::new(app_state))
                .configure(configure_routes),
        )
        .await;

        // Invalid item ID
        let req = test::TestRequest::post()
            .uri("/api/v1/add_item/1/invalid")
            .to_request();
        let resp = test::call_service(&mut app, req).await;

        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    #[actix_rt::test]
    async fn test_remove_item_invalid_table_id() {
        let mut mock_table_store = MockTableStore::new();
        let mock_order_store = MockOrderStore::new();

        // Setting up expectations for valid tables
        mock_table_store
            .expect_get_all_tables()
            .returning(|| Ok(vec![1, 2, 3]));

        let restaurant = Arc::new(SimpleRestaurant {
            table_store: Box::new(mock_table_store),
            order_store: Box::new(mock_order_store),
            menu_store: Box::new(MockMenuStore::new()),
        });

        let app_state = AppState { restaurant };
        let mut app = test::init_service(
            App::new()
                .app_data(web::Data::new(app_state))
                .configure(configure_routes),
        )
        .await;

        // Invalid table ID
        let req = test::TestRequest::delete()
            .uri("/api/v1/remove_item/invalid/1")
            .to_request();
        let resp = test::call_service(&mut app, req).await;

        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    #[actix_rt::test]
    async fn test_remove_item_invalid_item_id() {
        let mut mock_table_store = MockTableStore::new();
        let mock_order_store = MockOrderStore::new();

        // Setting up expectations for valid tables
        mock_table_store
            .expect_get_all_tables()
            .returning(|| Ok(vec![1, 2, 3]));

        let restaurant = Arc::new(SimpleRestaurant {
            table_store: Box::new(mock_table_store),
            order_store: Box::new(mock_order_store),
            menu_store: Box::new(MockMenuStore::new()),
        });

        let app_state = AppState { restaurant };
        let mut app = test::init_service(
            App::new()
                .app_data(web::Data::new(app_state))
                .configure(configure_routes),
        )
        .await;

        // Invalid item ID
        let req = test::TestRequest::delete()
            .uri("/api/v1/remove_item/1/invalid")
            .to_request();
        let resp = test::call_service(&mut app, req).await;

        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    #[actix_rt::test]
    async fn test_get_items_invalid_table_id() {
        let mut mock_table_store = MockTableStore::new();
        let mock_order_store = MockOrderStore::new();
        let mock_menu_store = MockMenuStore::new();

        // Setting up expectations for valid tables (won't be used in this case)
        mock_table_store
            .expect_get_all_tables()
            .returning(|| Ok(vec![1, 2, 3]));

        let restaurant = Arc::new(SimpleRestaurant {
            table_store: Box::new(mock_table_store),
            order_store: Box::new(mock_order_store),
            menu_store: Box::new(mock_menu_store),
        });

        let app_state = AppState { restaurant };
        let mut app = test::init_service(
            App::new()
                .app_data(web::Data::new(app_state))
                .configure(configure_routes),
        )
        .await;

        // Invalid table ID
        let req = test::TestRequest::get()
            .uri("/api/v1/get_items/invalid")
            .to_request();
        let resp = test::call_service(&mut app, req).await;

        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    #[actix_rt::test]
    async fn test_get_item_invalid_table_id() {
        let mut mock_table_store = MockTableStore::new();
        let mock_order_store = MockOrderStore::new();
        let mock_menu_store = MockMenuStore::new();

        // Setting up expectations for valid tables (won't be used in this case)
        mock_table_store
            .expect_get_all_tables()
            .returning(|| Ok(vec![1, 2, 3]));

        let restaurant = Arc::new(SimpleRestaurant {
            table_store: Box::new(mock_table_store),
            order_store: Box::new(mock_order_store),
            menu_store: Box::new(mock_menu_store),
        });

        let app_state = AppState { restaurant };
        let mut app = test::init_service(
            App::new()
                .app_data(web::Data::new(app_state))
                .configure(configure_routes),
        )
        .await;

        // Invalid table ID
        let req = test::TestRequest::get()
            .uri("/api/v1/get_item/invalid/1")
            .to_request();
        let resp = test::call_service(&mut app, req).await;

        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    #[actix_rt::test]
    async fn test_get_item_invalid_item_id() {
        let mut mock_table_store = MockTableStore::new();
        let mock_order_store = MockOrderStore::new();
        let mock_menu_store = MockMenuStore::new();

        // Setting up expectations for valid tables and items (won't be used in this case)
        mock_table_store
            .expect_get_all_tables()
            .returning(|| Ok(vec![1, 2, 3]));

        let restaurant = Arc::new(SimpleRestaurant {
            table_store: Box::new(mock_table_store),
            order_store: Box::new(mock_order_store),
            menu_store: Box::new(mock_menu_store),
        });

        let app_state = AppState { restaurant };
        let mut app = test::init_service(
            App::new()
                .app_data(web::Data::new(app_state))
                .configure(configure_routes),
        )
        .await;

        // Invalid item ID
        let req = test::TestRequest::get()
            .uri("/api/v1/get_item/1/invalid")
            .to_request();
        let resp = test::call_service(&mut app, req).await;

        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }
}
