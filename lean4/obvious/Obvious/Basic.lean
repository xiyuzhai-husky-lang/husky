import Mathlib

namespace Obvious

def hello := "world"

syntax "obvious" : tactic

macro_rules
  | `(tactic| obvious) => `(tactic|
      congr;
      try first
      | simp
      | (nlinarith)
      | ((apply div_nonneg; repeat obvious))
      | (field_simp; ring)
      | linarith
    )

end Obvious
