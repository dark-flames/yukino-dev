use crate::entity::def::EntityDefinition;
use crate::err::{CliError, ResolveError, YukinoError};
use crate::resolver::entity::EntityResolver;
use crate::resolver::field::{FieldPath, FieldResolver};
use crate::resolver::path::FileTypePathResolver;
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

pub struct ResolverConfig {
    pub source: Vec<File>,
}

#[allow(dead_code)]
pub struct DefinitionResolver {
    config: ResolverConfig,
    entity_resolver: HashMap<usize, EntityResolver>,
    field_resolver: HashMap<FieldPath, FieldResolver>,
    waiting_for_field: HashMap<FieldPath, Vec<FieldPath>>,
    waiting_for_entity: HashMap<usize, Vec<FieldPath>>,
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
            entity_resolver: HashMap::new(),
            field_resolver: HashMap::new(),
            waiting_for_field: HashMap::new(),
            waiting_for_entity: HashMap::new(),
        }
    }

    pub fn resolve(mut self) -> Result<AchievedSchemaResolver, CliError> {
        for file in self.config.source.iter_mut() {
            let mut content = String::new();
            file.read_to_string(&mut content)
                .map_err(|e| ResolveError::FsError(e.to_string()).as_cli_err(None))?;
            #[allow(unused_variables, unused_mut)]
            let mut type_resolver = FileTypePathResolver::new();
            unimplemented!()
        }

        Ok(AchievedSchemaResolver {
            statements: vec![TokenStream::new()],
            definitions: vec![],
        })
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
