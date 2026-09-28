#![no_std]
extern crate perfect_pi;

#[unsafe(no_mangle)]
pub fn perfect_pi_binary16_pi_bits() -> u16 {
    perfect_pi::PI_BINARY16.to_bits()
}

#[unsafe(no_mangle)]
pub fn perfect_pi_binary16_tau_bits() -> u16 {
    perfect_pi::TAU_BINARY16.to_bits()
}
