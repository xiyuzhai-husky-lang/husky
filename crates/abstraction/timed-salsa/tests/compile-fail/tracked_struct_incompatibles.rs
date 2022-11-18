#[timed_salsa::jar(db = Db)]
struct Jar(
    TrackedWithRetRef,
    TrackedSructWithSpecify,
    TrackedStructWithNoEq,
    TrackedStructWithDb,
    TrackedStructWithRecover,
    TrackedStructWithLru,
);

trait Db: timed_salsa::DbWithJar<Jar> {}

#[timed_salsa::tracked(jar = Jar, return_ref)]
struct TrackedWithRetRef {
    field: u32,
}

#[timed_salsa::tracked(jar = Jar, specify)]
struct TrackedSructWithSpecify {
    field: u32,
}

#[timed_salsa::tracked(jar = Jar, no_eq)]
struct TrackedStructWithNoEq {
    field: u32,
}

#[timed_salsa::tracked(jar = Jar, db = Db)]
struct TrackedStructWithDb {
    field: u32,
}

#[timed_salsa::tracked(jar = Jar, recover_fn = recover)]
struct TrackedStructWithRecover {
    field: u32,
}

#[timed_salsa::tracked(jar = Jar, lru =12)]
struct TrackedStructWithLru {
    field: u32,
}
fn main() {}
