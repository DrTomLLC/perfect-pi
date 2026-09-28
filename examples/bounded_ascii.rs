use perfect_pi::Pi;

fn main() {
    let value = Pi::<12>::round_nearest_even();
    let mut output = [0_u8; 14];
    match value.write_ascii(&mut output) {
        Ok(written) => print_output(&output, written),
        Err(error) => eprintln!("output failed: {error}"),
    }
}

fn print_output(output: &[u8], written: usize) {
    let Some(bytes) = output.get(..written) else {
        eprintln!("invalid bounded output length");
        return;
    };
    match core::str::from_utf8(bytes) {
        Ok(text) => println!("bounded_pi={text}"),
        Err(error) => eprintln!("unexpected UTF-8 error: {error}"),
    }
}
