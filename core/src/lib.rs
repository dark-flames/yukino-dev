#![feature(associated_type_bounds)]
#![feature(extend_one)]

pub mod db;
pub mod entity;
pub mod err;
pub mod expr;
pub mod query;
pub mod resolver;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
