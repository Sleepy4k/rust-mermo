use sqlx::{PgPool};
use crate::{ws_response};
use tide::{Body, Request, Response};

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

#[doc = "Define the struct of the parameter for \"get_spare_part\", \"add_spare_part\", \"find_spare_part\", \"update_spare_part\""]
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct SparePart {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub price:Option<i32>
}

#[doc = "function to get all spare part"]
pub async fn get_spare_part(req: Request<PgPool>) -> tide::Result<Response> {
    match req.query() {
        Ok(param) => {
            let pool = req.state();

            let mut sparepart: Vec<SparePart> = sqlx::query_as!(AppPartList,
                "select id, name, price from spare_part;")
                .fetch_all(pool).await?;
        
            let resp = Response::builder(200)
                .body(Body::from_json(&sparepart)?)
                .build();
        
            Ok(resp)
        }
        Err(e) => {
            eprintln!("Error get: {:?}", e);
            
            let resp = Response::builder(200)
                .body(Body::from_json(&ws_response(false, "gagal menampilkan spare part"))?)
                .build();

            Ok(resp)
        }
    }
}

#[doc = "function to add spare part"]
pub async fn add_spare_part(mut req: Request<PgPool>) -> tide::Result<Response> {
    match req.query() {
        Ok(param) => {
            let param: SparePart = req.body_json().await?;
            let pool = req.state();

            match sqlx::query("insert into spare_part (name, price) values ($1, $2);")
                .bind(name)
                .bind(price)
                .execute(pool).await {
                    Ok(row) => {
                        let resp = Response::builder(200)
                            .body(Body::from_json(&ws_response(true, "berhasil menambahkan spare part"))?)
                            .build();

                        Ok(resp)
                    }
                    Err(e) => {
                        let resp = Response::builder(200)
                            .body(Body::from_json(&ws_response(false, "gagal menambahkan spare part"))?)
                            .build();

                        Ok(resp)
                    }
                }
        }
        Err(e) => {
            eprintln!("Error add: {:?}", e);
            
            let resp = Response::builder(200)
                .body(Body::from_json(&ws_response(false, "gagal menambahkan spare part"))?)
                .build();

            Ok(resp)
        }
    }
}

#[doc = "function to find spare part"]
pub async fn find_spare_part(req: Request<PgPool>) -> tide::Result<Response> {
    match req.query() {
        Ok(param) => {
            let param: FindParam = param;
            let pool = req.state();

            let mut sparepart: Vec<SparePart> = sqlx::query_as!(SparePart,
                "select id, name, price from spare_part where id=$1;")
                .bind(param.id)
                .fetch_all(pool).await?;

            let resp = Response::builder(200)
                .body(Body::from_json(&sparepart)?)
                .build();

            Ok(resp)
        }
        Err(e) => {
            eprintln!("Error find: {:?}", e);
            let resp = Response::builder(200)
                .body(Body::from_json(&ws_response(false, msg.as_str()))?)
                .build();

            Ok(resp)
        }
    }
}

#[doc = "function to update spare part"]
pub async fn update_spare_part(mut req:Request<PgPool>) -> tide::Result<Response> {
    match req.query() {
        Ok(param) => {
            let param: SparePart = req.body_json().await?;
            let pool = req.state();

            match sqlx::query("update spare_part set name=$1, price=$2 where id=$3;")
                .bind(param.name)
                .bind(param.price)
                .bind(param.id)
                .execute(pool).await {
                    Ok(row) => {
                        let resp = Response::builder(200)
                            .body(Body::from_json(&ws_response(true, "berhasil mengubah spare part"))?)
                            .build();

                        Ok(resp)
                    }
                    Err(e) => {
                        let resp = Response::builder(200)
                            .body(Body::from_json(&ws_response(false, "gagal mengubah spare part"))?)
                            .build();

                        Ok(resp)
                    }
                }
        }
        Err(e) => {
            eprintln!("Error update: {:?}", e);

            let resp = Response::builder(200)
                .body(Body::from_json(&ws_response(false, "gagal mengubah spare part"))?)
                .build();

            Ok(resp)
        }
    }
}

#[doc = "function to delete spare part"]
pub async fn delete_spare_part(req: Request<PgPool>) -> tide::Result<Response> {
    match req.query() {
        Ok(param) => {
            let param: DelParam = param;
            let pool = req.state();

            match sqlx::query("delete from spare_part where id=$1;")
                .bind(param.id)
                .execute(pool).await {
                    Ok(row) => {
                        ws_response(true, "berhasil menghapus spare part")
                    },
                    Err(e) => {
                        eprintln!("Error query: {}", e);
                        ws_response(false, "gagal menghapus spare part")
                    }
                }
        }
        Err(e) => {
            eprintln!("Error delete: {:?}", e);

            let resp = Response::builder(200)
                .body(Body::from_json(&ws_response(false, "gagal menghapus spare part"))?)
                .build();

            Ok(resp)
        }
    }
}
