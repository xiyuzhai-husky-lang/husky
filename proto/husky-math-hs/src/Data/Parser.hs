{-# OPTIONS_GHC -Wno-unrecognised-pragmas #-}

{-# HLINT ignore "Use newtype instead of data" #-}
module Data.Parser (Parser) where
import Control.Monad ( ap )

data Parser stream  state  a = Parser { parse_f :: stream -> state -> (state, a) }

instance Functor (Parser stream state) where
  fmap :: (a -> b) -> Parser stream  state  a -> Parser stream  state  b
  fmap f p = Parser {
    parse_f = \stream state -> let (state', a) = parse_f p stream state in (state', f a)
  }

instance Applicative (Parser stream state) where
  pure :: a -> Parser stream state a
  pure a = Parser (\ _ state -> (state, a))
  (<*>) :: Parser stream state (a -> b) -> Parser stream state a -> Parser stream state b
  (<*>) = ap

instance Monad (Parser stream state) where
  (>>=) :: Parser stream state a -> (a -> Parser stream state b) -> Parser stream state b
  (>>=) parser subsequent_f = Parser (\ stream state -> 
    let (state', a) = parse_f parser stream state in
    let parser' = subsequent_f a in
    let (state'', b) = parse_f parser' stream state' in
        (state'', b))
  return :: a -> Parser stream state a
  return = pure
