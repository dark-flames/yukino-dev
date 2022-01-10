use yukino::prelude::*;
use yukino::view::TupleExprView;
use yukino_tests::*;

#[test]
fn test_filter_map() {
    let query = Foo::all().filter(|b| lt!(b.int, 114514)).generate_query().0;

    println!("{}", query);
}

#[test]
fn test_fold() {
    let query = Foo::all()
        .filter(|b| lt!(b.int, 114514))
        .filter(|b| bt!(b.int, 1919))
        .fold(|b| b.int.average())
        .generate_query()
        .0;

    println!("{}", query);
}

#[test]
fn test_group() {
    let query = Foo::all()
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
    let query = Foo::all()
        .filter(|b| lt!(b.int, 114514))
        .sort(|b| b.int.asc())
        .generate_query()
        .0;

    println!("{}", query);
}

#[test]
fn test_group_order_by() {
    let query = Foo::all()
        .filter(|b| lt!(b.int, 114514))
        .filter(|b| bt!(b.int, 1919))
        .group_by(|b| (b.int, b.short))
        .filter(|(a, _)| eq!(a, 910))
        .sort(|(a, b)| (a.asc(), b.desc()))
        .generate_query()
        .0;

    println!("{}", query);
}

#[test]
fn test_map() {
    let query = Foo::all()
        .filter(|b| lt!(b.int, 114514))
        .filter(|b| bt!(b.int, 1919))
        .group_by(|b| (b.int, b.short))
        .filter(|(a, _)| eq!(a, 910))
        .sort(|(a, b)| (a.asc(), b.desc()))
        .map(|(a, b)| (a, b))
        .generate_query()
        .0;

    println!("{}", query);
}

#[test]
fn test_group_fold_map() {
    let query = Foo::all()
        .filter(|b| lt!(b.int, 114514))
        .filter(|b| bt!(b.int, 1919))
        .group_by(|b| (b.int, b.short))
        .fold_group(|b| (b.clone().sort(|i| i.long.asc()).string.join(Some(", ")), b.int.count()))
        .map(|(a, b), (c, _)| make_tuple!(a, b, c))
        .generate_query()
        .0;

    println!("{}", query);
}
