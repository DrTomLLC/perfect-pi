#![no_std]
extern crate perfect_pi;

#[unsafe(no_mangle)]
pub fn perfect_pi_d3_truncated_f32_bits() -> u32 {
    perfect_pi::Pi::<3>::truncated().to_f32_lossy().to_bits()
}

#[unsafe(no_mangle)]
pub fn perfect_pi_d3_rounded_f32_bits() -> u32 {
    perfect_pi::Pi::<3>::round_nearest_even()
        .to_f32_lossy()
        .to_bits()
}

#[unsafe(no_mangle)]
pub fn perfect_pi_d14_truncated_f64_bits() -> u64 {
    perfect_pi::Pi::<14>::truncated().to_f64_lossy().to_bits()
}

#[unsafe(no_mangle)]
pub fn perfect_pi_d14_rounded_f64_bits() -> u64 {
    perfect_pi::Pi::<14>::round_nearest_even()
        .to_f64_lossy()
        .to_bits()
}
