use std::thread::scope;
use std::time::Instant;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use tikv_jemallocator::Jemalloc;

#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

const SAMPLES: u64 = 2u64.pow(24);
const STRING_SIZE: usize = 64;
const TOTAL_THREAD_COUNT: usize = 12;

fn main() {
    let base: String = std::iter::repeat('A').take(STRING_SIZE).collect();

    println!("Running on {TOTAL_THREAD_COUNT} threads concurrently.");
    println!("Using strings of size {STRING_SIZE}.");
    println!("Running {SAMPLES} samples.");

    let arc_base: Arc<str> = base.as_str().into();
    bench_clone(arc_base);
    let string_base = base.clone();
    bench_clone(string_base);
}

fn bench_clone<T: Clone + Send>(base: T) {
    let elapsed = scope(|s| {
        let running = Arc::new(AtomicBool::new(true));

        for _ in 1..TOTAL_THREAD_COUNT {
            let local_base = base.clone();
            let local_running = running.clone();

            s.spawn(move || while local_running.load(Ordering::Relaxed) {
                let _ = std::hint::black_box(local_base.clone());
            });
        }

        let start = Instant::now();

        for _ in 0..SAMPLES {
            let _ = std::hint::black_box(base.clone());
        }

        let elapsed = start.elapsed();

        running.store(false, Ordering::Relaxed);

        elapsed
    });

    println!("Took {}ms to perform {} clone operations on {}.", elapsed.as_millis(), SAMPLES, std::any::type_name::<T>())
}
