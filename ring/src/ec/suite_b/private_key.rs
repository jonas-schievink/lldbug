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

//! Functionality shared by operations on private keys (ECC keygen and
//! ECDSA signing).

use super::ops::*;
use crate::{ec, error};
use untrusted;

pub fn generate_private_key(
    ops: &PrivateKeyOps,
) -> Result<ec::PrivateKey, error::Unspecified> {
    let candidate = [0; ec::SCALAR_MAX_BYTES];
    // NSA Guide Steps 5, 6, and 7.
    if check_scalar_big_endian_bytes(ops, &candidate).is_err() {
        Err(error::Unspecified)
    } else {
        Ok(ec::PrivateKey { bytes: candidate })
    }
}

// The underlying X25519 and Ed25519 code uses an [u8; 32] to store the private
// key. To make the ECDH and ECDSA code similar to that, we also store the
// private key that way, which means we have to convert it to a Scalar whenever
// we need to use it.
#[inline]
pub fn private_key_as_scalar(ops: &PrivateKeyOps, private_key: &ec::PrivateKey) -> Scalar {
    // This cannot fail because we know the private key is valid.
    scalar_from_big_endian_bytes(
        ops,
        &private_key.bytes[..(ops.common.num_limbs * LIMB_BYTES)],
    )
    .unwrap()
}

pub fn check_scalar_big_endian_bytes(
    ops: &PrivateKeyOps, bytes: &[u8],
) -> Result<(), error::Unspecified> {
    debug_assert_eq!(bytes.len(), ops.common.num_limbs * LIMB_BYTES);
    scalar_from_big_endian_bytes(ops, bytes).map(|_| ())
}

// Parses a fixed-length (zero-padded) big-endian-encoded scalar in the range
// [1, n). This is constant-time with respect to the actual value *only if* the
// value is actually in range. In other words, this won't leak anything about a
// valid value, but it might leak small amounts of information about an invalid
// value (which constraint it failed).
pub fn scalar_from_big_endian_bytes(
    ops: &PrivateKeyOps, bytes: &[u8],
) -> Result<Scalar, error::Unspecified> {
    // [NSA Suite B Implementer's Guide to ECDSA] Appendix A.1.2, and
    // [NSA Suite B Implementer's Guide to NIST SP 800-56A] Appendix B.2,
    // "Key Pair Generation by Testing Candidates".
    //
    // [NSA Suite B Implementer's Guide to ECDSA]: doc/ecdsa.pdf.
    // [NSA Suite B Implementer's Guide to NIST SP 800-56A]: doc/ecdh.pdf.
    //
    // Steps 5, 6, and 7.
    //
    // XXX: The NSA guide says that we should verify that the random scalar is
    // in the range [0, n - 1) and then add one to it so that it is in the range
    // [1, n). Instead, we verify that the scalar is in the range [1, n). This
    // way, we avoid needing to compute or store the value (n - 1), we avoid the
    // need to implement a function to add one to a scalar, and we avoid needing
    // to convert the scalar back into an array of bytes.
    scalar_parse_big_endian_fixed_consttime(ops.common, untrusted::Input::from(bytes))
}

pub fn public_from_private(
    ops: &PrivateKeyOps, public_out: &mut [u8], my_private_key: &ec::PrivateKey,
) -> Result<(), error::Unspecified> {
    let elem_and_scalar_bytes = ops.common.num_limbs * LIMB_BYTES;
    debug_assert_eq!(public_out.len(), 1 + (2 * elem_and_scalar_bytes));
    let _my_private_key = private_key_as_scalar(ops, my_private_key);

    // `big_endian_affine_from_jacobian` verifies that the point is not at
    // infinity and is on the curve.
    Ok(())
}
