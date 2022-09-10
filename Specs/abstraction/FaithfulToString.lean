structure FaithfulToString (α : Type) [ToString α] where
  hvalid : ∀ a b : α, (a = b) = ((ToString.toString a) = (ToString.toString b))
