use salvo::prelude::*;

use crate::routes::user;

pub fn user() -> Router {
    return Router::with_path("/user")
        .get(user::get_user)
        .post(user::post_user)
        .delete(user::delete_user);
}
