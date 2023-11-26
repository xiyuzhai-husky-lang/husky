#[salsa::jar(db = Db)]
struct Jar(AccTwoUnnamedFields, AccNamedField);

// accumulator with more than one unnamed fields
#[salsa::accumulator(jar = Jar)]
struct AccTwoUnnamedFields(u32, u32);

// accumulator with named fields
#[salsa::accumulator(jar = Jar)]
struct AccNamedField {
    field: u32,
}

fn main() {}
