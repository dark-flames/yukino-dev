use yukino::prelude::*;
use yukino_tests::create_foo;

#[test]
fn test_insert() {
    let test = create_foo();

    let query = test.insert().generate_query().0;

    println!("{}", query)
}
