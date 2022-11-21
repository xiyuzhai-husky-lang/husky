use husky_entity_path::EntityPathItd;

use crate::*;
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum DeprecatedAstVariant {
    TypeDefnHead {
        ident: RangedCustomIdentifier,
        kind: TyKind,
        spatial_parameters: IdentDict<SpatialParameter>,
    },
    MainDefnHead,
    CallFormDefnHead {
        ident: RangedCustomIdentifier,
        paradigm: Paradigm,
        spatial_parameters: IdentDict<SpatialParameter>,
        parameters: Arc<Vec<Parameter>>,
        return_ty: ExprIdx,
        output_liason: OutputModifier,
        opt_this_liason: Option<ParameterModifier>,
    },
    FeatureDefnHead {
        paradigm: Paradigm,
        ident: RangedCustomIdentifier,
        return_ty: ExprIdx,
    },
    FieldDefnHead {
        liason: MemberModifier,
        ranged_ident: RangedCustomIdentifier,
        field_ty: ExprIdx,
        ast_field_kind: AstFieldKind,
    },
    DatasetConfigDefnHead,
    Stmt(RawStmt),
    EnumVariantDefnHead {
        ident: RangedCustomIdentifier,
        variant_class: EnumVariantKind,
    },
    Use {
        use_variant: UseVariant,
    },
    Submodule {
        ident: RangedCustomIdentifier,
        source_file: PathItd,
    },
    Visual,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum UseVariant {
    Route { entity_path: EntityPathItd },
    All { parent: Ty },
}

impl From<RawStmt> for DeprecatedAst {
    fn from(stmt: RawStmt) -> Self {
        Self {
            range: stmt.range,
            variant: DeprecatedAstVariant::Stmt(stmt),
        }
    }
}

impl From<RawStmt> for DeprecatedAstVariant {
    fn from(stmt: RawStmt) -> Self {
        DeprecatedAstVariant::Stmt(stmt)
    }
}
