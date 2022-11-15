use crate::*;
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum AstVariant {
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
        return_ty: RangedEntityRoute,
        output_liason: OutputModifier,
        opt_this_liason: Option<ParameterModifier>,
    },
    FeatureDefnHead {
        paradigm: Paradigm,
        ident: RangedCustomIdentifier,
        return_ty: RangedEntityRoute,
    },
    FieldDefnHead {
        liason: MemberModifier,
        ranged_ident: RangedCustomIdentifier,
        field_ty: RangedEntityRoute,
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
        source_file: FileItd,
    },
    Visual,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum UseVariant {
    Route { route: Ty },
    All { parent: Ty },
}

impl From<RawStmt> for Ast {
    fn from(stmt: RawStmt) -> Self {
        Self {
            range: stmt.range,
            variant: AstVariant::Stmt(stmt),
        }
    }
}

impl From<RawStmt> for AstVariant {
    fn from(stmt: RawStmt) -> Self {
        AstVariant::Stmt(stmt)
    }
}
