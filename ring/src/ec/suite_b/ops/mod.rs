// Copyright 2016 Brian Smith.
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHORS DISCLAIM ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHORS BE LIABLE FOR ANY
// SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION
// OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF OR IN
// CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.

use arithmetic::montgomery::*;
use crate::error;
use untrusted;

pub use limb::*; // XXX // XXX

/// A field element, i.e. an element of ℤ/qℤ for the curve's field modulus
/// *q*.
pub type Elem<E> = elem::Elem<Q, E>;

/// Represents the (prime) order *q* of the curve's prime field.
#[derive(Clone, Copy)]
pub enum Q {}

pub type Scalar<E = Unencoded> = elem::Elem<N, E>;

/// Represents the prime order *n* of the curve's group.
#[derive(Clone, Copy)]
pub enum N {}


#[cfg(all(target_pointer_width = "32", target_endian = "little"))]
macro_rules! limbs {
    ( $limb_b:expr, $limb_a:expr, $limb_9:expr, $limb_8:expr,
      $limb_7:expr, $limb_6:expr, $limb_5:expr, $limb_4:expr,
      $limb_3:expr, $limb_2:expr, $limb_1:expr, $limb_0:expr ) => {
        [
            $limb_0, $limb_1, $limb_2, $limb_3, $limb_4, $limb_5, $limb_6, $limb_7, $limb_8,
            $limb_9, $limb_a, $limb_b,
        ]
    };
}

#[cfg(all(target_pointer_width = "64", target_endian = "little"))]
macro_rules! limbs {
    ( $limb_b:expr, $limb_a:expr, $limb_9:expr, $limb_8:expr,
      $limb_7:expr, $limb_6:expr, $limb_5:expr, $limb_4:expr,
      $limb_3:expr, $limb_2:expr, $limb_1:expr, $limb_0:expr ) => {
        [
            (($limb_1 | 0u64) << 32) | $limb_0,
            (($limb_3 | 0u64) << 32) | $limb_2,
            (($limb_5 | 0u64) << 32) | $limb_4,
            (($limb_7 | 0u64) << 32) | $limb_6,
            (($limb_9 | 0u64) << 32) | $limb_8,
            (($limb_b | 0u64) << 32) | $limb_a,
        ]
    };
}

/// Operations and values needed by all curve operations.
pub struct CommonOps {
    pub num_limbs: usize,
    n: Elem<Unencoded>,
}

/// Operations on private keys, for ECDH and ECDSA signing.
pub struct PrivateKeyOps {
    pub common: &'static CommonOps,
}

#[inline]
pub fn scalar_parse_big_endian_fixed_consttime(
    ops: &CommonOps, bytes: untrusted::Input,
) -> Result<Scalar, error::Unspecified> {
    parse_big_endian_fixed_consttime(ops, bytes, AllowZero::No, &ops.n.limbs[..ops.num_limbs])
}

fn parse_big_endian_fixed_consttime<M>(
    ops: &CommonOps, bytes: untrusted::Input, allow_zero: AllowZero, max_exclusive: &[Limb],
) -> Result<elem::Elem<M, Unencoded>, error::Unspecified> {
    if bytes.len() != ops.num_limbs * LIMB_BYTES {
        return Err(error::Unspecified);
    }
    let mut r = elem::Elem::zero();
    parse_big_endian_in_range_and_pad_consttime(
        bytes,
        allow_zero,
        max_exclusive,
        &mut r.limbs[..ops.num_limbs],
    )?;
    Ok(r)
}

mod elem;
pub mod p384;
