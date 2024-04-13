{-# LANGUAGE TupleSections #-}
module Monad (LogicM, runLogicM, introM, introArbFromNounM) where
import State (LogicState, intro)
import Intro

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

introM::LogicIntro -> LogicM ()
introM new_intro = LogicM {
    runLogicM = \s ->
        let s' = intro s new_intro
        in ((), s')
}

introArbFromNounM::String -> LogicM LogicIntro
introArbFromNounM typename' = do
    return LogicIntro {varid=0,kind = Arb,typename=typename'}