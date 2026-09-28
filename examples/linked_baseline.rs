fn main() {
    let mut output = [0_u8; 42];
    std::hint::black_box(&mut output);
    std::hint::black_box(output);
}
