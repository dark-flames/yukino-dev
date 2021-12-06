pub trait YukinoEntity: 'static {
    fn table_name() -> &'static str;
}

pub trait Association<Parent: YukinoEntity, ForeignKey: 'static + Clone>: YukinoEntity {
    fn foreign_key(&self) -> &ForeignKey;

    fn foreign_key_name() -> &'static str where Self: Sized;
}

pub trait WithPrimaryKey<PrimaryKey: 'static + Clone>: YukinoEntity {
    fn primary_key(&self) -> &PrimaryKey;

    fn primary_key_name() -> &'static str where Self: Sized;
}