use yukino::operator::*;
use yukino::query_builder::Alias;
use yukino::view::*;
use yukino::{and, bt, bte, eq, lt, lte, neq, or};
use yukino_tests::schema::*;

pub fn cmp_view<T: Value>(view: ExprViewBox<T>, query: &str) {
    assert_eq!(
        view.collect_expr().into_iter().next().unwrap().to_string(),
        query.to_string(),
    );
}

#[test]
pub fn test_numeric() {
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

    let add = view.long.clone() + view.long.clone();
    let sub = view.long.clone() - view.long.clone();
    let mul = view.long.clone() * view.long.clone();
    let div = view.long.clone() / view.long.clone();
    let rem = view.long.clone() % view.long.clone();
    let shl = view.long.clone() << view.long.clone();
    let shr = view.long.clone() >> view.long.clone();
    let and = view.long.clone() & view.long.clone();
    let or = view.long.clone() | view.long.clone();
    let xor = view.long.clone() ^ view.long;

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

#[test]
pub fn test_boolean() {
    let alias = Alias {
        name: "b".to_string(),
    };

    let view = BasicView::pure(&alias);

    let and_const = and!(view.boolean.clone(), true);
    let or_const = or!(view.boolean.clone(), false);
    let eq_const = eq!(view.boolean.clone(), false);
    let neq_const = neq!(view.u_int.clone(), 114514);
    let bt_const = bt!(view.string.clone(), "test".to_string());
    let bte_const = bte!(view.character.clone(), 'c');
    let lt_const = lt!(view.double.clone(), 114.514);
    let lte_const = gt!(view.float.clone(), 19.19);

    cmp_view(and_const, "b.boolean AND true");
    cmp_view(or_const, "b.boolean OR false");
    cmp_view(eq_const, "b.boolean == false");
    cmp_view(neq_const, "b.u_int != 114514");
    cmp_view(bt_const, "b.string > \"test\"");
    cmp_view(bte_const, "b.character >= 'c'");
    cmp_view(lt_const, "b.double < 114.514");
    cmp_view(lte_const, "b.float < 19.19");

    let and = and!(view.boolean.clone(), view.boolean.clone());
    let or = or!(view.boolean.clone(), view.boolean.clone());
    let eq = eq!(view.boolean.clone(), view.boolean);
    let neq = neq!(view.u_int.clone(), view.u_int);
    let bt = bt!(view.string.clone(), view.string);
    let bte = bte!(view.character.clone(), view.character);
    let lt = lt!(view.double.clone(), view.double);
    let lte = lte!(view.float.clone(), view.float);

    cmp_view(and, "b.boolean AND b.boolean");
    cmp_view(or, "b.boolean OR b.boolean");
    cmp_view(eq, "b.boolean == b.boolean");
    cmp_view(neq, "b.u_int != b.u_int");
    cmp_view(bt, "b.string > b.string");
    cmp_view(bte, "b.character >= b.character");
    cmp_view(lt, "b.double < b.double");
    cmp_view(lte, "b.float <= b.float");
}
