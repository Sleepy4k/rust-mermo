use sqlx::PgPool;
use tide::{Request, Response};
use crate::{response, response_with_data};

#[doc = "Define the struct of the parameter for \"get_spare_part\", \"find_spare_part\", \"update_spare_part\""]
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct SparePart {
    pub id: i32,
    pub name: String,
    pub price: i32
}

#[doc = "Define the struct of the parameter for \"add_spare_part\""]
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct AddSparePart {
    pub name: String,
    pub price: i32
}

#[doc = "Define the struct of the parameter for \"find_spare_part\""]
#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct FindParam {
    id: i32,
}

#[doc = "Define the struct of the parameter for \"delete_spare_part\""]
#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct DelParam {
    id: i32,
}

#[doc = "function to get all spare part"]
pub async fn get_spare_part(req: Request<PgPool>) -> tide::Result<Response> {
    let pool = req.state();

    let sparepart: Vec<SparePart> = sqlx::query_as!(SparePart, "select * from spare_part")
        .fetch_all(pool)
        .await?;

    response_with_data("OK", "berhasil menampilkan spare part", sparepart)
}

#[doc = "function to add spare part"]
pub async fn add_spare_part(mut req: Request<PgPool>) -> tide::Result<Response> {
    let param: AddSparePart = req.body_json().await?;
    let pool = req.state();

    match sqlx::query("insert into spare_part (name, price) values ($1, $2)")
        .bind(param.name)
        .bind(param.price)
        .execute(pool).await {
            Ok(_x) => {response("OK", "berhasil menambahkan spare part")}
            Err(e) => {
                eprintln!("Error add: {:?}", e);

                response("ERROR", "gagal menambahkan spare part")
            }
        }
}

#[doc = "function to find spare part"]
pub async fn find_spare_part(mut req: Request<PgPool>) -> tide::Result<Response> {
    let param: FindParam = req.body_json().await?;
    let pool = req.state();

    let sparepart: Vec<SparePart> = sqlx::query_as!(SparePart, "select * from spare_part where id = $1", param.id)
        .fetch_all(pool)
        .await?;

    response_with_data("OK", "berhasil menampilkan spare part", sparepart)
}

#[doc = "function to update spare part"]
pub async fn update_spare_part(mut req:Request<PgPool>) -> tide::Result<Response> {
    let param: SparePart = req.body_json().await?;
    let pool = req.state();

    match sqlx::query("update spare_part set name = $1, price = $2 where id = $3;")
        .bind(param.name)
        .bind(param.price)
        .bind(param.id)
        .execute(pool).await {
            Ok(_x) => {response("OK", "berhasil mengubah spare part")}
            Err(e) => {
                eprintln!("Error update: {:?}", e);

                response("ERROR", "gagal mengubah spare part")
            }
        }
}

#[doc = "function to delete spare part"]
pub async fn delete_spare_part(mut req: Request<PgPool>) -> tide::Result<Response> {
    let param: DelParam = req.body_json().await?;
    let pool = req.state();

    match sqlx::query("delete from spare_part where id=$1;")
        .bind(param.id)
        .execute(pool).await {
            Ok(_x) => {response("OK", "berhasil menghapus spare part")},
            Err(e) => {
                eprintln!("Error query: {}", e);

                response("ERROR", "gagal menghapus spare part")
            }
        }
}
