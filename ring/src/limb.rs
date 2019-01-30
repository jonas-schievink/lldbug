// Copyright 2016 David Judd.
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

//! Unsigned multi-precision integer arithmetic.
//!
//! Limbs ordered least-significant-limb to most-significant-limb. The bits
//! limbs use the native endianness.

use crate::{c, error, untrusted};

// XXX: Not correct for x32 ABIs.
#[cfg(target_pointer_width = "64")]
pub type Limb = u64;
#[cfg(target_pointer_width = "32")]
pub type Limb = u32;
#[cfg(target_pointer_width = "64")]
pub const LIMB_BITS: usize = 64;
#[cfg(target_pointer_width = "32")]
pub const LIMB_BITS: usize = 32;

#[allow(trivial_numeric_casts)]
#[cfg(target_pointer_width = "64")]
#[derive(Debug, PartialEq)]
#[repr(u64)]
pub enum LimbMask {
    True = 0xffff_ffff_ffff_ffff,
    False = 0,
}

#[cfg(target_pointer_width = "32")]
#[derive(Debug, PartialEq)]
#[repr(u32)]
pub enum LimbMask {
    True = 0xffff_ffff,
    False = 0,
}

pub const LIMB_BYTES: usize = (LIMB_BITS + 7) / 8;

#[cfg(all(any(test, feature = "rsa_signing"), target_pointer_width = "64"))]
#[inline]
pub fn limbs_as_bytes(src: &[Limb]) -> &[u8] {
    use crate::polyfill;
    polyfill::slice::u64_as_u8(src)
}

#[cfg(all(any(test, feature = "rsa_signing"), target_pointer_width = "32"))]
#[inline]
pub fn limbs_as_bytes(src: &[Limb]) -> &[u8] {
    use crate::polyfill;
    polyfill::slice::u32_as_u8(src)
}

#[inline]
pub fn limbs_less_than_limbs_consttime(a: &[Limb], b: &[Limb]) -> LimbMask {
    assert_eq!(a.len(), b.len());
    unsafe { LIMBS_less_than(a.as_ptr(), b.as_ptr(), b.len()) }
}

#[inline]
pub fn limbs_are_zero_constant_time(limbs: &[Limb]) -> LimbMask {
    unsafe { LIMBS_are_zero(limbs.as_ptr(), limbs.len()) }
}

#[cfg(any(test, feature = "rsa_signing"))]
#[inline]
pub fn limbs_equal_limb_constant_time(a: &[Limb], b: Limb) -> LimbMask {
    unsafe { LIMBS_equal_limb(a.as_ptr(), b, a.len()) }
}

#[derive(Clone, Copy, PartialEq)]
pub enum AllowZero {
    No,
    Yes,
}

/// Parses `input` into `result`, verifies that the value is less than
/// `max_exclusive`, and pads `result` with zeros to its length. If `allow_zero`
/// is not `AllowZero::Yes`, zero values are rejected.
///
/// This attempts to be constant-time with respect to the actual value *only if*
/// the value is actually in range. In other words, this won't leak anything
/// about a valid value, but it might leak small amounts of information about an
/// invalid value (which constraint it failed).
pub fn parse_big_endian_in_range_and_pad_consttime(
    input: untrusted::Input, allow_zero: AllowZero, max_exclusive: &[Limb], result: &mut [Limb],
) -> Result<(), error::Unspecified> {
    parse_big_endian_and_pad_consttime(input, result)?;
    if limbs_less_than_limbs_consttime(&result, max_exclusive) != LimbMask::True {
        return Err(error::Unspecified);
    }
    if allow_zero != AllowZero::Yes {
        if limbs_are_zero_constant_time(&result) != LimbMask::False {
            return Err(error::Unspecified);
        }
    }
    Ok(())
}

/// Parses `input` into `result`, padding `result` with zeros to its length.
/// This attempts to be constant-time with respect to the value but not with
/// respect to the length; it is assumed that the length is public knowledge.
pub fn parse_big_endian_and_pad_consttime(
    input: untrusted::Input, result: &mut [Limb],
) -> Result<(), error::Unspecified> {
    if input.is_empty() {
        return Err(error::Unspecified);
    }

    // `bytes_in_current_limb` is the number of bytes in the current limb.
    // It will be `LIMB_BYTES` for all limbs except maybe the highest-order
    // limb.
    let mut bytes_in_current_limb = input.len() % LIMB_BYTES;
    if bytes_in_current_limb == 0 {
        bytes_in_current_limb = LIMB_BYTES;
    }

    let num_encoded_limbs = (input.len() / LIMB_BYTES)
        + (if bytes_in_current_limb == LIMB_BYTES {
            0
        } else {
            1
        });
    if num_encoded_limbs > result.len() {
        return Err(error::Unspecified);
    }

    for r in &mut result[..] {
        *r = 0;
    }

    // XXX: Questionable as far as constant-timedness is concerned.
    // TODO: Improve this.
    input.read_all(error::Unspecified, |input| {
        for i in 0..num_encoded_limbs {
            let mut limb: Limb = 0;
            for _ in 0..bytes_in_current_limb {
                let b = input.read_byte()?;
                limb = (limb << 8) | (b as Limb);
            }
            result[num_encoded_limbs - i - 1] = limb;
            bytes_in_current_limb = LIMB_BYTES;
        }
        Ok(())
    })
}

extern "C" {
    fn LIMBS_are_zero(a: *const Limb, num_limbs: c::size_t) -> LimbMask;
    #[cfg(any(test, feature = "rsa_signing"))]
    fn LIMBS_equal_limb(a: *const Limb, b: Limb, num_limbs: c::size_t) -> LimbMask;
    fn LIMBS_less_than(a: *const Limb, b: *const Limb, num_limbs: c::size_t) -> LimbMask;
}
