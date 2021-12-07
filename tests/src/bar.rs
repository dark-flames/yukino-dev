use yukino::Entity;

use crate::Foo;

#[derive(Entity, Debug, Clone)]
#[belongs_to(Foo, foo_id)]
pub struct Bar {
    pub foo_id: u32,
    pub name: String,
}