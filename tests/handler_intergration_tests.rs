#![deny(warnings)]
#![deny(clippy::all)]

use actix_web::{http::StatusCode, test, web, App};
use futures::future::join_all;
use restaurant_menu_app::server::api::v1::handlers::AppState;
use restaurant_menu_app::server::api::v1::routes::configure_routes;
use restaurant_menu_app::server::data_model::models::{MenuItem, Restaurant};
use restaurant_menu_app::server::data_store::in_memory_menu_store::InMemoryMenuStore;
use restaurant_menu_app::server::data_store::in_memory_order_store::InMemoryOrderStore;
use restaurant_menu_app::server::data_store::in_memory_table_store::InMemoryTableStore;
use restaurant_menu_app::server::restaurant::SimpleRestaurant;
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::Mutex;

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

    let json_response: Value = test::read_body_json(resp).await;
    let items: Vec<MenuItem> = serde_json::from_value(json_response["data"].clone()).unwrap();
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

    let json_response: Value = test::read_body_json(resp).await;
    let fetched_item: MenuItem = serde_json::from_value(json_response["data"].clone()).unwrap();
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

#[actix_rt::test]
async fn test_concurrent_add_remove_items() {
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

    // Initialize Actix app inside an Arc<Mutex<_>> to allow shared mutable access
    let app = Arc::new(Mutex::new(
        test::init_service(
            App::new()
                .app_data(web::Data::new(app_state))
                .configure(configure_routes),
        )
        .await,
    ));

    let table_id = 1;
    let menu_item_id = 1;
    let table_items: Arc<Mutex<Vec<u32>>> = Arc::new(Mutex::new(Vec::new()));

    // Concurrently add items using futures
    let add_futures: Vec<_> = (0..10)
        .map(|_| {
            let table_items_clone = Arc::clone(&table_items);
            let app_clone = Arc::clone(&app); // Clone app wrapped in Arc<Mutex<_>>
            async move {
                let mut app_ref = app_clone.lock().await; // Lock the app to access it
                let url_add = format!("/api/v1/add_item/{}/{}", table_id, menu_item_id);
                let response = test::TestRequest::post().uri(&url_add).to_request();
                let resp = test::call_service(&mut *app_ref, response).await;

                if resp.status() == StatusCode::OK {
                    let mut table_items_lock = table_items_clone.lock().await;
                    table_items_lock.push(menu_item_id);
                }
            }
        })
        .collect();

    // Run all add operations in parallel
    join_all(add_futures).await;

    // Concurrently remove items using futures (without using tokio::spawn)
    let remove_futures: Vec<_> = (0..5)
        .map(|_| {
            let table_items_clone = Arc::clone(&table_items);
            let app_clone = Arc::clone(&app); // Clone app wrapped in Arc<Mutex<_>>
            async move {
                let mut app_ref = app_clone.lock().await; // Lock the app to access it
                let url_remove = format!("/api/v1/remove_item/{}/{}", table_id, menu_item_id);
                let response = test::TestRequest::delete().uri(&url_remove).to_request();
                let resp = test::call_service(&mut *app_ref, response).await;

                if resp.status() == StatusCode::OK {
                    let mut table_items_lock = table_items_clone.lock().await;
                    if let Some(pos) = table_items_lock.iter().position(|&x| x == menu_item_id) {
                        table_items_lock.remove(pos);
                    }
                }
            }
        })
        .collect();

    // Run all remove operations in parallel
    join_all(remove_futures).await;

    // Final check: Verify the remaining number of items
    let req = test::TestRequest::get()
        .uri(&format!("/api/v1/get_items/{}", table_id))
        .to_request();
    let mut app_ref = app.lock().await; // Lock the app to access it for final check
    let resp = test::call_service(&mut *app_ref, req).await;

    assert_eq!(resp.status(), StatusCode::OK);

    let json_response: Value = test::read_body_json(resp).await;
    let items: Vec<MenuItem> = serde_json::from_value(json_response["data"].clone()).unwrap();

    // After adding 10 times and removing 5 times, there should be 5 items remaining
    assert_eq!(items.len(), 5);
}
