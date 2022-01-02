use std::hash::Hash;

pub trait YukinoEntity: 'static {
    fn table_name() -> &'static str;
}

pub trait Association<Parent: YukinoEntity + WithPrimaryKey<PrimaryKeyType = Self::ForeignKeyType>>:
    YukinoEntity
{
    type ForeignKeyType: 'static + Clone;
    fn foreign_key(&self) -> &Self::ForeignKeyType;

    fn foreign_key_name() -> &'static str
    where
        Self: Sized;
}

pub type PrimaryKeyTypeOf<E> = <E as WithPrimaryKey>::PrimaryKeyType;

pub trait WithPrimaryKey: YukinoEntity {
    type PrimaryKeyType: 'static + Clone + Hash;
    fn primary_key(&self) -> &Self::PrimaryKeyType;

    fn primary_key_name() -> &'static str
    where
        Self: Sized;
}
