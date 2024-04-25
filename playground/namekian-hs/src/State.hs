module State (LogicState, defaultLogicState, intro) where

import Registry
import Intro
import Tree

data LogicState = LogicState {
    registry::LogicRegistry,
    intros:: [LogicIntro],
    tree::LogicTree
}
    deriving (Show)

defaultLogicState::LogicState
defaultLogicState =  LogicState {
    registry=defaultRegistry,
    intros=[],
    tree=defaultTree
}

intro::LogicState -> LogicIntro -> LogicState
intro s new_intro =
    let intros' = intros s ++ [new_intro]
    in s { intros = intros' }