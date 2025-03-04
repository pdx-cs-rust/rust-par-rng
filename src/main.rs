use std::thread;

use clap::Parser;
extern crate parse_size;
use rand::{rngs::StdRng, Rng, SeedableRng};

#[derive(Debug, Parser)]
struct Args {
    #[arg(short, long, default_value="1GB", value_parser=|s: &str| parse_size::parse_size(s))]
    size: u64,
    #[arg(short, long, default_value="1")]
    threads: usize,
}

fn main() {
    let args = Args::parse();
    let size = args.size as usize;
    let mut rands = vec![0u8; size];

    thread::scope(|scope| {
        for rands in rands.chunks_mut(size / args.threads) {
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
