/*
use yukino::interface::EntityView;
use yukino::query::Alias;
use yukino::view::{Value, ViewBox};
use yukino_tests::schema::*;

pub fn cmp_view<T: Value>(view: ViewBox<T>, query: &str) {
    assert_eq!(
        query.to_string(),
        view.collect_expr().first().unwrap().to_string()
    );
}


pub fn test_expr() {
    let alias = Alias {
        name: "b".to_string(),
    };

    let view = BasicView::pure(&alias);

    let add_const = view.int.clone() + 114514;
    let sub_const = view.int.clone() - 114514;
    let mul_const = view.int.clone() * 114514;
    let div_const = view.int.clone() / 114514;
    let add = view.long.clone() + view.long.clone();
    let sub = view.long.clone() - view.long.clone();
    let mul = view.long.clone() * view.long.clone();
    let div = view.long.clone() / view.long;

    cmp_view(add_const, "b.int + 114514");
    cmp_view(sub_const, "b.int - 114514");
    cmp_view(mul_const, "b.int * 114514");
    cmp_view(div_const, "b.int / 114514");
    cmp_view(add, "b.long + b.long");
    cmp_view(sub, "b.long - b.long");
    cmp_view(mul, "b.long * b.long");
    cmp_view(div, "b.long / b.long");
}*/

use crate::Test::{Bool, Int};

enum Test {
    Int(usize),
    Bool(bool),
}
