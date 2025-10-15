use utoipa::OpenApi;
use crate::models::{CreateUser, User, CreateDish, Dish, DietaryRestriction, DishCategory, CreateRating, Rating};
use crate::routes::users::__path_create_user;
use crate::routes::users::__path_get_users;
use crate::routes::users::__path_modify_user;
use crate::routes::users::__path_remove_user;
use crate::routes::dishes::__path_create_dish;
use crate::routes::dishes::__path_get_dishes;
use crate::routes::dishes::__path_modify_dish;
use crate::routes::dishes::__path_remove_dish;
use crate::routes::ratings::__path_create_rating;
use crate::routes::ratings::__path_get_ratings;
use crate::routes::ratings::__path_get_rating;
use crate::routes::ratings::__path_get_ratings_by_dish;
use crate::routes::ratings::__path_get_ratings_by_user;
use crate::routes::ratings::__path_modify_rating;
use crate::routes::ratings::__path_remove_rating;

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
        create_rating,
        get_ratings,
        get_rating,
        get_ratings_by_dish,
        get_ratings_by_user,
        modify_rating,
        remove_rating
    ),
    components(
        schemas(CreateUser, User, CreateDish, Dish, DietaryRestriction, DishCategory, CreateRating, Rating)
    ),
    tags(
        (name = "users", description = "User management endpoints"),
        (name = "dishes", description = "Dish management endpoints"),
        (name = "ratings", description = "Rating management endpoints")
    )
)]
pub struct ApiDoc;