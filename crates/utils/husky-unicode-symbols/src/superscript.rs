const SUPERSCRIPTS: [char; 10] = ['⁰', '¹', '²', '³', '⁴', '⁵', '⁶', '⁷', '⁸', '⁹'];

pub fn superscript(n: u8) -> Option<char> {
    SUPERSCRIPTS.get(n as usize).copied()
}
