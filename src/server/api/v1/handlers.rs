#![deny(warnings)]
#![deny(clippy::all)]

use crate::server::error::RestaurantError;
use crate::server::models::Restaurant;
use actix_web::{web, HttpResponse, Responder};
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub restaurant: Arc<dyn Restaurant + Send + Sync>,
}

#[utoipa::path(
    post,
    path = "/api/v1/add_item/{table_id}/{menu_item_id}",
    responses(
        (status = 200, description = "Menu ttem added successfully"),
        (status = 404, description = "Table or menu item not found"),
        (status = 500, description = "Internal server error")
    ),
    params(
        ("table_id" = u32, description = "ID of the table"),
        ("menu_item_id" = u32, description = "ID of the Menu"),
    ),
)]
pub async fn add_item(data: web::Data<AppState>, params: web::Path<(u32, u32)>) -> impl Responder {
    let restaurant = &data.restaurant;
    match restaurant.add_item(params.0, params.1) {
        Ok(_) => HttpResponse::Ok().json("Item added successfully"),
        Err(e) => restaurant_error_to_response(e),
    }
}

#[utoipa::path(
    delete,
    path = "/api/v1/remove_item/{table_id}/{menu_item_id}",
    responses(
        (status = 200, description = "Menu Item removed successfully"),
        (status = 404, description = "Table or menu item not found"),
        (status = 500, description = "Internal server error")
    ),
    params(
        ("table_id" = u32, description = "ID of the table"),
        ("menu_item_id" = u32, description = "ID of the menu item to remove")
    )
)]
pub async fn remove_item(
    data: web::Data<AppState>,
    params: web::Path<(u32, u32)>,
) -> impl Responder {
    let restaurant = &data.restaurant;
    match restaurant.remove_item(params.0, params.1) {
        Ok(_) => HttpResponse::Ok().json("Item removed successfully"),
        Err(e) => restaurant_error_to_response(e),
    }
}

#[utoipa::path(
    get,
    path = "/api/v1/get_items/{table_id}",
    responses(
        (status = 200, description = "List of menuitems added for the table", body = [MenuItem]),
        (status = 404, description = "Table not found"),
        (status = 500, description = "Internal server error")
    ),
    params(
        ("table_id" = u32, description = "ID of the table")
    )
)]
pub async fn get_items(data: web::Data<AppState>, table_id: web::Path<u32>) -> impl Responder {
    let restaurant = &data.restaurant;
    match restaurant.get_items(*table_id) {
        Ok(items) => HttpResponse::Ok().json(items),
        Err(e) => restaurant_error_to_response(e),
    }
}

#[utoipa::path(
    get,
    path = "/api/v1/get_item/{table_id}/{menu_item_id}",
    responses(
        (status = 200, description = "Menu item details", body = MenuItem),
        (status = 404, description = "Table or menu item not found"),
        (status = 500, description = "Internal server error")
    ),
    params(
        ("table_id" = u32, description = "ID of the table"),
        ("menu_item_id" = u32, description = "ID of the menu item")
    )
)]
pub async fn get_item(data: web::Data<AppState>, params: web::Path<(u32, u32)>) -> impl Responder {
    let restaurant = &data.restaurant;
    match restaurant.get_item(params.0, params.1) {
        Ok(item) => HttpResponse::Ok().json(item),
        Err(e) => restaurant_error_to_response(e),
    }
}

#[utoipa::path(
    get,
    path = "/api/v1/tables",
    responses(
        (status = 200, description = "List of available tables", body = [u32]),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_tables(data: web::Data<AppState>) -> impl Responder {
    let restaurant = &data.restaurant;
    match restaurant.get_all_tables() {
        Ok(tables) => HttpResponse::Ok().json(tables),
        Err(e) => restaurant_error_to_response(e),
    }
}

#[utoipa::path(
    get,
    path = "/api/v1/menus",
    responses(
        (status = 200, description = "List of available menus", body = [MenuItem]),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_menus(data: web::Data<AppState>) -> impl Responder {
    let restaurant = &data.restaurant;
    match restaurant.get_all_menus() {
        Ok(menus) => HttpResponse::Ok().json(menus),
        Err(e) => restaurant_error_to_response(e),
    }
}

fn restaurant_error_to_response(err: RestaurantError) -> HttpResponse {
    match err {
        RestaurantError::LockError(_) => HttpResponse::InternalServerError().json("Internal error"),
        RestaurantError::TableNotFound(_) => HttpResponse::NotFound().json("Table not found"),
        RestaurantError::MenuNotFound(_) => HttpResponse::NotFound().json("Menu item not found"),
        RestaurantError::MenusRetrieveError => {
            HttpResponse::InternalServerError().json("Error retrieving menus")
        }
        RestaurantError::TablesRetrieveError => {
            HttpResponse::InternalServerError().json("Error retrieving tables")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::server::api::v1::routes::configure_routes;
    use crate::server::models::{MenuItem, MockMenuStore, MockOrderStore, MockTableStore};
    use crate::server::restaurant::SimpleRestaurant;
    use actix_web::{http::StatusCode, test, web, App};
    use mockall::predicate::*;

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

        let items: Vec<MenuItem> = test::read_body_json(resp).await;
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

        let item: MenuItem = test::read_body_json(resp).await;
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
}
