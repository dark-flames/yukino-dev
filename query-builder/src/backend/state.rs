use std::collections::HashMap;
use std::fmt::{Result, Write};

use crate::DatabaseValue;

pub type Token = String;
pub type PlaceHolder = String;

#[derive(Default)]
pub struct QueryBuildState {
    params: HashMap<usize, DatabaseValue>,
    counter: usize,
    tokens: Vec<Token>,
}

impl QueryBuildState {
    pub fn create_param(&mut self, v: &DatabaseValue) -> PlaceHolder {
        self.params.insert(self.counter, v.clone());
        let place_holder = format!("${}", self.counter);
        self.counter += 1;

        place_holder
    }

    pub fn append_param(&mut self, v: &DatabaseValue) -> Result {
        let place_holder = self.create_param(v);

        write!(self, "{}", place_holder)
    }
}

impl Write for QueryBuildState {
    fn write_str(&mut self, s: &str) -> Result {
        self.tokens.push(s.to_string());
        Ok(())
    }
}
