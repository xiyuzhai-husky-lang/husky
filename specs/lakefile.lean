import Lake
open Lake DSL

package «husky-lean-specs» {
  -- add package configuration options here
}

lean_lib HuskyLeanSpecs {
  -- add library configuration options here
}

@[defaultTarget]
lean_exe «husky-lean-specs» {
  root := `Main
}
