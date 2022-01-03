use std::hash::Hash;

pub trait YukinoEntity: 'static {
    fn table_name() -> &'static str;
}

pub type TypeOfForeignField<A, P, F> = <A as Association<P, F>>::ForeignKeyType;

pub trait Association<
    Parent: YukinoEntity + WithPrimaryKey<PrimaryKeyType = Self::ForeignKeyType>,
    ForeignField: FieldMarker<Entity = Self, FieldType = Self::ForeignKeyType>,
>: YukinoEntity
{
    type ForeignKeyType: 'static + Clone + Ord + Hash;
    fn foreign_key(&self) -> &Self::ForeignKeyType;

    fn foreign_key_name() -> &'static str
    where
        Self: Sized;
}

pub type PrimaryKeyTypeOf<E> = <E as WithPrimaryKey>::PrimaryKeyType;

pub trait WithPrimaryKey: YukinoEntity {
    type PrimaryKeyType: 'static + Clone + Hash + Ord;
    fn primary_key(&self) -> &Self::PrimaryKeyType;

    fn primary_key_name() -> &'static str
    where
        Self: Sized;
}

pub trait FieldMarker {
    type Entity: YukinoEntity;
    type FieldType: 'static + Clone;
}
