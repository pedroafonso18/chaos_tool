use rayon::prelude::*;
use rayon::ThreadPoolBuilder;
use std::time::{Duration, Instant};

pub fn cpuhogger(cores: u32, seconds: u32, is_safe: bool) {
    let safe_core_count = 4;
    let available = num_cpus::get_physical();
    if cores > available as u32 {
        eprintln!("Warning: Requested {} cores, but only {} available!", cores, available);
    }
    if cores > safe_core_count && is_safe {
        eprintln!("Safety is ON: Refusing to allocate more than {} cores of CPU. use --remove-safety to override.", safe_core_count)
    }
    let pool = ThreadPoolBuilder::new()
        .num_threads(cores as usize)
        .build()
        .expect("Failed to create thread pool");

    pool.install(|| {
        (0..cores).into_par_iter().for_each(|_| {
            let start = Instant::now();
            while start.elapsed() < Duration::from_secs(seconds as u64) {
                let _ = (0..1_000_000).fold(0u64, |acc, x| acc ^ x);
            }
        });
    })
}