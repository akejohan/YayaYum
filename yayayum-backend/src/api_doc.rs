use utoipa::OpenApi;
use crate::models::{CreateUser, User};
use crate::routes::users::__path_create_user;
use crate::routes::users::__path_get_users;
use crate::routes::users::__path_remove_user;

#[derive(OpenApi)]
#[openapi(
    paths(
        create_user,
        get_users,
        remove_user,
    ),
    components(
        schemas(CreateUser, User)
    ),
    tags(
        (name = "users", description = "User management endpoints")
    )
)]
pub struct ApiDoc;