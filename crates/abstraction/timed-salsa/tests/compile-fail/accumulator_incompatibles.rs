#[timed_salsa::jar(db = Db)]
struct Jar(
    AccWithRetRef,
    AccWithSpecify,
    AccWithNoEq,
    AccWithData,
    AcWithcDb,
    AccWithRecover,
    AccWithLru,
    AccWithConstructor,
);

trait Db: timed_salsa::DbWithJar<Jar> {}

#[timed_salsa::accumulator(jar = Jar, return_ref)]
struct AccWithRetRef(u32);

#[timed_salsa::accumulator(jar = Jar, specify)]
struct AccWithSpecify(u32);

#[timed_salsa::accumulator(jar = Jar, no_eq)]
struct AccWithNoEq(u32);

#[timed_salsa::accumulator(jar = Jar, data = MyAcc)]
struct AccWithData(u32);

#[timed_salsa::accumulator(jar = Jar, db = Db)]
struct AcWithcDb(u32);

#[timed_salsa::accumulator(jar = Jar, recover_fn = recover)]
struct AccWithRecover(u32);

#[timed_salsa::accumulator(jar = Jar, lru =12)]
struct AccWithLru(u32);

#[timed_salsa::accumulator(jar = Jar, constructor = Constructor)]
struct AccWithConstructor(u32);

fn main() {}
