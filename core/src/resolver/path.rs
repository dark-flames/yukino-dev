use crate::err::{CliError, ResolveError, YukinoError};
use proc_macro2::Ident;
use quote::ToTokens;
use std::any::type_name;
use std::collections::HashMap;
use syn::spanned::Spanned;
use syn::{
    parse_quote, parse_str, File, GenericArgument, Item, ItemUse, PathArguments, PathSegment,
    ReturnType, Type, TypePath, UseTree,
};

pub type Entry = String;
pub type FullPath = Vec<Ident>;

pub struct FileTypePathResolver {
    map: HashMap<Entry, FullPath>,
}

pub enum TypeMatchResult {
    Mismatch,
    InOption,
    Match,
}

impl Default for FileTypePathResolver {
    fn default() -> Self {
        let mut result = FileTypePathResolver {
            map: Default::default(),
        };

        let item_use: File = parse_quote! {
            use core::option::Option;
        };

        item_use
            .items
            .iter()
            .map(|i| {
                if let Item::Use(item) = i {
                    item
                } else {
                    unreachable!()
                }
            })
            .for_each(|i| {
                result.append_use_item(i).unwrap();
            });

        result
    }
}

impl FileTypePathResolver {
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

    pub fn append_use_item(&mut self, item: &ItemUse) -> Result<(), CliError> {
        let result =
            Self::resolve_use_tree(&item.tree).map_err(|e| e.as_cli_err(Some(item.span())))?;

        self.map.extend(result.into_iter().rev());

        Ok(())
    }

    pub fn add_alias(&mut self, entry: Entry, path: FullPath) {
        self.map.insert(entry, path);
    }

    pub fn get_full_path(&self, ty: &TypePath) -> TypePath {
        let first_segment = ty.path.segments.first().unwrap();

        if let Some(full) = self.map.get(first_segment.ident.to_string().as_str()) {
            let mut result = ty.clone();
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
                .path
                .segments
                .iter_mut()
                .for_each(|seg| match &mut seg.arguments {
                    PathArguments::AngleBracketed(args) => {
                        args.args.iter_mut().for_each(|arg| match arg {
                            GenericArgument::Type(ty) => {
                                *ty = self.get_full_type(ty);
                            }
                            _ => {}
                        })
                    }
                    PathArguments::Parenthesized(args) => {
                        args.inputs.iter_mut().for_each(|ty| {
                            *ty = self.get_full_type(ty);
                        });
                        match &mut args.output {
                            ReturnType::Type(_, ty) => {
                                *ty = Box::new(self.get_full_type(ty.as_ref()));
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                });

            result
        } else {
            ty.clone()
        }
    }

    pub fn get_full_type(&self, ty: &Type) -> Type {
        match ty {
            Type::Path(path) => Type::Path(self.get_full_path(path)),
            others => others.clone(),
        }
    }

    pub fn compare_type_path(&self, l: &Type, r: &Type) -> bool {
        match (l, r) {
            (Type::Path(l_path), Type::Path(r_path)) => self
                .get_full_path(l_path)
                .path
                .segments
                .into_iter()
                .zip(self.get_full_path(r_path).path.segments.into_iter())
                .all(|(l_seg, r_seg)| {
                    l_seg.ident == r_seg.ident
                        && match (l_seg.arguments, r_seg.arguments) {
                            (PathArguments::None, PathArguments::None) => true,
                            (
                                PathArguments::Parenthesized(l_args),
                                PathArguments::Parenthesized(r_args),
                            ) => {
                                l_args
                                    .inputs
                                    .iter()
                                    .zip(r_args.inputs.iter())
                                    .all(|(l_ty, r_ty)| self.compare_type_path(l_ty, r_ty))
                                    && match (l_args.output, r_args.output) {
                                        (ReturnType::Default, ReturnType::Default) => true,
                                        (
                                            ReturnType::Type(_, l_ty_box),
                                            ReturnType::Type(_, r_ty_box),
                                        ) => self.compare_type_path(&l_ty_box, &r_ty_box),
                                        _ => false,
                                    }
                            }
                            (
                                PathArguments::AngleBracketed(l_args),
                                PathArguments::AngleBracketed(r_args),
                            ) => l_args
                                .args
                                .iter()
                                .zip(r_args.args.iter())
                                .all(|ty| match ty {
                                    (GenericArgument::Type(l_ty), GenericArgument::Type(r_ty)) => {
                                        self.compare_type_path(l_ty, r_ty)
                                    }
                                    (left, right) => {
                                        left.to_token_stream().to_string()
                                            == right.to_token_stream().to_string()
                                    }
                                }),
                            _ => false,
                        }
                }),
            (left, right) => {
                left.to_token_stream().to_string() == right.to_token_stream().to_string()
            }
        }
    }

    pub fn match_ty<T>(&self, ty: &Type) -> TypeMatchResult {
        let target_ty = parse_str(type_name::<T>()).unwrap();
        let target_ty_option = parse_str(type_name::<Option<T>>()).unwrap();
        if self.compare_type_path(ty, &target_ty) {
            TypeMatchResult::Match
        } else if self.compare_type_path(ty, &target_ty_option) {
            TypeMatchResult::InOption
        } else {
            TypeMatchResult::Mismatch
        }
    }
}

#[test]
fn test_type_comparison() {
    let resolver: FileTypePathResolver = Default::default();
    let left = parse_str(type_name::<Option<u32>>()).unwrap();
    let right1 = parse_str("Option<u32>").unwrap();
    assert!(resolver.compare_type_path(&left, &right1));
    let right2 = parse_str("Option<u64>").unwrap();
    assert_eq!(resolver.compare_type_path(&left, &right2), false)
}
