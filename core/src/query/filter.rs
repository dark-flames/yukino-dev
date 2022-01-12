use std::hash::Hash;
use std::marker::PhantomData;

use sqlx::Database;

use interface::{Association, FieldMarker, WithPrimaryKey};
use query_builder::{
    Alias, Expr, IntoSelectSource, OrderByItem, Select, SelectFrom, SelectItem, SelectQuery,
    YukinoQuery,
};

use crate::operator::{In, SortResult};
use crate::query::{
    AliasGenerator, AssociationBuilder, Delete, DeleteQueryResult, Executable, Fold,
    FoldQueryResult, FoldResult, GroupBy, GroupedQueryResult, GroupResult, Map, MultiRows,
    QueryResultMap, Sort, Update, UpdateQueryResult,
};
use crate::view::{
    AssociatedView, EntityView, EntityWithView, ExprBoxOfAssociatedView, ExprView,
    ExprViewBoxWithTag, FieldMarkerWithView, TagList, TagOfMarker, TagsOfEntity, TypeOfMarker,
    Value, ViewWithPrimaryKey,
};

pub struct QueryResultFilter<E: EntityWithView> {
    query: SelectFrom,
    root_alias: Alias,
    alias_generator: AliasGenerator,
    _entity: PhantomData<E>,
}

pub struct SortedQueryResultFilter<E: EntityWithView> {
    nested: QueryResultFilter<E>,
    order_by: Vec<OrderByItem>,
}

pub trait Filter<View> {
    #[must_use]
    fn filter<F, R: Into<ExprViewBoxWithTag<bool, Tags>>, Tags: TagList>(self, f: F) -> Self
    where
        F: Fn(View) -> R;
}

pub trait Filter2<View1, View2> {
    #[must_use]
    fn filter<F, R: Into<ExprViewBoxWithTag<bool, Tags>>, Tags: TagList>(self, f: F) -> Self
    where
        F: Fn(View1, View2) -> R;
}

impl<E: EntityWithView> Filter<E::View> for QueryResultFilter<E> {
    fn filter<F, R: Into<ExprViewBoxWithTag<bool, Tags>>, Tags: TagList>(mut self, f: F) -> Self
    where
        F: Fn(E::View) -> R,
    {
        let view = f(E::View::pure(&self.root_alias)).into();

        view.collect_expr().into_iter().for_each(|e| {
            self.query.and_where(e);
        });

        self
    }
}

impl<E: EntityWithView> Map<E::View> for QueryResultFilter<E> {
    type ResultType = MultiRows;
    fn map<
        R: Value,
        RTags: TagList,
        RV: Into<ExprViewBoxWithTag<R, RTags>>,
        F: Fn(E::View) -> RV,
    >(
        self,
        f: F,
    ) -> QueryResultMap<R, RTags, Self::ResultType> {
        let result_view = f(E::View::pure(&self.root_alias)).into();

        QueryResultMap::create(
            self.query.source(),
            vec![],
            result_view,
            self.alias_generator,
        )
    }
}

impl<E: EntityWithView> Fold<E::VerticalView> for QueryResultFilter<E> {
    fn fold<RV: FoldResult, F: Fn(E::VerticalView) -> RV>(self, f: F) -> FoldQueryResult<RV> {
        let result = f(E::View::pure(&self.root_alias).vertical());

        FoldQueryResult::create(self.query.source(), result, self.alias_generator)
    }
}

impl<E: EntityWithView> GroupBy<E, E::View> for QueryResultFilter<E> {
    fn group_by<RV: GroupResult, F: Fn(E::View) -> RV>(
        self,
        f: F,
    ) -> GroupedQueryResult<RV, (), E> {
        let result = f(E::View::pure(&self.root_alias));

        GroupedQueryResult::create(
            self.query.group_by(result.collect_expr_vec()),
            result,
            self.alias_generator,
            self.root_alias,
        )
    }
}

impl<E: EntityWithView> Sort<E::View> for QueryResultFilter<E> {
    type Result = SortedQueryResultFilter<E>;

    fn sort<R: SortResult, F: Fn(E::View) -> R>(self, f: F) -> Self::Result {
        let result = f(E::View::pure(&self.root_alias));

        SortedQueryResultFilter {
            nested: self,
            order_by: result.order_by_items(),
        }
    }
}

impl<E: EntityWithView, DB: Database> Executable<E, TagsOfEntity<E>, DB> for QueryResultFilter<E>
where
    SelectQuery: YukinoQuery<DB>,
{
    type ResultType = MultiRows;
    type Query = SelectQuery;

    fn generate_query(self) -> (Self::Query, ExprViewBoxWithTag<E, TagsOfEntity<E>>) {
        let view = E::View::pure(&self.root_alias);

        (
            SelectQuery::create(
                self.query.source(),
                self.alias_generator
                    .generate_select_list(view.collect_expr().into_iter(), true),
                vec![],
                None,
                0,
            ),
            Box::new(view),
        )
    }
}

impl<E: EntityWithView> Delete<E> for QueryResultFilter<E> {
    fn delete(self) -> DeleteQueryResult<E> {
        DeleteQueryResult::create(self.query)
    }
}

impl<E: EntityWithView> Delete<E> for SortedQueryResultFilter<E> {
    fn delete(self) -> DeleteQueryResult<E> {
        DeleteQueryResult::create_with_order(self.nested.query, self.order_by)
    }
}

impl<E: EntityWithView> Update<E> for QueryResultFilter<E> {
    fn update(self) -> UpdateQueryResult<E> {
        UpdateQueryResult::create(self.query)
    }
}

impl<E: EntityWithView> Update<E> for SortedQueryResultFilter<E> {
    fn update(self) -> UpdateQueryResult<E> {
        UpdateQueryResult::create_with_orders(self.nested.query, self.order_by)
    }
}

impl<E: EntityWithView> QueryResultFilter<E> {
    pub fn create() -> Self {
        let mut generator = AliasGenerator::create();
        let root_alias = generator.generate_root_alias::<E>();
        QueryResultFilter {
            query: Select::from(E::table_name().to_string(), root_alias.clone()),
            root_alias,
            alias_generator: generator,
            _entity: Default::default(),
        }
    }
}

impl<E: EntityWithView> Map<E::View> for SortedQueryResultFilter<E> {
    type ResultType = MultiRows;

    fn map<
        R: Value,
        TTags: TagList,
        RV: Into<ExprViewBoxWithTag<R, TTags>>,
        F: Fn(E::View) -> RV,
    >(
        self,
        f: F,
    ) -> QueryResultMap<R, TTags, Self::ResultType> {
        let result_view = f(E::View::pure(&self.nested.root_alias)).into();

        QueryResultMap::create(
            self.nested.query.source(),
            self.order_by,
            result_view,
            self.nested.alias_generator,
        )
    }
}

impl<E: EntityWithView, DB: Database> Executable<E, TagsOfEntity<E>, DB>
    for SortedQueryResultFilter<E>
where
    SelectQuery: YukinoQuery<DB>,
{
    type ResultType = MultiRows;
    type Query = SelectQuery;

    fn generate_query(self) -> (Self::Query, ExprViewBoxWithTag<E, TagsOfEntity<E>>) {
        let view = E::View::pure(&self.nested.root_alias);

        (
            SelectQuery::create(
                self.nested.query.source(),
                self.nested
                    .alias_generator
                    .generate_select_list(view.collect_expr(), true),
                vec![],
                None,
                0,
            ),
            Box::new(view),
        )
    }
}

impl<
        Children: EntityWithView
            + Association<Parent, ForeignField, ForeignKeyType = TypeOfMarker<ForeignField>>,
        Parent: EntityWithView + WithPrimaryKey<PrimaryKeyType = TypeOfMarker<ForeignField>>,
        ForeignField: FieldMarkerWithView + FieldMarker<Entity = Children>,
    > AssociationBuilder<Children, Parent, ForeignField> for QueryResultFilter<Parent>
where
    Parent::View: ViewWithPrimaryKey<PrimaryKeyType = TypeOfMarker<ForeignField>>,
    Children::View: AssociatedView<
        Parent,
        ForeignField,
        ForeignKeyType = TypeOfMarker<ForeignField>,
        ForeignKeyTags = TagOfMarker<ForeignField>,
    >,
    ExprBoxOfAssociatedView<Children::View, Parent, ForeignField>:
        In<<Parent as WithPrimaryKey>::PrimaryKeyType>,
    TypeOfMarker<ForeignField>: Value + Ord + Hash,
{
    fn build_query(self) -> QueryResultFilter<Children> {
        let subquery = self.query.select(vec![SelectItem {
            expr: self
                .root_alias
                .create_ident_expr(Parent::primary_key_name()),
            alias: Some("result_0".to_string()),
        }]);

        let mut result = Children::all();
        let ident = result
            .root_alias
            .create_ident_expr(Children::foreign_key_name());

        result.query.and_where(Expr::In(Box::new(ident), subquery));

        result
    }

    fn build_from_parent_view(parent_view: &Parent::View) -> QueryResultFilter<Children> {
        let mut result = Children::all();
        let primary_key_view = parent_view;

        let ident = result
            .root_alias
            .create_ident_expr(Children::foreign_key_name());

        result.query.and_where(Expr::Eq(
            Box::new(ident),
            Box::new(
                primary_key_view
                    .primary_key()
                    .collect_expr()
                    .into_iter()
                    .next()
                    .unwrap(),
            ),
        ));

        result
    }

    fn build_from_parent_entities(
        primary_keys: Vec<TypeOfMarker<ForeignField>>,
    ) -> QueryResultFilter<Children> {
        Children::all().filter(|view| view.foreign_key().clone().in_arr(primary_keys.clone()))
    }
}
