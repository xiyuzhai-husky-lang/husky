pub mod extend;
pub mod induct;
pub mod intro;
pub mod r#let;
pub mod prop;

use self::{
    extend::{ExtendShard, ExtendShardData},
    induct::{InductShard, InductShardData},
    intro::{IntroShard, IntroShardData},
    prop::{PropShard, PropShardData},
    r#let::{LetShard, LetShardData},
};

pub enum Shard {
    Extend(ExtendShard),
    Induct(InductShard),
    Intro(IntroShard),
    Let(LetShard),
    Prop(PropShard),
}

pub enum ShardData {
    Extend(ExtendShardData),
    Induct(InductShardData),
    Intro(IntroShardData),
    Let(LetShardData),
    Prop(PropShardData),
}

pub struct ShardId {}
