use md5::compute;
use std::sync::{
    Arc,
    atomic::{AtomicBool, AtomicU32, Ordering},
};

const THREADS: usize = 8;

pub fn part_2(input: &str) -> u32 {
    let input_bytes = input.trim().as_bytes();
    let found = Arc::new(AtomicBool::new(false));
    let result = Arc::new(AtomicU32::new(0));

    crossbeam::scope(|s| {
        for t in 0..THREADS {
            let found = found.clone();
            let result = result.clone();
            s.spawn(move |_| {
                let mut buffer = [0u8; 32];
                let mut num_buf = itoa::Buffer::new();

                let mut i = t as u32;
                while !found.load(Ordering::Relaxed) {
                    let num_bytes = num_buf.format(i).as_bytes();
                    let total_len = input_bytes.len() + num_bytes.len();

                    buffer[..input_bytes.len()].copy_from_slice(input_bytes);
                    buffer[input_bytes.len()..total_len].copy_from_slice(num_bytes);

                    let hash = compute(&buffer[..total_len]).0;
                    if hash[0] == 0 && hash[1] == 0 && hash[2] == 0 {
                        found.store(true, Ordering::Relaxed);
                        result.store(i, Ordering::Relaxed);
                        break;
                    }

                    i += THREADS as u32;
                }
            });
        }
    })
    .unwrap();

    result.load(Ordering::Relaxed)
}
