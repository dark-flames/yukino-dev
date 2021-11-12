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

    let add_const = view.int.clone() + 114514;
    let sub_const = view.int.clone() - 114514;
    let mul_const = view.int.clone() * 114514;
    let div_const = view.int.clone() / 114514;
    let rem_const = view.int.clone() % 114514;
    let shl_const = view.int.clone() << 114;
    let shr_const = view.int.clone() >> 114;
    let and_const = view.int.clone() & 114;
    let or_const = view.int.clone() | 114;
    let xor_const = view.int.clone() ^ 114;
    let add = view.long.clone() + view.long.clone();
    let sub = view.long.clone() - view.long.clone();
    let mul = view.long.clone() * view.long.clone();
    let div = view.long.clone() / view.long.clone();
    let rem = view.long.clone() % view.long.clone();
    let shl = view.long.clone() << view.long.clone();
    let shr = view.long.clone() >> view.long.clone();
    let and = view.long.clone() & view.long.clone();
    let or= view.long.clone() | view.long.clone();
    let xor = view.long.clone() ^ view.long;

    cmp_view(add_const, "b.int + 114514");
    cmp_view(sub_const, "b.int - 114514");
    cmp_view(mul_const, "b.int * 114514");
    cmp_view(div_const, "b.int / 114514");
    cmp_view(rem_const, "b.int % 114514");
    cmp_view(shl_const, "b.int << 114");
    cmp_view(shr_const, "b.int >> 114");
    cmp_view(and_const, "b.int & 114");
    cmp_view(or_const, "b.int | 114");
    cmp_view(xor_const, "b.int ^ 114");
    cmp_view(add, "b.long + b.long");
    cmp_view(sub, "b.long - b.long");
    cmp_view(mul, "b.long * b.long");
    cmp_view(div, "b.long / b.long");
    cmp_view(rem, "b.long % b.long");
    cmp_view(shl, "b.long << b.long");
    cmp_view(shr, "b.long >> b.long");
    cmp_view(and, "b.long & b.long");
    cmp_view(or, "b.long | b.long");
    cmp_view(xor, "b.long ^ b.long");
}
