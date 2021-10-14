use std::collections::HashMap;

#[allow(dead_code)]
pub struct UnassembledEntity {
    id: usize,
}

pub struct ResolvedEntity {
    pub id: usize,
}

#[allow(dead_code)]
#[derive(Default)]
pub struct EntityResolver {
    unassembled: HashMap<usize, UnassembledEntity>,
    finished: HashMap<usize, ResolvedEntity>,
}
