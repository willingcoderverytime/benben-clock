//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;


#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "task_info")]
pub struct Model {
    #[sea_orm(column_name = "Id", primary_key)]
    pub id: i32,
    #[sea_orm(column_name = "No")]
    pub no: String,
    #[sea_orm(column_name = "Name")]
    pub name: String,
    #[sea_orm(column_name = "Desc")]
    pub desc: String,
    #[sea_orm(column_name = "Type")]
    pub ttype: String,
    #[sea_orm(column_name = "Status")]
    pub status: String,
    #[sea_orm(column_name = "Hard")]
    pub hard: String,
    #[sea_orm(column_name = "Level")]
    pub level: String,
    #[sea_orm(column_name = "StartTime")]
    pub start_time: i64,
    #[sea_orm(column_name = "CashTomato")]
    pub cash_tomato: Option<i64>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}


