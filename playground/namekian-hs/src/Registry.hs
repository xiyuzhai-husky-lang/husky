module Registry (LogicRegistry, defaultRegistry) where

newtype LogicRegistry = LogicRegistry Int
    deriving (Show)

defaultRegistry::LogicRegistry
defaultRegistry = LogicRegistry 0