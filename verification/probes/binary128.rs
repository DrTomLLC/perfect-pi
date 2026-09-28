#![no_std]
extern crate perfect_pi;

#[unsafe(no_mangle)]
pub fn perfect_pi_binary128_pi_bits() -> u128 {
    perfect_pi::PI_BINARY128.to_bits()
}

#[unsafe(no_mangle)]
pub fn perfect_pi_binary128_tau_bits() -> u128 {
    perfect_pi::TAU_BINARY128.to_bits()
}
