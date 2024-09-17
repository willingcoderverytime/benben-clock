use sea_orm::{ColumnTrait, Condition, EntityTrait, IntoActiveModel, QueryFilter};
use sea_orm::ActiveValue::Set;
use sea_orm::sea_query::Expr;
use serde::{Deserialize, Serialize};
use tauri::command;
use uuid::Uuid;

use crate::config::dbconnetion::get_db;
use crate::entity::task::{ActiveModel, Column, Entity};
use crate::util::time_util::{get_date_from_time_stamp, get_second_timestamp_by_str};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct TaskInfoDTO {
    pub no: Option<String>,
    pub name: String,
    pub desc: String,
    pub hard: String,
    pub level: String,
    pub ttype: String,
    pub start_time: String,
    pub count_tomato: Option<i64>,
    pub status: Option<String>,

}


#[command]
pub(crate) async fn add_task(task: TaskInfoDTO) {
    use crate::entity::task::Entity;
    use crate::entity::task::ActiveModel;
    let string = Uuid::new_v4().to_string();
    let i = get_second_timestamp_by_str(task.start_time.as_str());
    let model = ActiveModel {
        id: Default::default(),
        no: Set(string),
        name: Set(task.name),
        desc: Set(task.desc),
        ttype: Set(task.ttype),
        status: Set("0".to_owned()),
        hard: Set(task.hard),
        level: Set(task.level),
        start_time: Set(i),
        cash_tomato: Default::default(),
    };
    Entity::insert(model).exec(get_db().expect(format!("获取db失败！").as_str())).await.unwrap();
}

// status          0  初始化    1 开始  2 取消 3 暂停 4 完成
// time 时间模式    0 今日 1 本周  2 本月
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct QueryTaskDTO {
    pub status: Option<String>,
    pub time: String,
}

#[command]
pub(crate) async fn query_task(query: QueryTaskDTO) -> Vec<TaskInfoDTO> {
    use crate::entity::task::Column;

    rest_yesterday_task().await;
    let mut condition = Condition::all();
    if query.status.is_some() {
        condition = condition.add(Column::Status.eq(query.status.unwrap()));
    }
    let mut start = crate::util::time_util::get_today_time_stamp();
    let mut end = match query.time.as_str() {
        "0" => { start + 24 * 3600 }
        "1" => { start + 7 * 24 * 3600 }
        "2" => { start + 30 * 24 * 3600 }
        _ => { start }
    };
    if query.time == "3" {
        end = start;
        start = end - 7 * 24 * 3600;
    }

    let condition1 = condition
        .add(Column::StartTime.gte(start))
        .add(Column::StartTime.lt(end));
    let vec = Entity::find().filter(condition1).all(get_db().expect(format!("获取db失败！").as_str())).await.unwrap_or_else(|e| {
        println!("{:?}", e);
        Vec::new()
    });
    let mut task_list = Vec::<TaskInfoDTO>::new();
    for x in vec {
        let x1 = TaskInfoDTO {
            no: Some(x.no),
            name: x.name,
            desc: x.desc,
            hard: x.hard,
            level: x.level,
            ttype: x.ttype,
            start_time: get_date_from_time_stamp(x.start_time),
            count_tomato: x.cash_tomato,
            status: Some(x.status),
        };
        task_list.push(x1);
    };

    task_list
}


#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct ControlDTO {
    pub no: String,
    //0 未开始 1 start  2 stop 3 finish 4 取消
    pub action: String,
}

#[command]
pub(crate) async fn control_task(action: ControlDTO) {
    use crate::entity::task::Column;
    let condition = Condition::all().add(Column::No.eq(action.no));
    let option = Entity::find().filter(condition.clone()).one(get_db().expect(format!("获取db失败！").as_str())).await.unwrap().unwrap();
    let mut model = option.into_active_model();
    if action.action == "3" || action.action == "4" {
        model.level = Set("0".to_string());
    }
    model.status = Set(action.action);
    Entity::update(model).filter(condition).exec(get_db().expect(format!("获取db失败！").as_str())).await.unwrap();
}


#[command]
pub(crate) async fn add_tomato_task(no: String) {
    use crate::entity::task::Column;
    let condition = Condition::all().add(Column::No.eq(no));
    let option = Entity::find().filter(condition.clone()).one(get_db().expect(format!("获取db失败！").as_str())).await.unwrap().unwrap();
    let new_count = &option.cash_tomato.unwrap_or_else(||0) + 1;
    let mut model = option.into_active_model();
    model.status = Set("2".to_owned());
    model.cash_tomato = Set(Some(new_count));
    Entity::update(model).filter(condition).exec(get_db().expect(format!("获取db失败！").as_str())).await.unwrap();
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub(crate) struct TaskStatics {
    finish_task_num: i64,
    un_finish_task_num: i64,
    cost_tomato_num: i64,
    tomorrow_task_num: i64,
    yesterday_task_num: i64,
}

///
#[command]
pub(crate) async fn query_task_statics() -> TaskStatics {
    let today = crate::util::time_util::get_today_time_stamp();
    let start = today - (24 * 3600);
    let end = today + (24 * 3600);
    let condition1 = Condition::all()
        .add(Column::StartTime.gte(start))
        .add(Column::StartTime.lte(end));
    let tasks = Entity::find().filter(condition1).all(get_db().expect(format!("获取db失败！").as_str())).await.unwrap_or_else(|e| {
        println!("{:?}", e);
        Vec::new()
    });
    let mut task_statics = TaskStatics {
        finish_task_num: 0,
        un_finish_task_num: 0,
        cost_tomato_num: 0,
        tomorrow_task_num: 0,
        yesterday_task_num: 0,
    };
    for task in tasks {
        if task.start_time == today {
            if task.status == "3" {
                task_statics.finish_task_num += 1;
            }
            task_statics.cost_tomato_num += task.cash_tomato.unwrap_or_else(|| { 0 });
        }
        if task.start_time == start {
            if task.status == "3" {
                task_statics.yesterday_task_num += 1;
            }
        }
        if task.start_time == end {
            task_statics.tomorrow_task_num += 1;
        }
    }
    task_statics
}

pub(crate) async fn rest_yesterday_task() {
    let today = crate::util::time_util::get_today_time_stamp();
    let start = today - (24 * 3600);
    let condition1 = Condition::all()
        .add(Column::StartTime.eq(start))
        .add(Column::Status.is_not_in(vec!["3", "4"]));
    /*    let tasks = Entity::find().filter(condition1).all().await.unwrap_or_else(|e| {
            println!("{:?}", e);
            Vec::new()
        });*/
    let update_result = Entity::update_many()
        .col_expr(Column::Status, Expr::value("0"))
        .col_expr(Column::CashTomato, Expr::value(Some(0)))
        .col_expr(Column::StartTime, Expr::value(today))
        .filter(condition1)
        .exec(get_db().expect(format!("获取db失败！").as_str()))
        .await;
}

