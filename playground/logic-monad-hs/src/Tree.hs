module Tree(LogicTree,defaultTree) where
import Registry

data LogicTree = LogicTree {
    nodes::[Int],
    registry::LogicRegistry
}
    deriving (Show)

defaultTree::LogicTree
defaultTree=LogicTree {
    nodes=[],
    registry=defaultRegistry
}