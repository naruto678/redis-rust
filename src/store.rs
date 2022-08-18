use std::time::Duration;
use std::{collections::HashMap, time::SystemTime};

static mut STORE: Option<Store> = None;

pub struct Store {
    pub key_store: HashMap<String, String>,
    pub date_time: HashMap<String, SystemTime>,
}

impl Store {
    pub fn set_with_expiry(key: &str, val: &str, expiry_time: u64) {
        unsafe {
            let store = STORE.as_mut().unwrap();
            store.key_store.insert(key.to_string(), val.to_string());
            store.date_time.insert(
                key.to_string(),
                SystemTime::now()
                    .checked_add(Duration::from_millis(expiry_time))
                    .unwrap(),
            );
        }
    }

    pub fn init() {
        unsafe {
            STORE = Some(Store {
                key_store: HashMap::new(),
                date_time: HashMap::new(),
            })
        }
    }

    pub fn set(key: &str, val: &str) {
        unsafe {
            let key = key.trim();
            let val = val.trim();
            STORE
                .as_mut()
                .unwrap()
                .key_store
                .insert(key.to_string(), val.to_string());
        }
    }

    pub fn get(key: &str) -> Option<&String> {
        unsafe {
            let key = key.trim();
            let store = STORE.as_mut().unwrap();
            if store.key_store.contains_key(key) && store.date_time.contains_key(key) {
                let expiry_time = store.date_time.get(key).unwrap();
                let current_time = SystemTime::now();
                if current_time.gt(expiry_time) {
                    println!("Expiry time was {:?}", expiry_time);
                    println!("Current time is {:?}", current_time);
                    None
                } else {
                    store.key_store.get(key)
                }
            } else if store.key_store.contains_key(key) && !store.date_time.contains_key(key) {
                store.key_store.get(key)
            } else {
                None
            }
        }
    }
}
