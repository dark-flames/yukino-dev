use crate::entity::def::EntityDefinition;
use crate::err::{CliError, ResolveError, YukinoError};
use crate::resolver::entity::EntityResolver;
use crate::resolver::field::{FieldResolver, ReadyEntities};
use crate::resolver::path::FileTypePathResolver;
use proc_macro2::TokenStream;
use quote::quote;
use std::fs::File;
use std::io::Read;

pub type CliResult<T> = Result<T, CliError>;

pub struct ResolverConfig {
    pub source: Vec<File>,
}

#[allow(dead_code)]
pub struct DefinitionResolver {
    config: ResolverConfig,
    entity_resolver: EntityResolver,
    field_resolver: FieldResolver,
}

#[allow(dead_code)]
pub struct AchievedSchemaResolver {
    statements: Vec<TokenStream>,
    definitions: Vec<EntityDefinition>,
}

impl DefinitionResolver {
    pub fn create(config: ResolverConfig) -> Self {
        DefinitionResolver {
            config,
            entity_resolver: Default::default(),
            field_resolver: Default::default(),
        }
    }

    pub fn resolve(mut self) -> CliResult<AchievedSchemaResolver> {
        for file in self.config.source.iter_mut() {
            let mut content = String::new();
            file.read_to_string(&mut content)
                .map_err(|e| ResolveError::FsError(e.to_string()).as_cli_err(None))?;
            #[allow(unused_variables, unused_mut)]
            let mut type_resolver: FileTypePathResolver = Default::default();
            unimplemented!()
        }

        Ok(AchievedSchemaResolver {
            statements: vec![TokenStream::new()],
            definitions: vec![],
        })
    }
    #[allow(dead_code)]
    fn handle_ready_entities(&mut self, entities: ReadyEntities) {
        for entity in entities.into_iter() {
            let ready_entities = self.field_resolver.apply_entity(
                self.entity_resolver
                    .assembled(entity, self.field_resolver.get_entity_fields(entity)),
            );
            self.handle_ready_entities(ready_entities);
        }
    }
}

impl AchievedSchemaResolver {
    pub fn unwrap(self) -> TokenStream {
        let statements = self.statements;
        quote! {
            #(#statements)*
        }
    }
}
