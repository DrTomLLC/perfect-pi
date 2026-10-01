use perfect_pi::{PI_F64, Pi};
use std::hint::black_box;
use std::time::Instant;

fn measure<T>(label: &str, iterations: u64, mut operation: impl FnMut() -> T) {
    let start = Instant::now();
    for _ in 0..iterations {
        black_box(operation());
    }
    let elapsed = start.elapsed();
    let total_ns = elapsed.as_nanos();
    let ns_per_op = total_ns as f64 / iterations as f64;
    println!("{label},{iterations},{total_ns},{ns_per_op:.3}");
}

fn main() {
    println!("operation,iterations,total_ns,ns_per_op");
    measure("native_f64_bits", 20_000_000, || {
        black_box(PI_F64).to_bits()
    });
    measure("bounded_round_40", 5_000_000, Pi::<40>::round_nearest_even);
    measure("bounded_to_f64_lossy", 5_000_000, || {
        Pi::<40>::round_nearest_even().to_f64_lossy()
    });

    #[cfg(feature = "runtime-generation")]
    {
        benchmark_runtime(100, 100);
        benchmark_runtime(1_000, 10);
        benchmark_runtime(10_000, 5);
        benchmark_runtime(100_000, 1);
    }
}

#[cfg(feature = "runtime-generation")]
fn benchmark_runtime(decimal_places: usize, iterations: u64) {
    use perfect_pi::{generate_pi_ascii, runtime_pi_ascii_len};

    let required = match runtime_pi_ascii_len(decimal_places) {
        Ok(required) => required,
        Err(error) => {
            eprintln!("benchmark setup failed: {error}");
            return;
        }
    };
    let mut output = vec![0_u8; required];
    measure(
        &format!("runtime_generate_{decimal_places}"),
        iterations,
        || generate_pi_ascii(decimal_places, black_box(&mut output)),
    );
}
