use yukino::query::Executable;
use yukino::view::{Deletable, Identifiable};
use yukino::WithPrimaryKey;
use yukino_tests::*;

#[test]
fn test_parse_primary_key() {
    let test = Foo {
        id: 114514,
        boolean: false,
        u_short: 0,
        short: 0,
        u_int: 0,
        int: 0,
        u_long: 0,
        long: 0,
        float: 0.0,
        double: 0.0,
        string: "".to_string(),
        optional: None,
    };

    assert_eq!(Foo::primary_key_name(), "id");
    assert_eq!(*test.primary_key(), 114514);
}

#[test]
fn test_get() {
    let query = Foo::get(114514).generate_query().0;

    println!("{}", query)
}

#[test]
fn test_delete() {
    let test = Foo {
        id: 114514,
        boolean: false,
        u_short: 0,
        short: 0,
        u_int: 0,
        int: 0,
        u_long: 0,
        long: 0,
        float: 0.0,
        double: 0.0,
        string: "".to_string(),
        optional: None,
    };

    let query = test.delete().generate_query().0;

    println!("{}", query)
}
