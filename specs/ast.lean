structure File

structure Range

structure RawStmt

inductive UseVariant

inductive AstVariant where
  | TypeDefnHead
  | MainDefnHead
  | CallFormDefnHead
  | FeatureDefnHead
  | FieldDefnHead
  | DatasetConfigDefnHead
  | Stmt (stmt : RawStmt) : AstVariant
  | Use (use_variant : UseVariant) : AstVariant
  | Submodule
  | Visual

structure Ast where
  file : File
  range : Range
  variant : AstVariant