/*
 * file: env.rs
 * author: kota kato 2022
 * description:
 *   Environment for AWK
 */

use crate::ast::def::AWKVal;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct AWKEnv {
    fields: Vec<String>,
    env: HashMap<String, AWKVal>,
}

impl AWKEnv {
    pub fn new() -> AWKEnv {
        AWKEnv {
            fields: vec![],
            env: HashMap::new(),
        }
    }
    pub fn set_field(&mut self, s: &str) {
        self.fields = s.trim().split_whitespace().map(|f| f.to_string()).collect();
    }
    pub fn set_field_n(&mut self, n: usize, s: &AWKVal) {
        if n <= 0 {
            // TODO: Error Handling
            panic!();
        };
        if self.fields.len() < n {
            for _ in 1..=(n - self.fields.len()) {
                self.fields.push("".to_string());
            }
        };
        self.fields[n - 1] = s.to_str();
    }
    pub fn get_field(&self, n: usize) -> Result<String, ()> {
        if n == 0 {
            Ok(self.fields.join(" "))
        } else if 1 <= n {
            if n <= self.fields.len() {
                Ok(self.fields[n - 1].clone())
            } else {
                Ok("".to_string())
            }
        } else {
            Err(())
        }
    }

    pub fn set_value(&mut self, name: &str, val: &AWKVal) {
        self.env.insert(name.to_string(), val.clone());
    }
    pub fn get_value(&self, name: &str) -> AWKVal {
        match self.env.get(name) {
            Some(v) => v.clone(),
            None => AWKVal::None,
        }
    }
}
