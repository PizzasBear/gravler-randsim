use rand::prelude::*;
use std::{
    sync::atomic::{self, AtomicU32},
    thread,
    time::Instant,
};

const TOTAL_ROUNDS: usize = 1000_000_000;

/// Takes 64-bit random number representing 32 numbers between 0 and 3 (inclusive)
/// and returns the number of 3s in the sequence.
fn count_hits(rand64: u64) -> u32 {
    // 8 bit example:
    //
    //   00011011  (0, 1, 2, 3 represented as 2 bits each)
    //    00011011 (number shifted right)
    // & 01010101  (0x55)
    //   --------
    //   00000001  -> 1 hit

    (rand64 >> 1 & rand64 & 0x5555_5555_5555_5555).count_ones()
}

fn thread_main(t: usize, thread_rounds: usize, best: &AtomicU32) {
    println!("Thread {t} with {thread_rounds} rounds");
    let mut rng = SmallRng::from_entropy();
    for gen in 0..thread_rounds {
        // 64 * 8 / 2 = 256 numbers
        // 256 - 231 = 25 numbers to remove

        // sets the first 25 random numbers to 0, and counts only the last 7 numbers
        let mut hits = count_hits(rng.gen::<u64>() >> 2 * 25);
        // counts 32 * 7 = 224 numbers
        hits += (0..7).map(|_| count_hits(rng.gen())).sum::<u32>();

        let best = best.fetch_max(hits, atomic::Ordering::SeqCst);
        if best == hits {
            println!("{t},{gen}: best={best}");
        }
    }
    println!("Thread {t} done");
}

fn main() {
    let start = Instant::now();

    let best = AtomicU32::new(0);
    thread::scope(|s| {
        let num_threads = num_cpus::get_physical();
        let mut rounds_left = TOTAL_ROUNDS;
        for t in 0..num_threads {
            let best = &best;

            let thread_rounds = rounds_left / (num_threads - t);
            rounds_left -= thread_rounds;
            s.spawn(move || thread_main(t, thread_rounds, best));
        }
        assert_eq!(rounds_left, 0);
    });

    println!("DONE! elapsed={:?}", start.elapsed());
}
