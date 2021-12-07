pub trait YukinoEntity: 'static {
    fn table_name() -> &'static str;
}

pub trait Association<Parent: YukinoEntity, ForeignKey: 'static + Clone>: YukinoEntity {
    fn foreign_key(&self) -> &ForeignKey;

    fn foreign_key_name() -> &'static str where Self: Sized;
}

pub trait WithPrimaryKey: YukinoEntity {
    type Type: 'static + Clone;
    fn primary_key(&self) -> &Self::Type;

    fn primary_key_name() -> &'static str where Self: Sized;
}