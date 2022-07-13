use xxhash_rust::const_xxh3::xxh3_64 as const_xxh3;
use xxhash_rust::xxh3::xxh3_64;

const TEST: u64 = const_xxh3(b"TEST");

fn test_input(text: &str) -> bool {
    match xxh3_64(text.as_bytes()) {
        TEST => true,
        _ => false,
    }
}

#[test]
fn it_works() {
    assert!(!test_input("tEST"));
    assert!(test_input("TEST"));
}
