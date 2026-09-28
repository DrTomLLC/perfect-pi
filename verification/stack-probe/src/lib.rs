#![no_std]

#[inline(never)]
pub fn bounded40(output: &mut [u8; 42]) -> usize {
    let value = perfect_pi::Pi::<40>::round_nearest_even();
    match value.write_ascii(output) {
        Ok(written) => written,
        Err(_) => 0,
    }
}

#[inline(never)]
pub fn conversion40() -> u64 {
    perfect_pi::Pi::<40>::round_nearest_even()
        .to_f64_lossy()
        .to_bits()
}
