import Specs.syntax

inductive EagerExpr
  | Variable ( varname : String )
  | ThisValue
  | ThisField ( field_ident: String )
  | PrimitiveLiteral (data : PrimitiveLiteralData)
  | EnumKindLiteral (route : EntityRoute)
  | Bracketed ( expr : EagerExpr )
  | Opn (opds : List EagerExpr)
  | Lambda
  | EntityThickFp
  | EntityFeature


instance EagerExpr.deq : DecidableEq EagerExpr
  -- Variable
  | Variable a, Variable b =>
    match String.decEq a b with
      | isTrue h => isTrue (by rw[h])
      | isFalse h => isFalse (by simp[h])
  | Variable _, ThisValue => isFalse (by simp)
  | Variable _, ThisField _ => isFalse (by simp)
  | Variable _, PrimitiveLiteral _ => isFalse (by simp)
  | Variable _, EnumKindLiteral _ => isFalse (by simp)
  | Variable _, Bracketed _ => isFalse (by simp)
  | Variable _, Opn _ => isFalse (by simp)
  | Variable _, Lambda => isFalse (by simp)
  | Variable _, EntityThickFp => isFalse (by simp)
  | Variable _, EntityFeature => isFalse (by simp)
  
  -- ThisValue
  | ThisValue, Variable _ => isFalse (by simp)
  | ThisValue, ThisValue => isTrue (by rfl)
  | ThisValue, ThisField _ => isFalse (by simp)
  | ThisValue, PrimitiveLiteral _ => isFalse (by simp)
  | ThisValue, EnumKindLiteral _ => isFalse (by simp)
  | ThisValue, Bracketed _ => isFalse (by simp)
  | ThisValue, Opn _ => isFalse (by simp)
  | ThisValue, Lambda => isFalse (by simp)
  | ThisValue, EntityThickFp => isFalse (by simp)
  | ThisValue, EntityFeature => isFalse (by simp)

  -- ThisField _
  | ThisField _, Variable _ => isFalse (by simp)
  | ThisField _, ThisValue => isFalse (by simp)
  | ThisField a, ThisField b =>
    match decEq a b with
      | isTrue h => isTrue (by rw[h])
      | isFalse h => isFalse (by simp[h])
  | ThisField _, PrimitiveLiteral _ => isFalse (by simp)
  | ThisField _, EnumKindLiteral _ => isFalse (by simp)
  | ThisField _, Bracketed _ => isFalse (by simp)
  | ThisField _, Opn _ => isFalse (by simp)
  | ThisField _, Lambda => isFalse (by simp)
  | ThisField _, EntityThickFp => isFalse (by simp)
  | ThisField _, EntityFeature => isFalse (by simp)

  -- PrimitiveLiteral _
  | PrimitiveLiteral _, Variable _ => isFalse (by simp)
  | PrimitiveLiteral _, ThisValue => isFalse (by simp)
  | PrimitiveLiteral _, ThisField _ => isFalse (by simp)
  | PrimitiveLiteral a, PrimitiveLiteral b =>
    match decEq a b with
      | isTrue h => isTrue (by rw[h])
      | isFalse h => isFalse (by simp[h])
  | PrimitiveLiteral _, EnumKindLiteral _ => isFalse (by simp)
  | PrimitiveLiteral _, Bracketed _ => isFalse (by simp)
  | PrimitiveLiteral _, Opn _ => isFalse (by simp)
  | PrimitiveLiteral _, Lambda => isFalse (by simp)
  | PrimitiveLiteral _, EntityThickFp => isFalse (by simp)
  | PrimitiveLiteral _, EntityFeature => isFalse (by simp)

  | _, _ => sorry

inductive Expr
  | Variable
  | Opn (opds : List Expr)

instance Expr.deq : DecidableEq Expr
| Variable, Variable => isTrue rfl
| Variable, Opn _ => isFalse (by simp)
| Opn _, Variable => isFalse (by simp)
| Opn [], Opn [] => isTrue rfl
| Opn (_ :: _), Opn [] => isFalse (by simp)
| Opn [], Opn (_ :: _) => isFalse (by simp)
| Opn (x :: xs), Opn (y :: ys) =>
  have := Expr.deq x y
  if hx : x = y then
    match Expr.deq (Opn xs) (Opn ys) with
    | isTrue h => isTrue (by { simp at h; simp [hx, h] })
    | isFalse h => isFalse (by { intro h'; apply h; simp at h'; simp [h'.2] })
  else isFalse (by simp [hx])