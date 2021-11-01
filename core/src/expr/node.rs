use crate::converter::Converter;
use crate::db::ty::{DatabaseValue, ValuePack};
use crate::err::{DataConvertError, RuntimeResult, YukinoError};
use crate::expr::computation::Computation;
use crate::expr::Value;
use crate::query::SelectedItem;

pub trait Node: Computation {
    fn collect_selected_items(&self) -> Vec<SelectedItem>;
}

pub enum Expr<T: Value> {
    QueryResult(QueryResultNode<T>),
    Computation(Box<dyn ComputationNode<Output=T>>),
    Const(ConstNode<T>),
}

#[derive(Clone)]
pub struct QueryResultNode<T: Value> {
    pub converter: &'static dyn Converter<Output=T>,
    pub aliases: Vec<String>,
}

pub trait ComputationNode: Node {
    fn box_clone(&self) -> Box<dyn ComputationNode<Output=Self::Output>>;
}

#[derive(Clone)]
pub struct ConstNode<T: Value> {
    value: T,
    converter: &'static dyn Converter<Output=T>,
}

impl<T: Value> Clone for Expr<T> {
    fn clone(&self) -> Self {
        match self {
            Expr::QueryResult(n) => Expr::QueryResult(n.clone()),
            Expr::Computation(n) => Expr::Computation(n.box_clone()),
            Expr::Const(n) => Expr::Const(n.clone()),
        }
    }
}

impl<T: Value> ConstNode<T> {
    pub fn to_database_value(&self) -> RuntimeResult<Vec<DatabaseValue>> {
        self.converter.serialize(&self.value)
    }
}

impl<T: Value> Computation for QueryResultNode<T> {
    type Output = T;

    fn eval(&self, v: &ValuePack) -> RuntimeResult<Self::Output> {
        let value = self
            .aliases
            .iter()
            .map(|alias| {
                v.get(alias)
                    .ok_or_else(|| DataConvertError::DataNotFound(alias.clone()).as_runtime_err())
            })
            .collect::<RuntimeResult<Vec<_>>>()?;

        let deserializer = self.converter.deserializer();
        (*deserializer)(&value)
    }
}

impl<T: Value> Computation for ConstNode<T> {
    type Output = T;

    fn eval(&self, _v: &ValuePack) -> RuntimeResult<Self::Output> {
        Ok(self.value.clone())
    }
}

impl<T: Value> Computation for Expr<T> {
    type Output = T;

    fn eval(&self, v: &ValuePack) -> RuntimeResult<Self::Output> {
        match self {
            Expr::QueryResult(n) => n.eval(v),
            Expr::Computation(n) => n.eval(v),
            Expr::Const(n) => n.eval(v),
        }
    }
}

impl<T: Value> Node for QueryResultNode<T> {
    fn collect_selected_items(&self) -> Vec<SelectedItem> {
        todo!()
    }
}

impl<T: Value> Node for ConstNode<T> {
    fn collect_selected_items(&self) -> Vec<SelectedItem> {
        vec![]
    }
}

impl<T: Value> Node for Expr<T> {
    fn collect_selected_items(&self) -> Vec<SelectedItem> {
        match self {
            Expr::QueryResult(n) => n.collect_selected_items(),
            Expr::Computation(n) => n.collect_selected_items(),
            Expr::Const(n) => n.collect_selected_items(),
        }
    }
}
