extern crate rmp_serde;

fn main() {
	rmp_serde::from_slice::<()>(&[128])
		.unwrap();
	parse_big_endian_in_range_and_pad_consttime(&mut []);
}

fn parse_big_endian_in_range_and_pad_consttime(
    result: &mut [u64],
) {
    for r in &mut result[..] {
        *r = 0;
    }

    unsafe { LIMBS_are_zero(result.as_ptr()) };
}

extern "C" {
    fn LIMBS_are_zero(a: *const u64);
}
