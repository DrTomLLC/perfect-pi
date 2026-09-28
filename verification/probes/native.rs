#![no_std]
extern crate perfect_pi;

#[unsafe(no_mangle)]
pub fn perfect_pi_native_f32_bits() -> u32 {
    perfect_pi::PI_F32.to_bits()
}

#[unsafe(no_mangle)]
pub fn perfect_pi_native_f64_bits() -> u64 {
    perfect_pi::PI_F64.to_bits()
}
