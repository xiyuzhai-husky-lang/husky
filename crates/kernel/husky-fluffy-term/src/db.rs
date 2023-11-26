use crate::*;
use husky_ethereal_signature::EtherealSignatureDb;

pub trait FluffyTermDb: salsa::DbWithJar<FluffyTermJar> + EtherealSignatureDb {}

impl FluffyTermDb for Db where Db: salsa::DbWithJar<FluffyTermJar> + EtherealSignatureDb {}
