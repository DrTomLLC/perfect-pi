#![no_std]
extern crate perfect_pi;

#[unsafe(no_mangle)]
pub fn perfect_pi_truncated_40() -> perfect_pi::DecimalPi<40> {
    perfect_pi::Pi::<40>::truncated()
}

#[unsafe(no_mangle)]
pub fn perfect_pi_rounded_40() -> perfect_pi::DecimalPi<40> {
    perfect_pi::Pi::<40>::round_nearest_even()
}
