use crate::{
    signature::{attr::AttrDecTemplate, HasDecTemplate},
    *,
};
use husky_dec_term::term::{DecItemPath, DecTerm};
use husky_entity_kind::MajorFormKind;
use husky_entity_path::path::major_item::form::MajorFormPath;
use vec_like::SmallVecSet;

#[salsa::tracked(return_ref)]
pub fn dec_var_full_projs(
    db: &::salsa::Db,
    var_path: MajorFormPath,
) -> DecSignatureResult<SmallVecSet<MajorFormPath, 2>> {
    assert!(var_path.is_var(db));
    let var_full_projs = dec_var_full_projs_unchecked(db, var_path).as_ref()?;
    for &var_full_proj in &var_full_projs[1..] {
        if dec_var_full_projs_unchecked(db, var_full_proj)
            .as_ref()?
            .contains(&var_path)
        {
            return Err(DecSignatureError::CyclicProj);
        }
    }
    Ok(var_full_projs.clone())
}

#[test]
fn dec_var_projs_works() {
    use husky_entity_path::path::major_item::MajorItemPath;
    use husky_entity_path::path::ItemPath;
    use husky_vfs::path::module_path::ModulePath;

    fn module_dec_var_projs(
        db: &::salsa::Db,
        module_path: ModulePath,
    ) -> Vec<(
        MajorFormPath,
        DecSignatureResult<SmallVecSet<MajorFormPath, 2>>,
    )> {
        husky_syn_decl::sheet::syn_decl_sheet(db, module_path)
            .decls(db)
            .iter()
            .copied()
            .filter_map(|(path, _)| {
                let ItemPath::MajorItem(MajorItemPath::Form(path)) = path else {
                    return None;
                };
                if !path.is_var(db) {
                    return None;
                }
                Some((path, dec_var_full_projs(db, path).clone()))
            })
            .collect()
    }

    DB::vfs_rich_test_debug_with_db(
        |db, module_path| module_dec_var_projs(db, module_path),
        &AstTestConfig::new(
            "module_dec_var_projs",
            FileExtensionConfig::Markdown,
            TestDomainsConfig::KERNEL,
        ),
    )
}

#[salsa::tracked(return_ref)]
fn dec_var_projs_unchecked(
    db: &::salsa::Db,
    var_path: MajorFormPath,
) -> DecSignatureResult<SmallVecSet<MajorFormPath, 2>> {
    use husky_entity_tree::node::attr::HasAttrPaths;

    assert!(var_path.is_var(db));
    let mut projs: SmallVecSet<MajorFormPath, 2> = Default::default();
    for &attr_path in var_path.attr_paths(db) {
        if let AttrDecTemplate::Proj(dec_template) = attr_path.dec_template(db)? {
            for shard in dec_template.shards(db) {
                match shard.proj_term(db) {
                    DecTerm::ItemPath(path) => match path {
                        DecItemPath::Form(form_path) => projs.insert(form_path),
                        DecItemPath::Trait(_) => todo!(),
                        DecItemPath::Type(_) => todo!(),
                        DecItemPath::TypeVariant(_) => todo!(),
                    },
                    _ => todo!(),
                }
            }
        }
    }
    Ok(projs)
}

#[salsa::tracked(return_ref)]
fn dec_var_full_projs_unchecked(
    db: &::salsa::Db,
    var_path: MajorFormPath,
) -> DecSignatureResult<SmallVecSet<MajorFormPath, 2>> {
    assert!(var_path.is_var(db));
    let mut var_paths: SmallVecSet<MajorFormPath, 2> = SmallVecSet::new_one_elem_set(var_path);
    let mut first_unsearched = 0usize;
    while first_unsearched < var_paths.len() {
        let first_unsearched = std::mem::replace(&mut first_unsearched, var_paths.len());
        for i in first_unsearched..var_paths.len() {
            var_paths.extend(dec_var_projs_unchecked(db, var_paths[i]).as_ref()?);
        }
    }
    Ok(var_paths)
}
