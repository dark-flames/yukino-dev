use crate::view::{
    AddTag, AggregateViewTag, ExprViewBoxWithTag, OffsetOfTag, SetBit, TagList, True, Value,
};

pub trait Average: Value {
    fn expr_avg<Tags: TagList + SetBit<OffsetOfTag<AggregateViewTag>, True>>(
        expr: ExprViewBoxWithTag<Self, Tags>,
    ) -> ExprViewBoxWithTag<Self, AddTag<Tags, AggregateViewTag>>;
}
