use yukino::*;
use yukino::Association;
use yukino::operator::{SubqueryExists, VerticalJoin};
use yukino::query::{BelongsToQueryResult, BelongsToView, Executable, Filter, Fold, Map};
use yukino::view::{EntityWithView, ExprView, SubqueryView};
use yukino_tests::*;

#[test]
fn test_association_impl() {
    let bar = Bar {
        foo_id: 1,
        name: "test".to_string()
    };
    assert_eq!(Bar::foreign_key_name(), "foo_id");
    assert_eq!(*bar.foreign_key(), 1);
}

#[test]
fn test_association_query() {
    let foo = Foo::all()
        .filter(|f| lt!(f.int, 114514));

    let bar = Bar::belonging_to_query(foo);

    let query = bar.filter(
        |b| eq!(b.name, "test".to_string())
    ).generate_query().0;

    println!("{}", query)
}

#[test]
fn test_subquery_from_view() {
    let query = Foo::all()
        .filter(|f| lt!(f.int, 114514))
        .filter(|f| {
            Bar::belonging_to_view(&f).map(
                |b| b.name
            ).exists()
        }).map(|f| {
            Bar::belonging_to_view(&f).fold(
                |b| b.name.join(Some(", "))
            ).expr_clone()
        }).generate_query().0;

    println!("{}", query)
}

#[test]
fn test_subquery_fn() {
    let query = Foo::all()
        .filter(|f| {
            lt!(f.string.clone(), Bar::belonging_to_view(&f).map(
                |b| b.name
            ).all())
        })
        .filter(|f| {
            bt!(Bar::belonging_to_view(&f).map(
                |b| b.name
            ).any(), f.string)
        }).map(|f| {
        Bar::belonging_to_view(&f).fold(
            |b| b.name.join(Some(", "))
        ).expr_clone()
    }).generate_query().0;

    println!("{}", query)
}