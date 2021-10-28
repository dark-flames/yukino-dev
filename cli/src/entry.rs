use std::ffi::OsStr;
use std::fs::{read_dir, remove_file, File, ReadDir};
use std::io::{Result as IoResult, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

use core::err::{CliError, CliResult, ResolveError, YukinoError};
use core::resolver::entity::EntityResolvePass;
use core::resolver::entity_resolver_pass::{
    EntityImplementPass, EntityStructPass, EntityViewImplementPass, FieldMakerPass,
};
use core::resolver::field::{FieldResolverSeed, FieldResolverSeedBox};
use core::resolver::field_resolve_cells::numeric::NumericFieldResolverSeed;
use core::resolver::DefinitionResolver;

pub struct CommandLineEntry {
    resolver: DefinitionResolver,
    output_file_path: PathBuf,
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
        entity_dir: String,
        output_file_path: String,
        mut entity_passes: Vec<Box<dyn EntityResolvePass>>,
        mut field_resolve_seeds: Vec<FieldResolverSeedBox>,
    ) -> CliResult<Self> {
        let entity_dir = read_dir(Path::new(&entity_dir))
            .map_err(|e| ResolveError::FsError(e.to_string()).as_cli_err(None))?;

        entity_passes.extend(
            [
                EntityStructPass::instance(),
                EntityViewImplementPass::instance(),
                EntityImplementPass::instance(),
                FieldMakerPass::instance(),
            ]
            .into_iter(),
        );

        field_resolve_seeds.extend([NumericFieldResolverSeed::instance()].into_iter());

        Ok(CommandLineEntry {
            resolver: DefinitionResolver::create(
                Self::files_under_dir(entity_dir)
                    .map_err(|e| ResolveError::FsError(e.to_string()).as_cli_err(None))?,
                entity_passes,
                field_resolve_seeds,
            ),
            output_file_path: PathBuf::from(output_file_path),
        })
    }

    pub fn export_implements(&mut self) -> CliResult<()> {
        let achieved = self.resolver.resolve()?.to_token_stream().to_string();
        if self.output_file_path.exists() {
            remove_file(&self.output_file_path)
                .map_err(|e| ResolveError::FsError(e.to_string()).as_cli_err(None))?;
        };

        let mut output_file = File::create(&self.output_file_path)
            .map_err(|e| ResolveError::FsError(e.to_string()).as_cli_err(None))?;

        output_file
            .write_all(achieved.as_bytes())
            .map_err(|e| ResolveError::FsError(e.to_string()).as_cli_err(None))
            .and_then(|_| {
                Command::new("cargo")
                    .arg("fmt")
                    .output()
                    .map_err(|e| CliError {
                        msg: e.to_string(),
                        pos: None,
                    })
                    .map(|_| ())
            })?;

        Ok(())
    }

    pub fn process(&mut self) -> CliResult<()> {
        self.export_implements()
    }
}
