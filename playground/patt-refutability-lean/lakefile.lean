import Lake
open Lake DSL

package «patt-refutability-lean» where
  -- add package configuration options here

lean_lib «PattRefutabilityLean» where
  -- add library configuration options here

@[default_target]
lean_exe «patt-refutability-lean» where
  root := `Main
