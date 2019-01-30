type Limb = u64;

/// Parses `input` into `result`, verifies that the value is less than
/// `max_exclusive`, and pads `result` with zeros to its length. If `allow_zero`
/// is not `AllowZero::Yes`, zero values are rejected.
///
/// This attempts to be constant-time with respect to the actual value *only if*
/// the value is actually in range. In other words, this won't leak anything
/// about a valid value, but it might leak small amounts of information about an
/// invalid value (which constraint it failed).
pub fn parse_big_endian_in_range_and_pad_consttime(
    result: &mut [Limb],
) {
    for r in &mut result[..] {
        *r = 0;
    }

    unsafe { LIMBS_are_zero(result.as_ptr()) };
}

extern "C" {
    fn LIMBS_are_zero(a: *const Limb);
}
