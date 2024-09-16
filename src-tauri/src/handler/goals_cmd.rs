use std::collections::HashMap;
use sea_orm::{ColumnTrait, Condition, DeriveColumn, EntityTrait, EnumIter, IntoActiveModel, QueryFilter, QuerySelect};
use sea_orm::ActiveValue::Set;
use sea_orm::sea_query::Expr;
use serde::{Deserialize, Serialize};
use tauri::command;
use uuid::Uuid;

use crate::config::dbconnetion::get_db;
use crate::entity::goals::{ActiveModel, Column, Entity};
use crate::entity::task;
use crate::util::time_util::{get_date_from_time_stamp, get_second_timestamp_by_str};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct GoalsDTO {
    pub no: Option<String>,
    pub name: String,
    pub desc: String,
    pub start_time: String,
    pub count_tomato: Option<i64>,
}


#[command]
pub(crate) async fn add_goals(goals: GoalsDTO) {
    let string = Uuid::new_v4().to_string();
    let i = get_second_timestamp_by_str(goals.start_time.as_str());
    let model = ActiveModel {
        id: Default::default(),
        no: Set(string),
        name: Set(goals.name),
        desc: Set(goals.desc),
        start_time: Set(i),
        cash_tomato: Default::default(),
    };
    Entity::insert(model).exec(get_db().expect(format!("获取db失败！").as_str())).await.unwrap();
}


#[command]
pub(crate) async fn query_goals() -> Vec<GoalsDTO> {
    let condition = Condition::all();

    let vec = Entity::find().filter(condition).all(get_db().expect(format!("获取db失败！").as_str())).await.unwrap_or_else(|e| {
        tracing::error!("{:?}", e);
        Vec::new()
    });
    let mut task_list = Vec::<GoalsDTO>::new();
    for x in vec {
        let x1 = GoalsDTO {
            no: Some(x.no),
            name: x.name,
            desc: x.desc,
            start_time: get_date_from_time_stamp(x.start_time),
            count_tomato: x.cash_tomato,
        };
        task_list.push(x1);
    };

    task_list
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
enum QueryAs {
    GoalsNo,
    CostTomato,
}

#[command]
pub(crate) async fn query_goals_detail() -> Vec<GoalsDTO> {
    let condition = Condition::all();

    let vec = Entity::find().filter(condition).all(get_db().expect(format!("获取db失败！").as_str())).await.unwrap_or_else(|e| {
        tracing::error!("{:?}", e);
        Vec::new()
    });
    let mut task_list = Vec::<GoalsDTO>::new();

    let res: Vec<(String, i64)> = task::Entity::find()
        .select_only()
        .column_as(task::Column::Ttype, QueryAs::GoalsNo)
        .column_as(task::Column::CashTomato.count(), QueryAs::CostTomato)
        .group_by(task::Column::Ttype)
        .into_values::<_, QueryAs>()
        .all(get_db().expect("get db failed"))
        .await.unwrap();

    for x in vec {
        let option = res.iter().find(|(key, num)| key.as_str() == x.no);
        let mut cash_tomato = 0;
        if option.is_some() {
            cash_tomato = option.unwrap().1;
        }
        let x1 = GoalsDTO {
            no: Some(x.no.clone()),
            name: x.name,
            desc: x.desc,
            start_time: get_date_from_time_stamp(x.start_time),
            count_tomato: Some(cash_tomato),
        };
        task_list.push(x1);
    };

    task_list
}



