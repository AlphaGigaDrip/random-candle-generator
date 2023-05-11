mod utils;
#[macro_use]
extern crate serde_derive;
use std::io::{Read, Write};
use serde::{Serialize, Deserialize};
use rmp_serde;
use rmp_serde::Serializer;

#[derive(Debug, Serialize, Deserialize)]
pub struct Candle {
    pub time_begin: u64,
    pub time_end: u64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
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
pub fn generate_one_month_minute_candles() -> Vec<Candle> {

    // params
    let num_candles = MINUTES_PER_DAY * DAYS_PER_MONTH;
    let seconds_per_month = SECOND_PER_MINUTE * MINUTES_PER_DAY * DAYS_PER_MONTH;
    let current_unix_timestamp = utils::get_current_unix();

    // state
    let mut last_close = START_PRICE_SOL;
    let mut last_time_begin = current_unix_timestamp - seconds_per_month as u64;
    let mut is_up = true;
    let mut candles = Vec::new();

    for _ in 0..num_candles {

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

        // gen and push new candle
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

        // update close and time
        last_time_begin += CANDLE_SIZE_SECONDS;
        last_close = new_close;
    }

    candles
}

pub fn serialize_and_save() {
    let candles = generate_one_month_minute_candles();
    let mut buf = Vec::new();
    candles.serialize(&mut Serializer::new(&mut buf)).unwrap();
    println!("serialized into buf of len: {:?}", buf.len());

    let path = std::path::Path::new("./tmp/candles.mp");
    let mut file = std::fs::File::create(path).expect("Failed to create file");
    let result= file.write(buf.as_slice());
    println!("got file write result: {:?}", result);

}

pub fn load_and_deserialize() {
    let path = std::path::Path::new("./tmp/candles.mp");
    let buf = std::fs::read(path).expect("failed to read file");
    println!("read buf with buf len: {:?}", buf.len());

    let candles: Vec<Candle> = rmp_serde::decode::from_slice(buf.as_slice()).unwrap();
    println!("deserialized candles vec of len: {:?}", candles.len());

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // let candles = generate_one_month_minute_candles();
        // println!("len candles: {:?}", candles.len());
        // for i in 0..2 {
        //     println!("{}: {:?}", i, candles[i]);
        // }
        // serialize_and_save();
        load_and_deserialize();
        println!("something.... yay?");

        // assert_eq!(result, 4);
    }
}
