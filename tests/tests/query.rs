use yukino::{bt, eq, lt};
use yukino::query_result::{
    ExecutableSelectQuery, Filter, Fold, GroupBy, GroupFold, Map, Map2, Sort,
};
use yukino::view::EntityWithView;
use yukino_tests::schema::*;

#[test]
fn test_filter_map() {
    let query = Basic::all()
        .filter(|b| lt!(b.int, 114514))
        .generate_query()
        .0;

    println!("{}", query);
}

#[test]
fn test_fold() {
    let query = Basic::all()
        .filter(|b| lt!(b.int, 114514))
        .filter(|b| bt!(b.int, 1919))
        .fold(|b, helper| (helper.average(b.short), helper.bit_and(b.int)))
        .generate_query()
        .0;

    println!("{}", query);
}

#[test]
fn test_group() {
    let query = Basic::all()
        .filter(|b| lt!(b.int, 114514))
        .filter(|b| bt!(b.int, 1919))
        .group_by(|b| (b.int, b.short))
        .filter(|(a, _)| eq!(a, 910))
        .fold(|(a, b), helper| (helper.average(a), helper.average(b)))
        .generate_query()
        .0;

    println!("{}", query);
}

#[test]
fn test_order_by() {
    let query = Basic::all()
        .filter(|b| lt!(b.int, 114514))
        .sort(|b, helper| helper.asc(b.int))
        .generate_query()
        .0;

    println!("{}", query);
}

#[test]
fn test_group_order_by() {
    let query = Basic::all()
        .filter(|b| lt!(b.int, 114514))
        .filter(|b| bt!(b.int, 1919))
        .group_by(|b| (b.int, b.short))
        .filter(|(a, _)| eq!(a, 910))
        .sort(|(a, b), helper| (helper.asc(a), helper.desc(b)))
        .generate_query()
        .0;

    println!("{}", query);
}

#[test]
fn test_map() {
    let query = Basic::all()
        .filter(|b| lt!(b.int, 114514))
        .filter(|b| bt!(b.int, 1919))
        .group_by(|b| (b.int, b.short))
        .filter(|(a, _)| eq!(a, 910))
        .sort(|(a, b), helper| (helper.asc(a), helper.desc(b)))
        .map(|(a, _)| a)
        .generate_query()
        .0;

    println!("{}", query);
}

#[test]
fn test_group_fold_map() {
    let query = Basic::all()
        .filter(|b| lt!(b.int, 114514))
        .filter(|b| bt!(b.int, 1919))
        .group_by(|b| (b.int, b.short))
        .fold_group(|b, helper| helper.average(b.long))
        .map(|(_, b), c| (b, c))
        .generate_query()
        .0;

    println!("{}", query);
}
