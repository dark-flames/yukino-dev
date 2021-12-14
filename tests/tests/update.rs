use yukino::{bt, lt};
use yukino::query::{Executable, Filter, Sort, Update};
use yukino::view::EntityWithView;
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
        .sort(|f, helper| helper.asc(f.id))
        .limit(20)
        .generate_query().0;

    println!("{}", query);
}