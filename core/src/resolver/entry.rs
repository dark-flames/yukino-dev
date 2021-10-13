use crate::err::{CliError, ResolveError, YukinoError};
use crate::resolver::path::FileTypePathResolver;
use proc_macro2::TokenStream;
use quote::quote;
use std::fs::File;
use std::io::Read;

pub struct ResolverConfig {
    pub source: Vec<File>,
}

pub struct SchemaResolver {
    config: ResolverConfig,
    result: Vec<TokenStream>,
}

pub struct AchievedSchemaResolver {
    token_stream: Vec<TokenStream>,
}

impl SchemaResolver {
    pub fn create(config: ResolverConfig) -> Self {
        SchemaResolver {
            config,
            result: vec![],
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
            token_stream: self.result,
        })
    }
}

impl AchievedSchemaResolver {
    pub fn unwrap(self) -> TokenStream {
        let statements = self.token_stream;
        quote! {
            #(#statements)*
        }
    }
}
