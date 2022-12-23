import Specs.syntax

inductive EagerExpr
  | Variable ( varname : String )
  | ThisValue
  | ThisField ( field_ident: String )
  | PrimitiveLiteral (data : LiteralToken)
  | EnumKindLiteral (route : EntityRoute)
  | Bracketed ( expr : EagerExpr )
  | Opn (opds : List EagerExpr)
  | Lambda (whatever : Nat)
  | EntityThickFp (whatever : Nat)
  | EntityFeature (whatever : Nat)


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
  | Variable _, Lambda _ => isFalse (by simp)
  | Variable _, EntityThickFp _ => isFalse (by simp)
  | Variable _, EntityFeature _ => isFalse (by simp)
  
  -- ThisValue
  | ThisValue, Variable _ => isFalse (by simp)
  | ThisValue, ThisValue => isTrue (by rfl)
  | ThisValue, ThisField _ => isFalse (by simp)
  | ThisValue, PrimitiveLiteral _ => isFalse (by simp)
  | ThisValue, EnumKindLiteral _ => isFalse (by simp)
  | ThisValue, Bracketed _ => isFalse (by simp)
  | ThisValue, Opn _ => isFalse (by simp)
  | ThisValue, Lambda _ => isFalse (by simp)
  | ThisValue, EntityThickFp _ => isFalse (by simp)
  | ThisValue, EntityFeature _ => isFalse (by simp)

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
  | ThisField _, Lambda _ => isFalse (by simp)
  | ThisField _, EntityThickFp _ => isFalse (by simp)
  | ThisField _, EntityFeature _ => isFalse (by simp)

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
  | PrimitiveLiteral _, Lambda _ => isFalse (by simp)
  | PrimitiveLiteral _, EntityThickFp _ => isFalse (by simp)
  | PrimitiveLiteral _, EntityFeature _ => isFalse (by simp)
  
  -- EnumKindLiteral _
  | EnumKindLiteral _, Variable _ => isFalse (by simp)
  | EnumKindLiteral _, ThisValue => isFalse (by simp)
  | EnumKindLiteral _, ThisField _ => isFalse (by simp)
  | EnumKindLiteral _, PrimitiveLiteral _ => isFalse (by simp)
  | EnumKindLiteral a, EnumKindLiteral b => 
    match decEq a b with
      | isTrue h => isTrue (by rw[h])
      | isFalse h => isFalse (by simp[h])
  | EnumKindLiteral _, Bracketed _ => isFalse (by simp)
  | EnumKindLiteral _, Opn _ => isFalse (by simp)
  | EnumKindLiteral _, Lambda _ => isFalse (by simp)
  | EnumKindLiteral _, EntityThickFp _ => isFalse (by simp)
  | EnumKindLiteral _, EntityFeature _ => isFalse (by simp)
  
  -- Bracketed _
  | Bracketed _, Variable _ => isFalse (by simp)
  | Bracketed _, ThisValue => isFalse (by simp)
  | Bracketed _, ThisField _ => isFalse (by simp)
  | Bracketed _, PrimitiveLiteral _ => isFalse (by simp)
  | Bracketed _, EnumKindLiteral _ => isFalse (by simp)
  | Bracketed a, Bracketed b =>
    match EagerExpr.deq a b with
      | isTrue h => isTrue (by rw[h])
      | isFalse h => isFalse (by simp[h])
  | Bracketed _, Opn _ => isFalse (by simp)
  | Bracketed _, Lambda _ => isFalse (by simp)
  | Bracketed _, EntityThickFp _ => isFalse (by simp)
  | Bracketed _, EntityFeature _ => isFalse (by simp)

  -- Lambda _
  | Lambda _, Variable _ => isFalse (by simp)
  | Lambda _, ThisValue => isFalse (by simp)
  | Lambda _, ThisField _ => isFalse (by simp)
  | Lambda _, PrimitiveLiteral _ => isFalse (by simp)
  | Lambda _, EnumKindLiteral _ => isFalse (by simp)
  | Lambda _, Bracketed b => isFalse (by simp)
  | Lambda _, Opn _ => isFalse (by simp)
  | Lambda a, Lambda b =>
    match decEq a b with
      | isTrue h => isTrue (by rw[h])
      | isFalse h => isFalse (by simp[h])
  | Lambda _, EntityThickFp _ => isFalse (by simp)
  | Lambda _, EntityFeature _ => isFalse (by simp)

  -- EntityThickFp _
  | EntityThickFp _, Variable _ => isFalse (by simp)
  | EntityThickFp _, ThisValue => isFalse (by simp)
  | EntityThickFp _, ThisField _ => isFalse (by simp)
  | EntityThickFp _, PrimitiveLiteral _ => isFalse (by simp)
  | EntityThickFp _, EnumKindLiteral _ => isFalse (by simp)
  | EntityThickFp _, Bracketed b => isFalse (by simp)
  | EntityThickFp _, Opn _ => isFalse (by simp)
  | EntityThickFp _, Lambda _ => isFalse (by simp)
  | EntityThickFp a, EntityThickFp b =>
    match decEq a b with
      | isTrue h => isTrue (by rw[h])
      | isFalse h => isFalse (by simp[h])
  | EntityThickFp _, EntityFeature _ => isFalse (by simp)

  -- EntityFeature _
  | EntityFeature _, Variable _ => isFalse (by simp)
  | EntityFeature _, ThisValue => isFalse (by simp)
  | EntityFeature _, ThisField _ => isFalse (by simp)
  | EntityFeature _, PrimitiveLiteral _ => isFalse (by simp)
  | EntityFeature _, EnumKindLiteral _ => isFalse (by simp)
  | EntityFeature _, Bracketed b => isFalse (by simp)
  | EntityFeature _, Opn _ => isFalse (by simp)
  | EntityFeature _, Lambda _ => isFalse (by simp)
  | EntityFeature _, EntityThickFp _ => isFalse (by simp)
  | EntityFeature a, EntityFeature b =>
    match decEq a b with
      | isTrue h => isTrue (by rw[h])
      | isFalse h => isFalse (by simp[h])

  -- Opn _
  | Opn _, Variable _ => isFalse (by simp)
  | Opn _, ThisValue => isFalse (by simp)
  | Opn _, ThisField _ => isFalse (by simp)
  | Opn _, PrimitiveLiteral _ => isFalse (by simp)
  | Opn _, EnumKindLiteral _ => isFalse (by simp)
  | Opn _, Bracketed b => isFalse (by simp)
  | Opn a, Opn b =>
    match a, b with
    | [], [] => isTrue (by simp)
    | a::as, [] => isFalse (by simp)
    | [], b::bs => isFalse (by simp)
    | _, _ => sorry
    -- match decEq a b with
    --   | isTrue h => isTrue (by rw[h])
    --   | isFalse h => isFalse (by simp[h])
  | Opn _, Lambda _ => isFalse (by simp)
  | Opn _, EntityThickFp _ => isFalse (by simp)
  | Opn _, EntityFeature _ => isFalse (by simp)

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