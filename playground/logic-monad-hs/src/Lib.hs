module Lib
    ( printStatement, LogicM
    ) where
import Monad
import State


printStatement :: IO ()
printStatement = do
    let state = defaultLogicState
    let (term, state') = runLogicM statement state
    putStrLn ("term = " ++ show term ++ ",\nstate = " ++ show state')

statement:: LogicM Term
statement = aArb dog

data Term = Noun String
    | NounPhrase String [NounModifier]
    | TermIntro
    deriving Show

data NounModifier = Adjective String
    | Other
    deriving Show


dog::LogicM Term
dog = pure $ Noun "dog"

aArb::LogicM Term -> LogicM Term
aArb term = do
    a <- term
    case a of
        Noun word -> do
            new_intro <- introArbFromNounM word
            return TermIntro
        NounPhrase word modifiers -> undefined
        TermIntro -> undefined