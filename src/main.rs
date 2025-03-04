use rand::{rngs::StdRng, Rng, SeedableRng};

const SIZE: usize = 1024*1024*1024;

fn main() {
    let mut seed_rng = StdRng::from_os_rng();
    let mut seed: Vec<u8> = Vec::with_capacity(SIZE);
    seed.resize(SIZE, 0);
    for v in &mut seed {
        *v = seed_rng.random();
    }
    println!("{}", seed.iter().fold(0, |a, v| v.wrapping_add(a)));
}
