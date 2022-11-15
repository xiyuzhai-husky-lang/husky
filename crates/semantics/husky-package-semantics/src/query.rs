use crate::*;

use husky_ast::AstText;

use husky_entity_route::EntityRouteVariant;
use husky_entity_semantics::EntityDefnQueryGroup;
use husky_file::FileItd;
use husky_semantics_error::*;

#[salsa::query_group(PackageQueryGroupStorage)]
pub trait PackageQueryGroup: EntityDefnQueryGroup {
    fn package(&self, target_entrance: husky_file::FileItd) -> SemanticResultArc<Package>;
    fn config(&self, target_entrance: husky_file::FileItd) -> SemanticResultArc<Config>;
}

fn package(
    db: &dyn PackageQueryGroup,
    target_entrance: husky_file::FileItd,
) -> SemanticResultArc<Package> {
    let module = db.module(target_entrance).unwrap();
    let ident = match module.variant {
        EntityRouteVariant::Package { ident, .. } => ident,
        _ => panic!(),
    };
    Ok(Arc::new(Package {
        ident,
        subentities: db.subentity_defns(module)?,
        main_defn: db.main_defn(target_entrance)?,
        config: db.config(target_entrance)?,
    }))
}

fn config(
    this: &dyn PackageQueryGroup,
    target_entrance: husky_file::FileItd,
) -> SemanticResultArc<Config> {
    let ast_text = this.ast_text(target_entrance).unwrap();
    config_from_ast(this, &ast_text, target_entrance)
}

fn config_from_ast(
    this: &dyn PackageQueryGroup,
    ast_text: &AstText,
    file: FileItd,
) -> SemanticResultArc<Config> {
    Ok(Arc::new(Config {
        dataset: dataset_config_from_ast_text(this, ast_text, file)?,
    }))
}

fn dataset_config_from_ast_text(
    _this: &dyn PackageQueryGroup,
    _ast_text: &AstText,
    _file: FileItd,
) -> SemanticResult<DatasetConfig> {
    todo!()
    // for item in ast_text.folded_results.iter() {
    //     match item.value.as_ref().unwrap().variant {
    //         AstVariant::DatasetConfigDefnHead => {
    //             return Ok(DatasetConfig::new(parse_func_stmts(
    //                 this.upcast(),
    //                 &ast_text.arena,
    //                 not_none!(item.opt_children),
    //                 file,
    //             )?))
    //         }
    //         _ => (),
    //     }
    // }
    // err!("dataset config not found")
}
