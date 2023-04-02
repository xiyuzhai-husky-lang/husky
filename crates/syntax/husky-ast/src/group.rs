use crate::AstIdxRange;
pub enum AstGroup {
    ModuleItems(ModuleItems),
    FormBody(FormBody),
    TypeImplBlockItems(TypeImplBlockItems),
    TraitForTypeImplBlockItems(TraitForTypeImplBlockItems),
    TypeVariants(TypeVariants),
}

pub struct ModuleItems {
    ast_idx_range: AstIdxRange,
}

pub struct FormBody {
    ast_idx_range: AstIdxRange,
}

pub struct TypeImplBlockItems {
    ast_idx_range: AstIdxRange,
}

pub struct TraitForTypeImplBlockItems {
    ast_idx_range: AstIdxRange,
}

pub struct TypeVariants {
    ast_idx_range: AstIdxRange,
}
