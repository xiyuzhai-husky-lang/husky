pub mod defn;
pub mod extend;
pub mod induct;
pub mod intro;
pub mod r#let;
pub mod prop;
pub mod theorem;

use self::{
    defn::{DefnShard, DefnShardData},
    extend::{ExtendShard, ExtendShardData},
    induct::{InductShard, InductShardData},
    intro::{IntroShard, IntroShardData},
    prop::{PropShard, PropShardData},
    r#let::{LetShard, LetShardData},
    theorem::TheoremShard,
};

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
