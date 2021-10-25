#![feature(associated_type_bounds)]

pub mod db;
pub mod interface;
pub mod err;
pub mod expr;
pub mod query;
pub mod resolver;
pub mod view;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
