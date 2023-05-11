use serde_json::json;

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
