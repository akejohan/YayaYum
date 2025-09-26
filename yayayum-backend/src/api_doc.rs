use utoipa::OpenApi;
use crate::models::{CreateUser, User};
use crate::routes::users::__path_create_user;

#[derive(OpenApi)]
#[openapi(
    paths(
        create_user,
    ),
    components(
        schemas(CreateUser, User)
    ),
    tags(
        (name = "users", description = "User management endpoints")
    )
)]
pub struct ApiDoc;