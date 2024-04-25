module Intro where

data LogicIntro = LogicIntro {
    varid::Int,
    kind::LogicIntroKind,
    typename::String
}
    deriving Show

data LogicIntroKind = Arb | Exists | NotExists
    deriving Show
