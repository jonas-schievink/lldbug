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
use core::marker::PhantomData;
use crate::{c, error};
use untrusted;

pub use self::elem::*;
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

pub struct Point {
    // The coordinates are stored in a contiguous array, where the first
    // `ops.num_limbs` elements are the X coordinate, the next
    // `ops.num_limbs` elements are the Y coordinate, and the next
    // `ops.num_limbs` elements are the Z coordinate. This layout is dictated
    // by the requirements of the GFp_nistz256 code.
    xyz: [Limb; 3 * MAX_LIMBS],
}

impl Point {
    pub fn new_at_infinity() -> Point {
        Point {
            xyz: [0; 3 * MAX_LIMBS],
        }
    }
}

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

static ONE: Elem<Unencoded> = Elem {
    limbs: limbs![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    m: PhantomData,
    encoding: PhantomData,
};

/// Operations and values needed by all curve operations.
pub struct CommonOps {
    pub num_limbs: usize,
    q: Modulus,
    pub n: Elem<Unencoded>,

    pub a: Elem<R>, // Must be -3 mod q
    pub b: Elem<R>,

    // In all cases, `r`, `a`, and `b` may all alias each other.
    elem_add_impl: unsafe extern "C" fn(r: *mut Limb, a: *const Limb, b: *const Limb),
    elem_mul_mont: unsafe extern "C" fn(r: *mut Limb, a: *const Limb, b: *const Limb),
    elem_sqr_mont: unsafe extern "C" fn(r: *mut Limb, a: *const Limb),

    point_add_jacobian_impl: unsafe extern "C" fn(r: *mut Limb, a: *const Limb, b: *const Limb),
}

impl CommonOps {
    #[inline]
    pub fn elem_add<E: Encoding>(&self, a: &mut Elem<E>, b: &Elem<E>) {
        binary_op_assign(self.elem_add_impl, a, b)
    }

    pub fn elems_are_equal(&self, a: &Elem<R>, b: &Elem<R>) -> bool {
        for i in 0..self.num_limbs {
            if a.limbs[i] != b.limbs[i] {
                return false;
            }
        }
        true
    }

    #[inline]
    pub fn elem_unencoded(&self, a: &Elem<R>) -> Elem<Unencoded> { self.elem_product(a, &ONE) }

    #[inline]
    pub fn elem_mul(&self, a: &mut Elem<R>, b: &Elem<R>) {
        binary_op_assign(self.elem_mul_mont, a, b)
    }

    #[inline]
    pub fn elem_product<EA: Encoding, EB: Encoding>(
        &self, a: &Elem<EA>, b: &Elem<EB>,
    ) -> Elem<<(EA, EB) as ProductEncoding>::Output>
    where
        (EA, EB): ProductEncoding,
    {
        mul_mont(self.elem_mul_mont, a, b)
    }

    #[inline]
    pub fn elem_square(&self, a: &mut Elem<R>) { unary_op_assign(self.elem_sqr_mont, a); }

    #[inline]
    pub fn elem_squared(&self, a: &Elem<R>) -> Elem<R> { unary_op(self.elem_sqr_mont, a) }

    #[inline]
    pub fn is_zero<M, E: Encoding>(&self, a: &elem::Elem<M, E>) -> bool {
        limbs_are_zero_constant_time(&a.limbs[..self.num_limbs]) == LimbMask::True
    }

    pub fn elem_verify_is_not_zero(&self, a: &Elem<R>) -> Result<(), error::Unspecified> {
        if self.is_zero(a) {
            Err(error::Unspecified)
        } else {
            Ok(())
        }
    }

    pub fn point_sum(&self, a: &Point, b: &Point) -> Point {
        let mut r = Point::new_at_infinity();
        unsafe {
            (self.point_add_jacobian_impl)(r.xyz.as_mut_ptr(), a.xyz.as_ptr(), b.xyz.as_ptr())
        }
        r
    }

    pub fn point_x(&self, p: &Point) -> Elem<R> {
        let mut r = Elem::zero();
        r.limbs[..self.num_limbs].copy_from_slice(&p.xyz[0..self.num_limbs]);
        r
    }

    pub fn point_y(&self, p: &Point) -> Elem<R> {
        let mut r = Elem::zero();
        r.limbs[..self.num_limbs].copy_from_slice(&p.xyz[self.num_limbs..(2 * self.num_limbs)]);
        r
    }

    pub fn point_z(&self, p: &Point) -> Elem<R> {
        let mut r = Elem::zero();
        r.limbs[..self.num_limbs]
            .copy_from_slice(&p.xyz[(2 * self.num_limbs)..(3 * self.num_limbs)]);
        r
    }
}

struct Modulus {
    p: [Limb; MAX_LIMBS],
    rr: [Limb; MAX_LIMBS],
}

/// Operations on private keys, for ECDH and ECDSA signing.
pub struct PrivateKeyOps {
    pub common: &'static CommonOps,
    elem_inv_squared: fn(a: &Elem<R>) -> Elem<R>,
    point_mul_base_impl: fn(a: &Scalar) -> Point,
    point_mul_impl: unsafe extern "C" fn(
        r: *mut Limb,          // [3][num_limbs]
        p_scalar: *const Limb, // [num_limbs]
        p_x: *const Limb,      // [num_limbs]
        p_y: *const Limb,      // [num_limbs]
    ),
}

impl PrivateKeyOps {
    #[inline(always)]
    pub fn point_mul_base(&self, a: &Scalar) -> Point { (self.point_mul_base_impl)(a) }

    #[inline(always)]
    pub fn point_mul(&self, p_scalar: &Scalar, (p_x, p_y): &(Elem<R>, Elem<R>)) -> Point {
        let mut r = Point::new_at_infinity();
        unsafe {
            (self.point_mul_impl)(
                r.xyz.as_mut_ptr(),
                p_scalar.limbs.as_ptr(),
                p_x.limbs.as_ptr(),
                p_y.limbs.as_ptr(),
            );
        }
        r
    }

    #[inline]
    pub fn elem_inverse_squared(&self, a: &Elem<R>) -> Elem<R> { (self.elem_inv_squared)(a) }
}

/// Operations and values needed by all operations on public keys (ECDH
/// agreement and ECDSA verification).
pub struct PublicKeyOps {
    pub common: &'static CommonOps,
}

impl PublicKeyOps {
    // The serialized bytes are in big-endian order, zero-padded. The limbs
    // of `Elem` are in the native endianness, least significant limb to
    // most significant limb. Besides the parsing, conversion, this also
    // implements NIST SP 800-56A Step 2: "Verify that xQ and yQ are integers
    // in the interval [0, p-1] in the case that q is an odd prime p[.]"
    pub fn elem_parse(&self, input: &mut untrusted::Reader) -> Result<Elem<R>, error::Unspecified> {
        let encoded_value = input.skip_and_get_input(self.common.num_limbs * LIMB_BYTES)?;
        let parsed = elem_parse_big_endian_fixed_consttime(self.common, encoded_value)?;
        let mut r = Elem::zero();
        // Montgomery encode (elem_to_mont).
        // TODO: do something about this.
        unsafe {
            (self.common.elem_mul_mont)(
                r.limbs.as_mut_ptr(),
                parsed.limbs.as_ptr(),
                self.common.q.rr.as_ptr(),
            )
        }
        Ok(r)
    }
}

// Operations used by both ECDSA signing and ECDSA verification. In general
// these must be side-channel resistant.
pub struct ScalarOps {
    pub common: &'static CommonOps,

    scalar_inv_to_mont_impl: fn(a: &Scalar) -> Scalar<R>,
    scalar_mul_mont: unsafe extern "C" fn(r: *mut Limb, a: *const Limb, b: *const Limb),
}

impl ScalarOps {
    #[inline]
    fn scalar_product<EA: Encoding, EB: Encoding>(
        &self, a: &Scalar<EA>, b: &Scalar<EB>,
    ) -> Scalar<<(EA, EB) as ProductEncoding>::Output>
    where
        (EA, EB): ProductEncoding,
    {
        mul_mont(self.scalar_mul_mont, a, b)
    }
}

/// Operations on public scalars needed by ECDSA signature verification.
pub struct PublicScalarOps {
    pub scalar_ops: &'static ScalarOps,
    pub public_key_ops: &'static PublicKeyOps,

    // XXX: `PublicScalarOps` shouldn't depend on `PrivateKeyOps`, but it does
    // temporarily until `twin_mul` is rewritten.
    pub private_key_ops: &'static PrivateKeyOps,

    pub q_minus_n: Elem<Unencoded>,
}

#[allow(non_snake_case)]
pub struct PrivateScalarOps {
    pub scalar_ops: &'static ScalarOps,

    pub oneRR_mod_n: Scalar<RR>, // 1 * R**2 (mod n). TOOD: Use One<RR>.
}


// Returns (`a` squared `squarings` times) * `b`.
fn elem_sqr_mul(ops: &CommonOps, a: &Elem<R>, squarings: usize, b: &Elem<R>) -> Elem<R> {
    debug_assert!(squarings >= 1);
    let mut tmp = ops.elem_squared(a);
    for _ in 1..squarings {
        ops.elem_square(&mut tmp);
    }
    ops.elem_product(&tmp, b)
}

// Sets `acc` = (`acc` squared `squarings` times) * `b`.
fn elem_sqr_mul_acc(ops: &CommonOps, acc: &mut Elem<R>, squarings: usize, b: &Elem<R>) {
    debug_assert!(squarings >= 1);
    for _ in 0..squarings {
        ops.elem_square(acc);
    }
    ops.elem_mul(acc, b)
}

#[inline]
pub fn elem_parse_big_endian_fixed_consttime(
    ops: &CommonOps, bytes: untrusted::Input,
) -> Result<Elem<Unencoded>, error::Unspecified> {
    parse_big_endian_fixed_consttime(ops, bytes, AllowZero::Yes, &ops.q.p[..ops.num_limbs])
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

extern "C" {
    fn LIMBS_add_mod(
        r: *mut Limb, a: *const Limb, b: *const Limb, m: *const Limb, num_limbs: c::size_t,
    );
}

mod elem;
pub mod p384;
