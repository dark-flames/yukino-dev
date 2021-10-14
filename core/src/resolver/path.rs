use crate::err::{CliError, ResolveError, YukinoError};
use proc_macro2::Ident;
use std::collections::HashMap;
use syn::spanned::Spanned;
use syn::{ItemUse, PathSegment, Type, TypePath, UseTree};

pub type Entry = String;
pub type FullPath = Vec<Ident>;

#[derive(Default)]
pub struct FileTypePathResolver {
    map: HashMap<Entry, FullPath>,
}

impl FileTypePathResolver {
    pub fn append_use_item(&mut self, item: &ItemUse) -> Result<(), CliError> {
        let result =
            Self::resolve_use_tree(&item.tree).map_err(|e| e.as_cli_err(Some(item.span())))?;

        self.map.extend(result.into_iter());

        Ok(())
    }

    fn resolve_use_tree(tree: &UseTree) -> Result<Vec<(Entry, FullPath)>, ResolveError> {
        Ok(match tree {
            UseTree::Name(use_name) => {
                vec![(use_name.ident.to_string(), vec![use_name.ident.clone()])]
            }
            UseTree::Rename(use_rename) => {
                vec![(
                    use_rename.rename.to_string(),
                    vec![use_rename.ident.clone()],
                )]
            }
            UseTree::Path(use_path) => {
                let current_segment = use_path.ident.to_string();
                let next = Self::resolve_use_tree(use_path.tree.as_ref()).map_err(|e| match e {
                    ResolveError::GlobInPathIsNotSupported(path) => {
                        ResolveError::GlobInPathIsNotSupported(format!(
                            "{}::{}",
                            current_segment, path
                        ))
                    }
                    others => others,
                })?;

                next.into_iter()
                    .map(|(name, mut full)| {
                        full.push(use_path.ident.clone());
                        (name, full)
                    })
                    .collect()
            }
            UseTree::Group(use_group) => use_group.items.iter().map(Self::resolve_use_tree).fold(
                Ok(vec![]),
                |carry, item_result| {
                    if let Ok(mut carry_vec) = carry {
                        if let Ok(mut item_vec) = item_result {
                            carry_vec.append(&mut item_vec);
                            Ok(carry_vec)
                        } else {
                            item_result
                        }
                    } else {
                        carry
                    }
                },
            )?,
            UseTree::Glob(_) => {
                return Err(ResolveError::GlobInPathIsNotSupported("*".to_string()))
            }
        })
    }

    pub fn get_full_path(&self, ty: TypePath) -> TypePath {
        let first_segment = ty.path.segments.first().unwrap();

        if let Some(full) = self.map.get(first_segment.ident.to_string().as_str()) {
            let mut result = ty;
            let mut full_iter = full.iter();

            if let Some(first) = result.path.segments.first_mut() {
                if let Some(segment) = full_iter.next() {
                    first.ident = segment.clone()
                }
            }

            for ident in full_iter {
                result
                    .path
                    .segments
                    .insert(0, PathSegment::from(ident.clone()))
            }

            result
        } else {
            ty
        }
    }

    pub fn get_full_type(&self, ty: Type) -> Type {
        match ty {
            Type::Path(path) => Type::Path(self.get_full_path(path)),
            others => others,
        }
    }
}
