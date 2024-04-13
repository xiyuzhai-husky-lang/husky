module State (LogicState, defaultLogicState) where

import Registry
import Intro

data LogicState = LogicState {
    registry::LogicRegistry,
    intros:: [LogicIntro]
}
    deriving (Show)

defaultLogicState::LogicState
defaultLogicState =  LogicState {
    registry=defaultRegistry,
    intros=[]
}