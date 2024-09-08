#![deny(warnings)]
#![deny(clippy::all)]

use crate::server::utils::error::RestaurantError;
use actix_web::HttpResponse;
use serde_json::json;

/// Returns a success response with data for GET requests.
pub fn success_response<T>(data: T) -> HttpResponse
where
    T: serde::Serialize,
{
    HttpResponse::Ok().json(json!({
        "status": "ok",
        "data": data
    }))
}

/// Returns an error response with a custom status code and message.
pub fn error_response(status_code: u16, message: &str) -> HttpResponse {
    HttpResponse::build(actix_web::http::StatusCode::from_u16(status_code).unwrap()).json(json!({
        "status": "error",
        "message": message
    }))
}

/// Returns a success response with a message for POST/DELETE requests.
pub fn success_message_response(message: &str) -> HttpResponse {
    HttpResponse::Ok().json(json!({
        "status": "ok",
        "message": message
    }))
}

/// converts restaurant erros to http response erros
pub fn restaurant_error_to_response(err: RestaurantError) -> HttpResponse {
    match err {
        RestaurantError::LockError(_) => HttpResponse::InternalServerError().finish(),
        RestaurantError::TableNotFound(table_id) => {
            error_response(404, &format!("Table not found for table id:{}", table_id))
        }
        RestaurantError::MenuNotFound(menu_id) => error_response(
            404,
            &format!("Menu item not found for menu id: {}", menu_id),
        ),
        RestaurantError::MenusRetrieveError => error_response(500, "Error retrieving menus"),
        RestaurantError::TablesRetrieveError => error_response(500, "Error retrieving tables"),
        RestaurantError::NoMenuForTable(table_id, menu_item_id) => error_response(
            404,
            &format!(
                "No Menu item with menu item id:{}, is found for Table with table id:{}",
                menu_item_id, table_id
            ),
        ),
        RestaurantError::NoMenusForTable(table_id) => error_response(
            404,
            &format!("No Menu items added for table with table id:{}", table_id),
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test;
    use serde_json::json;

    #[actix_rt::test]
    async fn test_success_response() {
        let data = json!({"key": "value"});
        let resp = success_response(data.clone());

        let service_resp = test::TestRequest::default().to_srv_response(resp);
        let body = test::read_body(service_resp).await;
        let body: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(body["status"], "ok");
        assert_eq!(body["data"], data);
    }

    #[actix_rt::test]
    async fn test_success_message_response() {
        let message = "Success!";
        let resp = success_message_response(message);

        let service_resp = test::TestRequest::default().to_srv_response(resp);
        let body = test::read_body(service_resp).await;
        let body: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(body["status"], "ok");
        assert_eq!(body["message"], message);
    }

    #[actix_rt::test]
    async fn test_error_response() {
        let message = "Something went wrong!";
        let resp = error_response(400, message);

        let service_resp = test::TestRequest::default().to_srv_response(resp);
        let body = test::read_body(service_resp).await;
        let body: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(body["status"], "error");
        assert_eq!(body["message"], message);
    }

    #[actix_rt::test]
    async fn test_restaurant_error_to_response_table_not_found() {
        let err = RestaurantError::TableNotFound(1);
        let resp = restaurant_error_to_response(err);

        let service_resp = test::TestRequest::default().to_srv_response(resp);
        let body = test::read_body(service_resp).await;
        let body: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(body["status"], "error");
        assert_eq!(body["message"], "Table not found for table id:1");
    }

    #[actix_rt::test]
    async fn test_restaurant_error_to_response_menu_not_found() {
        let err = RestaurantError::MenuNotFound(42);
        let resp = restaurant_error_to_response(err);

        let service_resp = test::TestRequest::default().to_srv_response(resp);
        let body = test::read_body(service_resp).await;
        let body: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(body["status"], "error");
        assert_eq!(body["message"], "Menu item not found for menu id: 42");
    }

    #[actix_rt::test]
    async fn test_restaurant_error_to_response_menus_retrieve_error() {
        let err = RestaurantError::MenusRetrieveError;
        let resp = restaurant_error_to_response(err);

        let service_resp = test::TestRequest::default().to_srv_response(resp);
        let body = test::read_body(service_resp).await;
        let body: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(body["status"], "error");
        assert_eq!(body["message"], "Error retrieving menus");
    }

    #[actix_rt::test]
    async fn test_restaurant_error_to_response_tables_retrieve_error() {
        let err = RestaurantError::TablesRetrieveError;
        let resp = restaurant_error_to_response(err);

        let service_resp = test::TestRequest::default().to_srv_response(resp);
        let body = test::read_body(service_resp).await;
        let body: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(body["status"], "error");
        assert_eq!(body["message"], "Error retrieving tables");
    }

    #[actix_rt::test]
    async fn test_restaurant_error_to_response_no_menu_for_table() {
        let err = RestaurantError::NoMenuForTable(1, 100);
        let resp = restaurant_error_to_response(err);

        let service_resp = test::TestRequest::default().to_srv_response(resp);
        let body = test::read_body(service_resp).await;
        let body: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(body["status"], "error");
        assert_eq!(
            body["message"],
            "No Menu item with menu item id:100, is found for Table with table id:1"
        );
    }

    #[actix_rt::test]
    async fn test_restaurant_error_to_response_no_menus_for_table() {
        let err = RestaurantError::NoMenusForTable(1);
        let resp = restaurant_error_to_response(err);

        let service_resp = test::TestRequest::default().to_srv_response(resp);
        let body = test::read_body(service_resp).await;
        let body: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(body["status"], "error");
        assert_eq!(
            body["message"],
            "No Menu items added for table with table id:1"
        );
    }
}
