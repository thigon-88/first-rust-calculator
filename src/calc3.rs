pub fn pow(a: u32, b: u32) -> u64 {
    a.pow(b).into()
}

pub fn log(a: u32, b: u32) -> f64 {
    let param = a as f64;
    let base = b as f64;
    param.log(base)
}