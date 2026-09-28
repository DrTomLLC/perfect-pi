use perfect_pi::Pi;

fn main() {
    let value = std::hint::black_box(Pi::<40>::round_nearest_even());
    let mut output = [0_u8; 42];
    let result = value.write_ascii(std::hint::black_box(&mut output));
    let _ = std::hint::black_box(result);
    std::hint::black_box(output);
}
