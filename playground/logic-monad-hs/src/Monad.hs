module Monad (LogicM, runLogicM) where
import State (LogicState)

newtype LogicM a = LogicM { runLogicM :: LogicState -> (a, LogicState) }

instance Functor LogicM where
  fmap f a = LogicM {
    runLogicM = \s ->
    let (b, new_s) = runLogicM a s
    in (f b, new_s)
}

instance Applicative LogicM where
  pure a = LogicM { runLogicM = (a,) }
  (<*>) f a = LogicM {
    runLogicM = \s ->
        let (f', s') = runLogicM f s in
        let (b, s'') = runLogicM a s'
        in (f' b, s'')
  }

instance Monad LogicM where
  (>>=) am f = LogicM {
    runLogicM = \s ->
        let (a, s') = runLogicM am s in
        runLogicM (f a) s'
  }