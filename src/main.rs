use std::thread::scope;
use std::time::Instant;
use std::sync::{Arc, OnceLock};
use std::sync::atomic::{AtomicBool, Ordering};

use tikv_jemallocator::Jemalloc;

#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

const SAMPLES: u64 = 2u64.pow(24);
const STRING_SIZE: usize = 64;
const TOTAL_THREAD_COUNT: usize = 12;

const DROP: bool = false;

const JSON: bool = true;

fn samples() -> u64 {
    SAMPLES
}

fn string_size() -> usize {
    STRING_SIZE
}

fn total_thread_count() -> usize {
    static ONCE: OnceLock<usize> = OnceLock::new();

    *ONCE.get_or_init(|| {
        if let Some(v) = std::env::var("TOTAL_THREAD_COUNT").ok().and_then(|s| s.parse().ok()) {
            v
        } else {
            if !JSON { println!("Using fallback value for TOTAL_THREAD_COUNT: {TOTAL_THREAD_COUNT}")};
            TOTAL_THREAD_COUNT
        }
    })
}

fn should_drop() -> bool {
    static ONCE: OnceLock<bool> = OnceLock::new();

    *ONCE.get_or_init(|| {
        match std::env::var("SHOULD_DROP").ok() {
            Some(v) if v == "yes" => true,
            Some(v) if v == "no" => false,
            _ => {
                if !JSON { println!("Using fallback value for SHOULD_DROP: {DROP}") };
                DROP
            }
        }
    })
}

fn main() {
    let base: String = std::iter::repeat('A').take(string_size()).collect();

    if !JSON {
        println!("Running on {} threads concurrently.", total_thread_count());
        println!("Using strings of size {}.", string_size());
        println!("Running {} samples.", samples());

        println!("{} cloned elements.", if DROP { "Dropping" } else { "Not dropping" });
    }

    let arc_base: Arc<str> = base.as_str().into();
    bench_clone(arc_base);
    let string_base = base.clone();
    bench_clone(string_base);
}

fn bench_clone<T: Clone + Send>(base: T) {
    let elapsed = scope(|s| {
        let running = Arc::new(AtomicBool::new(true));

        for _ in 1..total_thread_count() {
            let local_base = base.clone();
            let local_running = running.clone();

            s.spawn(move || while local_running.load(Ordering::Relaxed) {
                run_clone(&local_base)
            });
        }

        let start = Instant::now();

        for _ in 0..samples() {
            run_clone(&base);
        }

        let elapsed = start.elapsed();

        running.store(false, Ordering::Relaxed);

        elapsed
    });

    if JSON {
        println!(r#"{{"type": "{}", "threads": {}, "drop": {}, "time_millis": {}}}"#, std::any::type_name::<T>(), total_thread_count(), should_drop(), elapsed.as_millis());
    } else {
        println!("Took {}ms to perform {} clone operations on {}.", elapsed.as_millis(), samples(), std::any::type_name::<T>())
    }
}

fn run_clone<T: Clone>(base: &T) {
    let c = base.clone();

    if !should_drop() {
        std::mem::forget(c);
    }
}
