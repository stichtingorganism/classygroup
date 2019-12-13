//! This module wraps `blake2b_rfc` into a convenient hashing interface (`GeneralHasher`) and
//! exports the `hash_to_prime` function. `hash_to_prime` is optimized to produce 256-bit primes.
use crate::uint::u256;
use rug::integer::Order;
use rug::Integer;
use std::hash::Hash;
use mohan::hash::{
  blake256,
  H256
};
pub mod primality;


/// Calls `hash` with Blake2b hasher.
pub fn hash(t: &[u8]) -> H256 {
  blake256(&t)
}

/// Hashes t with an incrementing counter (with blake2b) until a prime is found.
pub fn hash_to_prime(t: &[u8]) -> Integer {
  let mut counter = 0_u64;
  loop {
    let mut buf = Vec::new();
    buf.extend_from_slice(t);
    buf.extend_from_slice(&counter.to_le_bytes());

    let hash = hash(&buf);
    let mut hash = hash.to_bytes();
    // Make the candidate prime odd. This gives ~7% performance gain on a 2018 Macbook Pro.
    hash[0] |= 1;
    let candidate_prime = u256(hash);
    if primality::is_prob_prime(&candidate_prime) {
      return Integer::from(candidate_prime);
    }
    counter += 1;
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_blake2() {
    let data = b"martian cyborg gerbil attack";
    hash(data);
  }

  #[test]
  fn test_hash_to_prime() {
    let b_1 = b"boom i got ur boyfriend";
    let b_2 = b"boom i got ur boyfriene";
    assert_ne!(b_1, b_2);
    let h_1 = hash_to_prime(b_1);
    let h_2 = hash_to_prime(b_2);
    assert_ne!(h_1, h_2);
    let mut digits1 = [0; 4];
    h_1.write_digits(&mut digits1, Order::Lsf);
    assert!(primality::is_prob_prime(&u256(digits1)));
    let mut digits2 = [0; 4];
    h_2.write_digits(&mut digits2, Order::Lsf);
    assert!(primality::is_prob_prime(&u256(digits2)));
  }
}
