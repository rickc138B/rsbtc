use uint::construct_uint;
use serde::{Serialize, Deserialize};
construct_uint! {
    // Construct an unsigned 256-bit integer
    // consisting of 4 x 64-bit words
    #[derive(Serialize, Deserialize)]
    pub struct U256(4);
}

// initial reward in bitcoin - multiply by 10^8 to get satoshis
pub const INITIAL_REWARD: u64 = 50;
// halving interval in blocks
pub const HALVING_INTERVAL: u64 = 210;
// ideal block time in seconds
pub const IDEAL_BLOCK_TIME: u64 = 10;
// minimum target
pub const MIN_TARGET: U256 = U256([
   0xFFFF_FFFF_FFFF_FFFF,
   0xFFFF_FFFF_FFFF_FFFF,
   0xFFFF_FFFF_FFFF_FFFF,
   0x0000_FFFF_FFFF_FFFF,
]);
// difficulty update interval in blocks
pub const DIFFICULTY_UPDATE_INTERVAL: u64 = 50;

pub mod sha256;
pub mod types;
pub mod util;
pub mod crypto;
pub mod error;






impl U256 {
    // Method to convert [u8; 32] into a U256
    pub fn from_bytes(bytes: [u8; 32]) -> Self {
        let mut words = [0u64; 4];
        for i in 0..4 {
            // Extract each 8-byte chunk, convert it to u64 (big-endian)
            words[3 - i] = u64::from_be_bytes(
                bytes[i * 8..(i + 1) * 8].try_into().unwrap()
            );
        }
        U256(words)
    }
    
}


// pub fn add(left: u64, right: u64) -> u64 {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
