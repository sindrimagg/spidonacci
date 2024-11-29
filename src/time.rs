use std::time::Instant;
use num_bigint::BigUint;

pub fn time_fn(input: usize, f: fn(usize) -> BigUint) -> u128 {
    let now = Instant::now();
    let _res = f(input);
    let elapsed = now.elapsed().as_nanos();
    //let res_approx = format!("{}e{}", (bits as f64 / 3.32193).ceil() as usize);
    //(elapsed, res)
    elapsed
}
