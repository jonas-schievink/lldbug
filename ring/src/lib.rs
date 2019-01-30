#![no_std]

extern crate libc;
extern crate lazy_static;
extern crate untrusted;

mod c;

mod error;

pub mod limb;

mod private {
    /// Traits that are designed to only be implemented internally in *ring*.
    //
    // Usage:
    // ```
    // use crate::private;
    //
    // pub trait MyType: private::Sealed {
    //     // [...]
    // }
    //
    // impl private::Sealed for MyType {}
    // ```
    pub trait Sealed {}
}
