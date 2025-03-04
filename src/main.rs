use std::thread;

use rand::{rngs::StdRng, Rng, SeedableRng};

const SIZE: usize = 1024*1024*1024;
const THREADS: usize = 2;

fn main() {
    let mut rands = vec![0u8; SIZE];

    thread::scope(|scope| {
        for rands in rands.chunks_mut(SIZE / THREADS) {
            scope.spawn(|| {
                let mut seed_rng = StdRng::from_os_rng();
                for v in rands {
                    *v = seed_rng.random();
                }
            });
        }
    });

    println!("{}", rands.iter().fold(0, |a, v| v.wrapping_add(a)));
}
