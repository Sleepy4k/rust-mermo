use crate::{
    get_spare_part, add_spare_part, find_spare_part, update_spare_part, delete_spare_part,
    get_product, add_product, find_product, update_product, delete_product,
    login, logout
};
use tide::Server;
use sqlx::{Pool, Postgres};

#[doc = "Path for spare part and product"]
pub async fn path(app: &mut Server<Pool<Postgres>>) {
    // Init auth path
    app.at("/login").post(login);
    app.at("/logout").post(logout);
    // app.at("/register").post(register);

    // Init spare part path
    app.at("/spare_parts").get(get_spare_part);
    app.at("/spare_part")
        .get(find_spare_part)
        .post(add_spare_part)
        .put(update_spare_part)
        .delete(delete_spare_part);

    // Init product path
    app.at("/products").get(get_product);
    app.at("/product")
        .get(find_product)
        .post(add_product)
        .put(update_product)
        .delete(delete_product);
}