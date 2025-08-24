pub fn add(a: u32, b: u32) -> u32 {
    a.saturating_add(b)
}

pub fn sub(a: u32, b: u32) -> u32 {
    if a < b {
        0
    } else {
        a - b
    }
}