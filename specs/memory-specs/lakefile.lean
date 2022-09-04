import Lake
open Lake DSL

package «memory-specs» {
  -- add package configuration options here
}

lean_lib MemorySpecs {
  -- add library configuration options here
}

@[defaultTarget]
lean_exe «memory-specs» {
  root := `Main
}
