//! Defines hir-level representation of structs, enums and unions

use std::sync::Arc;

use arena::{Arena, ArenaMap};
use base_db::CrateId;
use either::Either;
use hir_expand::{
    name::{AsName, Name},
    InFile,
};
use syntax::ast::{self, HasName, HasVisibility};
use tt::{Delimiter, DelimiterKind, Leaf, Subtree, TokenTree};

use crate::{
    body::LowerCtx,
    db::DefDatabase,
    intern::Interned,
    item_tree::{AttrOwner, Field, Fields, ItemTree, ModItem, RawVisibilityId},
    src::HasChildSource,
    src::HasSource,
    trace::Trace,
    type_ref::TypeRef,
    visibility::RawVisibility,
    EnumId, LocalEnumVariantId, LocalFieldId, Lookup, ModuleId, StructId, UnionId, VariantId,
};

/// Note that we use `StructData` for unions as well!
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructData {
    pub name: Name,
    pub variant_data: Arc<VariantData>,
    pub repr: Option<ReprKind>,
    pub visibility: RawVisibility,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumData {
    pub name: Name,
    pub variants: Arena<EnumVariantData>,
    pub visibility: RawVisibility,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumVariantData {
    pub name: Name,
    pub variant_data: Arc<VariantData>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VariantData {
    Record(Arena<FieldData>),
    Tuple(Arena<FieldData>),
    Unit,
}

/// A single field of an enum variant or struct
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldData {
    pub name: Name,
    pub type_ref: Interned<TypeRef>,
    pub visibility: RawVisibility,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReprKind {
    Packed,
    Other,
}

fn repr_from_value(
    db: &dyn DefDatabase,
    krate: CrateId,
    item_tree: &ItemTree,
    of: AttrOwner,
) -> Option<ReprKind> {
    item_tree
        .attrs(db, krate, of)
        .by_key("repr")
        .tt_values()
        .find_map(parse_repr_tt)
}

fn parse_repr_tt(tt: &Subtree) -> Option<ReprKind> {
    match tt.delimiter {
        Some(Delimiter {
            kind: DelimiterKind::Parenthesis,
            ..
        }) => {}
        _ => return None,
    }

    let mut it = tt.token_trees.iter();
    match it.next()? {
        TokenTree::Leaf(Leaf::Ident(ident)) if ident.text == "packed" => Some(ReprKind::Packed),
        _ => Some(ReprKind::Other),
    }
}

impl StructData {
    pub(crate) fn struct_data_query(db: &dyn DefDatabase, id: StructId) -> Arc<StructData> {
        let loc = id.lookup(db);
        let krate = loc.container.krate;
        let item_tree = loc.id.item_tree(db);
        let repr = repr_from_value(db, krate, &item_tree, ModItem::from(loc.id.value).into());

        let strukt = &item_tree[loc.id.value];
        let variant_data = lower_fields(db, krate, &item_tree, &strukt.fields, None);
        Arc::new(StructData {
            name: strukt.name.clone(),
            variant_data: Arc::new(variant_data),
            repr,
            visibility: item_tree[strukt.visibility].clone(),
        })
    }
    pub(crate) fn union_data_query(db: &dyn DefDatabase, id: UnionId) -> Arc<StructData> {
        let loc = id.lookup(db);
        let krate = loc.container.krate;
        let item_tree = loc.id.item_tree(db);
        let repr = repr_from_value(db, krate, &item_tree, ModItem::from(loc.id.value).into());

        let union = &item_tree[loc.id.value];
        let variant_data = lower_fields(db, krate, &item_tree, &union.fields, None);

        Arc::new(StructData {
            name: union.name.clone(),
            variant_data: Arc::new(variant_data),
            repr,
            visibility: item_tree[union.visibility].clone(),
        })
    }
}

impl EnumData {
    pub(crate) fn enum_data_query(db: &dyn DefDatabase, e: EnumId) -> Arc<EnumData> {
        todo!()
    }

    pub fn variant(&self, name: &Name) -> Option<LocalEnumVariantId> {
        let (id, _) = self
            .variants
            .iter()
            .find(|(_id, data)| &data.name == name)?;
        Some(id)
    }
}

impl HasChildSource<LocalEnumVariantId> for EnumId {
    type Value = ast::Variant;
    fn child_source(
        &self,
        db: &dyn DefDatabase,
    ) -> InFile<ArenaMap<LocalEnumVariantId, Self::Value>> {
        let src = self.lookup(db).source(db);
        let mut trace = Trace::new_for_map();
        lower_enum(db, &mut trace, &src, self.lookup(db).container);
        src.with_value(trace.into_map())
    }
}

fn lower_enum(
    db: &dyn DefDatabase,
    trace: &mut Trace<EnumVariantData, ast::Variant>,
    ast: &InFile<ast::Enum>,
    module_id: ModuleId,
) {
    todo!()
}

impl VariantData {
    fn new(db: &dyn DefDatabase, flavor: InFile<ast::StructKind>, module_id: ModuleId) -> Self {
        todo!()
    }

    pub fn fields(&self) -> &Arena<FieldData> {
        const EMPTY: &Arena<FieldData> = &Arena::new();
        match &self {
            VariantData::Record(fields) | VariantData::Tuple(fields) => fields,
            _ => EMPTY,
        }
    }

    pub fn field(&self, name: &Name) -> Option<LocalFieldId> {
        self.fields()
            .iter()
            .find_map(|(id, data)| if &data.name == name { Some(id) } else { None })
    }

    pub fn kind(&self) -> StructKind {
        match self {
            VariantData::Record(_) => StructKind::Record,
            VariantData::Tuple(_) => StructKind::Tuple,
            VariantData::Unit => StructKind::Unit,
        }
    }
}

impl HasChildSource<LocalFieldId> for VariantId {
    type Value = Either<ast::TupleField, ast::RecordField>;

    fn child_source(&self, db: &dyn DefDatabase) -> InFile<ArenaMap<LocalFieldId, Self::Value>> {
        todo!()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum StructKind {
    Tuple,
    Record,
    Unit,
}

fn lower_struct(
    db: &dyn DefDatabase,
    trace: &mut Trace<FieldData, Either<ast::TupleField, ast::RecordField>>,
    ast: &InFile<ast::StructKind>,
) -> StructKind {
    todo!()
}

fn lower_fields(
    db: &dyn DefDatabase,
    krate: CrateId,
    item_tree: &ItemTree,
    fields: &Fields,
    override_visibility: Option<RawVisibilityId>,
) -> VariantData {
    match fields {
        Fields::Record(flds) => {
            todo!()
        }
        Fields::Tuple(flds) => {
            todo!()
        }
        Fields::Unit => VariantData::Unit,
    }
}

fn lower_field(
    item_tree: &ItemTree,
    field: &Field,
    override_visibility: Option<RawVisibilityId>,
) -> FieldData {
    FieldData {
        name: field.name.clone(),
        type_ref: field.type_ref.clone(),
        visibility: item_tree[override_visibility.unwrap_or(field.visibility)].clone(),
    }
}
