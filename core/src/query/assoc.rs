use interface::{Association, WithPrimaryKey};

use crate::query::QueryResultFilter;
use crate::view::{EntityWithView, Value, ViewWithPrimaryKey};

pub trait AssociationBuilder<
    Children: EntityWithView + Association<Parent, ForeignKeyType = ForeignKey>,
    Parent: EntityWithView + WithPrimaryKey<Type = ForeignKey>,
    ForeignKey: Value,
> where
    Parent::View: ViewWithPrimaryKey<Type = ForeignKey>,
{
    fn build_query(self) -> QueryResultFilter<Children>;

    fn build_from_parent_view(parent_view: &Parent::View) -> QueryResultFilter<Children>;
}

pub trait BelongsToQueryResult<
    Parent: EntityWithView + WithPrimaryKey<Type = ForeignKey>,
    ForeignKey: Value,
>: EntityWithView + Association<Parent, ForeignKeyType = ForeignKey> where
    QueryResultFilter<Parent>: AssociationBuilder<Self, Parent, ForeignKey>,
    Parent::View: ViewWithPrimaryKey<Type = ForeignKey>,
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
    Parent: EntityWithView + WithPrimaryKey<Type = ForeignKey>,
    ForeignKey: Value,
> where
    QueryResultFilter<Parent>: AssociationBuilder<Children, Parent, ForeignKey>,
    Parent::View: ViewWithPrimaryKey<Type = ForeignKey>,
{
    fn belonging_to_view(r: &Parent::View) -> QueryResultFilter<Children>
    where
        Self: Sized,
    {
        QueryResultFilter::<Parent>::build_from_parent_view(r)
    }
}

impl<
        Children: EntityWithView + Association<Parent, ForeignKeyType = ForeignKey>,
        Parent: EntityWithView + WithPrimaryKey<Type = ForeignKey>,
        ForeignKey: Value,
    > BelongsToQueryResult<Parent, ForeignKey> for Children
where
    Parent::View: ViewWithPrimaryKey<Type = ForeignKey>,
{
}

impl<
        Children: EntityWithView + Association<Parent, ForeignKeyType = ForeignKey>,
        Parent: EntityWithView + WithPrimaryKey<Type = ForeignKey>,
        ForeignKey: Value,
    > BelongsToView<Children, Parent, ForeignKey> for Children
where
    Parent::View: ViewWithPrimaryKey<Type = ForeignKey>,
{
}
