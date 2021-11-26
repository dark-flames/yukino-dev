use yukino::{bt, eq, lt};
use yukino::operator::{average, bit_and};
use yukino::query::{Filter, Fold, GroupBy, Map};
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
        .fold(|b| (average(b.short), bit_and(b.int)))
        .map(|(a, _)| a + 16)
        .generate_query();

    println!("{}", query);
}

#[test]
fn test_group() {
    let query = Basic::all()
        .filter(|b| lt!(b.int, 114514))
        .filter(|b| bt!(b.int, 1919))
        .group_by(|b| (b.int, b.short))
        .filter(|(a, _)| eq!(a, 910))
        .fold(|(a, b)| (average(a), average(b)))
        .map(|(a, _)| a)
        .generate_query();

    println!("{}", query);
}