use yukino::{bt, eq, lt};
use yukino::operator::{VerticalAverage, VerticalBitAnd};
use yukino::query::{ExecutableSelectQuery, Filter, Fold, GroupBy, GroupFold, Map, Map2, Sort};
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
        .fold(|b| (b.short.average(), b.int.bit_and()))
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
        .fold(|(a, b)| (a.average(), b.average()))
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
        .fold_group(|b| b.long.average())
        .map(|(_, b), c| (b, c))
        .generate_query()
        .0;

    println!("{}", query);
}
