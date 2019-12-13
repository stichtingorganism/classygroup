//! Partial Euclidean algorithm.
///
/// Lehmer's version for computing GCD
/// (for Book's version of NUCOMP, NUDUPL, and NUCUBE algorithm).
///
/// Input:  R2 = R_{-1} , R1 = R_{0}, bound
///  - R_i is the R - sequence from "Solving the Pell Equation"
///   ( R_i = R_{i-2}-q_i R_{i-1} )
/// Output: R2 = R_{i-1}, R1 = R_i, C2 = C_{i-1}, C1 = C_i,
///  - R_i = 0 or R_i <= bound < R_{i-1}
///  - C_i sequence from "Solving the Pell Equation" defined as
///     C_{-1}=0, C_{1}=-1  C_i=C_{i-2}-q_i C_{i-1}
///
use crate::num::Mpz;
use gmp_mpfr_sys::gmp;

#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct PartialGCDContext {
    pub q: Mpz,
    pub r: Mpz,
    pub t1: Mpz,
    pub t2: Mpz,
}

impl Default for PartialGCDContext {
    fn default() -> Self {
        Self {
            q: Mpz::default(),
            r: Mpz::default(),
            t1: Mpz::default(),
            t2: Mpz::default(),
        }
    }
}

impl PartialGCDContext {
    /// This function is an implementation of Lehmer extended GCD with early termination.
    /// It terminates early when remainders fall below the specified bound.
    /// The initial values r1 and r2 are treated as successive remainders in the Euclidean algorithm
    /// and are replaced with the last two remainders computed. The values _co1 and _co2 are the last two
    /// cofactors and satisfy the identity _co2*r1 - _co1*r2 == +/- r2_orig upon termination, where
    /// r2_orig is the starting value of r2 supplied, and r1 and r2 are the final values.
    pub fn xgcd_partial(
        &mut self,
        c2: &mut Mpz,
        c1: &mut Mpz,
        r2: &mut Mpz,
        r1: &mut Mpz,
        bound: &Mpz,
    ) {
        c1.set_si(-1);
        c2.set_si(0);

        //loop index
        let mut _index = 0;

        while r1.sgn() != 0 && r1.cmp_mpz(&bound) > 0 {
            let mut _t = r2.bit_length();
            let mut _t1 = r1.bit_length();

            let mut bits = ((std::cmp::max(_t, _t1)) - (gmp::LIMB_BITS as usize) + 1) as u64;
            //println!("bits: {:?}", bits);

            //Bits
            if bits < 0 {
                bits = 0;
            }

            self.r.tdiv_q_2exp(&r2, bits);
            let mut rr2 = self.r.get_si();

            self.r.tdiv_q_2exp(&r1, bits);
            let mut rr1 = self.r.get_si();

            self.r.tdiv_q_2exp(&bound, bits);

            let bb: i64 = self.r.get_si();

            //reset values
            let mut a1: i64 = 1;
            let mut a2: i64 = 0;
            let mut b1: i64 = 0;
            let mut b2: i64 = 1;

            //reset inner loop index
            _index = 0;

            // Euclidean Step
            while rr1 != 0 && rr1 > bb {
                //println!("test_partical_gcd ");

                let qq: i64 = rr2 / rr1;

                //t1
                let t1 = rr2 - (qq * rr1);

                //t2
                let t2 = a2 - (qq * a1);

                //t3
                let t3 = b2 - (qq * b1);

                //check if it is even or odd
                if _index % 2 != 0 {
                    //index & 1
                    //its odd
                    if t1 < -t3 || rr1 - t1 < t2 - a1 {
                        break;
                    }
                } else {
                    //its even
                    if t1 < -t2 || rr2 - t1 < t3 - b1 {
                        break;
                    }
                }

                rr2 = rr1;
                rr1 = t1;

                a2 = a1;
                a1 = t2;

                b2 = b1;
                b1 = t3;

                //increment index
                _index += 1;
            }

            if _index == 0 {
                // multiprecision step
                let tmp = r2.clone();
                self.q.fdiv_qr(r2, &tmp, &r1); //i r2 is taken here what we do
                r2.swap(r1);
                c2.sub_mul(&c1, &self.q);
                c2.swap(c1);
            } else {
                // recombination
                // r = R2*B2 + R1*A2;  R1 = R2*B1 + R1*A1;  R2 = r;

                self.t1.mul_si(&r2, b2);
                self.t2.mul_si(&r1, a2);
                self.r.add(&self.t1, &self.t2);
                self.t1.mul_si(&r2, b1);
                self.t2.mul_si(&r1, a1);
                r1.add(&self.t1, &self.t2);
                r2.set(&self.r);
                self.t1.mul_si(&c2, b2);
                self.t2.mul_si(&c1, a2);
                self.r.add(&self.t1, &self.t2);
                self.t1.mul_si(&c2, b1);
                self.t2.mul_si(&c1, a1);
                c1.add(&self.t1, &self.t2);

                //
                c2.set(&self.r);

                // make sure R1 and R2 are positive
                if r1.sgn() < 0 {
                    r1.neg_mut();
                    c1.neg_mut();
                }

                if r2.sgn() < 0 {
                    r2.neg_mut();
                    c2.neg_mut();
                }
            }
        } //while loop end

        // make sure R2 is positive
        if r2.sgn() < 0 {
            r2.neg_mut();
            c1.neg_mut();
            c2.neg_mut();
        }
    } //function end
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::num::rand;

    #[test]
    fn test_partial_gcd() {
        let mut context = PartialGCDContext::default();
        // initilze the state object for the random generator functions
        let mut rand_state = rand::randinit();

        let mut c1 = Mpz::default();
        let mut c2 = Mpz::default();
        let mut f = Mpz::default();
        let mut g = Mpz::default();
        let mut t1 = Mpz::default();
        let mut t2 = Mpz::default();
        let mut l = Mpz::default();
        let mut tmp = Mpz::default();

        //Test co2*r1 - co1*r2 = r2_orig
        for i in 0..1000 {
            //println!("test_partical_gcd {:?}", i);
            //setup random
            //rand::randtest_unsigned(&mut g, &mut rand_state, 200);

            unsafe {
                gmp::mpz_urandomb(&mut g.inner, &mut rand_state.gmp, 200);
            }

            g.add_ui_mut(1);

            // rand::randm(&mut f, &mut rand_state, &g);

            unsafe {
                gmp::mpz_urandomm(&mut f.inner, &mut rand_state.gmp, &g.inner);
            }

            debug_assert!(f < g);

            unsafe {
                gmp::mpz_urandomb(&mut l.inner, &mut rand_state.gmp, 200);
            }
            //rand::randtest_unsigned(&mut l, &mut rand_state, 200);

            // if f > g {
            //   std::mem::swap(&mut f, &mut g);
            // }

            // f = Mpz::from_str("1427076718433602683841399960632061166757500579059120442883959").unwrap();
            // g = Mpz::from_str("608378832249340764288689654962749468145112247199666200394519").unwrap();
            // l = Mpz::from_str("372072546338428393062134977388178809720614805209817726682600").unwrap();

            // println!("partial g {:?}", &g);
            // println!("partial f {:?}", &f);
            // println!("partial L {:?}", l);

            //g.init = g*c1 - c2*f

            t2.set(&g);
            t2.abs_mut();

            context.xgcd_partial(&mut c2, &mut c1, &mut g, &mut f, &mut l);

            // println!("partial c1 {:?}", &c1);
            // println!("partial c2 {:?}", &c2);

            t1.mul(&c2, &f);
            t1.sub_mul(&c1, &g);
            t1.abs_mut();

            assert_eq!(t1, t2);

            // println!("partial t1 {:?}", &t1);
            // println!("partial t2 {:?}", &t2);
            //Test co2*r1 - co1*r2 = r2_orig

            //println!("partial pass");
        }
    }

    // #[test]
    // fn test_linear_congruence_solver_no_solution() {
    //     let mut context = CongruenceContext::default();
    //     // Let `g = gcd(a, m)`. If `b` is not divisible by `g`, there are no solutions. If `b` is
    //     // divisible by `g`, there are `g` solutions.
    //     let mut s = Mpz::default();
    //     let mut t = Mpz::default();

    //     ctx.solve_linear_congruence(s, Some(t), &Mpz::from(3), &Mpz::from(2), &Mpz::from(4));

    //     let result =
    //         solve_linear_congruence(&Mpz::from(33), &Mpz::from(7), &Mpz::from(143));
    //     assert!(result.is_none());

    //     let result =
    //         solve_linear_congruence(&Mpz::from(13), &Mpz::from(14), &Mpz::from(39));
    //     assert!(result.is_none());
    // }
}
