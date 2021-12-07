use interface::{Association, WithPrimaryKey};

use crate::query::QueryResultFilter;
use crate::view::{EntityWithView, Value};

pub trait AssociationBuilder<
    Children: EntityWithView + Association<Parent, ForeignKeyType=ForeignKey>,
    Parent: EntityWithView + WithPrimaryKey<Type=ForeignKey>,
    ForeignKey: Value
> {
    fn build_query(self) -> QueryResultFilter<Children>;
}

pub trait BelongsToQueryResult<
    Parent: EntityWithView + WithPrimaryKey<Type=ForeignKey>,
    ForeignKey: Value
>: EntityWithView + Association<Parent, ForeignKeyType=ForeignKey>
    where QueryResultFilter<Parent>: AssociationBuilder<Self, Parent, ForeignKey> {
    fn belonging_to(r: QueryResultFilter<Parent>) -> QueryResultFilter<Self> where Self: Sized {
        r.build_query()
    }
}

impl<
    Children: EntityWithView + Association<Parent, ForeignKeyType=ForeignKey>,
    Parent: EntityWithView + WithPrimaryKey<Type=ForeignKey>,
    ForeignKey: Value
> BelongsToQueryResult<Parent, ForeignKey> for Children {}