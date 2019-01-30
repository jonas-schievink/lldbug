// Copyright 2015-2017 Brian Smith.
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

use crate::{ec, error};


pub struct Curve {
    // Precondition: `bytes` is the correct length.
    pub check_private_key_bytes: fn(bytes: &[u8]) -> Result<(), error::Unspecified>,

    pub generate_private_key: fn() -> Result<ec::PrivateKey, error::Unspecified>,

    pub public_from_private:
        fn(public_out: &mut [u8], private_key: &ec::PrivateKey) -> Result<(), error::Unspecified>,
}

pub static P384: Curve = Curve {
    check_private_key_bytes: check_private_key_bytes,
    generate_private_key: generate_private_key,
    public_from_private: public_from_private,
};

fn check_private_key_bytes(bytes: &[u8]) -> Result<(), error::Unspecified> {
    ec::suite_b::private_key::check_scalar_big_endian_bytes(&ec::suite_b::ops::p384::PRIVATE_KEY_OPS, bytes)
}

fn generate_private_key() -> Result<ec::PrivateKey, error::Unspecified> {
    ec::suite_b::private_key::generate_private_key(&ec::suite_b::ops::p384::PRIVATE_KEY_OPS)
}

fn public_from_private(
    public_out: &mut [u8], private_key: &ec::PrivateKey,
) -> Result<(), error::Unspecified> {
    ec::suite_b::private_key::public_from_private(&ec::suite_b::ops::p384::PRIVATE_KEY_OPS, public_out, private_key)
}

/*

Current theory: As long as there's still a call to

scalar_from_big_endian_bytes

the bug reproduces

*/
