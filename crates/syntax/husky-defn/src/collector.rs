use crate::*;
use husky_ast::{Ast, AstSheet, AstTokenIdxRangeSheet};
use husky_entity_tree::{EntityTreeResult, ModuleSymbolContext};
use husky_token::{TokenSheetData};
use vec_like::VecPairMap;
pub(crate) struct DefnCollector<'a> {
    db: &'a dyn DefnDb,
    module_symbol_context: ModuleSymbolContext<'a>,
    token_sheet_data: &'a TokenSheetData,
    ast_sheet: &'a AstSheet,
    ast_range_sheet: &'a AstTokenIdxRangeSheet,
    decl_sheet: DeclSheet<'a>,
}

impl<'a> DefnCollector<'a> {
    pub(crate) fn new(db: &'a dyn DefnDb, module_path: ModulePath) -> EntityTreeResult<Self> {
        let module_symbol_context = db.module_symbol_context(module_path)?;
        Ok(Self {
            db,
            module_symbol_context,
            token_sheet_data: db.token_sheet_data(module_path)?,
            ast_sheet: db.ast_sheet(module_path)?,
            ast_range_sheet: db.ast_range_sheet(module_path)?,
            decl_sheet: db.decl_sheet(module_path)?,
        })
    }

    pub(crate) fn collect_all(self) -> DefnSheet<'a> {
        let mut defns: VecPairMap<DefnRegionPath, DeclResultRef<'a, Defn>> = Default::default();
        for (path, decl) in self.decl_sheet.decls().iter().copied() {
            defns
                .insert_new((
                    path.defn_region_path(),
                    decl.map(|decl| defn(self.db, decl)),
                ))
                .unwrap()
        }
        DefnSheet::new(defns)
    }
}

fn defn(db: &dyn DefnDb, decl: Decl) -> Defn {
    match decl {
        Decl::Type(decl) => ty_defn(db, decl).into(),
        Decl::Form(decl) => form_defn(db, decl).into(),
        Decl::Trait(decl) => trai_defn(db, decl).into(),
        Decl::Impl(decl) => Defn::Impl(decl),
        Decl::AssociatedItem(decl) => associated_item_defn(db, decl).into(),
        Decl::Variant(_) => todo!(),
    }
}

fn ty_defn(db: &dyn DefnDb, decl: TypeDecl) -> TypeDefn {
    match decl {
        TypeDecl::Enum(decl) => enum_ty_defn(db, decl).into(),
        TypeDecl::RegularStruct(decl) => regular_struct_ty_defn(db, decl).into(),
        TypeDecl::TupleStruct(decl) => tuple_struct_ty_defn(db, decl).into(),
        TypeDecl::UnitStruct(decl) => unit_struct_ty_defn(db, decl).into(),
        TypeDecl::Record(decl) => record_ty_defn(db, decl).into(),
        TypeDecl::Inductive(decl) => inductive_ty_defn(db, decl).into(),
        TypeDecl::Structure(decl) => structure_ty_defn(db, decl).into(),
        TypeDecl::Extern(decl) => alien_ty_defn(db, decl).into(),
        TypeDecl::Union(decl) => union_ty_defn(db, decl).into(),
    }
}

#[salsa::tracked(jar = DefnJar)]
pub(crate) fn enum_ty_defn(db: &dyn DefnDb, decl: EnumTypeDecl) -> EnumTypeDefn {
    let path = decl.path(db);
    EnumTypeDefn::new(db, path, decl)
}

#[salsa::tracked(jar = DefnJar)]
pub(crate) fn regular_struct_ty_defn(
    db: &dyn DefnDb,
    decl: RegularStructTypeDecl,
) -> RegularStructTypeDefn {
    let path = decl.path(db);
    RegularStructTypeDefn::new(db, path, decl)
}

#[salsa::tracked(jar = DefnJar)]
pub(crate) fn tuple_struct_ty_defn(
    db: &dyn DefnDb,
    decl: TupleStructTypeDecl,
) -> TupleStructTypeDefn {
    let path = decl.path(db);
    TupleStructTypeDefn::new(db, path, decl)
}

#[salsa::tracked(jar = DefnJar)]
pub(crate) fn unit_struct_ty_defn(db: &dyn DefnDb, decl: UnitStructTypeDecl) -> UnitStructTypeDefn {
    let path = decl.path(db);
    UnitStructTypeDefn::new(db, path, decl)
}

#[salsa::tracked(jar = DefnJar)]
pub(crate) fn inductive_ty_defn(db: &dyn DefnDb, decl: InductiveTypeDecl) -> InductiveTypeDefn {
    let path = decl.path(db);
    InductiveTypeDefn::new(db, path, decl)
}

#[salsa::tracked(jar = DefnJar)]
pub(crate) fn structure_ty_defn(db: &dyn DefnDb, decl: StructureTypeDecl) -> StructureTypeDefn {
    let path = decl.path(db);
    StructureTypeDefn::new(db, path, decl)
}

#[salsa::tracked(jar = DefnJar)]
pub(crate) fn alien_ty_defn(db: &dyn DefnDb, decl: ExternTypeDecl) -> ExternTypeDefn {
    let path = decl.path(db);
    ExternTypeDefn::new(db, path, decl)
}

#[salsa::tracked(jar = DefnJar)]
pub(crate) fn record_ty_defn(db: &dyn DefnDb, decl: RecordTypeDecl) -> RecordTypeDefn {
    let path = decl.path(db);
    RecordTypeDefn::new(db, path, decl)
}

#[salsa::tracked(jar = DefnJar)]
pub(crate) fn union_ty_defn(db: &dyn DefnDb, decl: UnionTypeDecl) -> UnionTypeDefn {
    let path = decl.path(db);
    UnionTypeDefn::new(db, path, decl)
}

#[salsa::tracked(jar = DefnJar)]
pub(crate) fn trai_defn(db: &dyn DefnDb, decl: TraitDecl) -> TraitDefn {
    let path = decl.path(db);
    TraitDefn::new(db, path, decl)
}

fn form_defn(db: &dyn DefnDb, decl: FormDecl) -> FormDefn {
    match decl {
        FormDecl::Function(decl) => function_defn(db, decl).into(),
        FormDecl::Feature(decl) => feature_defn(db, decl).into(),
        FormDecl::Morphism(_) => todo!(),
        FormDecl::Value(_) => todo!(),
    }
}

#[salsa::tracked(jar = DefnJar)]
pub(crate) fn function_defn(db: &dyn DefnDb, decl: FunctionDecl) -> FunctionDefn {
    let path = decl.path(db);
    let mut parser = expr_parser(
        db,
        DefnRegionPath::Entity(path.into()),
        Some(decl.expr_region(db)),
        AllowSelfType::False,
        AllowSelfValue::False,
    );
    let ast_idx = decl.ast_idx(db);
    let body = match parser.ast_sheet()[ast_idx] {
        Ast::Defn { body, .. } => parser
            .parse_block_expr(body)
            .ok_or(OriginalDefnError::ExpectBody.into()), // todo: change this to parse expected
        _ => unreachable!(),
    };
    let expr_region = parser.finish();
    FunctionDefn::new(db, path, decl, expr_region, body)
}

#[salsa::tracked(jar = DefnJar)]
pub(crate) fn feature_defn(db: &dyn DefnDb, decl: FeatureDecl) -> FeatureDefn {
    let path = decl.path(db);
    let mut parser = expr_parser(
        db,
        DefnRegionPath::Entity(path.into()),
        None,
        AllowSelfType::False,
        AllowSelfValue::False,
    );
    let ast_idx = decl.ast_idx(db);
    let body = match parser.ast_sheet()[ast_idx] {
        Ast::Defn { body, .. } => parser
            .parse_block_expr(body)
            .ok_or(OriginalDefnError::ExpectBody.into()), // todo: change this to parse expected
        _ => unreachable!(),
    };
    let expr_region = parser.finish();
    FeatureDefn::new(db, path, decl, expr_region, body)
}

#[salsa::tracked(jar = DefnJar)]
pub(crate) fn unit_variant_defn(_db: &dyn DefnDb, _decl: UnitVariantDecl) -> UnitVariantDefn {
    todo!()
}

#[salsa::tracked(jar = DefnJar)]
pub(crate) fn tuple_variant_defn(_db: &dyn DefnDb, _decl: TupleVariantDecl) -> TupleVariantDefn {
    todo!()
}

#[salsa::tracked(jar = DefnJar)]
pub(crate) fn props_variant_defn(_db: &dyn DefnDb, _decl: PropsVariantDecl) -> PropsVariantDefn {
    todo!()
}

fn associated_item_defn(db: &dyn DefnDb, decl: AssociatedItemDecl) -> AssociatedItemDefn {
    match decl {
        AssociatedItemDecl::TypeItem(decl) => ty_item_defn(db, decl).into(),
        AssociatedItemDecl::TraitItem(decl) => trai_item_defn(db, decl).into(),
        AssociatedItemDecl::TypeAsTraitItem(decl) => ty_as_trai_item_defn(db, decl).into(),
    }
}

fn ty_item_defn(db: &dyn DefnDb, decl: TypeItemDecl) -> TypeItemDefn {
    match decl {
        TypeItemDecl::Function(_) => todo!(),
        TypeItemDecl::Method(decl) => ty_method_defn(db, decl).into(),
        TypeItemDecl::ExternType(_) => todo!(),
        TypeItemDecl::Value(_) => todo!(),
        TypeItemDecl::Memo(decl) => ty_memo_defn(db, decl).into(),
    }
}

#[salsa::tracked(jar = DefnJar)]
pub(crate) fn ty_associated_function_defn(
    _db: &dyn DefnDb,
    _decl: TypeAssociatedFunctionDecl,
) -> TypeAssociatedFunctionDefn {
    todo!()
}

#[salsa::tracked(jar = DefnJar)]
pub(crate) fn ty_method_defn(db: &dyn DefnDb, decl: TypeMethodDecl) -> TypeMethodDefn {
    let path = decl.path(db);
    let mut parser = expr_parser(
        db,
        DefnRegionPath::AssociatedItem(decl.associated_item(db).id(db)),
        Some(decl.expr_region(db)),
        AllowSelfType::True,
        AllowSelfValue::True,
    );
    let ast_idx = decl.ast_idx(db);
    let body = match parser.ast_sheet()[ast_idx] {
        Ast::Defn { body, .. } => parser
            .parse_block_expr(body)
            .ok_or(OriginalDefnError::ExpectBody.into()), // todo: change this to parse expected
        _ => unreachable!(),
    };
    let expr_region = parser.finish();
    TypeMethodDefn::new(db, path, decl, expr_region, body)
}

#[salsa::tracked(jar = DefnJar)]
pub(crate) fn ty_associated_ty_defn(
    _db: &dyn DefnDb,
    _decl: TypeAssociatedTypeDecl,
) -> TypeAssociatedTypeDefn {
    todo!()
}

#[salsa::tracked(jar = DefnJar)]
pub(crate) fn ty_associated_value_defn(
    _db: &dyn DefnDb,
    _decl: TypeAssociatedValueDecl,
) -> TypeAssociatedValueDefn {
    todo!()
}

#[salsa::tracked(jar = DefnJar)]
pub(crate) fn ty_memo_defn(db: &dyn DefnDb, decl: TypeMemoDecl) -> TypeMemoDefn {
    let path = decl.path(db);
    let mut parser = expr_parser(
        db,
        DefnRegionPath::AssociatedItem(decl.associated_item(db).id(db)),
        Some(decl.expr_region(db)),
        AllowSelfType::True,
        AllowSelfValue::True,
    );
    let ast_idx = decl.ast_idx(db);
    let body = match parser.ast_sheet()[ast_idx] {
        Ast::Defn { body, .. } => parser
            .parse_block_expr(body)
            .ok_or(OriginalDefnError::ExpectBody.into()), // todo: change this to parse expected
        _ => unreachable!(),
    };
    let expr_region = parser.finish();
    TypeMemoDefn::new(db, path, decl, expr_region, body)
}

fn trai_item_defn(db: &dyn DefnDb, decl: TraitItemDecl) -> TraitItemDefn {
    match decl {
        TraitItemDecl::AssociatedFunction(decl) => trai_associated_function_defn(db, decl).into(),
        TraitItemDecl::Method(decl) => trai_method_defn(db, decl).into(),
        TraitItemDecl::AssociatedType(_decl) => todo!(),
        TraitItemDecl::Value(_decl) => todo!(),
    }
}

#[salsa::tracked(jar = DefnJar)]
pub(crate) fn trai_associated_function_defn(
    _db: &dyn DefnDb,
    _decl: TraitAssociatedFunctionDecl,
) -> TraitAssociatedFunctionDefn {
    todo!()
}

#[salsa::tracked(jar = DefnJar)]
pub(crate) fn trai_method_defn(_db: &dyn DefnDb, _decl: TraitMethodDecl) -> TraitMethodDefn {
    todo!()
}

#[salsa::tracked(jar = DefnJar)]
pub(crate) fn trai_associated_ty_defn(
    _db: &dyn DefnDb,
    _decl: TraitAssociatedTypeDecl,
) -> TraitAssociatedTypeDefn {
    todo!()
}

#[salsa::tracked(jar = DefnJar)]
pub(crate) fn trai_associated_value_defn(
    _db: &dyn DefnDb,
    _decl: TraitAssociatedValueDecl,
) -> TraitAssociatedValueDefn {
    todo!()
}

fn ty_as_trai_item_defn(db: &dyn DefnDb, decl: TypeAsTraitItemDecl) -> TypeAsTraitItemDefn {
    match decl {
        TypeAsTraitItemDecl::AssociatedFunction(_) => todo!(),
        TypeAsTraitItemDecl::Method(decl) => ty_as_trai_method_defn(db, decl).into(),
        TypeAsTraitItemDecl::AssociatedType(decl) => ty_as_trai_associated_ty_defn(db, decl).into(),
        TypeAsTraitItemDecl::AssociatedValue(decl) => {
            ty_as_trai_associated_value_defn(db, decl).into()
        }
    }
}

#[salsa::tracked(jar = DefnJar)]
pub(crate) fn ty_as_trai_associated_function_defn(
    _db: &dyn DefnDb,
    _decl: TypeAsTraitAssociatedFunctionDecl,
) -> TypeAsTraitAssociatedFunctionDefn {
    todo!()
}

#[salsa::tracked(jar = DefnJar)]
pub(crate) fn ty_as_trai_method_defn(
    db: &dyn DefnDb,
    decl: TypeAsTraitMethodDecl,
) -> TypeAsTraitMethodDefn {
    let path = decl.path(db);
    let mut parser = expr_parser(
        db,
        DefnRegionPath::AssociatedItem(decl.associated_item(db).id(db)),
        Some(decl.expr_region(db)),
        AllowSelfType::True,
        AllowSelfValue::True,
    );
    let ast_idx = decl.ast_idx(db);
    let body = match parser.ast_sheet()[ast_idx] {
        Ast::Defn { body, .. } => parser
            .parse_block_expr(body)
            .ok_or(OriginalDefnError::ExpectBody.into()), // todo: change this to parse expected
        _ => unreachable!(),
    };
    let expr_region = parser.finish();
    TypeAsTraitMethodDefn::new(db, path, decl, expr_region, body)
}

#[salsa::tracked(jar = DefnJar)]
pub(crate) fn ty_as_trai_associated_ty_defn(
    _db: &dyn DefnDb,
    _decl: TypeAsTraitAssociatedTypeDecl,
) -> TypeAsTraitAssociatedTypeDefn {
    todo!()
}

#[salsa::tracked(jar = DefnJar)]
pub(crate) fn ty_as_trai_associated_value_defn(
    _db: &dyn DefnDb,
    _decl: TypeAsTraitAssociatedValueDecl,
) -> TypeAsTraitAssociatedValueDefn {
    todo!()
}

// fn expr_parser(
//     db:&dyn DefnDb,
//     expr_path: DefnRegionPath,
//     decl_expr_region: Option<ExprRegion>,
//     allow_self_type: AllowSelfType,
//     allow_self_value: AllowSelfValue,
// ) -> BlockExprParser<'a> {
//     let parser = ExprParser::new(
//         self.db,
//         expr_path.into(),
//         self.token_sheet_data,
//         self.module_symbol_context,
//         decl_expr_region,
//         allow_self_type,
//         allow_self_value,
//     );
//     BlockExprParser::new(parser, self.ast_sheet, self.ast_range_sheet)
// }

fn expr_parser<'a>(
    db: &'a dyn DefnDb,
    expr_path: DefnRegionPath,
    decl_expr_region: Option<ExprRegion>,
    allow_self_type: AllowSelfType,
    allow_self_value: AllowSelfValue,
) -> BlockExprParser<'a> {
    let module_path = expr_path.module_path(db);
    let parser = ExprParser::new(
        db,
        expr_path.into(),
        db.token_sheet_data(module_path).unwrap(),
        db.module_symbol_context(module_path).unwrap(),
        decl_expr_region,
        allow_self_type,
        allow_self_value,
    );
    BlockExprParser::new(
        parser,
        db.ast_sheet(module_path).unwrap(),
        db.ast_range_sheet(module_path).unwrap(),
        None, // ad hoc
    )
}
