use std::collections::HashMap;
use std::fmt::{Result, Write};
use std::marker::PhantomData;

use crate::DatabaseValue;

pub type Token = String;
pub type PlaceHolder = String;

#[derive(Default)]
pub struct QueryBuildState<'t> {
    params: HashMap<usize, DatabaseValue>,
    counter: usize,
    tokens: Vec<Token>,
    _marker: PhantomData<&'t u8>
}

impl<'t> QueryBuildState<'t> {
    pub fn create_static() -> QueryBuildState<'static> {
        QueryBuildState {
            params: Default::default(),
            counter: 0,
            tokens: vec![],
            _marker: Default::default()
        }
    }
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

    pub fn sub(&'t self) -> QueryBuildState<'t> {
        QueryBuildState {
            params: Default::default(),
            counter: self.counter,
            tokens: vec![],
            _marker: Default::default()
        }
    }

    pub fn merge(&mut self, state: QueryBuildState) {
        self.counter = state.counter;
        self.tokens.extend(state.tokens);
        self.params.extend(state.params);
    }
}

impl<'t> Write for QueryBuildState<'t> {
    fn write_str(&mut self, s: &str) -> Result {
        self.tokens.push(s.to_string());
        Ok(())
    }
}
