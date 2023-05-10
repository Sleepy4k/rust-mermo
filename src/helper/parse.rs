use serde::Serialize;
use serde_json::{json, Value};
use tide::{Response, StatusCode};
use tide::http::cookies::Cookie;

#[doc = "define the struct of the response"]
#[derive(serde::Serialize)]
struct ServiceResponse {
    status: String,
    info: String,
    data: String
}

#[doc = "define the struct of the response with data"]
#[derive(serde::Serialize)]
struct ServiceResonseData<T> {
    status: String,
    info: String,
    data: Vec<T>
}

#[doc = "define the struct of the response with data and cookie"]
#[derive(serde::Serialize)]
struct ServiceResonseDataAndCookie {
    status: String,
    info: String,
    data: Value
}

#[doc = "function to convert vector of string to string"]
pub fn vec_to_string(vec: Vec<&str>) -> String {
    let mut res = String::new();

    for (i, el) in vec.into_iter().enumerate() {
        if i != 0 {res.push_str(",")}
        res.push_str("'");
        res.push_str(el);
        res.push_str("'");
    }

    res
}

#[doc = "function to convert any type to json"]
pub fn to_json(data: impl serde::Serialize) -> tide::Result<serde_json::Value> {
    Ok( json!(data) )
}

#[doc = "function to create response"]
pub fn response(status_val: &str, info_val: &str) -> tide::Result<Response> {
    let body = ServiceResponse {
        status: status_val.to_owned(),
        info: info_val.to_owned(),
        data: String::new(),
    };

    let json = serde_json::to_string(&body).unwrap();

    let response = Response::builder(StatusCode::Ok)
        .body(json)
        .content_type("application/json")
        .build();

    Ok(response)
}

#[doc = "function to create response with data"]
pub fn response_with_data<T: Serialize>(status_val: &str, info_val: &str, data: Vec<T>) -> tide::Result<Response> {
    let body = ServiceResonseData {
        status: status_val.to_owned(),
        info: info_val.to_owned(),
        data,
    };

    let json = serde_json::to_string(&body).unwrap();

    let response = Response::builder(StatusCode::Ok)
        .body(json)
        .content_type("application/json")
        .build();

    Ok(response)
}

#[doc = "function to create response with cookie"]
pub fn response_with_cookie(status_val: &str, info_val: &str, cookie_type: &str, cookie_title: &str, cookie_data: String) -> tide::Result<Response>  {
    let response = ServiceResponse {
        status: status_val.to_owned(),
        info: info_val.to_owned(),
        data: String::new(),
    };

    let json = serde_json::to_string(&response).unwrap();

    let mut response = Response::new(StatusCode::Ok);
    
    let title = cookie_title.to_owned();
    let data = cookie_data.to_owned();

    let mut cookies = Cookie::new(title, data);
    cookies.set_path("/");
    cookies.set_http_only(false);
    cookies.set_secure(true);
    cookies.set_same_site(tide::http::cookies::SameSite::Strict);

    if cookie_type == "remove" {
        response.remove_cookie(cookies);
    } else if cookie_type == "insert" {
        response.insert_cookie(cookies);
    }

    response.set_body(json);
    response.set_content_type("application/json");

    Ok(response)
}

#[doc = "function to create response with data and cookie"]
pub fn response_with_data_and_cookie<T: Serialize>(status_val: &str, info_val: &str, data: Vec<T>, cookie_type: &str, cookie_title: &str, cookie_data: String) -> tide::Result<Response> {
    let body = ServiceResonseData {
        status: status_val.to_owned(),
        info: info_val.to_owned(),
        data,
    };

    let json = serde_json::to_string(&body).unwrap();

    let mut response = Response::new(StatusCode::Ok);
    
    let title = cookie_title.to_owned();
    let data = cookie_data.to_owned();

    let mut cookies = Cookie::new(title, data);
    cookies.set_path("/");
    cookies.set_http_only(false);
    cookies.set_secure(true);
    cookies.set_same_site(tide::http::cookies::SameSite::Strict);

    if cookie_type == "remove" {
        response.remove_cookie(cookies);
    } else if cookie_type == "insert" {
        response.insert_cookie(cookies);
    }

    response.set_body(json);
    response.set_content_type("application/json");

    Ok(response)
}