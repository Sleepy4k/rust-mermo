use sqlx::{PgPool};
use crate::{response, response_with_data};
use tide::{Body, Request, Response};

#[doc = "Define the struct of the parameter for \"find_product\""]
#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct FindParam {
    id: i32,
}

#[doc = "Define the struct of the parameter for \"delete_product\""]
#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct DelParam {
    id: i32,
}

#[doc = "Define the struct of the parameter for \"get_product\", \"add_product\", \"find_product\", \"update_product\""]
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Product {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub amount:Option<i32>,
    pub price:Option<i32>
}

#[doc = "function to get all product"]
pub async fn get_product(req: Request<PgPool>) -> tide::Result<Response> {
    match req.query() {
        Ok(param) => {
            let pool = req.state();

            let mut prod: Vec<Product> = sqlx::query_as!(Product,
                "select id, name, amount, price from product;")
                .fetch_all(pool).await?;

            response_with_data("OK", "berhasil menampilkan data", prod)
        }
        Err(e) => {
            eprintln!("Error get: {:?}", e);
            
            response("ERROR", "gagal menampilkan product")
        }
    }
}

#[doc = "function to add product"]
pub async fn add_product(mut req: Request<PgPool>) -> tide::Result<Response> {
    let param: Product = req.body_json().await?;
    let pool = req.state();

    match sqlx::query("insert into product (name, amount, price) values ($1, $2, $3);")
        .bind(param.name)
        .bind(param.amount)
        .bind(param.price)
        .execute(pool).await {
            Ok(row) => {response("OK", "berhasil menambahkan product")}
            Err(e) => {response("ERROR", "gagal menambahkan product")}
        }
}

#[doc = "function to find product"]
pub async fn find_product(req: Request<PgPool>) -> tide::Result<Response> {
    match req.query() {
        Ok(param) => {
            let param: FindParam = param;
            let pool = req.state();

            let mut prod: Vec<Product> = sqlx::query_as!(Product,
                "select id, name, amount, price from product where id=$1;")
                .bind(param.id)
                .fetch_all(pool).await?;

            response_with_data("OK", "berhasil menampilkan detail data", prod)
        }
        Err(e) => {
            eprintln!("Error find: {:?}", e);

            response("ERROR", "gagal menampilkan detail product")
        }
    }
}

#[doc = "function to update product"]
pub async fn update_product(mut req:Request<PgPool>) -> tide::Result<Response> {
    let param: Product = req.body_json().await?;
    let pool = req.state();

    match sqlx::query("update product set name=$1, amount=$2, price=$3 where id=$4;")
        .bind(param.name)
        .bind(param.amount)
        .bind(param.price)
        .bind(param.id)
        .execute(pool).await {
            Ok(row) => {
                response("OK", "berhasil mengubah product")
            }
            Err(e) => {
                response("ERROR", "gagal mengubah product")
            }
        }
}

#[doc = "function to delete product"]
pub async fn delete_product(req: Request<PgPool>) -> tide::Result<Response> {
    match req.query() {
        Ok(param) => {
            let param: DelParam = param;
            let pool = req.state();

            match sqlx::query("delete from product where id=$1;")
                .bind(param.id)
                .execute(pool).await {
                    Ok(row) => {
                        response("OK", "berhasil menghapus product")
                    },
                    Err(e) => {
                        eprintln!("Error query: {}", e);
                        response("ERROR", "gagal menghapus product")
                    }
                }
        }
        Err(e) => {
            eprintln!("Error delete: {:?}", e);

            response("ERROR", "gagal menghapus product")
        }
    }
}
