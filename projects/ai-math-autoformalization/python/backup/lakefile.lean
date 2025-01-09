import Lake
open Lake DSL

package mathproof {
  -- add package configuration options here
}

require mathlib from git
  "https://github.com/leanprover-community/mathlib4.git"

@[default_target]
lean_lib Mathproof {
  -- add library configuration options here
}
