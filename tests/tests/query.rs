use yukino::{bt, lt};
use yukino::operator::average;
use yukino::query::{Filter, Fold, Map};
use yukino::view::EntityWithView;
use yukino_tests::schema::*;

#[test]
fn test_filter_map() {
    let query = Basic::all()
        .filter(|b| lt!(b.int, 114514))
        .map(|b| b.int * 2 + 114514)
        .generate_query();

    println!("{}", query);
}

#[test]
fn test_fold() {
    let query = Basic::all()
        .filter(|b| lt!(b.int, 114514))
        .filter(|b| bt!(b.int, 1919))
        .fold(|b| average(b.short))
        .map(|v| v + 16)
        .generate_query();

    println!("{}", query);
}