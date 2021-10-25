use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use proc_macro2::TokenStream;
use quote::quote;
use syn::spanned::Spanned;
use syn::{parse_file, Fields, File as SynFile, Item};

use crate::err::{CliResult, ResolveError, YukinoError};
use crate::interface::def::EntityDefinition;
use crate::resolver::entity::{EntityResolvePass, EntityResolver};
use crate::resolver::field::{FieldPath, FieldResolver, FieldResolverSeedBox, ReadyEntities};
use crate::resolver::path::FileTypePathResolver;

pub struct DefinitionResolver {
    source: Vec<PathBuf>,
    entity_resolver: EntityResolver,
    field_resolver: FieldResolver,
}

#[allow(dead_code)]
pub struct AchievedSchemaResolver {
    pub statements: Vec<TokenStream>,
    pub definitions: Vec<EntityDefinition>,
}

impl DefinitionResolver {
    pub fn create(
        source: Vec<PathBuf>,
        entity_passes: Vec<Box<dyn EntityResolvePass>>,
        field_resolve_seeds: Vec<FieldResolverSeedBox>,
    ) -> Self {
        DefinitionResolver {
            source,
            entity_resolver: EntityResolver::create(entity_passes),
            field_resolver: FieldResolver::create(field_resolve_seeds),
        }
    }

    pub fn resolve(&mut self) -> CliResult<AchievedSchemaResolver> {
        for path in self.source.clone() {
            let mut file = File::open(&path)
                .map_err(|e| ResolveError::FsError(e.to_string()).as_cli_err(None))?;
            let mut content = String::new();
            file.read_to_string(&mut content)
                .map_err(|e| ResolveError::FsError(e.to_string()).as_cli_err(None))?;
            let mut type_resolver: FileTypePathResolver = Default::default();

            let syntax: SynFile = parse_file(content.as_str()).map_err(|e| {
                ResolveError::FileParseError(path.as_path().display().to_string(), e.to_string())
                    .as_cli_err(Some(e.span()))
            })?;

            syntax
                .items
                .iter()
                .filter_map(|item| match item {
                    Item::Use(item_use) => Some(item_use),
                    _ => None,
                })
                .try_for_each(|item| type_resolver.append_use_item(item))?;

            syntax
                .items
                .iter()
                .filter(|item| !matches!(item, Item::Use(_)))
                .try_for_each(|item| match item {
                    Item::Struct(item_struct) => {
                        let (entity_id, count) = self.entity_resolver.resolve(item_struct)?;
                        self.field_resolver.set_entity_field_count(entity_id, count);

                        if let Fields::Named(fields) = &item_struct.fields {
                            fields.named.iter().try_for_each(|field| {
                                let field_path = FieldPath::create(
                                    entity_id,
                                    field.ident.as_ref().unwrap().to_string(),
                                );
                                self.field_resolver
                                    .resolve(&type_resolver, field_path, field)
                                    .map(|ready_entity| self.handle_ready_entities(ready_entity))?
                            })
                        } else {
                            Err(ResolveError::UnsupportedEntityStructType
                                .as_cli_err(Some(item_struct.span())))
                        }
                    }
                    _ => Err(ResolveError::UnsupportedSyntaxBlock.as_cli_err(Some(item.span()))),
                })?;
        }

        Ok(AchievedSchemaResolver {
            statements: self.entity_resolver.get_implements(),
            definitions: vec![],
        })
    }
    #[allow(dead_code)]
    fn handle_ready_entities(&mut self, entities: ReadyEntities) -> CliResult<()> {
        entities.into_iter().try_for_each(|entity| {
            let ready_entities = self.field_resolver.apply_entity(
                self.entity_resolver
                    .assembled(entity, self.field_resolver.get_entity_fields(entity))?,
            );
            self.handle_ready_entities(ready_entities)
        })
    }
}

impl AchievedSchemaResolver {
    pub fn to_token_stream(&self) -> TokenStream {
        let statements = &self.statements;
        quote! {
            #(#statements)*
        }
    }
}
