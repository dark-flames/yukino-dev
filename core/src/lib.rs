#![feature(associated_type_bounds)]

pub mod db;
pub mod entity;
pub mod err;
pub mod expr;
pub mod query;
pub mod resolver;
mod view;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
