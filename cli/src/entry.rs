use core::err::{ResolveError, YukinoError};
use core::resolver::entity::EntityResolvePass;
use core::resolver::field::FieldResolverSeedBox;
use core::resolver::{CliResult, DefinitionResolver};
use std::ffi::OsStr;
use std::fs::{read_dir, ReadDir};
use std::io::Result as IoResult;
use std::path::PathBuf;

#[allow(dead_code)]
pub struct CommandLineEntry {
    resolver: DefinitionResolver,
}

impl CommandLineEntry {
    pub fn files_under_dir(dir: ReadDir) -> IoResult<Vec<PathBuf>> {
        let entries = dir.into_iter().collect::<IoResult<Vec<_>>>()?;
        let mut paths = vec![];
        for entry in entries {
            let meta = entry.metadata()?;
            if meta.is_file() {
                if let Some("rs") = entry.path().extension().and_then(OsStr::to_str) {
                    paths.push(entry.path())
                }
            } else if meta.is_dir() {
                paths.extend(Self::files_under_dir(read_dir(entry.path())?)?.into_iter())
            }
        }
        Ok(paths)
    }

    pub fn create(
        entity_dir: ReadDir,
        entity_passes: Vec<Box<dyn EntityResolvePass>>,
        field_resolve_seeds: Vec<FieldResolverSeedBox>,
    ) -> CliResult<Self> {
        Ok(CommandLineEntry {
            resolver: DefinitionResolver::create(
                Self::files_under_dir(entity_dir)
                    .map_err(|e| ResolveError::FsError(e.to_string()).as_cli_err(None))?,
                entity_passes,
                field_resolve_seeds,
            ),
        })
    }
}
