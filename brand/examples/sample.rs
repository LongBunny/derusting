use std::any::type_name;
use brand::{Rng, Lcg};

fn main() {
    let mut rng = Lcg::new(16231235);
    sample(&mut rng, 10);
}

fn sample<R: Rng>(rng: &mut R, count: usize) {
    println!("=========== {} ===========", type_name::<R>().rsplit("::").next().unwrap().to_uppercase());
    for _ in 0..count {
        println!("{}\t", rng.next_u64());
    }
}