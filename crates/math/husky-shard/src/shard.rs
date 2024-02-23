pub mod defn;
pub mod extend;
pub mod induct;
pub mod intro;
pub mod r#let;
pub mod prop;
pub mod theorem;

use self::{
    defn::{DefnShard, DefnShardData, DefnShardPath, DefnShardPathData},
    extend::{ExtendShard, ExtendShardData, ExtendShardPath, ExtendShardPathData},
    induct::{InductShard, InductShardData, InductShardPath, InductShardPathData},
    intro::{IntroShard, IntroShardData, IntroShardPath, IntroShardPathData},
    prop::{PropShard, PropShardData, PropShardPath, PropShardPathData},
    r#let::{LetShard, LetShardData, LetShardPath, LetShardPathData},
    theorem::{TheoremShard, TheoremShardPath, TheoremShardPathData},
};

pub enum ShardPath {
    Defn(DefnShardPath),
    Extend(ExtendShardPath),
    Induct(InductShardPath),
    Intro(IntroShardPath),
    Let(LetShardPath),
    Prop(PropShardPath),
    Theorem(TheoremShardPath),
}

pub enum ShardPathData {
    Defn(DefnShardPathData),
    Extend(ExtendShardPathData),
    Induct(InductShardPathData),
    Intro(IntroShardPathData),
    Let(LetShardPathData),
    Prop(PropShardPathData),
    Theorem(TheoremShardPathData),
}

pub enum Shard {
    Defn(DefnShard),
    Extend(ExtendShard),
    Induct(InductShard),
    Intro(IntroShard),
    Let(LetShard),
    Prop(PropShard),
    Theorem(TheoremShard),
}

pub enum ShardData {
    Defn(DefnShardData),
    Extend(ExtendShardData),
    Induct(InductShardData),
    Intro(IntroShardData),
    Let(LetShardData),
    Prop(PropShardData),
}

pub struct ShardId {}
