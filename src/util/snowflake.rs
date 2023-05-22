use std::{fmt::Display, time::SystemTime};

use atomic::Atomic;
use bigdecimal::{Num, ToPrimitive, Zero};
use num_bigint::{BigInt, ToBigInt};
use serde::{Deserialize, Serialize};

const EPOCH: i64 = 1420070400000;
static WORKER_ID: u128 = 0;
static PROCESS_ID: u128 = 1;
lazy_static::lazy_static! {
    static ref INCREMENT: Atomic<u128> = Atomic::default();
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Snowflake(String);

impl Display for Snowflake {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Snowflake {
    pub fn to_binary(&self) -> String {
        let self_len = self.0.len();
        let high = self.0[..self_len - 10].parse::<u64>().unwrap_or(0);
        let low = self.0[self_len - 10..].parse::<u64>().unwrap();
        let mut low = low;
        let mut high = high;
        let mut bin = Vec::with_capacity(64);

        while low > 0 || high > 0 {
            bin.push((low & 1) as u8);
            low >>= 1;

            if high > 0 {
                low += 5_000_000_000 * (high % 2);
                high >>= 1;
            }
        }

        bin.iter()
            .rev()
            .map(|b| char::from_digit(*b as u32, 10).unwrap())
            .collect()
    }

    pub fn from_binary(num: &str) -> String {
        let mut num = BigInt::from_str_radix(num, 2).unwrap();
        let mut dec = Vec::with_capacity(18);

        let ten = 10.to_bigint().unwrap();
        let two = 2.to_bigint().unwrap();
        let thirty_two = 32.to_bigint().unwrap();

        while num.bits() > 50 {
            let high: BigInt = &num >> 32;
            let low: BigInt = (high.clone() % &ten) << 32 | &num & BigInt::from((1u64 << 32) - 1);

            let next: BigInt = (low.clone() % &ten);
            dec.push(next.to_u8().unwrap());
            num = (high / &ten) << 32 | low / &ten;
        }

        while !num.is_zero() {
            dec.push((num.clone() % &ten).to_u8().unwrap());
            num /= &ten;
        }

        dec.iter()
            .rev()
            .map(|d| char::from_digit(*d as u32, 10).unwrap())
            .collect()
    }

    pub fn generate_worker_process() -> u128 {
        let time = (chrono::Utc::now().naive_utc().timestamp_millis() - EPOCH) << 22;
        let worker = WORKER_ID << 17;
        let process = PROCESS_ID << 12;
        let increment = INCREMENT.load(atomic::Ordering::Relaxed);

        INCREMENT.store(increment + 1, atomic::Ordering::Relaxed);

        time as u128 | worker | process | increment
    }

    pub fn generate() -> Self {
        Self(Self::generate_worker_process().to_string())
    }

    pub fn deconstruct(&self) -> DeconstructedSnowflake {
        let binary = format!("{:0>64}", self.to_binary());

        let ts = i64::from_str_radix(&binary[0..42], 2).unwrap() + EPOCH;
        let wid = u64::from_str_radix(&binary[42..47], 2).unwrap();
        let pid = u64::from_str_radix(&binary[47..52], 2).unwrap();
        let increment = BigInt::from_str_radix(&binary[52..64], 2).unwrap();

        DeconstructedSnowflake {
            timestamp: ts,
            worker_id: wid,
            process_id: pid,
            increment: increment,
            binary: binary,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeconstructedSnowflake {
    pub timestamp: i64,
    pub worker_id: u64,
    pub process_id: u64,
    pub increment: BigInt,
    pub binary: String,
}

#[cfg(test)]
mod test {
    use crate::util::snowflake::Snowflake;

    #[test]
    fn test_new_snowflake() {
        let snow = Snowflake::generate();
        println!("{snow}");
    }

    #[test]
    fn snowflake_to_binary() {
        let snowflake = super::Snowflake("1104339392517902336".to_string());

        let bin = snowflake.to_binary();
        println!("{bin}");
    }

    #[test]
    fn binary_to_snowflake() {
        let snowflake = super::Snowflake::from_binary(
            "111101010011011001101101001110010010100000000001000000000000",
        );
        println!("{snowflake}");
    }

    #[test]
    fn test_deconstruct() {
        let new = super::Snowflake::generate();

        println!("{:?}", new.deconstruct());
    }
}
