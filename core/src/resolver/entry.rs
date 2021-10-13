use std::fs::File;

pub struct ResolverConfig {
    pub source: Vec<File>,
}

pub struct SchemaResolverEntry {
    pub config: ResolverConfig,
}

impl SchemaResolverEntry {
    pub fn create(config: ResolverConfig) -> Self {
        SchemaResolverEntry { config }
    }
}
