use yukino::prelude::*;
use yukino_tests::*;

#[test]
fn test_update() {
    let query = Foo::all()
        .filter(|b| lt!(b.int, 114514))
        .filter(|b| bt!(b.int, 1919))
        .update()
        .set(foo::boolean, false)
        .set_default(foo::id)
        .set_by(foo::long, |l| l + 1)
        .sort(|f| f.id.asc())
        .limit(20)
        .generate_query();

    println!("{}", query);
}
