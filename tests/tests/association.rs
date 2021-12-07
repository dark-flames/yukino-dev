use yukino::Association;
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