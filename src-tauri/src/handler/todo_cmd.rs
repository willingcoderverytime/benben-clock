use sea_orm::{ColumnTrait, Condition, EntityTrait, QueryFilter};
use sea_orm::ActiveValue::Set;
use serde::{Deserialize, Serialize};
use tauri::command;
use uuid::Uuid;
use log::info;

use crate::config::dbconnetion::get_db;
use crate::entity::todo::{ActiveModel, Column, Entity};
use crate::util::time_util::{get_second_timestamp_by_str, get_today_time_stamp};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct TodoDTO {
    pub no: Option<String>,
    pub name: String,
    pub desc: String,
}


#[command]
pub(crate) async fn add_todo(info: TodoDTO) {
    let string = Uuid::new_v4().to_string();
    let model = ActiveModel {
        id: Default::default(),
        no: Set(string),
        name: Set(info.name),
        desc: Set(info.desc),
    };
    Entity::insert(model).exec(get_db().expect(format!("获取db失败！").as_str())).await.unwrap();
}


#[command]
pub(crate) async fn query_todo() -> Vec<TodoDTO> {
    let condition = Condition::all();
    let vec = Entity::find().filter(condition).all(get_db().expect(format!("获取db失败！").as_str())).await.unwrap_or_else(|e| {
        println!("{:?}", e);
        Vec::new()
    });
    let mut todo_list = Vec::<TodoDTO>::new();
    for x in vec {
        let x1 = TodoDTO {
            no: Some(x.no),
            name: x.name,
            desc: x.desc,
        };
        todo_list.push(x1);
    };
    todo_list
}


#[command]
pub(crate) async fn add_to_task(info: TodoDTO)  {
    let condition = Condition::all().add(Column::No.eq(info.no));

    let vec = Entity::find().filter(condition.clone()).one(get_db().expect(format!("获取db失败！").as_str())).await.unwrap_or_else(|e| {
        println!("{:?}", e);
        None
    });
    if vec.is_some() {
        let task = vec.unwrap();
        use crate::entity::task::Entity;
        use crate::entity::task::ActiveModel;
        let string = Uuid::new_v4().to_string();
        let i = get_today_time_stamp();
        let model = ActiveModel {
            id: Default::default(),
            no: Set(string),
            name: Set(task.name),
            desc: Set(task.desc),
            ttype: Set("common".to_owned()),
            status: Set("0".to_owned()),
            hard: Set("1".to_owned()),
            level: Set("1".to_owned()),
            start_time: Set(i),
            cash_tomato: Default::default(),
        };
        Entity::insert(model).exec(get_db().expect(format!("获取db失败！").as_str())).await.unwrap();
    }
    Entity::delete_many().filter(condition).exec(get_db().expect(format!("获取db失败！").as_str())).await.unwrap();

}


