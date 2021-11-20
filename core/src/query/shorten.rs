pub trait Shorten {
    fn limit(&mut self, limit: usize) -> &mut Self
        where
            Self: Sized;

    fn offset(&mut self, offset: usize) -> &mut Self
        where
            Self: Sized;
}
