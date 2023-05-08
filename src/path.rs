use crate::{
    // get_spare_part, add_spare_part, find_spare_part, update_spare_part, delete_spare_part,
    get_product, add_product, update_product, delete_product,
    // get_all_users, edit_pass, delete_user,
    // signup, signin, signout,
};
use sqlx::{Pool, Postgres};
use tide::{Server};

pub async fn path(app: &mut Server<Pool<Postgres>>) {
    // app.at("/signup").post(signup);
    // app.at("/signin").post(signin);
    // app.at("/signout").post(signout);

    // app.at("/users")
    //     .get(get_all_users)
    //     .put(edit_pass)
    //     .delete(delete_user);

    // app.at("/spare_part")
    //     .get(get_all_spare_part)
    //     .post(add_spare_part)
    //     .put(update_spare_part)
    //     .delete(delete_spare_part);
    // app.at("/spare_part/:id").get(get_spare_part);

    app.at("/product")
        .get(get_product)
        .post(add_product)
        .put(update_product)
        .delete(delete_product);
    // app.at("/product/:id").get(find_product);
}