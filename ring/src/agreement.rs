
use crate::{ec, error, rand};
use untrusted;

/// A key agreement algorithm.
#[derive(Eq, PartialEq)]
pub struct Algorithm {
    pub(crate) i: ec::AgreementAlgorithmImpl,
}

derive_debug_via_self!(Algorithm, self.i);
