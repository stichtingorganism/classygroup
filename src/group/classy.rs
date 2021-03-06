//! Class Group implementation

use super::CLASS_GROUP_DISCRIMINANT;
use crate::group::{ClassCtx, ClassElem};
use crate::mut_tuple_elems;
use crate::num::Mpz;
use rug::Integer;
use std::cell::RefCell;

const EXP_THRESH: i64 = 31;
const THRESH: i64 = ((1 as u64) << 31) as i64;

thread_local! {
  // Thread-local context for class group operations.
  static CTX: RefCell<ClassCtx> = Default::default();
}

// Runs the given closure with the Class Context. The expression passed must be
// a closure that takes in an element of type &mut ClassElem. Furthermore, the lambda
// cannot contain subroutines which themselves call the `with_ctx` macro, or the
// compiler will not be happy.
macro_rules! with_ctx {
    ($logic:expr) => {
        CTX.with(|refcell| {
            let mut ctx_ = refcell.borrow_mut();
            $logic(&mut ctx_)
        })
    };
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ClassGroup {}

#[inline]
fn log2(x: u64) -> u32 {
    63 - x.leading_zeros()
}

#[inline]
pub fn signed_shift(op: u64, shift: i64) -> u64 {
    match shift {
        x if x > 0 => op << shift,
        x if x <= -64 => 0,
        _ => op >> (-shift),
    }
}

#[inline]
pub fn mpz_get_si_2exp(op: &Mpz) -> (i64, i64) {
    let size = op.size();
    let last = op.getlimbn((size - 1) as i64);
    let lg2 = log2(last) + 1;
    let mut exp = lg2 as i64;
    let mut ret = signed_shift(last, 63 - exp);
    if size > 1 {
        exp += ((size - 1) * 64) as i64;
        let prev = op.getlimbn((size - 2) as i64);
        ret += signed_shift(prev, (-1 - lg2 as i32) as i64);
    }
    if op.is_neg() {
        return (-(ret as i64), exp);
    }
    (ret as i64, exp)
}

#[inline]
pub fn test_reduction(x: &mut ClassElem) -> bool {
    let a_b = x.a.cmpabs(&x.b);
    let c_b = x.c.cmpabs(&x.b);

    if a_b < 0 || c_b < 0 {
        return false;
    }

    let a_c = x.a.cmp_mpz(&x.c);

    if a_c > 0 {
        x.a.swap(&mut x.c);
        x.b.neg_mut();
    }
    if a_c == 0 && x.b.is_neg() {
        x.b.neg_mut();
    }
    true
}

impl ClassGroup {
    fn discriminant(a: &Mpz, b: &Mpz, c: &Mpz) -> Mpz {
        with_ctx!(|ctx: &mut ClassCtx| {
            let (scratch,) = mut_tuple_elems!(ctx.op_ctx, 0);

            let mut d = Mpz::default();
            d.mul(&b, &b);
            scratch.mul(&a, &c);
            scratch.mul_ui_mut(4);
            d.sub_mut(&scratch);
            d
        })
    }

    #[allow(non_snake_case)]
    pub fn square(x: &mut ClassElem) {
        // Jacobson, Michael J., and Alfred J. Van Der Poorten. "Computational aspects of NUCOMP."
        // Algorithm 2 (Alg 2).

        with_ctx!(|ctx: &mut ClassCtx| {
            let (
                G_sq_op,
                scratch,
                mut y_sq_op,
                By_sq_op,
                Dy_sq_op,
                mut bx_sq_op,
                mut by_sq_op,
                dx_sq_op,
                q_sq_op,
                t_sq_op,
                ax_sq_op,
                ay_sq_op,
                Q1_sq_op,
                mut x_sq_op,
                z_sq_op,
                dy_sq_op,
            ) = mut_tuple_elems!(ctx.op_ctx, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15);

            let L_sq_op = &mut ctx.L;

            // Step 1 in Alg 2.
            G_sq_op.gcdext(scratch, y_sq_op, &x.a, &x.b);
            By_sq_op.divexact(&x.a, &G_sq_op);
            Dy_sq_op.divexact(&x.b, &G_sq_op);

            // Step 2 in Alg 2.
            bx_sq_op.mul(&y_sq_op, &x.c);
            bx_sq_op.modulo_mut(&By_sq_op);
            by_sq_op.set(&By_sq_op);

            if by_sq_op.cmpabs(&L_sq_op) <= 0 {
                // Step 4 in Alg 2.
                dx_sq_op.mul(&bx_sq_op, &Dy_sq_op);
                dx_sq_op.sub_mut(&x.c);
                dx_sq_op.divexact_mut(&By_sq_op);
                x.a.mul(&by_sq_op, &by_sq_op);
                x.c.mul(&bx_sq_op, &bx_sq_op);
                t_sq_op.add(&bx_sq_op, &by_sq_op);
                t_sq_op.square_mut();

                x.b.sub_mut(&t_sq_op);
                x.b.add_mut(&x.a);
                x.b.add_mut(&x.c);
                t_sq_op.mul(&G_sq_op, &dx_sq_op);
                x.c.sub_mut(&t_sq_op);
                return;
            }

            // Subroutine as handled by top entry to the Chia VDF competition "bulaiden."
            // Lehmer partial extended GCD.
            ctx.partial_context.xgcd_partial(
                &mut y_sq_op,
                &mut x_sq_op,
                &mut by_sq_op,
                &mut bx_sq_op,
                &L_sq_op,
            ); //L should be const

            x_sq_op.neg_mut();
            if x_sq_op.sgn() > 0 {
                y_sq_op.neg_mut();
            } else {
                by_sq_op.neg_mut();
            }

            ax_sq_op.mul(&G_sq_op, &x_sq_op);
            ay_sq_op.mul(&G_sq_op, &y_sq_op);

            // Step 5 in Alg 2.
            t_sq_op.mul(&Dy_sq_op, &bx_sq_op);
            t_sq_op.submul(&x.c, &x_sq_op);
            dx_sq_op.divexact(&t_sq_op, &By_sq_op);
            Q1_sq_op.mul(&y_sq_op, &dx_sq_op);
            dy_sq_op.add(&Q1_sq_op, &Dy_sq_op);
            x.b.add(&dy_sq_op, &Q1_sq_op);
            x.b.mul_mut(&G_sq_op);
            dy_sq_op.divexact_mut(&x_sq_op);
            x.a.mul(&by_sq_op, &by_sq_op);
            x.c.mul(&bx_sq_op, &bx_sq_op);
            t_sq_op.add(&bx_sq_op, &by_sq_op);
            x.b.submul(&t_sq_op, &t_sq_op);
            x.b.add_mut(&x.a);
            x.b.add_mut(&x.c);
            x.a.submul(&ay_sq_op, &dy_sq_op);
            x.c.submul(&ax_sq_op, &dx_sq_op);
        });

        Self::reduce_mut(x);
    }

    fn reduce_mut(x: &mut ClassElem) {
        Self::normalize_mut(x);
        Self::reduce(x);
        Self::normalize_mut(x);
    }

    fn reduce(elem: &mut ClassElem) {
        with_ctx!(|ctx: &mut ClassCtx| {
            let (
                x,
                s,
                ra,
                rb,
                h,
                g,
                j,
                k,
                rw,
                l,
                mut r_norm,
                mut denom_norm,
                mut mu_norm,
                mut s_norm,
                mut ra_norm,
                mut rb_norm,
            ) = mut_tuple_elems!(ctx.op_ctx, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15);

            while !test_reduction(elem) {
                let (mut a, a_exp) = mpz_get_si_2exp(&elem.a);
                let (mut b, b_exp) = mpz_get_si_2exp(&elem.b);
                let (mut c, c_exp) = mpz_get_si_2exp(&elem.c);

                let mut max_exp = a_exp;
                let mut min_exp = a_exp;

                use std::cmp::max;
                use std::cmp::min;

                max_exp = max(max_exp, b_exp);
                max_exp = max(max_exp, c_exp);
                min_exp = min(min_exp, b_exp);
                min_exp = min(min_exp, c_exp);

                //println!("about to check normalize");

                if max_exp - min_exp > EXP_THRESH {
                    //Self::normalize_mut(elem);
                    ClassGroup::normalizer(
                        elem,
                        &mut r_norm,
                        &mut denom_norm,
                        &mut mu_norm,
                        &mut s_norm,
                        &mut ra_norm,
                        &mut rb_norm,
                    );
                    //ClassGroup::normalize_(&mut elem.a, &mut elem.b, &mut elem.c);
                    continue;
                }
                //  println!("a: {}", x.a);
                //  println!("b: {}", x.b);
                //  println!("c: {}", x.c);
                max_exp += 1; // for overflow safety
                a >>= max_exp - a_exp;
                b >>= max_exp - b_exp;
                c >>= max_exp - c_exp;

                let mut u_ = 1;
                let mut v_ = 0;
                let mut w_ = 0;
                let mut y_ = 1;

                let mut u;
                let mut v;
                let mut w;
                let mut y;

                //    println!("starting do-while loop");
                loop {
                    //println!("start of loop");
                    u = u_;
                    v = v_;
                    w = w_;
                    y = y_;
                    let delta = if b >= 0 {
                        //      println!("top");
                        (b + c) / (c << 1)
                    } else {
                        //      println!("bottom");
                        -(-b + c) / (c << 1)
                    };
                    let a_ = c;
                    let mut c_ = c * delta;
                    let b_ = -b + (c_ << 1);
                    let gamma = b - c_;
                    //    println!("a: {}", a);
                    //    println!("delta: {}", delta);
                    //    println!("gamma: {}", gamma);
                    c_ = a - delta * gamma;

                    a = a_;
                    b = b_;
                    c = c_;

                    u_ = v;
                    v_ = -u + delta * v;
                    w_ = y;
                    y_ = -w + delta * y;
                    if !((v_.abs() | y_.abs()) <= THRESH && a > c && c > 0) {
                        break;
                    }
                }
                //println!("finished loop");
                if (v_.abs() | y_.abs()) <= THRESH {
                    u = u_;
                    v = v_;
                    w = w_;
                    y = y_;
                }
                let aa = u * u;
                //println!("aa: {}", aa);
                let ab = u * w;
                //println!("ab: {}", ab);
                let ac = w * w;
                //println!("ac: {}", ac);
                let ba = (u * v) << 1;
                //println!("ba: {}", ba);
                let bb = u * y + v * w;
                //println!("bb: {}", bb);
                let bc = (w * y) << 1;
                //println!("bc: {}", bc);
                let ca = v * v;
                //println!("ca: {}", ca);
                let cb = v * y;
                //println!("cb: {}", cb);
                let cc = y * y;
                //sprintln!("cc: {}", cc);

                ra.mul_si(&elem.a, aa); // a = faa
                rb.mul_si(&elem.b, ab); // b = fab
                h.mul_si(&elem.c, ac); // h = fac

                g.mul_si(&elem.a, ba); // g = fba
                j.mul_si(&elem.b, bb); // j = fbb
                k.mul_si(&elem.c, bc); // k = fbc

                s.mul_si(&elem.a, ca); // s = fca
                rw.mul_si(&elem.b, cb); // w = fcb
                l.mul_si(&elem.c, cc); // l = fcc

                elem.a.add(&ra, &rb);
                elem.a.add_mut(&h);

                elem.b.add(&g, &j);
                elem.b.add_mut(&k);

                elem.c.add(&s, &rw);
                elem.c.add_mut(&l);
            }
        })
    }

    fn normalize_mut(x: &mut ClassElem) {
        let already_normal = with_ctx!(|ctx: &mut ClassCtx| {
            let (scratch,) = mut_tuple_elems!(ctx.op_ctx, 0);
            if Self::elem_is_normal(scratch, &x.a, &x.b, &x.c) {
                return true;
            }
            false
        });

        if !already_normal {
            ClassGroup::normalize(&mut x.a, &mut x.b, &mut x.c);
        }
    }

    fn normalize(a: &mut Mpz, b: &mut Mpz, c: &mut Mpz) {
        with_ctx!(|ctx: &mut ClassCtx| {
            let (r, denom, old_b, ra) = mut_tuple_elems!(ctx.op_ctx, 0, 1, 2, 3);

            // Binary Quadratic Forms, 5.1.1
            r.sub(&a, &b);
            denom.mul_ui(&a, 2);
            r.fdiv_q_mut(&denom);

            old_b.set(&b);

            ra.mul(&r, &a);
            b.add_mut(&ra);
            b.add_mut(&ra);

            ra.mul_mut(&r);
            c.add_mut(&ra);

            ra.set(&r);
            ra.mul_mut(&old_b);
            c.add_mut(&ra);
        })
    }

    fn normalizer(
        elem: &mut ClassElem,
        r: &mut Mpz,
        denom: &mut Mpz,
        mu: &mut Mpz,
        s: &mut Mpz,
        ra: &mut Mpz,
        rb: &mut Mpz,
    ) {
        mu.add(&elem.b, &elem.c);
        s.mul_ui(&elem.c, 2);

        denom.fdiv_q(&mu, &s);

        ra.set(&elem.c);

        s.mul_ui(&denom, 2);
        rb.neg(&elem.b);
        rb.add_mul(&elem.c, &s);

        r.set(&elem.a);
        r.submul(&elem.b, &denom);
        denom.square_mut();
        r.add_mul(&elem.c, &denom);

        elem.a.set(&ra);
        elem.b.set(&rb);
        elem.c.set(&r);
    }

    //WIP NUCOMP
    pub fn nucomp(x: &ClassElem, y: &ClassElem) -> ClassElem {
        //a1, a2, c2, ca, cb, cc, k, s, sp, ss, m, t, u2, v1, v2;
        let mut unreduced = with_ctx!(|ctx: &mut ClassCtx| {
            let (a1, a2, c2, mut co1, mut co2, m1, k, s, sp, ss, m, t, mut u2, v1, mut v2, mut temp) = mut_tuple_elems!(
                ctx.op_ctx, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15
            );

            let L_sq_op = &mut ctx.L;

            if x.a.cmp_mpz(&y.a) > 0 {
                Self::nucomp(y, x);
            } 
            
            /* nucomp calculation */
            let mut ret = ClassElem::default();
            //TODO: Remove these allocations
            let mut r1 = Mpz::default();
            let mut r2 = Mpz::default();
            let mut m2 = Mpz::default();
            // a1 = x.a;
            // a2 = y.a;
            // c2 = y.c;

            ss.add(&x.a, &y.a);
            ss.fdiv_q_ui_mut(2);

            m.sub(&x.b, &y.b);
            m.fdiv_q_ui_mut(2);

            t.fdiv_r(&y.a, &x.a);

            if t.is_zero() {
                v1.set_ui(0);
                sp.set(&x.a);
            } else {
                //gcdinv
            }

            k.mul(&m, &v1);
            k.fdiv_r_mut(&a1);

            if !sp.is_one() {
                s.gcdext(&mut v2, &mut u2, &ss, &sp);
                k.mul_mut(&u2);
                t.mul(&v2, &c2);
                k.sub_mut(&t);

                if !s.is_one() {
                    a1.fdiv_q_mut(&s);
                    a2.fdiv_q_mut(&s);
                    c2.mul_mut(&s);
                }

                k.fdiv_r_mut(&a1);
            }

            if a1.cmp_mpz(&L_sq_op) < 0 {
                t.mul(&a2, &k);
                ret.a.mul(&a2, &a1);

                ret.b.mul_ui(&t, 2);
                ret.b.add_mut(&y.b);

                ret.c.add(&y.b, &t);
                ret.c.mul_mut(&k);
                ret.c.add_mut(&c2);
                
                ret.c.fdiv_q_mut(&a1);
            } else {
                // fmpz_t m1, m2, r1, r2, co1, co2, temp;
                // fmpz_set(r2, a1);
                // fmpz_set(r1, k);
          
                // Lehmer partial extended GCD.
                ctx.partial_context.xgcd_partial(
                    &mut co2,
                    &mut co1,
                    &mut r2,
                    &mut r1,
                    &L_sq_op,
                ); //L should be const

                t.mul(&a2, &r1);
                m1.mul(&m, &co1);
                m1.add_mut(&t);
                m1.tdiv_q_mut(&a1);

                m2.mul(&ss, &r1);
                temp.mul(&c2, &co1);
                m2.sub_mut(&temp);
                m2.tdiv_q_mut(&a1);

                ret.a.mul(&r1, &m1);
                temp.mul(&co1, &m2);

                if co1.sgn() < 0 {
                    ret.a.sub_mut(&temp); //could be sub other way around
                } else {
                    ret.a.sub_mut(&temp);//could be sub other way around
                }
                
                ret.b.mul(&ret.a, &co2);
                ret.b.sub_mut(&t); //could be sub other way around
                ret.b.mul_ui_mut(2);
                ret.b.fdiv_q_mut(&co1);
                ret.b.sub_mut(&y.b);

                temp.mul_ui(&ret.a, 2);
                ret.b.fdiv_r_mut(&temp);

                ret.c.mul(&ret.b, &ret.b);
                ret.c.sub_mut(&ctx.D);
                ret.c.fdiv_q_mut(&ret.a);
                ret.c.fdiv_q_ui_mut(2);

                if ret.a.sgn() < 0 {
                    ret.a.neg_mut();
                    ret.c.neg_mut();
                }

            }

            ret
        });

        Self::reduce_mut(&mut unreduced);
        unreduced
    }

    pub fn op(x: &ClassElem, y: &ClassElem) -> ClassElem {
        let mut unreduced = with_ctx!(|ctx: &mut ClassCtx| {
            let (g, h, j, w, r, s, t, u, a, b, l, m, mut mu, mut v, mut lambda, mut sigma, k) = mut_tuple_elems!(
                ctx.op_ctx, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16
            );

            // Binary Quadratic Forms, 6.1.1
            g.add(&x.b, &y.b);
            g.fdiv_q_ui_mut(2);
            h.sub(&y.b, &x.b);
            h.fdiv_q_ui_mut(2);
            w.gcd(&x.a, &y.a);
            w.gcd_mut(&g);
            j.set(&w);
            r.set_ui(0);
            s.fdiv_q(&x.a, &w);
            t.fdiv_q(&y.a, &w);
            u.fdiv_q(&g, &w);
            a.mul(&t, &u);
            b.mul(&h, &u);
            m.mul(&s, &x.c);
            b.add_mut(&m);
            m.mul(&s, &t);
            ctx.lin_cong_ctx
                .solve_linear_congruence(&mut mu, &mut v, &a, &b, &m)
                .unwrap();

            a.mul(&t, &v);
            m.mul(&t, &mu);
            b.sub(&h, &m);
            m.set(&s);
            ctx.lin_cong_ctx
                .solve_linear_congruence(&mut lambda, &mut sigma, &a, &b, &m)
                .unwrap();

            a.mul(&v, &lambda);
            k.add(&mu, &a);
            l.mul(&k, &t);
            l.sub_mut(&h);
            l.fdiv_q_mut(&s);
            m.mul(&t, &u);
            m.mul_mut(&k);
            a.mul(&h, &u);
            m.sub_mut(&a);
            a.mul(&x.c, &s);
            m.sub_mut(&a);
            a.mul(&s, &t);
            m.fdiv_q_mut(&a);

            let mut ret = ClassElem::default();

            ret.a.mul(&s, &t);
            a.mul(&r, &u);
            ret.a.sub_mut(&a);

            ret.b.mul(&j, &u);
            a.mul(&m, &r);
            ret.b.add_mut(&a);
            a.mul(&k, &t);
            ret.b.sub_mut(&a);
            a.mul(&l, &s);
            ret.b.sub_mut(&a);

            ret.c.mul(&k, &l);
            a.mul(&j, &m);
            ret.c.sub_mut(&a);
            ret
        });

        Self::reduce_mut(&mut unreduced);
        unreduced
    }

    fn id() -> ClassElem {
        with_ctx!(|ctx: &mut ClassCtx| {
            let (a,) = mut_tuple_elems!(ctx.op_ctx, 0);

            // Binary Quadratic Forms, Definition 5.4
            // The identity is the Principal Form of Discriminant d.
            let mut ret = ClassElem::default();
            ret.a.set_ui(1);
            ret.b.set_ui(1);
            a.sub(&ret.b, &CLASS_GROUP_DISCRIMINANT);
            ret.c.fdiv_q_ui(&a, 4);
            ret
        })
    }

    fn inv(x: &ClassElem) -> ClassElem {
        let mut ret = ClassElem::default();
        ret.a.set(&x.a);
        ret.b.neg(&x.b);
        ret.c.set(&x.c);
        ret
    }

    pub fn pow(a: &ClassElem, n: &Integer) -> ClassElem {
        let (mut val, mut a, mut n) = {
            if *n < Integer::from(0) {
                (Self::id(), Self::inv(&a), Integer::from(-n))
            } else {
                (Self::id(), a.clone(), n.clone())
            }
        };
        loop {
            if n == Integer::from(0) {
                return val;
            }

            if n.is_odd() {
                val = Self::op(&val, &a);
            }

            Self::square(&mut a);
            n >>= 1;
        }
    }

    /// The generator element
    pub fn unknown_order_elem() -> ClassElem {
        // Binary Quadratic Forms, Definition 5.4
        let mut ret = ClassElem::default();
        ret.a.set_ui(2);
        ret.b.set_ui(1);
        ret.c.set_ui(1);
        ret.c.sub_mut(&CLASS_GROUP_DISCRIMINANT);
        ret.c.fdiv_q_ui_mut(8);

        Self::reduce(&mut ret);
        ClassElem {
            a: ret.a,
            b: ret.b,
            c: ret.c,
        }
    }

    /// The generator element
    pub fn unknown_order_elem_disc(disc: &Mpz) -> ClassElem {
        // Binary Quadratic Forms, Definition 5.4
        let mut ret = ClassElem::default();
        ret.a.set_ui(2);
        ret.b.set_ui(1);
        ret.c.set_ui(1);
        ret.c.sub_mut(disc);
        ret.c.fdiv_q_ui_mut(8);

        Self::reduce(&mut ret);
        ClassElem {
            a: ret.a,
            b: ret.b,
            c: ret.c,
        }
    }

    fn validate(a: &Mpz, b: &Mpz, c: &Mpz) -> bool {
        ClassGroup::discriminant(a, b, c) == *CLASS_GROUP_DISCRIMINANT
    }

    fn elem_is_normal(scratch: &mut Mpz, a: &Mpz, b: &Mpz, _c: &Mpz) -> bool {
        scratch.neg(&a);
        *scratch < *b && b <= a
    }

    pub fn elem(abc: (Mpz, Mpz, Mpz)) -> ClassElem {
        let mut el = ClassElem {
            a: abc.0,
            b: abc.1,
            c: abc.2,
        };
        ClassGroup::reduce(&mut el);

        // Ideally, this should return an error and the
        // return type of ElemFrom should be Result<Self::Elem, Self:err>,
        // but this would require a lot of ugly "unwraps" in the accumulator
        // library. Besides, users should not need to create new class group
        // elements, so an invalid ElemFrom here should signal a severe internal error.
        assert!(ClassGroup::validate(&el.a, &el.b, &el.c));

        el
    }
}

//  Caveat: tests that use "ground truth" use outputs from
//  Chia's sample implementation in python:
//    https://github.com/Chia-Network/vdf-competition/blob/master/inkfish/classgroup.py.
#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use std::str::FromStr;

    use std::{
        fs::File,
        io::{BufRead, BufReader},
        path::PathBuf,
    };

    // Makes a class elem tuple but does not reduce.
    fn construct_raw_elem_from_strings(a: &str, b: &str, c: &str) -> ClassElem {
        ClassElem {
            a: Mpz::from_str(a).unwrap(),
            b: Mpz::from_str(b).unwrap(),
            c: Mpz::from_str(c).unwrap(),
        }
    }

    #[should_panic]
    #[test]
    fn test_bad_elem() {
        let _ = ClassGroup::elem((Mpz::from(1), Mpz::from(2), Mpz::from(3)));
    }

    #[test]
    fn test_elem_from() {
        let a1 = Mpz::from_str("16").unwrap();
        let b1 = Mpz::from_str("105").unwrap();
        let c1 = Mpz::from_str(
      "47837607866886756167333839869251273774207619337757918597995294777816250058331116325341018110\
      672047217112377476473502060121352842575308793237621563947157630098485131517401073775191194319\
      531549483898334742144138601661120476425524333273122132151927833887323969998955713328783526854\
      198871332313399489386997681827578317938792170918711794684859311697439726596656501594138449739\
      494228617068329664776714484742276158090583495714649193839084110987149118615158361352488488402\
      038894799695420483272708933239751363849397287571692736881031223140446926522431859701738994562\
      9057462766047140854869124473221137588347335081555186814207",
    )
    .unwrap();

        let a2 = Mpz::from_str("16").unwrap();
        let b2 = Mpz::from_str("9").unwrap();
        let c2 = Mpz::from_str(
      "47837607866886756167333839869251273774207619337757918597995294777816250058331116325341018110\
      672047217112377476473502060121352842575308793237621563947157630098485131517401073775191194319\
      531549483898334742144138601661120476425524333273122132151927833887323969998955713328783526854\
      198871332313399489386997681827578317938792170918711794684859311697439726596656501594138449739\
      494228617068329664776714484742276158090583495714649193839084110987149118615158361352488488402\
      038894799695420483272708933239751363849397287571692736881031223140446926522431859701738994562\
      9057462766047140854869124473221137588347335081555186814036",
    )
    .unwrap();

        let reduced_elem = ClassGroup::elem((a1, b1, c1));
        let also_reduced_elem = ClassGroup::elem((a2, b2, c2));
        assert_eq!(reduced_elem, also_reduced_elem);
    }

    #[test]
    fn test_equality() {
        let mut not_reduced = construct_raw_elem_from_strings(
      "16",
      "105",
      "47837607866886756167333839869251273774207619337757918597995294777816250058331116325341018110\
      672047217112377476473502060121352842575308793237621563947157630098485131517401073775191194319\
      531549483898334742144138601661120476425524333273122132151927833887323969998955713328783526854\
      198871332313399489386997681827578317938792170918711794684859311697439726596656501594138449739\
      494228617068329664776714484742276158090583495714649193839084110987149118615158361352488488402\
      038894799695420483272708933239751363849397287571692736881031223140446926522431859701738994562\
      9057462766047140854869124473221137588347335081555186814207"
    );

        let reduced_ground_truth = construct_raw_elem_from_strings(
      "16",
      "9",
      "47837607866886756167333839869251273774207619337757918597995294777816250058331116325341018110\
      672047217112377476473502060121352842575308793237621563947157630098485131517401073775191194319\
      531549483898334742144138601661120476425524333273122132151927833887323969998955713328783526854\
      198871332313399489386997681827578317938792170918711794684859311697439726596656501594138449739\
      494228617068329664776714484742276158090583495714649193839084110987149118615158361352488488402\
      038894799695420483272708933239751363849397287571692736881031223140446926522431859701738994562\
      9057462766047140854869124473221137588347335081555186814036"
    );

        let diff_elem = construct_raw_elem_from_strings(
      "4",
      "1",
      "19135043146754702466933535947700509509683047735103167439198117911126500023332446530136407244\
      268818886844950990589400824048541137030123517295048625578863052039394052606960429510076477727\
      812619793559333896857655440664448190570209733309248852860771133554929587999582285331513410741\
      679548532925359795754799072731031327175516868367484717873943724678975890638662600637655379895\
      797691446827331865910685793896910463236233398285859677535633644394859647446063344540995395360\
      815557919878168193309083573295900545539758915028677094752412489256178770608972743880695597825\
      16229851064188563419476497892884550353389340326220747256139"
    );

        assert!(not_reduced != reduced_ground_truth);
        assert!(not_reduced == not_reduced.clone());
        assert!(reduced_ground_truth == reduced_ground_truth.clone());
        assert!(not_reduced != diff_elem);
        assert!(reduced_ground_truth != diff_elem);

        ClassGroup::reduce(&mut not_reduced);
        assert!(not_reduced == reduced_ground_truth);
    }

    #[test]
    fn test_hash() {
        let mut not_reduced = construct_raw_elem_from_strings(
      "16",
      "105",
      "47837607866886756167333839869251273774207619337757918597995294777816250058331116325341018110\
      672047217112377476473502060121352842575308793237621563947157630098485131517401073775191194319\
      531549483898334742144138601661120476425524333273122132151927833887323969998955713328783526854\
      198871332313399489386997681827578317938792170918711794684859311697439726596656501594138449739\
      494228617068329664776714484742276158090583495714649193839084110987149118615158361352488488402\
      038894799695420483272708933239751363849397287571692736881031223140446926522431859701738994562\
      9057462766047140854869124473221137588347335081555186814207"
    );

        let reduced_ground_truth = construct_raw_elem_from_strings(
      "16",
      "9",
      "47837607866886756167333839869251273774207619337757918597995294777816250058331116325341018110\
      672047217112377476473502060121352842575308793237621563947157630098485131517401073775191194319\
      531549483898334742144138601661120476425524333273122132151927833887323969998955713328783526854\
      198871332313399489386997681827578317938792170918711794684859311697439726596656501594138449739\
      494228617068329664776714484742276158090583495714649193839084110987149118615158361352488488402\
      038894799695420483272708933239751363849397287571692736881031223140446926522431859701738994562\
      9057462766047140854869124473221137588347335081555186814036"
    );

        let diff_elem = construct_raw_elem_from_strings(
      "4",
      "1",
      "19135043146754702466933535947700509509683047735103167439198117911126500023332446530136407244\
      268818886844950990589400824048541137030123517295048625578863052039394052606960429510076477727\
      812619793559333896857655440664448190570209733309248852860771133554929587999582285331513410741\
      679548532925359795754799072731031327175516868367484717873943724678975890638662600637655379895\
      797691446827331865910685793896910463236233398285859677535633644394859647446063344540995395360\
      815557919878168193309083573295900545539758915028677094752412489256178770608972743880695597825\
      16229851064188563419476497892884550353389340326220747256139"
    );

        let mut hasher_lh = DefaultHasher::new();
        let mut hasher_rh = DefaultHasher::new();
        not_reduced.hash(&mut hasher_lh);
        reduced_ground_truth.hash(&mut hasher_rh);

        assert!(hasher_lh.finish() != hasher_rh.finish());
        assert!(hasher_lh.finish() == hasher_lh.finish());
        assert!(hasher_rh.finish() == hasher_rh.finish());

        hasher_lh = DefaultHasher::new();
        hasher_rh = DefaultHasher::new();
        ClassGroup::reduce(&mut not_reduced);
        not_reduced.hash(&mut hasher_lh);
        reduced_ground_truth.hash(&mut hasher_rh);
        assert!(hasher_lh.finish() == hasher_rh.finish());

        hasher_lh = DefaultHasher::new();
        hasher_rh = DefaultHasher::new();
        not_reduced.hash(&mut hasher_lh);
        diff_elem.hash(&mut hasher_rh);
        assert!(hasher_lh.finish() != hasher_rh.finish());
    }

    #[test]
    fn test_reduce_basic() {
        let mut to_reduce = construct_raw_elem_from_strings(
      "59162244921619725812008939143220718157267937427074598447911241410131470159247784852210767449\
      675610037288729551814191198624164179866076352187405442496568188988272422133088755036699145362\
      385840772236403043664778415471196678638241785773530531198720497580622741709880533724904220122\
      358854068046553219863419609777498761804625479650772123754523807001976654588225908928022367436\
      8",
      "18760351095004839755193532164856605650590306627169248964100884295652838905828158941233738613\
      175821849253748329102319504958410190952820220503570113920576542676928659211807590199941027958\
      195895385446372444261885022800653454209101497963588809819572703579484085278913354621371362285\
      341138299691587953249270188429393417132110841259813122945626515477865766896056280729710478647\
      13",
      "14872270891432803054791175727694631095755964943358394411314110783404577714102170379700365256\
      599679049493824862742803590079461712691146098397470840896560034332315858221821103076776907123\
      277315116632337385101204055232891361405428635972040596205450316747012080794838691280547894128\
      246741601088755087359234554141346980837292342320288111397175220296098629890108459305643419353\
      36"
    );

        let reduced_ground_truth = construct_raw_elem_from_strings(
      "26888935961824081232597112540509824504614070059776273347136888921115497522070287009841688662\
      983066376019079593372296556420848446780369918809384119124783870290778875424468497961559643807\
      918398860928578027038014112641529893817109240852544158309292025321122680747989987560029531021\
      808743313150630063377037854944",
      "14529985196481999393995154363327100184407232892559561136140792409262328867440167480822808496\
      853924547751298342980606034124112579835255733824790020119078588372593288210628255956605240171\
      744703418426092073347584357826862813733154338737148962212641444735717023402201569115323580814\
      54099903972209626147819759991",
      "28467266502267127591420289007165819749231433586093061478772560429058231137856046130384492811\
      816456933286039468940950129263300933723839212086399375780796041634531383342902918719073416087\
      614456845205980227091403964285870107268917183244016635907926846271829374679124848388403486656\
      1564478239095738726823372184204"
    );

        let already_reduced = reduced_ground_truth.clone();
        assert_eq!(already_reduced, reduced_ground_truth);

        assert_ne!(to_reduce, reduced_ground_truth);
        ClassGroup::reduce(&mut to_reduce);
        assert_eq!(to_reduce, reduced_ground_truth);
    }

    #[test]
    fn test_normalize_basic() {
        let mut unnorm_a = Mpz::from_str("16").unwrap();
        let mut unnorm_b = Mpz::from_str("105").unwrap();
        let mut unnorm_c = Mpz::from_str(
      "4783760786688675616733383986925127377420761933775791859799529477781625005833111632534101811\
       0672047217112377476473502060121352842575308793237621563947157630098485131517401073775191194\
       3195315494838983347421441386016611204764255243332731221321519278338873239699989557133287835\
       2685419887133231339948938699768182757831793879217091871179468485931169743972659665650159413\
       8449739494228617068329664776714484742276158090583495714649193839084110987149118615158361352\
       4884884020388947996954204832727089332397513638493972875716927368810312231404469265224318597\
       017389945629057462766047140854869124473221137588347335081555186814207",
    )
    .unwrap();

        let norm_a = Mpz::from_str("16").unwrap();
        let norm_b = Mpz::from_str("9").unwrap();
        let norm_c = Mpz::from_str(
      "4783760786688675616733383986925127377420761933775791859799529477781625005833111632534101811\
       06720472171123774764735020601213528425753087932376215639471576300984851315174010737751911943\
       19531549483898334742144138601661120476425524333273122132151927833887323969998955713328783526\
       85419887133231339948938699768182757831793879217091871179468485931169743972659665650159413844\
       97394942286170683296647767144847422761580905834957146491938390841109871491186151583613524884\
       88402038894799695420483272708933239751363849397287571692736881031223140446926522431859701738\
       9945629057462766047140854869124473221137588347335081555186814036",
    )
    .unwrap();

        ClassGroup::normalize(&mut unnorm_a, &mut unnorm_b, &mut unnorm_c);
        assert_eq!((norm_a, norm_b, norm_c), (unnorm_a, unnorm_b, unnorm_c));
    }

    #[test]
    fn test_discriminant_across_ops() {
        let id = ClassGroup::id();
        let g1 = ClassGroup::unknown_order_elem();
        let g2 = ClassGroup::op(&g1, &g1);
        let g3 = ClassGroup::op(&id, &g2);
        let g3_inv = ClassGroup::inv(&g3);

        assert!(ClassGroup::validate(&id.a, &id.b, &id.c));
        assert!(ClassGroup::validate(&g1.a, &g1.b, &g1.c));
        assert!(ClassGroup::validate(&g2.a, &g2.b, &g2.c));
        assert!(ClassGroup::validate(&g3.a, &g3.b, &g3.c));
        assert!(ClassGroup::validate(&g3_inv.a, &g3_inv.b, &g3_inv.c));
    }

    #[test]
    fn test_op_single() {
        let a = construct_raw_elem_from_strings(
      "4",
      "1",
      "19135043146754702466933535947700509509683047735103167439198117911126500023332446530136407244\
      268818886844950990589400824048541137030123517295048625578863052039394052606960429510076477727\
      812619793559333896857655440664448190570209733309248852860771133554929587999582285331513410741\
      679548532925359795754799072731031327175516868367484717873943724678975890638662600637655379895\
      797691446827331865910685793896910463236233398285859677535633644394859647446063344540995395360\
      815557919878168193309083573295900545539758915028677094752412489256178770608972743880695597825\
      16229851064188563419476497892884550353389340326220747256139"
    );

        let b = construct_raw_elem_from_strings(
      "16",
      "41",
      "47837607866886756167333839869251273774207619337757918597995294777816250058331116325341018110\
      672047217112377476473502060121352842575308793237621563947157630098485131517401073775191194319\
      531549483898334742144138601661120476425524333273122132151927833887323969998955713328783526854\
      198871332313399489386997681827578317938792170918711794684859311697439726596656501594138449739\
      494228617068329664776714484742276158090583495714649193839084110987149118615158361352488488402\
      038894799695420483272708933239751363849397287571692736881031223140446926522431859701738994562\
      9057462766047140854869124473221137588347335081555186814061"
    );

        let ground_truth = construct_raw_elem_from_strings(
      "64",
      "9",
      "11959401966721689041833459967312818443551904834439479649498823694454062514582779081335254527\
      668011804278094369118375515030338210643827198309405390986789407524621282879350268443797798579\
      882887370974583685536034650415280119106381083318280533037981958471830992499738928332195881713\
      549717833078349872346749420456894579484698042729677948671214827924359931649164125398534612434\
      873557154267082416194178621185569039522645873928662298459771027746787279653789590338122122100\
      50972369992385512081817723330993784096234932189292318422025780578511173163060796492543474864\
      07264365691511785213717281118305284397086833770388796703509"
    );

        assert_eq!(ClassGroup::op(&a, &b), ground_truth);
    }

    #[test]
    fn test_op_alternating() {
        let g_anchor = ClassGroup::unknown_order_elem();
        let mut g = ClassGroup::id();
        let mut g_star = ClassGroup::id();

        // g
        g = ClassGroup::op(&g_anchor, &g);

        // g^2, g^* = g^2
        g = ClassGroup::op(&g_anchor, &g);
        g_star = ClassGroup::op(&g, &g_star);

        // g^3
        g = ClassGroup::op(&g_anchor, &g);

        // g^4, g^* = g^2 * g^4 = g^6
        g = ClassGroup::op(&g_anchor, &g);
        g_star = ClassGroup::op(&g, &g_star);

        let ground_truth = construct_raw_elem_from_strings(
      "64",
      "9",
      "11959401966721689041833459967312818443551904834439479649498823694454062514582779081335254527\
      668011804278094369118375515030338210643827198309405390986789407524621282879350268443797798579\
      882887370974583685536034650415280119106381083318280533037981958471830992499738928332195881713\
      549717833078349872346749420456894579484698042729677948671214827924359931649164125398534612434\
      873557154267082416194178621185569039522645873928662298459771027746787279653789590338122122100\
      509723699923855120818177233309937840962349321892923184220257805785111731630607964925434748640\
      7264365691511785213717281118305284397086833770388796703509"
    );

        assert_eq!(ground_truth, g_star);
    }

    #[test]
    fn test_op_complex() {
        // 1. Take g^100, g^200, ..., g^1000.
        // 2. Compute g^* = g^100 * ... * g^1000.
        // 3. For each of g^100, g^200, ..., g^1000 compute the inverse of that element and assert that
        //    g^* * current_inverse = product of g^100, g^200, ..., g^1000 without the inversed-out
        //    element.
        let g_anchor = ClassGroup::unknown_order_elem();
        let mut g = ClassGroup::id();

        let mut gs = vec![];
        let mut gs_invs = vec![];

        let mut g_star = ClassGroup::id();
        for i in 1..=1000 {
            g = ClassGroup::op(&g_anchor, &g);
            assert!(ClassGroup::validate(&g.a, &g.b, &g.c));
            if i % 100 == 0 {
                gs.push(g.clone());
                gs_invs.push(ClassGroup::inv(&g));
                g_star = ClassGroup::op(&g, &g_star);
                assert!(ClassGroup::validate(&g.a, &g.b, &g.c));
            }
        }

        let elems_n_invs = gs.iter().zip(gs_invs.iter());
        for (g_elem, g_inv) in elems_n_invs {
            assert!(ClassGroup::validate(&g_elem.a, &g_elem.b, &g_elem.c));
            assert!(ClassGroup::validate(&g_inv.a, &g_inv.b, &g_inv.c));
            let mut curr_prod = ClassGroup::id();
            for elem in &gs {
                if elem != g_elem {
                    curr_prod = ClassGroup::op(&curr_prod, &elem);
                    assert!(ClassGroup::validate(
                        &curr_prod.a,
                        &curr_prod.b,
                        &curr_prod.c
                    ));
                }
            }
            assert_eq!(ClassGroup::id(), ClassGroup::op(&g_inv, &g_elem));
            assert_eq!(curr_prod, ClassGroup::op(&g_inv, &g_star));
        }
    }

    #[test]
    fn test_id_basic() {
        let g = ClassGroup::unknown_order_elem();
        let id = ClassGroup::id();
        assert_eq!(g, ClassGroup::op(&g, &id));
        assert_eq!(g, ClassGroup::op(&id, &g));
        assert_eq!(id, ClassGroup::op(&id, &id));
    }

    #[test]
    fn test_id_repeated() {
        let mut id = ClassGroup::id();
        let g_anchor = ClassGroup::unknown_order_elem();
        let mut g = ClassGroup::unknown_order_elem();
        for _ in 0..1000 {
            id = ClassGroup::op(&id, &id);
            assert_eq!(id, ClassGroup::id());
            g = ClassGroup::op(&g, &ClassGroup::id());
            assert_eq!(g, g_anchor);
        }
    }

    #[test]
    fn test_inv() {
        let id = ClassGroup::id();
        let g_anchor = ClassGroup::unknown_order_elem();
        let mut g = ClassGroup::unknown_order_elem();

        for _ in 0..1000 {
            g = ClassGroup::op(&g, &g_anchor);
            let g_inv = ClassGroup::inv(&g);
            assert_eq!(id, ClassGroup::op(&g_inv, &g));
            assert_eq!(id, ClassGroup::op(&g, &g_inv));
            assert_eq!(g, ClassGroup::inv(&g_inv));
        }
    }

    #[test]
    fn test_exp_basic() {
        let g_anchor = ClassGroup::unknown_order_elem();
        let mut g = ClassGroup::id();

        for i in 1..=1000 {
            g = ClassGroup::op(&g, &g_anchor);
            assert_eq!(&g, &ClassGroup::pow(&g_anchor, &Integer::from(i)));
        }
    }

    #[test]
    fn test_square_basic() {
        let g = ClassGroup::unknown_order_elem();
        let mut g4 = ClassGroup::id();

        // g^4
        for _ in 0..4 {
            g4 = ClassGroup::op(&g, &g4);
        }

        // g^2
        let mut g2 = g.clone();
        // g^4
        ClassGroup::square(&mut g2);
        ClassGroup::square(&mut g2);

        assert_eq!(&g2, &g4);
    }

    #[test]
    fn test_square_repeated() {
        let mut g = ClassGroup::unknown_order_elem();
        let g_ = g.clone();

        for i in 0..12 {
            ClassGroup::square(&mut g);
            let mut base = ClassGroup::id();

            for _ in 0..(2i32.pow(i + 1)) {
                base = ClassGroup::op(&g_, &base);
            }

            assert_eq!(g, base);
        }
    }

    fn split_into_three_pieces(line: &str, c: char) -> [&str; 3] {
        let mut iter = line.split(c);
        let fst = iter.next().expect("bad test file");
        let snd = iter.next().expect("bad test file");
        let thd = iter.next().expect("bad test file");
        assert!(iter.next().is_none(), "bad test file");
        [fst, snd, thd]
    }

    // #[test]
    // fn multiplication_is_correct_test_file() {
    //     let manifest_path =
    //         std::env::var("CARGO_MANIFEST_DIR").expect("cargo should have set this");

    //     let mut path = PathBuf::from(&manifest_path);
    //     path.push("tests/multiply.txt");

    //     let mut f = BufReader::new(File::open(path).expect("test file missing or unreadable"));
    //     let mut buffer = String::new();

    //     loop {
    //         let bytes_read = f
    //             .read_line(&mut buffer)
    //             .expect("could not read from test file");

    //         assert!(bytes_read == buffer.len());

    //         if bytes_read == 0 {
    //             break;
    //         }

    //         if buffer.ends_with('\n') {
    //             buffer.pop();
    //         }

    //         if buffer.ends_with('\r') {
    //             buffer.pop();
    //         }

    //         let mut current_discriminant: Option<Integer> = None;

    //         let q: Vec<_> = split_into_three_pieces(&buffer, '|')
    //             .iter()
    //             .map(|i| {
    //                 let k = split_into_three_pieces(i, ',');

    //                 let a = Integer::from_str(k[0]).expect("bad test file");
    //                 let b = Integer::from_str(k[1]).expect("bad test file");
    //                 let c = Integer::from_str(k[2]).expect("bad test file");

    //                 //b^2 - 4ac
    //                 let mut discriminant: Integer = Integer::default();
    //                 discriminant.mul_mut(&b);
    //                 discriminant.mul_mut(&b);

    //                 let mut minuand: Integer = (4u64).into();
    //                 // minuand *= &a * &c;
    //                 minuand.mul_mut(&a);
    //                 minuand.mul_mut(&c);
    //                 //discriminant -= &minuand;
    //                 discriminant.sub_mut(&minuand);
    //                 assert!(discriminant < Integer::zero());

    //                 if let Some(ref q) = current_discriminant {
    //                     assert_eq!(q, &discriminant, "mismatching discriminant in test files");
    //                 } else {
    //                     current_discriminant = Some(discriminant.clone());
    //                 }

    //                 Group::from_ab_discriminant(a, b, discriminant)
    //             })
    //             .collect();

    //         assert_eq!(q.len(), 3, "len is not valid");

    //         if q[0] == q[1] {
    //             let mut i = q[0].clone();
    //             Group::square(&mut i);
    //             assert_eq!(i, q[2], "mismatching square to multiplication");
    //         }

    //         assert_eq!(Group::op(&q[1], &q[0]), q[2], "multiplication not valid");
    //         assert_eq!(Group::op(&q[0], &q[1]) , q[2], "multiplication not valid");

    //         buffer.clear();
    //     }
    // }
}
