use generic_array::typenum::U1;

use crate::view::{ExistsView, ExprViewBox, ExprViewBoxWithTag, InSubqueryView, NotExistsView, SubqueryView, TagList, Value};

pub trait InSubquery<T: Value<L=U1>> {
    fn in_subquery<S: SubqueryView<T>>(&self, subquery: S) -> ExprViewBox<bool>;
}

pub trait SubqueryExists<T: Value<L=U1>>: SubqueryView<T> {
    fn exists(&self) -> ExprViewBox<bool>;

    fn not_exists(&self) -> ExprViewBox<bool>;
}

impl<T: Value<L=U1>> InSubquery<T> for T {
    fn in_subquery<S: SubqueryView<T>>(&self, subquery: S) -> ExprViewBox<bool> {
        let view = self.view();
        Box::new(InSubqueryView::create(
            view.collect_expr().into_iter().next().unwrap(),
            subquery.subquery()
        ))
    }
}

impl<T: Value<L=U1>, TTags: TagList> InSubquery<T> for ExprViewBoxWithTag<T, TTags> {
    fn in_subquery<S: SubqueryView<T>>(&self, subquery: S) -> ExprViewBox<bool> {
        Box::new(InSubqueryView::create(
            self.collect_expr().into_iter().next().unwrap(),
            subquery.subquery()
        ))
    }
}

impl<T: Value<L=U1>, Subquery: SubqueryView<T>> SubqueryExists<T> for Subquery {
    fn exists(&self) -> ExprViewBox<bool> {
        Box::new(ExistsView::create(self.subquery()))
    }

    fn not_exists(&self) -> ExprViewBox<bool> {
        Box::new(NotExistsView::create(self.subquery()))
    }
}