pub extern crate generic_array;
pub extern crate lazy_static;
pub extern crate query_builder;

pub use core::*;
pub use derive::Entity;
pub use interface::*;

pub mod prelude {
    pub use derive::Entity;

    pub use crate::{and, bt, bte, eq, lt, lte, neq, or, tuple};
    pub use crate::operator::{
        InSubquery, SortOrder, SubqueryExists, VerticalAverage, VerticalBitAnd, VerticalBitOr,
        VerticalBitXor, VerticalCount, VerticalCountDistinct, VerticalJoin, VerticalMax,
        VerticalMin,
    };
    pub use crate::query::{
        BelongsToQueryResult, BelongsToView, Delete, Executable, FetchMulti, FetchOne, Filter,
        Filter2, Fold, Fold2, GroupBy, GroupFold, Map, Map2, Sort, Sort2, Update,
    };
    pub use crate::view::{
        Deletable, EntityWithView, Identifiable, Insertable, SingleRowSubqueryView,
        SubqueryIntoView, SubqueryView, VerticalView,
    };
}
