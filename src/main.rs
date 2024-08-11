use rand::prelude::*;
use std::{
    sync::atomic::{self, AtomicU32},
    thread,
    time::Instant,
};

const NUM_THREADS: usize = 12;
const TOTAL_ROUNDS: usize = 1000_000_000;

fn thread_main(t: usize, thread_rounds: usize, best: &AtomicU32) {
    println!("Thread {t} with {thread_rounds} rounds");
    let mut rng = SmallRng::from_entropy();
    for gen in 0..thread_rounds {
        let mut hits = 0;
        for _ in 0..231 {
            hits += (rng.gen_range(0..4u8) == 0) as u32;
        }

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
        let mut rounds_left = TOTAL_ROUNDS;
        for t in 0..NUM_THREADS {
            let best = &best;

            let thread_rounds = rounds_left / (NUM_THREADS - t);
            rounds_left -= thread_rounds;
            s.spawn(move || thread_main(t, thread_rounds, best));
        }
    });

    println!("DONE! elapsed={:?}", start.elapsed());
}
