use crate::{Alias, AliasedTable, Expr};
use interface::{FieldDefinition, JoinType};

pub struct Join {
    pub ty: JoinType,
    pub table: AliasedTable,
    pub on: Expr,
}

pub trait IntoJoin {
    fn generate_join(
        &self,
        alias: &Alias,
        gen_alias: impl Fn(usize) -> Alias,
        get_field: impl Fn(usize, &str) -> &'static FieldDefinition,
    ) -> Option<Join>;
}
