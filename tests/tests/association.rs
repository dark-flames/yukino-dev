use yukino::*;
use yukino::Association;
use yukino::query::{BelongsToQueryResult, ExecutableSelectQuery, Filter};
use yukino::view::EntityWithView;
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

    let bar = Bar::belonging_to(foo);

    let query = bar.filter(
        |b| eq!(b.name, "test".to_string())
    ).generate_query().0;

    println!("{}", query)
}