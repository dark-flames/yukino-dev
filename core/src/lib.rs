#![feature(generic_associated_types)]
#![feature(associated_type_bounds)]
pub mod query;
pub mod entity;
pub mod expr;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
