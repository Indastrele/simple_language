use core::f64;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Variables {
    variables: HashMap<String, f64>,
}

#[allow(dead_code)]
impl Variables {
    pub fn new() -> Self {
        Variables {
            variables: HashMap::<String, f64>::from([
                ("PI".to_string(), f64::consts::PI),
                ("E".to_string(), f64::consts::E),
                ("TAU".to_string(), f64::consts::TAU),
            ]),
        }
    }

    pub fn does_exist(&self, key: &'static str) -> bool {
        self.variables.contains_key(key)
    }

    pub fn get(&self, key: String) -> f64 {
        if let Some(data) = self.variables.get(&key) {
            return *data;
        }

        f64::MIN
    }

    pub fn set(&mut self, key: String, val: f64) {
        self.variables.insert(key, val);
    }
}
