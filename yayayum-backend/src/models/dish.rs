use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug, PartialEq)]
pub enum DietaryRestriction {
    Vegetarian,
    Vegan,
    GlutenFree,
    DairyFree,
    NutFree,
    Halal,
    Kosher,
    LowCarb,
    Keto,
    None,
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug, PartialEq)]
pub enum DishCategory {
    WokWithNoodles,
    SpecialDish,
    Stew,
    WokWithRice,
    Ramen,
    KidsMenu,
    SideOrder,
}

#[derive(Serialize, ToSchema, Deserialize, FromRow)]
pub struct Dish {
    pub id: i32,
    pub nr: i32,
    pub name: String,
    pub description: String,
    pub price_kr: i32,
    pub dietary_restrictions: Vec<DietaryRestriction>,
    pub category: DishCategory,
}

#[derive(Deserialize, ToSchema)]
pub struct CreateDish {
    pub nr: i32,
    pub name: String,
    pub description: String,
    pub price_kr: i32,
    pub dietary_restrictions: Vec<DietaryRestriction>,
    pub category: DishCategory,
}