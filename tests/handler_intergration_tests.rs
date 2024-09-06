use actix_web::{http::StatusCode, test, web, App};
use restaurant_menu_app::server::api::v1::handlers::AppState;
use restaurant_menu_app::server::api::v1::routes::configure_routes;
use restaurant_menu_app::server::in_memory_menu_store::InMemoryMenuStore;
use restaurant_menu_app::server::in_memory_order_store::InMemoryOrderStore;
use restaurant_menu_app::server::in_memory_table_store::InMemoryTableStore;
use restaurant_menu_app::server::models::MenuItem;
use restaurant_menu_app::server::models::Restaurant;
use restaurant_menu_app::server::restaurant::SimpleRestaurant;

use std::sync::Arc; // Import the Restaurant trait

#[actix_rt::test]
async fn test_add_item() {
    let menu_store = InMemoryMenuStore::new(vec![MenuItem {
        id: 1,
        name: "Burger".to_string(),
        cooking_time: 10,
    }]);
    let order_store = InMemoryOrderStore::new();
    let table_store = InMemoryTableStore::new();
    let restaurant = Arc::new(SimpleRestaurant::new(
        Box::new(menu_store),
        Box::new(order_store),
        Box::new(table_store),
    )) as Arc<dyn Restaurant + Send + Sync>;

    let app_state = AppState {
        restaurant: Arc::clone(&restaurant),
    };
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
async fn test_add_item_to_nonexistent_table() {
    let menu_store = InMemoryMenuStore::new(vec![MenuItem {
        id: 1,
        name: "Burger".to_string(),
        cooking_time: 10,
    }]);
    let order_store = InMemoryOrderStore::new();
    let table_store = InMemoryTableStore::new();
    let restaurant = Arc::new(SimpleRestaurant::new(
        Box::new(menu_store),
        Box::new(order_store),
        Box::new(table_store),
    )) as Arc<dyn Restaurant + Send + Sync>;

    let app_state = AppState {
        restaurant: Arc::clone(&restaurant),
    };
    let mut app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_state))
            .configure(configure_routes),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/api/v1/add_item/999/1") // Nonexistent table ID
        .to_request();
    let resp = test::call_service(&mut app, req).await;

    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

#[actix_rt::test]
async fn test_remove_item() {
    let menu_store = InMemoryMenuStore::new(vec![MenuItem {
        id: 1,
        name: "Burger".to_string(),
        cooking_time: 10,
    }]);
    let order_store = InMemoryOrderStore::new();
    let table_store = InMemoryTableStore::new();
    let restaurant = Arc::new(SimpleRestaurant::new(
        Box::new(menu_store),
        Box::new(order_store),
        Box::new(table_store),
    )) as Arc<dyn Restaurant + Send + Sync>;

    let app_state = AppState {
        restaurant: Arc::clone(&restaurant),
    };
    let mut app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_state))
            .configure(configure_routes),
    )
    .await;

    let item = MenuItem {
        id: 1,
        name: "Burger".to_string(),
        cooking_time: 10,
    };
    restaurant.add_item(1, item.id).unwrap();

    let req = test::TestRequest::delete()
        .uri("/api/v1/remove_item/1/1") // Table ID 1, Item ID 1
        .to_request();
    let resp = test::call_service(&mut app, req).await;

    assert_eq!(resp.status(), StatusCode::OK);
}

#[actix_rt::test]
async fn test_remove_nonexistent_item() {
    let menu_store = InMemoryMenuStore::new(vec![MenuItem {
        id: 1,
        name: "Burger".to_string(),
        cooking_time: 10,
    }]);
    let order_store = InMemoryOrderStore::new();
    let table_store = InMemoryTableStore::new();
    let restaurant = Arc::new(SimpleRestaurant::new(
        Box::new(menu_store),
        Box::new(order_store),
        Box::new(table_store),
    )) as Arc<dyn Restaurant + Send + Sync>;

    let app_state = AppState {
        restaurant: Arc::clone(&restaurant),
    };
    let mut app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_state))
            .configure(configure_routes),
    )
    .await;

    let req = test::TestRequest::delete()
        .uri("/api/v1/remove_item/1/999") // Nonexistent item ID
        .to_request();
    let resp = test::call_service(&mut app, req).await;

    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

#[actix_rt::test]
async fn test_get_items() {
    let menu_store = InMemoryMenuStore::new(vec![MenuItem {
        id: 1,
        name: "Burger".to_string(),
        cooking_time: 10,
    }]);
    let order_store = InMemoryOrderStore::new();
    let table_store = InMemoryTableStore::new();
    let restaurant = Arc::new(SimpleRestaurant::new(
        Box::new(menu_store),
        Box::new(order_store),
        Box::new(table_store),
    )) as Arc<dyn Restaurant + Send + Sync>;

    let app_state = AppState {
        restaurant: Arc::clone(&restaurant),
    };
    let mut app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_state))
            .configure(configure_routes),
    )
    .await;

    let item = MenuItem {
        id: 1,
        name: "Burger".to_string(),
        cooking_time: 10,
    };
    restaurant.add_item(1, item.id).unwrap();

    let req = test::TestRequest::get()
        .uri("/api/v1/get_items/1") // Table ID 1
        .to_request();
    let resp = test::call_service(&mut app, req).await;

    assert_eq!(resp.status(), StatusCode::OK);

    let items: Vec<MenuItem> = test::read_body_json(resp).await;
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].name, "Burger");
}

#[actix_rt::test]
async fn test_get_items_for_nonexistent_table() {
    let menu_store = InMemoryMenuStore::new(vec![MenuItem {
        id: 1,
        name: "Burger".to_string(),
        cooking_time: 10,
    }]);
    let order_store = InMemoryOrderStore::new();
    let table_store = InMemoryTableStore::new();
    let restaurant = Arc::new(SimpleRestaurant::new(
        Box::new(menu_store),
        Box::new(order_store),
        Box::new(table_store),
    )) as Arc<dyn Restaurant + Send + Sync>;

    let app_state = AppState {
        restaurant: Arc::clone(&restaurant),
    };
    let mut app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_state))
            .configure(configure_routes),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/api/v1/get_items/999") // Nonexistent table ID
        .to_request();
    let resp = test::call_service(&mut app, req).await;

    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

#[actix_rt::test]
async fn test_get_item() {
    let menu_store = InMemoryMenuStore::new(vec![MenuItem {
        id: 1,
        name: "Burger".to_string(),
        cooking_time: 10,
    }]);
    let order_store = InMemoryOrderStore::new();
    let table_store = InMemoryTableStore::new();
    let restaurant = Arc::new(SimpleRestaurant::new(
        Box::new(menu_store),
        Box::new(order_store),
        Box::new(table_store),
    )) as Arc<dyn Restaurant + Send + Sync>;

    let app_state = AppState {
        restaurant: Arc::clone(&restaurant),
    };
    let mut app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_state))
            .configure(configure_routes),
    )
    .await;

    let item = MenuItem {
        id: 1,
        name: "Burger".to_string(),
        cooking_time: 10,
    };
    restaurant.add_item(1, item.id).unwrap();

    let req = test::TestRequest::get()
        .uri("/api/v1/get_item/1/1") // Table ID 1, Item ID 1
        .to_request();
    let resp = test::call_service(&mut app, req).await;

    assert_eq!(resp.status(), StatusCode::OK);

    let fetched_item: MenuItem = test::read_body_json(resp).await;
    assert_eq!(fetched_item.name, "Burger");
}

#[actix_rt::test]
async fn test_get_item_from_non_existent_table() {
    let menu_store = InMemoryMenuStore::new(vec![MenuItem {
        id: 1,
        name: "Burger".to_string(),
        cooking_time: 10,
    }]);
    let order_store = InMemoryOrderStore::new();
    let table_store = InMemoryTableStore::new();
    let restaurant = Arc::new(SimpleRestaurant::new(
        Box::new(menu_store),
        Box::new(order_store),
        Box::new(table_store),
    )) as Arc<dyn Restaurant + Send + Sync>;

    let app_state = AppState {
        restaurant: Arc::clone(&restaurant),
    };
    let mut app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_state))
            .configure(configure_routes),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/api/v1/get_item/999/1") // Table ID 999, Item ID 1 (non-existent table)
        .to_request();
    let resp = test::call_service(&mut app, req).await;

    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}
