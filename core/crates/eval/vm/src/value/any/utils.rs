pub(super) fn print_sequence<A>(
    start: &str,
    items: impl Iterator<Item = A>,
    f: &impl Fn(A) -> String,
    end: &str,
    max_len: usize,
) -> String {
    let mut result = start.to_string();
    for (i, item) in items.enumerate() {
        if result.len() > max_len {
            result.push_str(", ...");
            result.push_str(end);
            return result;
        }
        if i > 0 {
            result.push_str(", ");
        }
        result.push_str(&f(item))
    }
    result.push_str(end);
    result
}
