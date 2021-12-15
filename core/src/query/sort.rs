use crate::operator::SortResult;

pub trait Sort<View> {
    type Result;
    fn sort<R: SortResult, F: Fn(View) -> R>(self, f: F) -> Self::Result;
}

pub trait Sort2<View1, View2> {
    type Result;
    fn sort<R: SortResult, F: Fn(View1, View2) -> R>(self, f: F) -> Self::Result;
}
