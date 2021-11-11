use yukino::interface::EntityView;
use yukino::query::Alias;
use yukino::view::{ExprViewBox, Value, ValueCount};
use yukino_tests::schema::*;

pub fn cmp_view<T: Value<L = L>, L: ValueCount>(view: ExprViewBox<T, L>, query: &str) {
    assert_eq!(
        query.to_string(),
        view.collect_expr().into_iter().next().unwrap().to_string()
    );
}

#[test]
pub fn test_expr() {
    let alias = Alias {
        name: "b".to_string(),
    };

    let view = BasicView::pure(&alias);

    let add_const = view.int.expr_clone() + 114514i32;
    let sub_const = view.int.expr_clone() - 114514i32;
    let mul_const = view.int.expr_clone() * 114514i32;
    let div_const = view.int.expr_clone() / 114514i32;
    let add = view.long.expr_clone() + view.long.expr_clone();
    let sub = view.long.expr_clone() - view.long.expr_clone();
    let mul = view.long.expr_clone() * view.long.expr_clone();
    let div = view.long.expr_clone() / view.long;

    cmp_view(add_const, "b.int + 114514");
    cmp_view(sub_const, "b.int - 114514");
    cmp_view(mul_const, "b.int * 114514");
    cmp_view(div_const, "b.int / 114514");
    cmp_view(add, "b.long + b.long");
    cmp_view(sub, "b.long - b.long");
    cmp_view(mul, "b.long * b.long");
    cmp_view(div, "b.long / b.long");
}
