use crate::{c, error, untrusted};

pub type Limb = u64;
pub const LIMB_BITS: usize = 64;

#[derive(Debug, PartialEq)]
#[repr(u64)]
enum LimbMask {
    True = 0xffff_ffff_ffff_ffff,
    False = 0,
}

pub const LIMB_BYTES: usize = (LIMB_BITS + 7) / 8;

#[inline]
fn limbs_less_than_limbs_consttime(a: &[Limb], b: &[Limb]) -> LimbMask {
    unsafe { LIMBS_less_than(a.as_ptr(), b.as_ptr(), b.len()) }
}

#[inline]
fn limbs_are_zero_constant_time(limbs: &[Limb]) -> LimbMask {
    unsafe { LIMBS_are_zero(limbs.as_ptr(), limbs.len()) }
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

    limbs_less_than_limbs_consttime(&result, max_exclusive);
    limbs_are_zero_constant_time(&result);
    Ok(())
}

fn parse_big_endian_and_pad_consttime(
    input: untrusted::Input, result: &mut [Limb],
) -> Result<(), error::Unspecified> {
    for r in &mut result[..] {
        *r = 0;
    }

    Ok(())
}

extern "C" {
    fn LIMBS_are_zero(a: *const Limb, num_limbs: c::size_t) -> LimbMask;
    fn LIMBS_less_than(a: *const Limb, b: *const Limb, num_limbs: c::size_t) -> LimbMask;
}
