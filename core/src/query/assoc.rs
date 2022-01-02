use std::collections::HashMap;

use interface::{Association, PrimaryKeyTypeOf, WithPrimaryKey};

use crate::operator::In;
use crate::query::QueryResultFilter;
use crate::view::{
    AssociatedView, EntityWithView, ExprBoxOfAssociatedView, Value, ViewWithPrimaryKey,
};

pub struct JoinedResult<
    Children: EntityWithView + Association<Parent, ForeignKeyType = PrimaryKeyTypeOf<Parent>>,
    Parent: EntityWithView + WithPrimaryKey,
> {
    _parent: Vec<Parent>,
    _grouped_children: HashMap<PrimaryKeyTypeOf<Parent>, Vec<Children>>,
}

pub trait AssociationBuilder<
    Children: EntityWithView + Association<Parent, ForeignKeyType = ForeignKey>,
    Parent: EntityWithView + WithPrimaryKey<PrimaryKeyType = ForeignKey>,
    ForeignKey: Value,
> where
    Parent::View: ViewWithPrimaryKey<PrimaryKeyType = ForeignKey>,
{
    fn build_query(self) -> QueryResultFilter<Children>;

    fn build_from_parent_view(parent_view: &Parent::View) -> QueryResultFilter<Children>;

    fn build_from_parent_entities(primary_keys: Vec<ForeignKey>) -> QueryResultFilter<Children>;
}

pub trait BelongsToQueryResult<
    Parent: EntityWithView + WithPrimaryKey<PrimaryKeyType = ForeignKey>,
    ForeignKey: Value,
>: EntityWithView + Association<Parent, ForeignKeyType = ForeignKey> where
    QueryResultFilter<Parent>: AssociationBuilder<Self, Parent, ForeignKey>,
    Parent::View: ViewWithPrimaryKey<PrimaryKeyType = ForeignKey>,
{
    fn belonging_to_query(r: QueryResultFilter<Parent>) -> QueryResultFilter<Self>
    where
        Self: Sized,
    {
        r.build_query()
    }
}

pub trait BelongsToView<
    Children: EntityWithView + Association<Parent, ForeignKeyType = ForeignKey>,
    Parent: EntityWithView + WithPrimaryKey<PrimaryKeyType = ForeignKey>,
    ForeignKey: Value,
> where
    QueryResultFilter<Parent>: AssociationBuilder<Children, Parent, ForeignKey>,
    Parent::View: ViewWithPrimaryKey<PrimaryKeyType = ForeignKey>,
{
    fn belonging_to_view(r: &Parent::View) -> QueryResultFilter<Children>
    where
        Self: Sized,
    {
        QueryResultFilter::<Parent>::build_from_parent_view(r)
    }
}

pub trait BelongsToEntities<
    Children: EntityWithView + Association<Parent, ForeignKeyType = ForeignKey>,
    Parent: EntityWithView + WithPrimaryKey<PrimaryKeyType = ForeignKey>,
    ForeignKey: Value,
> where
    QueryResultFilter<Parent>: AssociationBuilder<Children, Parent, ForeignKey>,
    Parent::View: ViewWithPrimaryKey<PrimaryKeyType = ForeignKey>,
{
    fn belonging_to(r: &[Parent]) -> QueryResultFilter<Children>
    where
        Self: Sized,
    {
        QueryResultFilter::<Parent>::build_from_parent_entities(
            r.iter().map(|i| i.primary_key().clone()).collect(),
        )
    }
}

pub trait JoinChildren<
    Children: EntityWithView + Association<Parent, ForeignKeyType = PrimaryKeyTypeOf<Parent>>,
    Parent: EntityWithView + WithPrimaryKey,
>
{
    fn join(self, children: Vec<Children>) -> JoinedResult<Children, Parent>;
}

impl<
        Children: EntityWithView + Association<Parent, ForeignKeyType = PrimaryKeyTypeOf<Parent>>,
        Parent: EntityWithView + WithPrimaryKey,
    > JoinChildren<Children, Parent> for Vec<Parent>
{
    fn join(self, _children: Vec<Children>) -> JoinedResult<Children, Parent> {
        todo!()
    }
}

impl<
        Children: EntityWithView + Association<Parent, ForeignKeyType = ForeignKey>,
        Parent: EntityWithView + WithPrimaryKey<PrimaryKeyType = ForeignKey>,
        ForeignKey: Value,
    > BelongsToQueryResult<Parent, ForeignKey> for Children
where
    Parent::View: ViewWithPrimaryKey<PrimaryKeyType = ForeignKey>,
    Children::View: AssociatedView<Parent, ForeignKeyType = ForeignKey>,
    ExprBoxOfAssociatedView<Children::View, Parent>: In<<Parent as WithPrimaryKey>::PrimaryKeyType>,
{
}

impl<
        Children: EntityWithView + Association<Parent, ForeignKeyType = ForeignKey>,
        Parent: EntityWithView + WithPrimaryKey<PrimaryKeyType = ForeignKey>,
        ForeignKey: Value,
    > BelongsToView<Children, Parent, ForeignKey> for Children
where
    Parent::View: ViewWithPrimaryKey<PrimaryKeyType = ForeignKey>,
    Children::View: AssociatedView<Parent, ForeignKeyType = ForeignKey>,
    ExprBoxOfAssociatedView<Children::View, Parent>: In<<Parent as WithPrimaryKey>::PrimaryKeyType>,
{
}
