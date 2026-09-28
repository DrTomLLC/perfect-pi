#![feature(f16)]
#![feature(f128)]

fn main() {
    assert_eq!(core::f16::consts::PI.to_bits(), 0x4248);
    assert_eq!(core::f16::consts::TAU.to_bits(), 0x4648);
    assert_eq!(core::f16::consts::FRAC_PI_2.to_bits(), 0x3e48);
    assert_eq!(core::f16::consts::FRAC_PI_3.to_bits(), 0x3c30);
    assert_eq!(core::f16::consts::FRAC_PI_4.to_bits(), 0x3a48);
    assert_eq!(core::f16::consts::FRAC_PI_6.to_bits(), 0x3830);
    assert_eq!(core::f16::consts::FRAC_PI_8.to_bits(), 0x3648);
    assert_eq!(core::f16::consts::FRAC_1_PI.to_bits(), 0x3518);
    assert_eq!(core::f16::consts::FRAC_2_PI.to_bits(), 0x3918);
    assert_eq!(core::f16::consts::FRAC_2_SQRT_PI.to_bits(), 0x3c83);

    assert_eq!(
        core::f128::consts::PI.to_bits(),
        0x4000_921f_b544_42d1_8469_898c_c517_01b8
    );
    assert_eq!(
        core::f128::consts::TAU.to_bits(),
        0x4001_921f_b544_42d1_8469_898c_c517_01b8
    );
    assert_eq!(
        core::f128::consts::FRAC_PI_2.to_bits(),
        0x3fff_921f_b544_42d1_8469_898c_c517_01b8
    );
    assert_eq!(
        core::f128::consts::FRAC_PI_3.to_bits(),
        0x3fff_0c15_2382_d736_5846_5bb3_2e0f_567b
    );
    assert_eq!(
        core::f128::consts::FRAC_PI_4.to_bits(),
        0x3ffe_921f_b544_42d1_8469_898c_c517_01b8
    );
    assert_eq!(
        core::f128::consts::FRAC_PI_6.to_bits(),
        0x3ffe_0c15_2382_d736_5846_5bb3_2e0f_567b
    );
    assert_eq!(
        core::f128::consts::FRAC_PI_8.to_bits(),
        0x3ffd_921f_b544_42d1_8469_898c_c517_01b8
    );
    assert_eq!(
        core::f128::consts::FRAC_1_PI.to_bits(),
        0x3ffd_45f3_06dc_9c88_2a53_f84e_afa3_ea6a
    );
    assert_eq!(
        core::f128::consts::FRAC_2_PI.to_bits(),
        0x3ffe_45f3_06dc_9c88_2a53_f84e_afa3_ea6a
    );
    assert_eq!(
        core::f128::consts::FRAC_2_SQRT_PI.to_bits(),
        0x3fff_20dd_7504_29b6_d11a_e3a9_14fe_d7fe
    );

    println!("PASS: nightly native f16/f128 π-family constants match Perfectπ IEEE adapters");
}
