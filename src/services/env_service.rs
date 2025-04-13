use crate::model::so::app_config::AppConfig;
use dotenv::dotenv;
use std::cell::RefCell;
use std::env;
use std::sync::OnceLock;
static ENV_SERVICE: OnceLock<EnvService> = OnceLock::new();

#[derive(Debug)]
pub struct EnvService {
    cache: RefCell<std::collections::HashMap<String, String>>,
}

impl EnvService {
    pub fn instance() -> &'static EnvService {
        ENV_SERVICE.get_or_init(|| {
            dotenv().ok();

            EnvService {
                cache: RefCell::new(std::collections::HashMap::new()),
            }
        })
    }

    pub fn get_required(&self, key: &str) -> String {
        // Check cache first
        if let Some(cached_value) = self.cache.borrow().get(key) {
            return cached_value.clone();
        }

        let value = env::var(key).unwrap_or_else(|_| {
            panic!("Required environment variable '{}' not found", key)
        });

        // Update cache
        self.cache.borrow_mut().insert(key.to_string(), value.clone());

        value
    }

    pub fn get_or_default(&self, key: &str, default: &str) -> String {
        // Check cache first
        if let Some(cached_value) = self.cache.borrow().get(key) {
            return cached_value.clone();
        }

        let value = env::var(key).unwrap_or_else(|_| default.to_string());
        self.cache.borrow_mut().insert(key.to_string(), value.clone());

        value
    }

    pub fn get_as<T: std::str::FromStr>(&self, key: &str, default: T) -> T
    where
        T::Err: std::fmt::Debug,
    {
        let string_value = self.get_or_default(key, &default.to_string());
        string_value.parse::<T>().unwrap_or_else(|_| {
            eprintln!(
                "Warning: Could not parse environment variable '{}' with value '{}' as requested type, using default",
                key, string_value
            );
            default
        })
    }

    pub fn cache_value(&self, key: &str, value: &str) {
        self.cache.borrow_mut().insert(key.to_string(), value.to_string());
    }

    pub fn reload(&self) {
        dotenv().ok();
        self.cache.borrow_mut().clear();
    }
}

pub trait AppEnv {
    fn app_port(&self) -> u16;
    fn app_host(&self) -> String;
    fn file_path(&self) -> String;
    fn allow_create(&self) -> bool;
    fn enable_folder(&self) -> bool;
    fn app_config(&self) -> AppConfig;
}

impl AppEnv for EnvService {
    fn app_port(&self) -> u16 {
        self.get_as("APP_PORT", 8080)
    }

    fn app_host(&self) -> String {
        self.get_required("APP_HOST")
    }

    fn file_path(&self) -> String {
        self.get_required("FILE_PATH")
    }

    fn allow_create(&self) -> bool {
        self.get_as("ALLOW_CREATE", false)
    }

    fn enable_folder(&self) -> bool {
        self.get_as("ENABLE_FOLDER", false)
    }

    fn app_config(&self) -> AppConfig {
        AppConfig {
            file_path: self.file_path().into(),
        }
    }
}