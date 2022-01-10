use yukino::prelude::*;
use yukino::WithPrimaryKey;
use yukino_tests::*;

#[test]
fn test_parse_primary_key() {
    let foo = create_foo();

    assert_eq!(Foo::primary_key_name(), "id");
    assert_eq!(*foo.primary_key(), 114514);
}

#[test]
fn test_get() {
    let query = Foo::get(114514).generate_query().0;

    println!("{}", query)
}

#[test]
fn test_delete() {
    let foo = create_foo();

    let query = foo.delete().generate_query().0;

    println!("{}", query)
}
