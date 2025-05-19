use std::{thread, time::Duration};

pub fn memhogger(megabytes: u32, seconds: u32, is_safe: bool) -> Result<(), String> {
    let max_safe_mb = 4096;
    if !is_safe && megabytes > max_safe_mb {
        return Err(format!(
            "Safety is ON: Refusing to allocate more than {} MB. Use --remove-safety to override.",
            max_safe_mb
        ));
    }

    let size = megabytes as usize * 1024 * 1024;
    println!("Allocating {} MB of RAM for {} seconds...", megabytes, seconds);

    let mut hog = match std::panic::catch_unwind(|| vec![0u8; size]) {
        Ok(v) => v,
        Err(_) => return Err("Failed to allocate requested memory (out of memory)".to_string()),
    };
    for byte in hog.iter_mut() {
        *byte = 0xAA;
    }

    thread::sleep(Duration::from_secs(seconds as u64));
    println!("Done hogging memory.");
    Ok(())
}