use std::any::type_name;
use brand::{Rng};
use brand::lcg::{Lcg, TinyLcg};

fn main() {
    let mut rng = Lcg::new(16231235);
    sample(&mut rng, 10);
    
    let mut rng = TinyLcg::new(16231235);
    sample(&mut rng, 10);
}

fn sample<R: Rng>(rng: &mut R, count: usize) {
    println!("=========== {} ===========", type_name::<R>().rsplit("::").next().unwrap().to_uppercase());
    for _ in 0..count {
        let value = rng.next_u64();
        println!("{value:20}  {value:016X}");
    }
}