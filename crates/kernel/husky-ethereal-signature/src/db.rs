use crate::*;

pub trait EtherealSignatureDb: salsa::DbWithJar<EtherealSignatureJar> + EtherealTermDb {}

impl<Db> EtherealSignatureDb for Db where Db : salsa::DbWithJar<EtherealSignatureJar> + EtherealTermDb {}