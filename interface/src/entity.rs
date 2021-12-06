pub trait YukinoEntity: 'static {
    fn table_name() -> &'static str;
}
