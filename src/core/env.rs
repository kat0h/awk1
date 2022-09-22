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
}
