use crate::entity::def::EntityDefinition;
use crate::err::{CliError, ResolveError, YukinoError};
use crate::resolver::entity::EntityResolver;
use crate::resolver::field::{FieldPath, FieldResolver, ReadyEntities};
use crate::resolver::path::FileTypePathResolver;
use proc_macro2::TokenStream;
use quote::quote;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use syn::spanned::Spanned;
use syn::{parse_file, Fields, File as SynFile, Item};

pub type CliResult<T> = Result<T, CliError>;

pub struct ResolverConfig {
    pub source_supplier: Box<dyn Fn() -> Vec<PathBuf>>,
}

pub struct DefinitionResolver {
    config: ResolverConfig,
    entity_resolver: EntityResolver,
    field_resolver: FieldResolver,
}

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
        for path in (self.config.source_supplier)().iter() {
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

            syntax.items.iter().filter_map(|item| match item {
                Item::Use(item_use) => Some(item_use),
                _ => None,
            }).try_for_each(|item| type_resolver.append_use_item(item))?;

            syntax.items.iter().filter(
                |item| if let Item::Use(_) = item { false } else { true }
            ).try_for_each(|item| {
                match item {
                    Item::Struct(item_struct) => {
                        let (entity_id, count) = self.entity_resolver.resolve(item_struct)?;
                        self.field_resolver.set_entity_field_count(entity_id, count);

                        if let Fields::Named(fields) = &item_struct.fields {
                            fields.named.iter().try_for_each(|field| {
                                let field_path = FieldPath::create(
                                    entity_id,
                                    field.ident.as_ref().unwrap().to_string(),
                                );
                                self.field_resolver.resolve(
                                    &type_resolver,
                                    field_path,
                                    field,
                                ).map(|ready_entity| {
                                    self.handle_ready_entities(ready_entity);
                                })
                            })
                        } else {
                            Err(ResolveError::UnsupportedEntityStructType
                                .as_cli_err(Some(item_struct.span())))?
                        }
                    }
                    _ => {
                        Err(
                            ResolveError::UnsupportedSyntaxBlock.as_cli_err(Some(item.span()))
                        )?
                    }
                }
            })?;
        }

        Ok(AchievedSchemaResolver {
            statements: self.entity_resolver.get_implements(),
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
    pub fn unwrap(self) -> (TokenStream, Vec<EntityDefinition>) {
        let statements = self.statements;
        (quote! {
            #(#statements)*
        }, self.definitions)
    }
}
