use perfect_pi::PI_F64;

fn main() {
    let bits = std::hint::black_box(PI_F64).to_bits();
    std::hint::black_box(bits);
}
