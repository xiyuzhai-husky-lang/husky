import Lake
open Lake DSL

package «specs» {
  -- add package configuration options here
}

lean_lib Specs {
  -- add library configuration options here
}

@[default_target]
lean_exe «specs» {
  root := `Main
}
