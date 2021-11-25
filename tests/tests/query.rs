use yukino::{bt, lt};
use yukino::operator::average;
use yukino::query::{Filter, Fold, Map};
use yukino::view::{EntityWithView, ExprView};
use yukino_tests::schema::*;

#[test]
fn test_filter_map() {
    let query = Basic::all()
        .filter(|b| lt!(b.clone().int, 114514))
        .map(|b| b.clone().int * 2 + 114514)
        .generate_query();

    println!("{}", query);
}

#[test]
fn test_fold() {
    let query = Basic::all()
        .filter(|b| lt!(b.clone().int, 114514))
        .filter(|b| bt!(b.clone().int, 1919))
        .fold(|b| average(b.short.clone()))
        .map(|v| v.expr_clone() + 16)
        .generate_query();

    println!("{}", query);
}