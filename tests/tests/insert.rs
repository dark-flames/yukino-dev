use yukino::query::Executable;
use yukino::view::Insertable;
use yukino_tests::Foo;

#[test]
fn test_insert() {
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
        character: 'c',
        optional: None
    };

    let query = test.insert().generate_query().0;


    println!("{}", query)
}