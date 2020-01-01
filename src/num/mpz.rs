//! Mpz wrappers.

use gmp_mpfr_sys::gmp::{self, limb_t, mpz_t};
use std::cmp::Ordering;
use std::ffi::CString;
use std::hash::{Hash, Hasher};
use std::mem::uninitialized;
use std::os::raw::{c_int, c_ulong};
use std::slice;
use std::str::FromStr;
use std::ffi::c_void;

#[derive(Debug)]
#[cfg_attr(repr_transparent, repr(transparent))]
pub struct Mpz {
    pub inner: mpz_t,
}

unsafe impl Send for Mpz {}
unsafe impl Sync for Mpz {}
impl Eq for Mpz {}

impl Default for Mpz {
    fn default() -> Self {
        let inner = unsafe {
            let mut ret = uninitialized();
            gmp::mpz_init(&mut ret);
            ret
        };
        Self { inner }
    }
}

impl Clone for Mpz {
    fn clone(&self) -> Self {
        let mut ret = Mpz::default();
        ret.set(&self);
        ret
    }
}

impl PartialEq for Mpz {
    fn eq(&self, other: &Mpz) -> bool {
        self.cmp_mpz(&other) == 0
    }
}

impl PartialOrd for Mpz {
    fn partial_cmp(&self, other: &Mpz) -> Option<Ordering> {
        match self.cmp_mpz(&other) {
            x if x < 0 => Some(Ordering::Less),
            0 => Some(Ordering::Equal),
            _ => Some(Ordering::Greater),
        }
    }
}

impl Ord for Mpz {
    fn cmp(&self, other: &Mpz) -> Ordering {
        match self.cmp_mpz(&other) {
            x if x < 0 => Ordering::Less,
            0 => Ordering::Equal,
            _ => Ordering::Greater,
        }
    }
}

impl Hash for Mpz {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let size = self.inner.size;
        size.hash(state);
        if size != 0 {
            let limbs = size.checked_abs().expect("overflow") as usize;
            let slice = unsafe { slice::from_raw_parts(self.inner.d, limbs) };
            slice.hash(state);
        }
    }
}

impl From<u64> for Mpz {
    fn from(x: u64) -> Self {
        let mut ret = Mpz::default();
        unsafe { gmp::mpz_set_ui(&mut ret.inner, x) };
        ret
    }
}

impl FromStr for Mpz {
    type Err = std::ffi::NulError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ret = Mpz::default();
        let c_str = CString::new(s)?;
        ret.set_cstr(&c_str);
        Ok(ret)
    }
}

// Defines wrappers around gmp_mpfr_sys.  Functions ending
// with `_mut` correspond to giving the underlying GMP function
// the same Mpz variable for the first two arguments, e.g.
// to provide an interface for operations like x += y or x /= y.
impl Mpz {
    #[inline]
    pub fn abs(&mut self, x: &Mpz) {
        unsafe { gmp::mpz_abs(&mut self.inner, &x.inner) }
    }

    #[inline]
    pub fn abs_mut(&mut self) {
        unsafe { gmp::mpz_abs(&mut self.inner, &self.inner) }
    }

    #[inline]
    pub fn add(&mut self, x: &Mpz, y: &Mpz) {
        unsafe {
            gmp::mpz_add(&mut self.inner, &x.inner, &y.inner);
        }
    }

    #[inline]
    pub fn add_mut(&mut self, x: &Mpz) {
        unsafe { gmp::mpz_add(&mut self.inner, &self.inner, &x.inner) }
    }

    #[inline]
    pub fn add_ui_mut(&mut self, x: u64) {
        unsafe { gmp::mpz_add_ui(&mut self.inner, &self.inner, x) }
    }

    #[inline]
    pub fn sub_ui_mut(&mut self, x: u64) {
        unsafe { gmp::mpz_sub_ui(&mut self.inner, &self.inner, x) }
    }

    #[inline]
    pub fn sub_mul(&mut self, x: &Mpz, y: &Mpz) {
        unsafe {
            gmp::mpz_submul(&mut self.inner, &x.inner, &y.inner);
        }
    }

    #[inline]
    pub fn cmp_mpz(&self, other: &Mpz) -> i32 {
        unsafe { gmp::mpz_cmp(&self.inner, &other.inner) }
    }

    #[inline]
    pub fn cmpabs(&self, other: &Mpz) -> i32 {
        unsafe { gmp::mpz_cmpabs(&self.inner, &other.inner) }
    }

    #[inline]
    pub fn cmp_si(&self, val: i64) -> i32 {
        unsafe { gmp::mpz_cmp_si(&self.inner, val) }
    }

    #[inline]
    pub fn cdiv_q(&mut self, x: &Mpz, y: &Mpz) {
        unsafe {
            gmp::mpz_cdiv_q(&mut self.inner, &x.inner, &y.inner);
        }
    }

    #[inline]
    pub fn cdiv_r(&mut self, x: &Mpz, y: &Mpz) {
        unsafe {
            gmp::mpz_cdiv_r(&mut self.inner, &x.inner, &y.inner);
        }
    }

    #[inline]
    pub fn divexact(&mut self, n: &Mpz, d: &Mpz) {
        unsafe { gmp::mpz_divexact(&mut self.inner, &n.inner, &d.inner) }
    }

    #[inline]
    pub fn divexact_mut(&mut self, d: &Mpz) {
        unsafe {
            gmp::mpz_divexact(&mut self.inner, &self.inner, &d.inner);
        }
    }

    #[inline]
    pub fn fdiv_q(&mut self, x: &Mpz, y: &Mpz) {
        unsafe {
            gmp::mpz_fdiv_q(&mut self.inner, &x.inner, &y.inner);
        }
    }

    #[inline]
    pub fn fdiv_q_mut(&mut self, x: &Mpz) {
        unsafe { gmp::mpz_fdiv_q(&mut self.inner, &self.inner, &x.inner) }
    }

    #[inline]
    pub fn fdiv_r(&mut self, x: &Mpz, y: &Mpz) {
        unsafe { gmp::mpz_fdiv_r(&mut self.inner, &x.inner, &y.inner) }
    }

    #[inline]
    pub fn fdiv_r_mut(&mut self, x: &Mpz) {
        unsafe { gmp::mpz_fdiv_r(&mut self.inner, &self.inner, &x.inner) }
    }

    #[inline]
    pub fn fdiv_qr(&mut self, r: &mut Mpz, x: &Mpz, y: &Mpz) {
        unsafe { gmp::mpz_fdiv_qr(&mut self.inner, &mut r.inner, &x.inner, &y.inner) }
    }

    #[inline]
    pub fn fdiv_q_ui(&mut self, x: &Mpz, val: u64) {
        unsafe {
            gmp::mpz_fdiv_q_ui(&mut self.inner, &x.inner, val);
        }
    }

    #[inline]
    pub fn fdiv_q_ui_mut(&mut self, val: u64) {
        unsafe {
            gmp::mpz_fdiv_q_ui(&mut self.inner, &self.inner, val);
        }
    }

    #[inline]
    pub fn tdiv_q_mut(&mut self, x: &Mpz) {
        unsafe { gmp::mpz_tdiv_q(&mut self.inner, &self.inner, &x.inner) }
    }

    #[inline]
    pub fn fits_slong_p(&self) -> i32 {
        unsafe { gmp::mpz_fits_slong_p(&self.inner) }
    }

    #[inline]
    pub fn gcd(&mut self, x: &Mpz, y: &Mpz) {
        unsafe { gmp::mpz_gcd(&mut self.inner, &x.inner, &y.inner) }
    }

    #[inline]
    pub fn gcd_mut(&mut self, x: &Mpz) {
        unsafe { gmp::mpz_gcd(&mut self.inner, &self.inner, &x.inner) }
    }

    #[inline]
    pub fn gcdext(&mut self, d: &mut Mpz, e: &mut Mpz, a: &Mpz, m: &Mpz) {
        unsafe {
            gmp::mpz_gcdext(
                &mut self.inner,
                &mut d.inner,
                &mut e.inner,
                &a.inner,
                &m.inner,
            )
        }
    }

    #[inline]
    pub fn get_si(&self) -> i64 {
        unsafe { gmp::mpz_get_si(&self.inner) }
    }

    #[inline]
    pub fn modulo(&mut self, x: &Mpz, y: &Mpz) {
        unsafe { gmp::mpz_mod(&mut self.inner, &x.inner, &y.inner) }
    }

    #[inline]
    pub fn modulo_mut(&mut self, x: &Mpz) {
        unsafe { gmp::mpz_mod(&mut self.inner, &self.inner, &x.inner) }
    }

    #[inline]
    pub fn mul(&mut self, x: &Mpz, y: &Mpz) {
        unsafe { gmp::mpz_mul(&mut self.inner, &x.inner, &y.inner) }
    }

    #[inline]
    pub fn mul_mut(&mut self, x: &Mpz) {
        unsafe { gmp::mpz_mul(&mut self.inner, &self.inner, &x.inner) }
    }

    #[inline]
    pub fn mul_ui(&mut self, x: &Mpz, val: u64) {
        unsafe { gmp::mpz_mul_ui(&mut self.inner, &x.inner, val) }
    }
    #[inline]
    pub fn mul_si(&mut self, x: &Mpz, val: i64) {
        unsafe { gmp::mpz_mul_si(&mut self.inner, &x.inner, val) }
    }

    #[inline]
    pub fn mul_ui_mut(&mut self, val: u64) {
        unsafe { gmp::mpz_mul_ui(&mut self.inner, &self.inner, val) }
    }

    #[inline]
    pub fn neg(&mut self, x: &Mpz) {
        unsafe { gmp::mpz_neg(&mut self.inner, &x.inner) }
    }
    #[inline]
    pub fn neg_mut(&mut self) {
        unsafe { gmp::mpz_neg(&mut self.inner, &self.inner) }
    }

    #[inline]
    pub fn odd(&self) -> i32 {
        unsafe { gmp::mpz_odd_p(&self.inner) }
    }

    #[inline]
    pub fn root_mut(&mut self, x: u64) -> i32 {
        unsafe { gmp::mpz_root(&mut self.inner, &self.inner, x) }
    }

    #[inline]
    pub fn set(&mut self, x: &Mpz) {
        unsafe { gmp::mpz_set(&mut self.inner, &x.inner) }
    }

    #[inline]
    pub fn set_cstr(&mut self, cs: &CString) {
        unsafe {
            gmp::mpz_set_str(&mut self.inner, cs.as_ptr(), 10);
        }
    }

    #[inline]
    pub fn set_si(&mut self, val: i64) {
        unsafe { gmp::mpz_set_si(&mut self.inner, val) }
    }

    #[inline]
    pub fn set_ui(&mut self, val: u64) {
        unsafe { gmp::mpz_set_ui(&mut self.inner, val) }
    }

    #[inline]
    pub fn sgn(&self) -> i32 {
        unsafe { gmp::mpz_sgn(&self.inner) }
    }

    #[inline]
    pub fn square_mut(&mut self) {
        unsafe { gmp::mpz_mul(&mut self.inner, &self.inner, &self.inner) }
    }

    #[inline]
    pub fn sub(&mut self, x: &Mpz, y: &Mpz) {
        unsafe { gmp::mpz_sub(&mut self.inner, &x.inner, &y.inner) }
    }

    #[inline]
    pub fn submul(&mut self, x: &Mpz, y: &Mpz) {
        unsafe { gmp::mpz_submul(&mut self.inner, &x.inner, &y.inner) }
    }

    #[inline]
    pub fn sub_mut(&mut self, x: &Mpz) {
        unsafe { gmp::mpz_sub(&mut self.inner, &self.inner, &x.inner) }
    }

    #[inline]
    pub fn swap(&mut self, a: &mut Mpz) {
        unsafe { gmp::mpz_swap(&mut self.inner, &mut a.inner) }
    }

    #[inline]
    pub fn tdiv_q_2exp(&mut self, op1: &Mpz, op2: u64) {
        unsafe { gmp::mpz_tdiv_q_2exp(&mut self.inner, &op1.inner, op2) }
    }

    #[inline]
    pub fn bit_length(&self) -> usize {
        unsafe { gmp::mpz_sizeinbase(&self.inner, 2) as usize }
    }

    #[inline]
    pub fn is_neg(&self) -> bool {
        self.sgn() < 0
    }

    #[inline]
    pub fn size(&self) -> usize {
        unsafe { gmp::mpz_size(&self.inner) }
    }

    #[inline]
    pub fn getlimbn(&self, n: i64) -> limb_t {
        unsafe { gmp::mpz_getlimbn(&self.inner, n) }
    }

    #[inline]
    pub fn is_zero(&self) -> bool {
        self.inner.size == 0
    }

    #[inline]
    pub fn is_one(&self) -> bool {
        let mut one = Mpz::default();
        one.set_ui(1);
        self == &one
    }

    #[inline]
    pub fn div_floor(&mut self, numerator: &Mpz, denom: &Mpz) {
        unsafe {
            if denom.is_zero() {
                panic!("divide by zero")
            }

            gmp::mpz_fdiv_q(&mut self.inner, &numerator.inner, &denom.inner);
        }
    }

    #[inline]
    pub fn add_mul(&mut self, x: &Mpz, y: &Mpz) {
        unsafe {
            gmp::mpz_addmul(&mut self.inner, &x.inner, &y.inner);
        }
    }

    #[inline]
    pub fn from_bytes(data: &[u8]) -> Self {
        raw_import(data)
    }

    #[inline]
    pub fn to_u64(&self) -> Option<u64> {
        unsafe { Some(gmp::mpz_get_ui(&self.inner)) }
    }

    #[inline]
    pub fn one() -> Mpz {
        unsafe {
            let mut mpz = std::mem::uninitialized();
            gmp::mpz_init_set_ui(&mut mpz, 1);

            Mpz { inner: mpz }
        }
    }

    #[inline]
    pub fn zero() -> Mpz {
        Mpz::default()
    }

    /// Determine whether n is prime.
    ///
    /// This function performs some trial divisions, then reps Miller-Rabin probabilistic primality tests. A higher reps value will reduce the chances of a non-prime being identified as “probably prime”. A composite number will be identified as a prime with a probability of less than 4^(-reps). Reasonable values of reps are between 15 and 50.
    #[inline]
    pub fn probab_prime(&self, reps: i32) -> ProbabPrimeResult {
        match unsafe { gmp::mpz_probab_prime_p(&self.inner, reps as c_int) as u8 } {
            2 => ProbabPrimeResult::Prime,
            1 => ProbabPrimeResult::ProbablyPrime,
            0 => ProbabPrimeResult::NotPrime,
            x => panic!(
                "Undocumented return value {} from gmp::mpz_probab_prime_p",
                x
            ),
        }
    }

    #[inline]
    pub fn is_prime(&self, iterations: usize) -> bool {
        let is_prime = self.probab_prime(iterations as i32);
        if is_prime == ProbabPrimeResult::ProbablyPrime || is_prime == ProbabPrimeResult::Prime {
            return true;
        }

        return false;
    }

    #[inline]
    pub fn crem_u16(&self, modulus: u16) -> u16 {
        let res = unsafe { gmp::mpz_cdiv_ui(&self.inner, c_ulong::from(modulus)) };

        //assert!(res <= std::u16::MAX.into());
        res as u16
    }
}

/// Flint Port:
/// Given integers f, g with 0 ≤ f < g, computes the greatest common 
/// divisor d = gcd(f, g) and the modular inverse a = f−1 (mod g), whenever f ̸= 0.
/// Assumes that d and a are not aliased.
pub fn fmpz_gcdinv(d: &mut Mpz, a: &mut Mpz, f: &Mpz, g: &Mpz) {
    
}

/// The result of running probab_prime
#[derive(PartialEq)]
pub enum ProbabPrimeResult {
    NotPrime,
    ProbablyPrime,
    Prime,
}

// /// Helper function to import Mpz from raw network bytes
// fn raw_import(buf: &[u8]) -> Mpz {
//     let mut obj = Mpz::default();

//     unsafe {
//         gmp::mpz_import(
//             &mut obj.inner,
//             buf.len(),
//             1,
//             1,
//             1,
//             0,
//             buf.as_ptr() as *const _,
//         )
//     }
//     obj
// }


/// Returns `true` if `z` is negative and not zero.  Otherwise,
/// returns `false`.
#[inline]
pub fn mpz_is_negative(z: &Mpz) -> bool {
    if z.sgn() < 0 {
        true
    } else {
        false
    }
    //unsafe { (*(z as *const _ as *const MpzStruct)).mp_size < 0 }
}

/// Given integers f, g with 0 ≤ f < g, computes the greatest common divisor 
/// d = gcd(f, g) and the modular inverse a = f−1 (mod g), whenever f ̸= 0.
/// Assumes that d and a are not aliased.
#[inline]
pub fn mpz_gcdinv(d: &mut Mpz, a: &mut Mpz, f: &Mpz, g: &Mpz) {
   
}


fn raw_import(buf: &[u8]) -> Mpz {
    let mut obj = Mpz::default();

    unsafe { 
        gmp::mpz_import(
            &mut obj.inner, 
            buf.len(), 
            1, 
            1, 
            1, 
            0, 
            buf.as_ptr() as *const _
        ) 
    }
    obj
}


/// Helper function to export Mpz to raw network bytes
fn raw_export(raw: &Mpz) -> Vec<u8> {
    //let mut buf = Vec::<u8>::with_capacity(raw.bit_length());
    let mut buf = Vec::new();

    unsafe {
        let buf_ptr = buf.as_mut_ptr();
        let mut count = std::mem::MaybeUninit::uninit();
        let count_ptr = count.as_mut_ptr();

        let ptr2 = gmp::mpz_export(
            buf_ptr as *mut c_void,
            count_ptr,
            1, //countp
            1, //size
            1, //endian
            0, //nails
            &raw.inner
        );
        //assert_eq!(buf_ptr, ptr2);
    }

    println!("exbuf: {:?}", buf);
    buf
}

// pub fn raw_export(raw: &Mpz) -> Vec<u8> {
//     let mut buf = Vec::new();
//     let res = export_obj(raw, &mut buf);
//     //assert!(res.is_ok());
//     println!("exbufs: {:?}", res);
//     // println!("exbuf: {:?}", buf);
//     buf
// }



// fn check_rem() {
//     	        assert_eq!(mpz_crem_u16(&(-100i64).into(), 3), 1);
//     	        assert_eq!(mpz_crem_u16(&(100i64).into(), 3), 2);
//     	    }

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_import_export() {
        // let mut obj = Mpz::default();
        // let ex = raw_export(&obj);
        // let im = raw_import(&ex);
        // assert_eq!(im, obj);

        // let mut obj = Mpz::default();
        // obj.set_ui(55);
        // println!("ex: {:?}", obj);
        // let ex = raw_export(&obj);
        // println!("ex2: {:?}", ex);
        // let im = raw_import(&ex);
        // assert_eq!(im, obj);
    }
}