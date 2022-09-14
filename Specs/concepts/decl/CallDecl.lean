import Specs.concepts.Concept

structure ParameterDecl where
  concept : Concept

structure CallDecl where
  parameters : List ParameterDecl