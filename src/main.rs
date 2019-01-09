extern crate failure;
extern crate ring;

use failure::Error;

fn main() {
	rmp_serde::from_slice::<()>(&[128])
		.map_err(Error::from)
		.unwrap();
}
