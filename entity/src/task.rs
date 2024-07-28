//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "task")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i32,
    pub description: String,
    #[sea_orm(column_type = "Timestamp")]
    pub complete_date: Option<DateTimeUtc>,
}

impl Model {
    pub fn new(description: String) -> Model {
        Model {
            id: 0,
            description,
            complete_date: None,
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
