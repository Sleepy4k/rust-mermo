use serde::Serialize;
use serde_json::{to_string, Value};
use tide::{Response, StatusCode, http::cookies::Cookie};

#[doc = "Struct of the response"]
#[derive(Debug, Serialize)]
struct ResponseStruct {
    status: String,
    message: String,
    data: Vec<Value>
}

pub fn response_json(status: String, message: String, data: Vec<Value>) -> tide::Result<Response> {
    // Init response
    let mut code = StatusCode::Accepted;
    let stats = status.to_owned().to_lowercase();

    // Check status
    if stats == "error" {
        code = StatusCode::BadRequest;
    } else if stats == "failed" {
        code = StatusCode::InternalServerError;
    } else if stats == "success" {
        code = StatusCode::Ok;
    } else if stats == "unauthorized" {
        code = StatusCode::Unauthorized;
    } else if stats == "forbidden" {
        code = StatusCode::Forbidden;
    }
    
    // Init body struct
    let body = ResponseStruct {
        status,
        message,
        data
    };

    // Convert body to json
    let body_json = to_string(&body).unwrap();

    // Init response
    let mut response = Response::new(code);

    // Create response
    response.set_body(body_json);
    response.set_content_type("application/json");

    Ok(response)
}

pub fn response_cookie_json(status: String, message: String, data: Vec<Value>, cookie_type: String, cookie_title: String, cookie_value: String) -> tide::Result<Response> {
    // Init response
    let mut code = StatusCode::Accepted;
    let stats = status.to_owned().to_lowercase();

    // Check status
    if stats == "error" {
        code = StatusCode::BadRequest;
    } else if stats == "failed" {
        code = StatusCode::InternalServerError;
    } else if stats == "success" {
        code = StatusCode::Ok;
    } else if stats == "unauthorized" {
        code = StatusCode::Unauthorized;
    } else if stats == "forbidden" {
        code = StatusCode::Forbidden;
    }
    
    // Init body struct
    let body = ResponseStruct {
        status,
        message,
        data
    };

    // Convert body to json
    let body_json = to_string(&body).unwrap();

    // Init response
    let mut response = Response::new(code);

    // Init cookie
    let method = cookie_type.to_owned().to_lowercase();

    if method == "set" {
        let mut cookie = Cookie::new(cookie_title, cookie_value);
        cookie.set_secure(true);
        cookie.set_http_only(false);
        
        response.insert_cookie(cookie);
    } else if method == "delete" {
        let cookie = Cookie::named(cookie_title);

        response.remove_cookie(cookie);
    }

    // Create response
    response.set_body(body_json);
    response.set_content_type("application/json");

    Ok(response)
}
