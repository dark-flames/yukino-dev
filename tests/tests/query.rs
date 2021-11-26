use yukino::lt;
use yukino::query::{Filter, Map};
use yukino::view::EntityWithView;
use yukino_tests::schema::*;

#[test]
fn test_filter_map() {
    let query = Basic::all()
        .filter(|b| lt!(b.clone().int, 114514))
        .map(|b| b.clone().int * 2 + 114514)
        .generate_query();

    println!("{}", query);
}
