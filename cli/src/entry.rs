use core::err::{ResolveError, YukinoError};
use core::resolver::entity::EntityResolvePass;
use core::resolver::field::FieldResolverSeedBox;
use core::resolver::{CliResult, DefinitionResolver};
use std::ffi::OsStr;
use std::fs::{read_dir, remove_file, File, ReadDir};
use std::io::{Result as IoResult, Write};
use std::path::{Path, PathBuf};

pub struct CommandLineEntry {
    resolver: DefinitionResolver,
    output_file_path: String,
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
        output_file_path: String,
    ) -> CliResult<Self> {
        Ok(CommandLineEntry {
            resolver: DefinitionResolver::create(
                Self::files_under_dir(entity_dir)
                    .map_err(|e| ResolveError::FsError(e.to_string()).as_cli_err(None))?,
                entity_passes,
                field_resolve_seeds,
            ),
            output_file_path,
        })
    }

    pub fn resolve(&mut self) -> CliResult<()> {
        let achieved = self.resolver.resolve()?.unwrap().to_string();

        let path = Path::new(&self.output_file_path);
        if path.exists() {
            remove_file(path).map_err(|e| ResolveError::FsError(e.to_string()).as_cli_err(None))?;
        };

        let mut output_file = File::create(path)
            .map_err(|e| ResolveError::FsError(e.to_string()).as_cli_err(None))?;

        output_file
            .write_all(achieved.as_bytes())
            .map_err(|e| ResolveError::FsError(e.to_string()).as_cli_err(None))?;

        Ok(())
    }
}
