use serde::Serialize;
use serde_json::{json};
use tide::{Response, StatusCode};

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
    let response = ServiceResponse {
        status: status_val.to_owned(),
        info: info_val.to_owned(),
        data: String::new(),
    };

    let json = serde_json::to_string(&response).unwrap();

    let res = Response::builder(StatusCode::Ok)
        .body(json)
        .content_type("application/json")
        .build();

    Ok(res)
}

#[doc = "function to create response with data"]
pub fn response_with_data<T: Serialize>(status_val: &str, info_val: &str, data: Vec<T>) -> tide::Result<Response> {
    let response = ServiceResonseData {
        status: status_val.to_owned(),
        info: info_val.to_owned(),
        data,
    };

    let json = serde_json::to_string(&response).unwrap();

    let res = Response::builder(StatusCode::Ok)
        .body(json)
        .content_type("application/json")
        .build();

    Ok(res)
}