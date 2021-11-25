use std::fmt::{Display, Formatter};

use interface::JoinType;

use crate::{AliasedTable, Expr};

pub struct Join {
    pub ty: JoinType,
    pub table: AliasedTable,
    pub on: Expr,
}

impl Display for Join {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} ON {}", self.ty, self.table, self.on)
    }
}
