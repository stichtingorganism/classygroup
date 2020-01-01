//! Reusable memory context for class groups.

use super::lin_congruence_ctx::LinCongruenceCtx;
use crate::group::CLASS_GROUP_DISCRIMINANT;
use crate::num::{partial, Mpz};

#[allow(clippy::type_complexity)]
pub struct OpCtx {
    pub inner: (
        Mpz,
        Mpz,
        Mpz,
        Mpz,
        Mpz,
        Mpz,
        Mpz,
        Mpz,
        Mpz,
        Mpz,
        Mpz,
        Mpz,
        Mpz,
        Mpz,
        Mpz,
        Mpz,
        Mpz,
    ),
}

impl Default for OpCtx {
    fn default() -> Self {
        Self {
            inner: (
                Mpz::default(),
                Mpz::default(),
                Mpz::default(),
                Mpz::default(),
                Mpz::default(),
                Mpz::default(),
                Mpz::default(),
                Mpz::default(),
                Mpz::default(),
                Mpz::default(),
                Mpz::default(),
                Mpz::default(),
                Mpz::default(),
                Mpz::default(),
                Mpz::default(),
                Mpz::default(),
                Mpz::default(),
            ),
        }
    }
}

#[allow(non_snake_case)]
#[allow(clippy::type_complexity)]
pub struct ClassCtx {
    pub L: Mpz,

    // Discrimenant
    pub D: Mpz,

    // Context for general class group ops implemented in mod.rs
    pub op_ctx: OpCtx,

    // Context that knows how to solve linear congruences.
    pub lin_cong_ctx: LinCongruenceCtx,

    // Context that handles partial extended GCD.
    pub partial_context: partial::PartialGCDContext,
}

impl ClassCtx {
    fn from_discriminant(disc: &Mpz) -> Self {
        let mut s = Self {
            L: Mpz::default(),
            D: disc.clone(),
            op_ctx: OpCtx::default(),
            lin_cong_ctx: LinCongruenceCtx::default(),
            partial_context: Default::default(),
        };

        // Precomputation needed for NUDULP.
        s.L.abs(disc);
        s.L.root_mut(4);
        s
    }
}

impl Default for ClassCtx {
    fn default() -> Self {
        let mut s = Self {
            L: Mpz::default(),
            D: CLASS_GROUP_DISCRIMINANT.clone(),
            op_ctx: OpCtx::default(),
            lin_cong_ctx: LinCongruenceCtx::default(),
            partial_context: Default::default(),
        };

        // Precomputation needed for NUDULP.
        s.L.abs(&CLASS_GROUP_DISCRIMINANT);
        s.L.root_mut(4);
        s
    }
}
