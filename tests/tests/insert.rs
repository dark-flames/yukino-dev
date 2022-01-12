use yukino::prelude::*;
use yukino_tests::{create_foo, create_new_foo};

#[test]
fn test_insert() {
    let test = create_foo();

    let query = test.insert().generate_query();

    println!("{}", query)
}

#[test]
fn test_new() {
    let test = create_new_foo();
    let query = test.insert().generate_query();

    println!("{}", query)
}
