mod utils;

// generate 1 month of minute candles
use rand;
use std::time::{SystemTime, UNIX_EPOCH};

struct Candle {
    time_begin: u64,
    time_end: u64,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    volume: f64,
}

const MIN_PRICE_SOL: f64 = 0.1;
const MAX_PRICE_SOL: f64 = 100.;
const START_PRICE_SOL: f64 = 50.;

const MIN_VOLUME_SOL: f64 = 1.;
const MAX_VOLUME_SOL: f64 = 100.;

const MIN_FRAC_DELTA: f64 = 0.0001;
const MAX_FRAC_DELTA: f64 = 0.1000;

const MIN_WICK_FRAC: f64 = 0.1;
const MAX_WICK_FRAC: f64 = 0.5;

const SECOND_PER_MINUTE: usize = 60;
const MINUTES_PER_DAY: usize = 1440;
const DAYS_PER_MONTH: usize = 30;
const PROB_REVERSAL: f64 = 0.6;

// const DELTA_COEFFICIENT_SOL: f64 = 1.0;

const CANDLE_SIZE_SECONDS: u64 = 60;

//
pub fn generate_one_month_minute_candles(){

    // params
    let num_candles = MINUTES_PER_DAY * DAYS_PER_MONTH;
    let seconds_per_month = SECOND_PER_MINUTE * MINUTES_PER_DAY * DAYS_PER_MONTH;
    let current_unix_timestamp = utils::get_current_unix();

    // state
    let mut last_close = START_PRICE_SOL;
    let mut last_time_begin = current_unix_timestamp - seconds_per_month as u64;
    let mut is_up = true;
    let mut candles = Vec::new();

    for i in 0..num_candles {

        let frac_delta = utils::get_random_in_range(MIN_FRAC_DELTA, MAX_FRAC_DELTA);
        let frac_wick = utils::get_random_in_range(MIN_WICK_FRAC, MAX_WICK_FRAC);
        let delta = last_close * frac_delta;
        let wick = delta * frac_wick;


        let new_close;
        let high;
        let low;
        if is_up {
            new_close = last_close + delta;
            high = new_close + wick;
            low = last_close - wick;
        } else {
            new_close = last_close - delta;
            high = last_close + wick;
            low = new_close - wick;
        }

        if utils::get_random_in_range(0., 1.) < PROB_REVERSAL {
            is_up = !is_up;
        }

        if new_close > MAX_PRICE_SOL {
            is_up = false;
        }
        if new_close < MIN_PRICE_SOL {
            is_up = true;
        }


        let new_candle = Candle {
            time_begin: last_time_begin,
            time_end: last_time_begin + CANDLE_SIZE_SECONDS,
            open: last_close,
            high,
            low,
            close: new_close,
            volume: utils::get_random_in_range(MIN_VOLUME_SOL, MAX_VOLUME_SOL),
        };
        candles.push(new_candle);

        // update close
        last_time_begin += CANDLE_SIZE_SECONDS;
        last_close = new_close;
    }




}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // generate_one_month_minute_candles();
        let rval = utils::get_random_in_range(1., 10.);
        println!("got rval: {:?}", rval);

        // assert_eq!(result, 4);
    }
}
