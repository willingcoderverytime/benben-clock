// Copyright 2024-2025 BenBen ID within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT
// @author benben
// @date  2024-05-13

use std::{
    collections::HashMap,
    sync::{Arc, OnceLock},
};
// todo 后续切换成 tokio + onece
use std::sync::Mutex;

/// [ConfigKV] 用于存储减值对
/// 通过Config OnceLock 的value 可以实现全局变量的一直的生命周期
/// OnceLock 允许其在const 以及全局状态先进行使用。
/// 通过Arc + Mutex + HashMap 实现全局 互斥 可变的HashMap变量。
#[derive(Debug, Clone)]
pub struct ConfigKV<T: Clone> {
    pub value: OnceLock<Arc<Mutex<HashMap<String, T>>>>,
}

// todo 将此段代码进行macro化？？？ 是否有意义， 似乎没得意义纯纯的炫技行为。
// let dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
impl<T: Clone> ConfigKV<T> {
    pub const fn new() -> ConfigKV<T> {
        let once_lock = OnceLock::new();
        ConfigKV { value: once_lock }
    }
    fn get(&self) -> Arc<Mutex<HashMap<String, T>>> {
        self.value
            .get_or_init(|| Arc::new(Mutex::new(HashMap::<String, T>::new())))
            .clone()
    }
    pub fn put_config_by_key(&self, config_key: String, t: T) {
        let once_lock = self.value.get();

        if let None = once_lock {
            self.get();
        }

        let xx = self.value.get().unwrap();
        let mut config = xx.lock().unwrap();
        config.insert(config_key, t);
    }
    pub fn get_config_by_key(&self, config_key: &String) -> Option<T> {
        let once_lock = self.value.get();

        if let None = once_lock {
            self.get();
        }

        let xx = self.value.get().unwrap();
        let config = xx.lock().unwrap();
        let x = config.get(config_key);
        match x {
            None => None,
            Some(a) => {
                let a = a.clone();
                Some(a)
            }
        }
    }
}

#[test]
fn test_create_config() {
    static mut ROOT_CONFIG: ConfigKV<String> = ConfigKV::<String>::new();

    {
        let test = unsafe { ROOT_CONFIG.get() };
        test.lock()
            .unwrap()
            .insert("String".to_string(), "String".to_string());
        let mut maps = test.lock().unwrap();
        maps.insert("String".to_string(), "String".to_string());
        let value = maps.get("String");
        println!("{:?}", value);
        println!("{:?}", maps);
    }
    {
        let test = unsafe { ROOT_CONFIG.get() };
        let xxx = test.lock().unwrap();
        println!("{:?}", xxx);
    }
    {
        let test = unsafe { ROOT_CONFIG.get() };
        let xxx = test.lock().unwrap();
        println!("{:?}", xxx);
    }
}

/// Trait [InitConfig]
pub trait InitConfig {
    fn init_config() -> Self;
}

/// [ConfigStruct] 用于存储结构化配置
///
///
///
#[derive(Debug)]
pub struct ConfigStruct<T: Send + Sync + 'static + InitConfig> {
    value: OnceLock<Arc<Mutex<StructInfo<T>>>>,
}

/// 用于内部存储Mutex中的可变值的
///
#[derive(Debug, Clone)]
struct StructInfo<T> {
    info: T,
}

impl<T: InitConfig + Clone + Send + Sync + 'static> ConfigStruct<T> {
    pub const fn new() -> ConfigStruct<T> {
        let once_lock = OnceLock::new();
        ConfigStruct { value: once_lock }
    }
    fn get(&self) -> Arc<Mutex<StructInfo<T>>> {
        self.value
            .get_or_init(|| {
                Arc::new(Mutex::new(StructInfo {
                    info: T::init_config(),
                }))
            })
            .clone()
    }

    pub fn get_struct(&self) -> &'static T {
        let once_lock = self.value.get();

        if let None = once_lock {
            self.get();
        }

        let xx = self.value.get().unwrap();
        let x = xx.lock().unwrap();
        unsafe {
            let x = &x.info as *const T;
            &*x
        }
    }
    pub fn get_struct_into(&self) -> T {
        let once_lock = self.value.get();

        if let None = once_lock {
            self.get();
        }

        let xx = self.value.get().unwrap();
        let x = xx.lock().unwrap();
        x.info.clone()
    }
    pub fn set_struct(&mut self, t: T) {
        let xx = self.value.get();
        if let None = xx {
            self.get();
        }
        let mut x = self.value.get().unwrap().lock().unwrap();
        x.info = t;
    }
}
