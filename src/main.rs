extern crate ring;
extern crate rmp_serde;

fn main() {
	rmp_serde::from_slice::<()>(&[128])
		.unwrap();
}
