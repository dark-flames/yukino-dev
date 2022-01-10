use yukino::prelude::*;
use yukino_tests::create_foo;

#[test]
fn test_insert() {
    let test = create_foo();

    let query = test.insert_all().generate_query().0;

    println!("{}", query)
}
