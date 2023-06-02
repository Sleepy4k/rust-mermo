use serde::Serialize;
use serde_json::{Value, json};

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
    let data = json!(data);

    Ok(data)
}

#[doc = "function to convert any struct to json"]
pub fn convert_vec_to_values<T: Serialize>(data: Vec<T>) -> Vec<Value> {
    data.into_iter().map(|item| serde_json::to_value(item).unwrap()).collect()
}