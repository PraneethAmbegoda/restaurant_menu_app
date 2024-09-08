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
