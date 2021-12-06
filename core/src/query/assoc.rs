use interface::{Association, WithPrimaryKey};

use crate::query::QueryResultFilter;
use crate::view::{EntityWithView, Value};

pub trait AssociationBuilder<
    Children: EntityWithView + Association<Parent, ForeignKey>,
    Parent: EntityWithView + WithPrimaryKey<ForeignKey>,
    ForeignKey: Value
> {
    fn build_query(self) -> QueryResultFilter<Children>;
}

pub trait BelongsTo<
    Parent: EntityWithView + WithPrimaryKey<ForeignKey>,
    ForeignKey: Value
>: EntityWithView + Association<Parent, ForeignKey>
    where QueryResultFilter<Parent>: AssociationBuilder<Self, Parent, ForeignKey> {
    fn belongs_to(&self, r: QueryResultFilter<Parent>) -> QueryResultFilter<Self> {
        r.build_query()
    }
}