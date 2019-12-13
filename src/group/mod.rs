//! This module contains implementations for different mathematical groups, each of which satisfies
//! our `UnknownOrderGroup` trait. They can be used with the accumulator and vector commitment
//! structures, or standalone if you have a custom application.
//!

use rug::Integer;
use std::fmt::Debug;
use std::hash::Hash;
use std::marker::Sized;

mod elem;
pub use elem::ClassElem;

mod class_ctx;
use class_ctx::ClassCtx;

mod lin_congruence_ctx;

mod create_discriminant;
pub use create_discriminant::create_discriminant;

mod discriminant;
pub use discriminant::CLASS_GROUP_DISCRIMINANT;

mod classy;
pub use classy::ClassGroup;

// pub fn multi_exp<G: Group>(alphas: &[G::Elem], x: &[Integer]) -> G::Elem {
//     if alphas.len() == 1 {
//         return alphas[0].clone();
//     }

//     let n_half = alphas.len() / 2;
//     let alpha_l = &alphas[..n_half];
//     let alpha_r = &alphas[n_half..];
//     let x_l = &x[..n_half];
//     let x_r = &x[n_half..];
//     let x_star_l = x_l.iter().product();
//     let x_star_r = x_r.iter().product();
//     let l = multi_exp::<G>(alpha_l, x_l);
//     let r = multi_exp::<G>(alpha_r, x_r);
//     G::op(&G::exp(&l, &x_star_r), &G::exp(&r, &x_star_l))
// }

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_multi_exp() {
    //   let alpha_1 = Rsa2048::elem(2);
    //   let alpha_2 = Rsa2048::elem(3);
    //   let x_1 = Integer::from(3);
    //   let x_2 =  Integer::from(2);
    //   let res = multi_exp::<Rsa2048>(
    //     &[alpha_1.clone(), alpha_2.clone()],
    //     &[x_1.clone(), x_2.clone()],
    //   );
    //   assert!(res == Rsa2048::elem(108));
    //   let alpha_3 = Rsa2048::elem(5);
    //   let x_3 = Integer::from(1);
    //   let res_2 = multi_exp::<Rsa2048>(&[alpha_1, alpha_2, alpha_3], &[x_1, x_2, x_3]);
    //   assert!(res_2 == Rsa2048::elem(1_687_500));
    // }
}
