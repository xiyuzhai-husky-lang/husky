structure MiracleM (γ: Type -> Type) (α : Type) where
  exec_batch : {o : Type} -> (List (α -> γ o)) -> γ o

instance [Monad γ] [inst: Alternative γ] : Monad <| MiracleM γ where
  pure x := {
    exec_batch := fun rs =>
      rs.foldr (fun r acc => r x <|> acc) failure
  }
  bind := fun m f => {
    exec_batch := fun rs =>
      m.exec_batch [fun a => (f a).exec_batch rs]
  }
