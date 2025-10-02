use utoipa::OpenApi;
use crate::models::{CreateUser, User, CreateDish, Dish, DietaryRestriction, DishCategory};
use crate::routes::users::__path_create_user;
use crate::routes::users::__path_get_users;
use crate::routes::users::__path_modify_user;
use crate::routes::users::__path_remove_user;
use crate::routes::dishes::__path_create_dish;
use crate::routes::dishes::__path_get_dishes;
use crate::routes::dishes::__path_modify_dish;
use crate::routes::dishes::__path_remove_dish;

#[derive(OpenApi)]
#[openapi(
    paths(
        create_user,
        get_users,
        modify_user,
        remove_user,
        create_dish,
        get_dishes,
        modify_dish,
        remove_dish,
    ),
    components(
        schemas(CreateUser, User, CreateDish, Dish, DietaryRestriction, DishCategory)
    ),
    tags(
        (name = "users", description = "User management endpoints"),
        (name = "dishes", description = "Dish management endpoints")
    )
)]
pub struct ApiDoc;