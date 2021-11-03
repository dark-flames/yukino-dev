use yukino::interface::EntityView;
use yukino::query::Alias;
use yukino::view::{Value, ViewBox};
use yukino_tests::schema::*;

pub fn cmp_view<T: Value>(view: ViewBox<T>, query: &str) {
    assert_eq!(
        query.to_string(),
        view.collect_expr().first().unwrap().to_string()
    );
}

#[test]
pub fn test_expr() {
    let view = BasicView::pure(Alias {
        name: "b".to_string(),
    });

    let a = view.int.clone() + 114514;
    let b = view.int.clone() - 114514;
    let c = view.int.clone() * 114514;
    let d = view.int / 114514;

    cmp_view(a, "b.int + 114514");
    cmp_view(b, "b.int - 114514");
    cmp_view(c, "b.int * 114514");
    cmp_view(d, "b.int / 114514");
}
