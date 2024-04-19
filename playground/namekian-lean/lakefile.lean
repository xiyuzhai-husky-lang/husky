import Lake
open Lake DSL

package «namekian-lean» where
  -- add package configuration options here

lean_lib «NamekianLean» where
  -- add library configuration options here

@[default_target]
lean_exe «namekian-lean» where
  root := `Main
