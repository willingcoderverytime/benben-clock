// Copyright 2024-2025 BenBen ID within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT
// @author benben
// @date  2024-05-16

use std::fs;
use std::fs::File;
use std::time::Duration;

use sea_orm::{ConnectionTrait, ConnectOptions, Database, DatabaseConnection};
use crate::config::config::{ConfigStruct, InitConfig};


/// [DBCOnfig] 用于存储共享状态的DataBase
///
#[derive(Debug, Clone)]
struct DBCOnfig {
    db: Option<DatabaseConnection>,
    init_able: bool,
}

impl InitConfig for DBCOnfig {
    fn init_config() -> Self {
        DBCOnfig {
            db: None,
            init_able: false,
        }
    }
}

static mut DBCONFIG: ConfigStruct<DBCOnfig> = ConfigStruct::<DBCOnfig>::new();

impl DBCOnfig {
    fn get_db(&self ) -> &'static DatabaseConnection {
        unsafe {
            let x = &self.db;
            if let Some(x) = x {
                let x: *const DatabaseConnection = x as *const DatabaseConnection;
                &*x
            } else {
                panic!("local db init failed or not init")
            }
        }
    }
}

///  [init_db]dataBase init function
///  and store it as a global varible
// @author benben
// @date  2024-05-16
/// # Platforms
/// # Examples
/// ```,no_run
///
/// ```
/// # Panics
///
///
pub async fn init_db() {
    let local_db = init_local_db().await;
    unsafe {
        DBCONFIG.set_struct(DBCOnfig {
            db: Some(local_db),
            init_able: true,
        })
    }
}

async fn init_local_db() -> DatabaseConnection {
    let mut _path: std::path::PathBuf = std::env::current_dir().unwrap();
    _path.push(".benben\\local_db.db");
    // 判断文件是否存在，若是不存在则进行创建并进行sql 脚本初始化。
    let mut need_create_table = false;
    if !_path.exists() {
        let _ = File::create(&_path).unwrap();
        need_create_table = true;
    }
    // sqlite 默认情况下是单线程进行使用
    let x = String::from("sqlite:") + _path.to_str().unwrap() + "?mode=rwc";
    let sqlite_opt = ConnectOptions::new(x);
    let local_db = Database::connect(sqlite_opt).await.unwrap();
    if need_create_table {
        // 需要进行table创建
        let mut _path: std::path::PathBuf = std::env::current_dir().unwrap();
        _path.push(".benben\\local_db.ddl");
        let result = tokio::fs::read_to_string(_path.clone()).await.unwrap();

        let result =  match tokio::fs::read_to_string(_path.clone()).await {
            Ok(re) => {re}
            Err(e) => {
                tracing::error!("读取文件失败: {}", e);
                panic!("读取ddl失败")
            }
        };
        local_db.execute_unprepared(result.as_str()).await.unwrap_or_else(|err|{
            tracing::info!("数据库脚本执行失败");
            match fs::remove_file(_path) {
                Ok(()) => tracing::info!("文件删除成功"),
                Err(e) => tracing::error!("删除文件时出错: {}", e),
            }
            panic!("数据库脚本执行失败")
        });
        tracing::info!("数据库脚本执行成功");
    }
    local_db
}



pub fn get_db() -> Result<&'static DatabaseConnection, String> {
    let db_config = unsafe { DBCONFIG.get_struct() };
    if !db_config.init_able {
        return Err("db init faild or not init , may try init_db(url) in your code".to_owned());
    }
    let get_db = db_config.get_db();
    // let rt = tokio::runtime::Builder::new_multi_thread().enable_all().build().unwrap();
    Ok(get_db)
}

pub fn close_db() {
    unsafe {
        let db_config: DBCOnfig = DBCONFIG.get_struct_into();
        tokio::spawn(db_config.db.unwrap().close());
    }
}




