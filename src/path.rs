use crate::{
    get_spare_part, add_spare_part, update_spare_part, delete_spare_part,
    get_product, add_product, update_product, delete_product,
};
use sqlx::{Pool, Postgres};
use tide::{Server};

#[doc = "Path for spare part and product"]
pub async fn path(app: &mut Server<Pool<Postgres>>) {
    app.at("/spare_part")
        .get(get_spare_part)
        .post(add_spare_part)
        .put(update_spare_part)
        .delete(delete_spare_part);

    app.at("/product")
        .get(get_product)
        .post(add_product)
        .put(update_product)
        .delete(delete_product);
}