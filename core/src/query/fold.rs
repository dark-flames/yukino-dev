use query_builder::SelectSource;

use crate::query::{AliasGenerator, Map, QueryResultMap};
use crate::view::{
    AggregateViewTag, ExprViewBoxWithTag, InList, TagList, Value, ValueCount, ViewBox,
};

#[allow(dead_code)]
pub struct FoldQueryResult<T1: Value, T1Tag: TagList>
where
    AggregateViewTag: InList<T1Tag>,
{
    query: Box<dyn SelectSource>,
    alias_generator: AliasGenerator,
    view: ExprViewBoxWithTag<T1, T1Tag>,
}

impl<T1: Value, T1Tag: TagList> FoldQueryResult<T1, T1Tag>
where
    AggregateViewTag: InList<T1Tag>,
{
    pub fn create(
        query: Box<dyn SelectSource>,
        alias_generator: AliasGenerator,
        view: ExprViewBoxWithTag<T1, T1Tag>,
    ) -> Self {
        FoldQueryResult {
            query,
            alias_generator,
            view,
        }
    }
}

impl<T1: Value, T1Tag: TagList> Map<ExprViewBoxWithTag<T1, T1Tag>> for FoldQueryResult<T1, T1Tag>
where
    AggregateViewTag: InList<T1Tag>,
{
    fn map<
        R: 'static,
        RL: ValueCount,
        RV: Into<ViewBox<R, RL>>,
        F: Fn(ExprViewBoxWithTag<T1, T1Tag>) -> RV,
    >(
        mut self,
        f: F,
    ) -> QueryResultMap<R, RL> {
        let mut result = f(self.view).into();
        let mut visitor = self.alias_generator.substitute_visitor();
        result.apply_mut(&mut visitor);

        QueryResultMap::create(self.query, self.alias_generator, result)
    }
}

macro_rules! fold_query_result {
    ($name: ident, $([$param: ident, $tag: ident]),*) => {
        #[allow(dead_code)]
        pub struct $name<$($param: Value, $tag: TagList),*>
            where
                AggregateViewTag: $(InList<$tag>+)*
        {
            query: Box<dyn SelectSource>,
            alias_generator: AliasGenerator,
            view: ($(ExprViewBoxWithTag<$param, $tag>,)*),
        }

        impl<$($param: Value, $tag: TagList),*> $name<$($param, $tag),*>
        where AggregateViewTag: $(InList<$tag>+)* {
            pub fn create(
                query: Box<dyn SelectSource>,
                alias_generator: AliasGenerator,
                view: ($(ExprViewBoxWithTag<$param, $tag>,)*)
            ) -> Self {
                $name {
                    query, alias_generator, view
                }
            }
        }

        impl<$($param: Value, $tag: TagList),*> Map<($(ExprViewBoxWithTag<$param, $tag>,)*)> for $name<$($param, $tag),*>
        where AggregateViewTag: $(InList<$tag>+)* {
            fn map<
                R: 'static,
                RL: ValueCount,
                RV: Into<ViewBox<R, RL>>,
                F: Fn(($(ExprViewBoxWithTag<$param, $tag>,)*)) -> RV,
            >(
                mut self,
                f: F,
            ) -> QueryResultMap<R, RL> {
                let mut result = f(self.view).into();
                let mut visitor = self.alias_generator.substitute_visitor();
                result.apply_mut(&mut visitor);

                QueryResultMap::create(self.query, self.alias_generator, result)
            }
        }
    }
}

macro_rules! generate_fold_trait {
    ($({$method: ident, $name: ident, $([$param: ident, $tag: ident]),*}),*) => {
        $(fold_query_result!($name, $([$param, $tag]),*);)*
        pub trait Fold<View> {
            fn fold<
                R1: Value, R1Tag: TagList,
                F: Fn(View) -> ExprViewBoxWithTag<R1, R1Tag>
            >(self, f: F) -> FoldQueryResult<R1, R1Tag> where AggregateViewTag: InList<R1Tag>;

            $(
                fn $method<
                    $($param: Value, $tag: TagList,)*
                    F: Fn(View) -> ($(ExprViewBoxWithTag<$param, $tag>,)*)
                >(self, f: F) -> $name<$($param, $tag),*> where AggregateViewTag: $(InList<$tag> +)*;

            )*
        }
    }
}

generate_fold_trait!(
    {fold2, FoldQueryResult2, [T1, T1Tag], [T2, T2Tag]}
);
