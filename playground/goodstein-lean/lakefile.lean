import Lake
open Lake DSL

package «goodstein-lean» where
  -- add package configuration options here

lean_lib «GoodsteinLean» where
  -- add library configuration options here

@[default_target]
lean_exe «goodstein-lean» where
  root := `Main
