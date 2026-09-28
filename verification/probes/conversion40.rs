#![no_std]
extern crate perfect_pi;

#[unsafe(no_mangle)]
pub fn perfect_pi_truncated_40_f32_bits() -> u32 {
    perfect_pi::Pi::<40>::truncated().to_f32_lossy().to_bits()
}

#[unsafe(no_mangle)]
pub fn perfect_pi_rounded_40_f32_bits() -> u32 {
    perfect_pi::Pi::<40>::round_nearest_even()
        .to_f32_lossy()
        .to_bits()
}

#[unsafe(no_mangle)]
pub fn perfect_pi_truncated_40_f64_bits() -> u64 {
    perfect_pi::Pi::<40>::truncated().to_f64_lossy().to_bits()
}

#[unsafe(no_mangle)]
pub fn perfect_pi_rounded_40_f64_bits() -> u64 {
    perfect_pi::Pi::<40>::round_nearest_even()
        .to_f64_lossy()
        .to_bits()
}
