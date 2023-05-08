use tide::{Body, Response};
use serde_json::{json, Map, Value};

#[doc = "define the struct of the response"]
#[derive(serde::Serialize)]
struct ServiceResponse {
    status: String,
    info: String,
}

#[doc = "define the struct of the response with data"]
#[derive(serde::Serialize)]
struct ServiceResonseData {
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
    let mut res = Response::new(200);

    let data = ServiceResponse {
        status: status_val.into(),
        info: info_val.into()
    };

    let mut body = Map::new();
    body.insert("status".into(), data.status.into());
    body.insert("info".into(), data.info.into());
    body.insert("data".into(), Value::Null);

    res.set_body(Body::from_json(&body)?);

    Ok(res)
}

#[doc = "function to create response with data"]
pub fn response_with_data(status_val: &str, info_val: &str, value: Value) -> tide::Result<Response> {
    let mut res = Response::new(200);

    let data = ServiceResonseData {
        status: status_val.into(),
        info: info_val.into(),
        data: Value
    };

    let mut body = Map::new();
    body.insert("status".into(), data.status.into());
    body.insert("info".into(), data.info.into());
    body.insert("data".into(), data.data);

    res.set_body(Body::from_json(&body)?);

    Ok(res)
}