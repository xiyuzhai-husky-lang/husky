use super::*;

#[salsa::tracked(db = TracePathDb, jar = TracePathJar)]
pub struct ValItemTracePath {
    val_path: FugitivePath,
}
