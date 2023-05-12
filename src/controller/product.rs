use sqlx::PgPool;
use tide::{Request, Response};
use crate::{response, response_with_data};

#[doc = "Define the struct of the parameter for \"get_or_find_product\", \"update_product\""]
#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct Product {
    id: i32,
    name: String,
    amount: i32,
    price: i32
}

#[doc = "Define the struct of the parameter for \"get_or_find_product\""]
#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct FindProduct {
    id: i32,
}

#[doc = "Define the struct of the parameter for \"add_product\""]
#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct AddProduct {
    name: String,
    amount: i32,
    price: i32
}

#[doc = "Define the struct of the parameter for \"delete_product\""]
#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct DeleteProduct {
    id: i32,
}

#[doc = "function to get or find product"]
pub async fn get_or_find_product(req: Request<PgPool>) -> tide::Result<Response> {
    match req.query() {
        Ok(query_param) => {
            let param: FindProduct = query_param;
            let pool = req.state();

            let prod: Vec<Product> = sqlx::query_as!(Product,
                "select * from product where id = $1",
                param.id
            ).fetch_all(pool).await?;

            response_with_data("OK", "berhasil menampilkan detail data", prod)
        },
        Err(_) => {
            let pool = req.state();

            let prod: Vec<Product> = sqlx::query_as!(Product,
                "select * from product"
            ).fetch_all(pool).await?;

            response_with_data("OK", "berhasil menampilkan product", prod)
        }
    }
}

#[doc = "function to add product"]
pub async fn add_product(mut req: Request<PgPool>) -> tide::Result<Response> {
    let param: AddProduct = req.body_json().await?;
    let pool = req.state();

    match sqlx::query!(
        "insert into product (name, amount, price) values ($1, $2, $3)",
        param.name, param.amount, param.price
    ).execute(pool).await {
        Ok(_x) => {response("OK", "berhasil menambahkan product")}
        Err(e) => {
            eprintln!("Error add: {:?}", e);

            response("ERROR", "gagal menambahkan product")
        }
    }
}

#[doc = "function to update product"]
pub async fn update_product(mut req:Request<PgPool>) -> tide::Result<Response> {
    let param: Product = req.body_json().await?;
    let pool = req.state();

    match sqlx::query!(
        "update product set name = $1, amount = $2, price = $3 where id = $4",
        param.name, param.amount, param.price, param.id
    ).execute(pool).await {
        Ok(_x) => {response("OK", "berhasil mengubah product")}
        Err(e) => {
            eprintln!("Error update: {:?}", e);

            response("ERROR", "gagal mengubah product")
        }
    }
}

#[doc = "function to delete product"]
pub async fn delete_product(mut req: Request<PgPool>) -> tide::Result<Response> {
    let param: DeleteProduct = req.body_json().await?;
    let pool = req.state();

    match sqlx::query!(
        "delete from product where id = $1",
        param.id
    ).execute(pool).await {
        Ok(_x) => {response("OK", "berhasil menghapus product")},
        Err(e) => {
            eprintln!("Error query: {}", e);
            
            response("ERROR", "gagal menghapus product")
        }
    }
}
