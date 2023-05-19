use tide::Server;
use sqlx::{Pool, Postgres};
use crate::{
    get_or_find_spare_part, add_spare_part, update_spare_part, delete_spare_part,
    get_or_find_product, add_product, update_product, delete_product,
    login, logout, register, welcome,
};

#[doc = "Path for spare part and product"]
pub async fn path(app: &mut Server<Pool<Postgres>>) {
    app.at("/")
        .get(welcome)
        .post(welcome)
        .put(welcome)
        .delete(welcome);

    app.at("/login").post(login);
    app.at("/logout").post(logout);
    app.at("/register").post(register);

    app.at("/spare_part")
        .get(get_or_find_spare_part)
        .post(add_spare_part)
        .put(update_spare_part)
        .delete(delete_spare_part);

    app.at("/product")
        .get(get_or_find_product)
        .post(add_product)
        .put(update_product)
        .delete(delete_product);
}