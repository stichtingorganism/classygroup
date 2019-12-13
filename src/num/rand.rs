//! Mpz randomness functions, ported from Flint

use gmp_mpfr_sys::gmp::{
    // limb_t,
    randinit_default,
    // LIMB_BITS,
    // mpz_rrandomb,
    // mpz_urandomm
    randstate_t,
};

/// Holds the gmp randomness
pub struct RandState {
    pub gmp: randstate_t,
    pub randval: BigDigit,
    pub randval2: BigDigit,
}

#[cfg(target_pointer_width = "32")]
pub type BigDigit = u32;
#[cfg(target_pointer_width = "64")]
pub type BigDigit = u64;

#[cfg(target_pointer_width = "32")]
pub type CoeffMax = i32;
#[cfg(target_pointer_width = "64")]
pub type CoeffMax = i64;

// #[cfg(target_pointer_width = "64")]
// fn to_bigdigit(i: &Mpz) -> BigDigit {
//     i.to_u64().unwrap()
// }

// #[cfg(target_pointer_width = "32")]
// fn to_bigdigit(i: &Mpz) -> BigDigit {
//     i.to_u32().unwrap()
// }

/// initilze the state object for the random generator functions
#[cfg(target_pointer_width = "32")]
pub fn randinit() -> RandState {
    unsafe {
        let mut mpz = std::mem::uninitialized();
        randinit_default(&mut mpz);
        RandState {
            gmp: mpz,
            randval: u32::from(4187301858u32),
            randval2: u32::from(3721271368u32),
        }
    }
}

#[cfg(target_pointer_width = "64")]
pub fn randinit() -> RandState {
    unsafe {
        let mut mpz = std::mem::uninitialized();
        randinit_default(&mut mpz);
        RandState {
            gmp: mpz,
            randval: u64::from(13845646450878251009u64),
            randval2: u64::from(13142370077570254774u64),
        }
    }
}

// #[cfg(target_pointer_width = "64")]
// pub fn n_randlimb(state: &mut RandState) -> limb_t {

//     state.randval = (state.randval.overflowing_mul(13282407956253574709)).0 + BigDigit::from(286824421u32);
//     state.randval2 = (state.randval.overflowing_mul(7557322358563246341)).0 + BigDigit::from(286824421u32);

//     return (state.randval >> 32) + ((state.randval2 >> 32) << 32);
// }

// #[cfg(target_pointer_width = "32")]
// pub fn n_randlimb(state: &mut RandState) -> limb_t {

//     state.randval = (state.randval* BigDigit::from(1543932465u32) +  BigDigit::from(1626832771u32));
//     state.randval2 = (state.randval2* BigDigit::from(2495927737u32) +  BigDigit::from(1626832771u32));

//     return (state.randval >> 16) + ((state.randval2 >> 16) << 16);
// }

// //
// pub fn n_randint(state: &mut RandState, limit: BigDigit) -> limb_t {
//     if limit == 0 as BigDigit {
//        return n_randlimb(state);
//     } else {
//        return n_randlimb(state) % limit;
//     }
// }

// //
// fn n_randbits(state: &mut RandState, bits: BigDigit) -> limb_t {
//    if bits == 0 {
//        return BigDigit::from(0u32);
//    } else {
//        return (BigDigit::from(1u32) << (bits - 1)) | n_randint(state, BigDigit::from(1u32) << bits);
//    }
// }

// //fmpz_randm
// pub fn randm(f: &mut Mpz, state: &mut RandState, m: &Mpz) {

//     let bits = m.bit_length();
//     let sgn = m.sign();

//     if bits <= (LIMB_BITS - 2) as usize {
//         if sgn == Sign::Plus {
//            *f = Mpz::from(n_randint(state, to_bigdigit(&m)));
//         } else {
//            *f = -Mpz::from(n_randint(state, to_bigdigit(&-m)));
//         }
//     } else {
//         unsafe { mpz_urandomm(&mut f.inner, &mut state.gmp, &m.inner); }

//         if sgn == Sign::Minus {
//             f.neg_mut();
//         }
//     }
// }

// fn n_randtest_bits(state: &mut RandState, bits: BigDigit) -> BigDigit {
//     let mut n;

//     let mut m = n_randlimb(state);

//     if m & BigDigit::from(7u32) == 1 {
//         n = n_randbits(state, bits);
//     } else {
//         m >>= 3;

//         match m & BigDigit::from(7u32) {
//             0 =>  { n = 0; }
//             1 =>  { n = 1; }
//             2 =>  { n = CoeffMax::max_value() as BigDigit; }
//             3 =>  { n = i64::max_value() as BigDigit; }
//             4 =>  { n = u64::max_value(); }
//             5 =>  {
//                 n = (BigDigit::from(1u32) << n_randint(state, LIMB_BITS as BigDigit)) - (BigDigit::from(1u32) << n_randint(state, LIMB_BITS as BigDigit));
//             }
//             6 => {
//                 n = BigDigit::from(1u32) << n_randint(state, LIMB_BITS as BigDigit);
//             }
//             7 => {
//                 //BigDigit::from(-(BigDigit::from(1) << n_randint(state, LIMB_BITS as BigDigit)));
//                 n = BigDigit::from(1u32) << n_randint(state, LIMB_BITS as BigDigit);
//             }

//             _ => { n = 0; }

//         }

//         // mask it off
//         if bits < LIMB_BITS as BigDigit {
//              n &= (BigDigit::from(1u32)<<bits) - BigDigit::from(1u32);
//         }

//         /* set most significant bit */
//         if bits != 0 {
//             n |= BigDigit::from(1u32) << (bits - 1);
//         } else {
//             n = 0;
//         }

//     }

//     return n;
// }

// pub fn randtest_unsigned(f: &mut Mpz, state: &mut RandState, bits: BigDigit) {

//     let mut m = n_randlimb(state);
//     let bits = n_randint(state, bits + 1);

//     if bits <= (LIMB_BITS - 2) as BigDigit {

//         if BigDigit::from(m) & BigDigit::from(3u32) == 1 {

//             *f = Mpz::from(n_randtest_bits(state, bits));

//         } else {

//             m >>= 2;

//             if bits == 0 {

//                 *f = Mpz::default();
//             } else if bits < (LIMB_BITS - 2) as BigDigit {

//                 *f = Mpz::from(m & BigDigit::from(1u32));
//             } else {
//                 *f = Mpz::from(CoeffMax::max_value());
//             }
//         }
//     } else {
//         unsafe { mpz_rrandomb(&mut f.inner, &mut state.gmp, bits); }
//     }
// }
