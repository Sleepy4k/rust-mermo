use sqlx::PgPool;
use tide::{Request, Response};

use crate::helper::{response::*, parse::convert_vec_to_values};

#[doc = "Define the struct of the parameter for \"get_or_find_spare_part\", \"update_spare_part\""]
#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct SparePart {
    id: i32,
    name: String,
    price: i32
}

#[doc = "Define the struct of the parameter for \"get_or_find_spare_part\""]
#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct FindSparePart {
    id: i32,
}

#[doc = "Define the struct of the parameter for \"add_spare_part\""]
#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct AddSparePart {
    name: String,
    price: i32
}

#[doc = "Define the struct of the parameter for \"delete_spare_part\""]
#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct DeleteSparePart {
    id: i32,
}

#[doc = "function to get or find product"]
pub async fn get_or_find_spare_part(req: Request<PgPool>) -> tide::Result<Response> {
    match req.query() {
        Ok(query_param) => {
            let param: FindSparePart = query_param;
            let pool = req.state();

            let sparepart = sqlx::query_as!(SparePart,
                "select * from spare_part where id = $1",
                param.id
            ).fetch_all(pool).await?;

            let result = convert_vec_to_values(sparepart);

            response_json("success".to_string(), "berhasil menampilkan spare part".to_string(), result)
        },
        Err(_) => {
            let pool = req.state();

            let sparepart = sqlx::query_as!(SparePart,
                "select * from spare_part"
            ).fetch_all(pool).await?;

            let result = convert_vec_to_values(sparepart);

            response_json("success".to_string(), "berhasil menampilkan spare part".to_string(), result)
        }
    }
}

#[doc = "function to add spare part"]
pub async fn add_spare_part(mut req: Request<PgPool>) -> tide::Result<Response> {
    let param: AddSparePart = req.body_json().await?;
    let pool = req.state();

    match sqlx::query!(
        "insert into spare_part (name, price) values ($1, $2)",
        param.name, param.price
    ).execute(pool).await {
        Ok(_x) => {response_json("success".to_string(), "berhasil menambahkan spare part".to_string(), vec![])}
        Err(e) => {
            eprintln!("Error add: {:?}", e);

            response_json("error".to_string(), "gagal menambahkan spare part".to_string(), vec![])
        }
    }
}

#[doc = "function to update spare part"]
pub async fn update_spare_part(mut req:Request<PgPool>) -> tide::Result<Response> {
    let param: SparePart = req.body_json().await?;
    let pool = req.state();

    match sqlx::query!(
        "update spare_part set name = $1, price = $2 where id = $3",
        param.name, param.price, param.id
    ).execute(pool).await {
        Ok(_x) => {response_json("success".to_string(), "berhasil mengubah spare part".to_string(), vec![])}
        Err(e) => {
            eprintln!("Error update: {:?}", e);

            response_json("error".to_string(), "gagal mengubah spare part".to_string(), vec![])
        }
    }
}

#[doc = "function to delete spare part"]
pub async fn delete_spare_part(mut req: Request<PgPool>) -> tide::Result<Response> {
    let param: DeleteSparePart = req.body_json().await?;
    let pool = req.state();

    match sqlx::query!(
        "delete from spare_part where id = $1",
        param.id
    ).execute(pool).await {
        Ok(_x) => {response_json("success".to_string(), "berhasil menghapus spare part".to_string(), vec![])}
        Err(e) => {
            eprintln!("Error delete: {:?}", e);

            response_json("error".to_string(), "gagal menghapus spare part".to_string(), vec![])
        }
    }
}
