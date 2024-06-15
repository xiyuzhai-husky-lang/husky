import Lake
open Lake DSL

package «cybertron-lean» where
  -- add package configuration options here

lean_lib «CybertronLean» where
  -- add library configuration options here

@[default_target]
lean_exe «cybertron-lean» where
  root := `Main
