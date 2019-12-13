//
// Copyright (c) Stichting Organism. All Rights Reserved.
//

//! Discriminant from a seed
//https://eprint.iacr.org/2011/481.pdf

//Handle Precomputes
include!(concat!(env!("OUT_DIR"), "/constants.rs"));

use crate::num::Mpz;
use bacteria::Transcript;

/// Create a discriminant from a seed (a byte string) and a bit length (a
/// `u16`).  The discriminant is guaranteed to be a negative prime number that
/// fits in `length` bits, except with negligible probability (less than
/// 2^(-100)).  It is also guaranteed to equal 7 modulo 8.
///
/// This function uses Shake128 as an extensible output function to expand the seed.  
/// Therefore, different seeds will result in completely different discriminants with
/// overwhelming probability, unless `length` is very small.  However, this function is
/// deterministic: if it is called twice with identical seeds and lengths, it
/// will always return the same discriminant.
///
/// This function is guaranteed not to panic for any inputs whatsoever, unless
/// memory allocation fails and the allocator in use panics in that case.
pub fn create_discriminant(seed: &[u8], length: u64) -> Mpz {
    //1. Create a Merlin transcript
    let mut transcript = Transcript::new(b"Classygroup.create_discriminant");
    //2. Commit our seed
    transcript.append_message(b"seed", seed);
    //3. Commit seed length
    transcript.append_u64(b"length", length);

    // The number of “extra” bits (that don’t evenly fit in a byte)
    let extra = (length % 8) as u8;

    // The number of random bytes needed (the number of bytes that hold `length`
    // bits, plus 2).
    let random_bytes_len: u64 = {
        let t = length >> 3;
        if extra == 0 {
            t + 2
        } else {
            t + 3
        }
    };

    //println!("random_bytes_len2: {:?}", random_bytes_len);

    //get our random bytes sequence derived from seed
    let mut random_bytes = vec![0u8; random_bytes_len as usize];
    transcript.challenge_bytes(b"random_bytes", &mut random_bytes);

    // The number of random bytes needed (the number of bytes that hold `length`
    // bits, plus 2).
    let (n_tmp, last_2) = random_bytes.split_at(random_bytes_len as usize - 2);
    let numerator = (usize::from(last_2[0]) << 8) + usize::from(last_2[1]);

    //println!("random_bytes_len: {:?}", n_tmp);
    let mut n: Mpz = Mpz::from_bytes(n_tmp);
    //println!("random_bytes_len: {:?}", n.bit_length());

    // n -= n.clone() % M;
    //let rem = n.clone() % Mpz::from(M as u64);
    let mut rem = Mpz::zero();
    rem.modulo(&n, &Mpz::from(M as u64));
    //n = n - rem;
    n.sub_mut(&rem);
    //println!("n plus: {:?}", RESIDUES[numerator % RESIDUES.len()]);
    let residue = RESIDUES[numerator % RESIDUES.len()];
    let residue = Mpz::from(residue as u64);
    //n = n + residue;
    n.add_mut(&residue);

    debug_assert!(n >= Mpz::zero());

    // This generates the smallest prime ≥ n that is of the form n + m*x.
    loop {
        // Speed up prime-finding by quickly ruling out numbers
        // that are known to be composite.
        let mut sieve = ::bit_vec::BitVec::from_elem(1 << 16, false);

        //Optimize for gains
        for &(p, q) in SIEVE_INFO.iter() {
            // The reference implementation changes the sign of `n` before taking its
            // remainder. Instead, we leave `n` as positive, but use ceiling
            // division instead of floor division.  This is mathematically
            // equivalent and potentially faster.
            let mut i: usize = (n.crem_u16(p) as usize * q as usize) % p as usize;
            while i < sieve.len() {
                sieve.set(i, true);
                i += p as usize;
            }
        }

        for (i, x) in sieve.iter().enumerate() {
            let i = i as u32;

            if !x {
                //-(n + m*i)
                let q = u64::from(M) * u64::from(i);
                //n = n + q;
                n.add_ui_mut(q);

                //test if we found our target
                if n.is_prime(2) {
                    //set sign to negative
                    n.neg_mut();
                    return n;
                }

                //n = n - q;
                n.sub_ui_mut(q);
            }
        }

        // M is set to a number with many prime factors so the results are
        // more uniform https://eprint.iacr.org/2011/401.pdf
        //n = n + (u64::from(M) * (1 << 16)) as u64;
        n.add_ui_mut((u64::from(M) * (1 << 16)));
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // use crate::biggie::BigNum;
    use std::str::FromStr;

    // #[test]
    // fn check_rem() {
    //     let negh = BigUint::from(100u32);
    //     let modulo = BigUint::from(3u32);
    //     let res = ((modulo.clone() - BigUint::one()) * negh.clone()) % modulo.clone();

    //     assert_eq!(res, 2u32.into());
    // }

    // #[test]
    // fn check_discriminant_length() {
    //     assert_eq!(create_discriminant::<BigInt>(b"\xaa", 2048).get_bits(), 2048);

    // }

    #[test]
    fn check_discriminant_length_mpz() {
        //assert_eq!(create_discriminant(b"\xaa", 512).bit_length(), 512);
        assert_eq!(create_discriminant(b"\xaa", 1024).bit_length(), 1024);
        assert_eq!(create_discriminant(b"\xaa", 2048).bit_length(), 2048);
    }

    // #[test]
    // fn check_discriminant_bytes_mpz_big() {
    //     let a = create_discriminant::<BigInt>(b"\xaa", 2048).to_bytes();
    //     let b = create_discriminant::<Mpz>(b"\xaa", 2048).to_bytes();
    //     assert_eq!(a, b);
    // }

    // #[test]
    // fn check_discriminant_bytes_mpz_big_y() {
    //     let negh = BigInt::from_str("-100").unwrap();
    //     let modulo = Mpz::from_str("-100").unwrap();

    //     assert_eq!(negh.to_bytes(), modulo.to_bytes());
    // }

    // #[test]
    // fn check_discriminant_seed_cross() {
    //     let seed = "03/Jan/2009 Chancellor on brink of second bailout for banks,".to_string();

    //     let a = create_discriminant::<BigInt>(seed.as_bytes(), 2048).to_bytes();
    //     let b = create_discriminant::<Mpz>(seed.as_bytes(), 2048).to_bytes();
    //     assert_eq!(a, b);
    // }
}
