import Lake
open Lake DSL

package «super-computation-graph-lean» where
  -- add package configuration options here

lean_lib «SuperComputationGraphLean» where
  -- add library configuration options here

@[default_target]
lean_exe «super-computation-graph-lean» where
  root := `Main
