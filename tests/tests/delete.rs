use yukino::prelude::*;
use yukino_tests::*;

#[test]
fn test_delete() {
    let query = Foo::all()
        .filter(|b| lt!(b.int, 114514))
        .filter(|b| bt!(b.int, 1919))
        .sort(|f| f.id.asc())
        .delete()
        .limit(10)
        .generate_query()
        .0;

    println!("{}", query);
}
