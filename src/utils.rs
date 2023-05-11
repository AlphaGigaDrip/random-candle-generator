use std::time::{SystemTime, UNIX_EPOCH};
use rand;

pub fn get_current_unix() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
}

pub fn get_random_in_range(min: f64, max: f64) -> f64 {
    let range = max - min;
    let rval: f64 = rand::random::<f64>() * range;
    rval + min
}